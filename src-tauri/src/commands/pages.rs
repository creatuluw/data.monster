use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;

#[tauri::command]
pub fn list_pages(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    eprintln!("[pages] Listing");
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT slug, title, CAST(created_at AS VARCHAR), CAST(updated_at AS VARCHAR) FROM d8a_monster_pages ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let pages: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let slug: String = row.get(0)?;
            let title: String = row.get(1)?;
            let created_at: Option<String> = row.get(2)?;
            let updated_at: Option<String> = row.get(3)?;
            Ok(json!({
                "slug": slug,
                "title": title,
                "createdAt": created_at,
                "updatedAt": updated_at
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "pages": pages }))
}

#[tauri::command]
pub fn get_page(slug: String, state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    eprintln!("[pages] Getting '{}'", slug);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.query_row(
        "SELECT slug, title, spec, CAST(created_at AS VARCHAR), CAST(updated_at AS VARCHAR) FROM d8a_monster_pages WHERE slug = ?",
        duckdb::params![&slug],
        |row| {
            let slug: String = row.get(0)?;
            let title: String = row.get(1)?;
            let spec: String = row.get(2)?;
            let created_at: Option<String> = row.get(3)?;
            let updated_at: Option<String> = row.get(4)?;
            Ok(json!({
                "slug": slug,
                "title": title,
                "spec": spec,
                "createdAt": created_at,
                "updatedAt": updated_at
            }))
        },
    )
    .map_err(|_| "Page not found".to_string())
}

#[tauri::command]
pub fn save_page(
    slug: String,
    title: String,
    spec: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    eprintln!("[pages] Saving '{}'", slug);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "INSERT OR REPLACE INTO d8a_monster_pages (slug, title, spec, created_at, updated_at)
         VALUES (?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_pages WHERE slug = ?), CURRENT_TIMESTAMP), CURRENT_TIMESTAMP)",
        duckdb::params![&slug, &title, &spec, &slug],
    )
    .map_err(|e| format!("Failed to save page: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_page(slug: String, state: State<'_, DuckDbState>) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "DELETE FROM d8a_monster_pages WHERE slug = ?",
        duckdb::params![&slug],
    )
    .map_err(|e| format!("Failed to delete page: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn
    }

    #[test]
    fn test_upsert_preserves_created_at() {
        let conn = setup();
        conn.execute(
            "INSERT INTO d8a_monster_pages (slug, title, spec, created_at, updated_at)
             VALUES (?, ?, ?, TIMESTAMP '2020-01-01 00:00:00', TIMESTAMP '2020-01-01 00:00:00')",
            duckdb::params!["sales", "Sales", "{}"],
        ).unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO d8a_monster_pages (slug, title, spec, created_at, updated_at)
             VALUES (?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_pages WHERE slug = ?), CURRENT_TIMESTAMP), CURRENT_TIMESTAMP)",
            duckdb::params!["sales", "Sales v2", "{\"rows\":1}", "sales"],
        ).unwrap();

        let (title, created_at): (String, String) = conn.query_row(
            "SELECT title, CAST(created_at AS VARCHAR) FROM d8a_monster_pages WHERE slug = ?",
            duckdb::params!["sales"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        assert_eq!(title, "Sales v2");
        assert!(created_at.starts_with("2020-01-01"), "created_at should be preserved, got {}", created_at);
    }
}
