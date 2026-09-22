//! One-time migration: export the `d8a_monster_*` content tables to `dm/` files, then
//! drop the tables (workspace-file-first FR-6). Files are canonical from here on.
//!
//! Crash-safe ordering: write ALL missing files first, verify each expected path
//! exists, and only then drop tables. A crash at any point leaves tables intact and
//! already-written files in place — the next run skips existing files (files win) and
//! finishes the job. Idempotent: no tables = no-op.
//!
//! Also hosts the workspace git bootstrap: generate `.gitignore` when missing
//! (never overwrite), per the secrets rule.

use duckdb::Connection;
use std::fs;
use std::path::Path;

use crate::commands::{connections, dm_store, items, pages, relationships, saved_queries};

fn table_exists(conn: &Connection, name: &str) -> Result<bool, String> {
    let n: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = ?",
            duckdb::params![name],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(n > 0)
}

pub(crate) fn migrate_content_tables(conn: &Connection, ws: &Path) -> Result<(), String> {
    // --- pages --------------------------------------------------------------
    if table_exists(conn, "d8a_monster_pages")? {
        let mut stmt = conn
            .prepare("SELECT slug, spec FROM d8a_monster_pages")
            .map_err(|e| e.to_string())?;
        let rows: Vec<(String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        let mut expected = Vec::new();
        for (slug, spec) in rows {
            let path = dm_store::page_path(ws, &slug)?;
            if !path.exists() {
                pages::save_page_in(ws, &slug, &spec)?;
            }
            expected.push(path);
        }
        verify(&expected)?;
        conn.execute("DROP TABLE d8a_monster_pages", [])
            .map_err(|e| format!("drop d8a_monster_pages: {e}"))?;
        eprintln!("[migration] d8a_monster_pages → dm/pages/");
    }

    // --- master items ---------------------------------------------------------
    if table_exists(conn, "d8a_monster_items")? {
        let mut stmt = conn
            .prepare("SELECT id, kind, table_name, expr, label, fmt, description FROM d8a_monster_items")
            .map_err(|e| e.to_string())?;
        let rows: Vec<(String, String, String, String, String, Option<String>, Option<String>)> = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        let mut expected = Vec::new();
        for (id, kind, table_name, expr, label, fmt, description) in rows {
            let path = dm_store::item_path(ws, &kind, &id)?;
            if !path.exists() {
                items::save_item_in(ws, &id, &kind, &table_name, &expr, &label, fmt.as_deref(), description.as_deref())?;
            }
            expected.push(path);
        }
        verify(&expected)?;
        conn.execute("DROP TABLE d8a_monster_items", [])
            .map_err(|e| format!("drop d8a_monster_items: {e}"))?;
        eprintln!("[migration] d8a_monster_items → dm/master-items/");
    }

    // --- relationships (one doc — files-win means skip entirely if it exists) ---
    if table_exists(conn, "d8a_monster_relationships")? {
        if !dm_store::relationships_path(ws).exists() {
            let mut stmt = conn
                .prepare("SELECT id, from_table, from_column, to_table, to_column FROM d8a_monster_relationships")
                .map_err(|e| e.to_string())?;
            let rows: Vec<(String, String, String, String, String)> = stmt
                .query_map([], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
                })
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();
            for (id, from_table, from_column, to_table, to_column) in rows {
                relationships::save_relationship_in(ws, &from_table, &from_column, &to_table, &to_column, Some(&id))?;
            }
            // empty graph still materializes its canonical doc (verify gate expects it)
            if !dm_store::relationships_path(ws).exists() {
                relationships::store_in(ws, &[])?;
            }
        }
        verify(&[dm_store::relationships_path(ws)])?;
        conn.execute("DROP TABLE d8a_monster_relationships", [])
            .map_err(|e| format!("drop d8a_monster_relationships: {e}"))?;
        eprintln!("[migration] d8a_monster_relationships → dm/relationships.json");
    }

    // --- saved queries ---------------------------------------------------------
    if table_exists(conn, "d8a_monster_saved_queries")? {
        let mut stmt = conn
            .prepare("SELECT slug, query_name, query_sql, description, tags FROM d8a_monster_saved_queries")
            .map_err(|e| e.to_string())?;
        let rows: Vec<(String, String, String, Option<String>, Option<String>)> = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        let mut expected = Vec::new();
        for (slug, query_name, query_sql, description, tags) in rows {
            let path = dm_store::saved_query_path(ws, &slug)?;
            if !path.exists() {
                saved_queries::write_query_file(ws, &slug, &query_name, &query_sql, description.as_deref(), tags.as_deref())?;
            }
            expected.push(path);
        }
        verify(&expected)?;
        conn.execute("DROP TABLE d8a_monster_saved_queries", [])
            .map_err(|e| format!("drop d8a_monster_saved_queries: {e}"))?;
        eprintln!("[migration] d8a_monster_saved_queries → dm/saved-queries/");
    }

    Ok(())
}

/// Crash-safety gate: every expected file must exist before we drop anything.
fn verify(expected: &[std::path::PathBuf]) -> Result<(), String> {
    for p in expected {
        if !p.exists() {
            return Err(format!("migration verification failed: {} missing", p.display()));
        }
    }
    Ok(())
}

/// Generate the workspace `.gitignore` when missing (never overwrite).
pub(crate) fn bootstrap_gitignore(ws: &Path) -> Result<(), String> {
    let path = ws.join(".gitignore");
    if path.exists() {
        return Ok(());
    }
    dm_store::atomic_write(&path, b".env\n*.duckdb\n*.duckdb.wal\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::database::initialize_schema;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("mig-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn seeded_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn.execute(
            "INSERT INTO d8a_monster_pages (slug, title, spec, created_at, updated_at)
             VALUES ('revenue', 'Revenue', '{\"slug\":\"revenue\",\"title\":\"Revenue\",\"rows\":[]}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO d8a_monster_items (id, kind, table_name, expr, label, fmt, description)
             VALUES ('total_revenue', 'measure', 'orders', 'sum(amount)', 'Total revenue', NULL, NULL)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO d8a_monster_relationships (id, from_table, from_column, to_table, to_column)
             VALUES ('rel-orders-customers', 'orders', 'customer_id', 'customers', 'id')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO d8a_monster_saved_queries (slug, query_name, query_sql, description, tags)
             VALUES ('my-query', 'My Query', 'SELECT 1', 'demo', 'sales, demo')",
            [],
        ).unwrap();
        conn
    }

    #[test]
    fn exports_all_content_then_drops_tables() {
        let ws = tmp_ws("full");
        let conn = seeded_conn();
        migrate_content_tables(&conn, &ws).unwrap();

        // store-level reads see everything (format consistency proven by reuse)
        let pages = pages::list_pages_in(&ws).unwrap();
        assert_eq!(pages["pages"].as_array().unwrap().len(), 1);
        assert_eq!(pages["pages"][0]["slug"].as_str().unwrap(), "revenue");

        let items = items::list_items_in(&ws, None).unwrap();
        assert_eq!(items["items"].as_array().unwrap().len(), 1);
        assert_eq!(items["items"][0]["id"].as_str().unwrap(), "total_revenue");

        let rels = relationships::list_relationships_in(&ws).unwrap();
        assert_eq!(rels["relationships"].as_array().unwrap().len(), 1);

        let queries = saved_queries::list_saved_queries_in(&ws).unwrap();
        let q = &queries["queries"].as_array().unwrap()[0];
        assert_eq!(q["slug"].as_str().unwrap(), "my-query");
        assert_eq!(q["name"].as_str().unwrap(), "My Query");
        assert_eq!(q["tags"].as_str().unwrap(), "sales, demo");

        for t in ["d8a_monster_pages", "d8a_monster_items", "d8a_monster_relationships", "d8a_monster_saved_queries"] {
            assert!(!table_exists(&conn, t).unwrap(), "{t} should be dropped");
        }
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn idempotent_second_run_is_noop() {
        let ws = tmp_ws("idempotent");
        let conn = seeded_conn();
        migrate_content_tables(&conn, &ws).unwrap();
        migrate_content_tables(&conn, &ws).unwrap(); // tables gone: no-op, still Ok
        assert_eq!(pages::list_pages_in(&ws).unwrap()["pages"].as_array().unwrap().len(), 1);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn files_win_over_table_rows() {
        let ws = tmp_ws("files-win");
        let conn = seeded_conn();
        // a NEWER file already exists (post-crash edit); the stale table row must NOT overwrite it
        pages::save_page_in(&ws, "revenue", "{\"title\":\"Edited post-crash\",\"rows\":[]}").unwrap();
        migrate_content_tables(&conn, &ws).unwrap();
        let got = pages::get_page_in(&ws, "revenue").unwrap();
        assert_eq!(got["title"].as_str().unwrap(), "Edited post-crash");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn partial_state_retry_completes_and_drops() {
        let ws = tmp_ws("retry");
        let conn = seeded_conn();
        // simulate a crash after exporting pages only (items/rels/queries still only in tables)
        pages::save_page_in(&ws, "revenue", "{\"slug\":\"revenue\",\"title\":\"Revenue\",\"rows\":[]}").unwrap();
        migrate_content_tables(&conn, &ws).unwrap();
        assert!(dm_store::page_path(&ws, "revenue").unwrap().exists());
        assert!(items::get_item_in(&ws, "total_revenue").is_ok());
        assert!(!table_exists(&conn, "d8a_monster_saved_queries").unwrap());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn no_tables_is_noop() {
        let ws = tmp_ws("noop");
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        // fresh schema HAS the tables but they are empty — export writes nothing, drops them
        migrate_content_tables(&conn, &ws).unwrap();
        assert!(!dm_store::page_path(&ws, "x").unwrap().exists());
        assert!(!table_exists(&conn, "d8a_monster_pages").unwrap());

        // a workspace conn with NO schema at all
        let conn2 = Connection::open_in_memory().unwrap();
        migrate_content_tables(&conn2, &ws).unwrap();
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn gitignore_written_when_missing_never_overwritten() {
        let ws = tmp_ws("gitignore");
        bootstrap_gitignore(&ws).unwrap();
        assert_eq!(fs::read_to_string(ws.join(".gitignore")).unwrap(), ".env\n*.duckdb\n*.duckdb.wal\n");
        fs::write(ws.join(".gitignore"), "my own rules\n").unwrap();
        bootstrap_gitignore(&ws).unwrap();
        assert_eq!(fs::read_to_string(ws.join(".gitignore")).unwrap(), "my own rules\n");
        let _ = fs::remove_dir_all(&ws);
    }
}
