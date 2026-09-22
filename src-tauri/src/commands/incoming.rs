//! `data/incoming/` drop folder (workspace-file-first FR-13): drop a CSV/Parquet/JSON
//! file here and the watcher ingests it into the workspace DB with smart defaults
//! (table name = filename stem), reusing the Connect pipeline's readers. Success moves
//! the file to `data/main/`; failure emits `dm:error` and leaves the file untouched.

use duckdb::Connection;
use std::fs;
use std::path::Path;

use crate::commands::dm_store;

fn read_fn_for(ext: &str) -> Option<&'static str> {
    match ext.to_ascii_lowercase().as_str() {
        "csv" => Some("read_csv_auto"),
        "parquet" => Some("read_parquet"),
        "json" => Some("read_json_auto"),
        _ => None,
    }
}

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

/// Ingest one dropped file. Returns the created table name.
pub fn ingest_incoming_file(conn: &Connection, ws: &Path, path: &Path) -> Result<String, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_string();
    let read_fn = read_fn_for(&ext)
        .ok_or_else(|| format!("unsupported file type .{ext} (csv, parquet, json only)"))?;

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("invalid file name")?;
    let table = stem.to_lowercase().replace([' ', '-'], "_");
    if !dm_store::valid_name(&table) {
        return Err(format!("cannot derive a table name from {stem:?}"));
    }
    if table_exists(conn, &table)? {
        return Err(format!("table \"{table}\" already exists — rename the file to ingest as a new table"));
    }

    let escaped = path
        .to_str()
        .ok_or("invalid path")?
        .replace('\'', "''")
        .replace('\\', "/");
    let fn_call = if read_fn == "read_csv_auto" {
        format!("{read_fn}('{escaped}', ignore_errors=true)")
    } else {
        format!("{read_fn}('{escaped}')")
    };
    let sanitized = table.replace('"', "\"\"");
    conn.execute(
        &format!("CREATE TABLE \"{sanitized}\" AS SELECT * FROM {fn_call}"),
        [],
    )
    .map_err(|e| format!("ingest failed: {e}"))?;

    // success: move into data/main (the canonical source-file home)
    let dest = ws.join("data").join("main").join(path.file_name().unwrap());
    fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| format!("failed to ensure data/main: {e}"))?;
    fs::rename(path, &dest).map_err(|e| format!("ingested, but failed to move into data/main: {e}"))?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::database::initialize_schema;

    fn tmp(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("incoming-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("data").join("incoming")).unwrap();
        dir
    }

    #[test]
    fn ingests_csv_creates_table_and_moves_file() {
        let ws = tmp("csv");
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let drop = ws.join("data/incoming/orders.csv");
        fs::write(&drop, "id,region,amount\n1,EMEA,100\n2,APAC,250\n").unwrap();

        let table = ingest_incoming_file(&conn, &ws, &drop).unwrap();
        assert_eq!(table, "orders");
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM orders", [], |r| r.get(0)).unwrap();
        assert_eq!(n, 2);
        assert!(!drop.exists());
        assert!(ws.join("data/main/orders.csv").exists());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn unsupported_type_and_duplicate_table_error_without_touching_file() {
        let ws = tmp("errs");
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let txt = ws.join("data/incoming/notes.txt");
        fs::write(&txt, "hello").unwrap();
        let err = ingest_incoming_file(&conn, &ws, &txt).unwrap_err();
        assert!(err.contains("unsupported"), "{err}");
        assert!(txt.exists());

        let csv = ws.join("data/incoming/orders.csv");
        fs::write(&csv, "id\n1\n").unwrap();
        ingest_incoming_file(&conn, &ws, &csv).unwrap();
        let csv2 = ws.join("data/incoming/orders.csv");
        fs::write(&csv2, "id\n2\n").unwrap();
        let err = ingest_incoming_file(&conn, &ws, &csv2).unwrap_err();
        assert!(err.contains("already exists"), "{err}");
        assert!(csv2.exists(), "failed ingest leaves the file in place");
        let _ = fs::remove_dir_all(&ws);
    }
}
