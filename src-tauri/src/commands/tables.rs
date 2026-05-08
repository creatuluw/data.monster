use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::metadata_helpers::{get_table_source_row, register_table_metadata, remove_table_metadata, rename_table_metadata};

#[tauri::command]
pub fn list_tables(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    eprintln!("[tables] Listing");
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' AND table_name NOT LIKE 'd8a_monster_%' ORDER BY table_name",
        )
        .map_err(|e| e.to_string())?;

    let tables: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(json!({ "name": name }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "tables": tables }))
}

#[tauri::command]
pub fn drop_table(table_name: String, state: State<'_, DuckDbState>) -> Result<(), String> {
    eprintln!("[tables] Dropping '{}'", table_name);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sanitized = table_name.replace('"', "\"\"");
    conn.execute(&format!("DROP TABLE IF EXISTS \"{}\"", sanitized), [])
        .map_err(|e| format!("Failed to drop table: {}", e))?;

    remove_table_metadata(conn, &table_name)?;

    conn.execute(
        "DELETE FROM d8a_monster_table_labels WHERE table_name = ?",
        [&table_name],
    )
    .map_err(|e| format!("Failed to remove labels: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn create_table_from_query(
    table_name: String,
    sql: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sanitized = table_name.replace('"', "\"\"");
    let create_sql = format!("CREATE TABLE \"{}\" AS {}", sanitized, sql);

    conn.execute(&create_sql, [])
        .map_err(|e| format!("Failed to create table: {}", e))?;

    register_table_metadata(conn, &table_name, "model")?;

    Ok(())
}

#[tauri::command]
pub fn rename_table(
    old_name: String,
    new_name: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let old_sanitized = old_name.replace('"', "\"\"");
    let new_sanitized = new_name.replace('"', "\"\"");

    conn.execute(
        &format!("ALTER TABLE \"{}\" RENAME TO \"{}\"", old_sanitized, new_sanitized),
        [],
    )
    .map_err(|e| format!("Failed to rename table: {}", e))?;

    rename_table_metadata(conn, &old_name, &new_name)?;

    conn.execute(
        "UPDATE d8a_monster_table_labels SET table_name = ? WHERE table_name = ?",
        duckdb::params![&new_name, &old_name],
    )
    .map_err(|e| format!("Failed to update labels: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn save_table_source(
    table_name: String,
    creation_query: String,
    source_path: String,
    source_type: Option<String>,
    original_source: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let table_type = determine_table_type(&source_type);

    conn.execute(
        "INSERT INTO d8a_monster_table_metadata (table_name, table_type, creation_query, source_path, source_type, original_source, created_at)
         VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT (table_name) DO UPDATE SET
            table_type = EXCLUDED.table_type,
            creation_query = EXCLUDED.creation_query,
            source_path = EXCLUDED.source_path,
            source_type = EXCLUDED.source_type,
            original_source = EXCLUDED.original_source",
        duckdb::params![table_name, table_type, creation_query, source_path, source_type, original_source],
    )
    .map_err(|e| format!("Failed to save table source: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_table_source(
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let result: Option<(Option<String>, Option<String>, Option<String>, Option<String>)> = conn
        .query_row(
            "SELECT creation_query, source_path, source_type, original_source FROM d8a_monster_table_metadata WHERE table_name = ?",
            duckdb::params![table_name],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .ok();

    match result {
        Some((creation_query, source_path, source_type, original_source)) => Ok(json!({
            "creationQuery": creation_query,
            "sourcePath": source_path,
            "sourceType": source_type,
            "originalSource": original_source,
        })),
        None => Ok(json!({ "creationQuery": null, "sourcePath": null, "sourceType": null, "originalSource": null })),
    }
}

#[tauri::command]
pub fn get_table_types(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare("SELECT table_name, table_type FROM d8a_monster_table_metadata")
        .map_err(|e| e.to_string())?;

    let rows: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let typ: String = row.get(1)?;
            Ok(json!({ "tableName": name, "tableType": typ }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "types": rows }))
}

#[tauri::command]
pub fn refresh_table_from_source(
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let source = get_table_source_row(conn, &table_name)?;
    let (_table_type, creation_query, _source_path) = source
        .ok_or("No metadata found for table")?;
    let sql = creation_query
        .ok_or("No creation query stored for table")?;

    let sanitized = table_name.replace('"', "\"\"");
    conn.execute(&format!("DROP TABLE IF EXISTS \"{}\"", sanitized), [])
        .map_err(|e| format!("Failed to drop table: {}", e))?;

    conn.execute(&sql, [])
        .map_err(|e| format!("Failed to recreate table: {}", e))?;

    Ok(())
}

fn determine_table_type(source_type: &Option<String>) -> String {
    match source_type.as_deref() {
        Some("url") => "url".to_string(),
        Some("postgres") => "postgres".to_string(),
        _ => "file".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
use crate::utils::metadata_helpers::{get_table_source_row, register_table_metadata, remove_table_metadata, rename_table_metadata};
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn
    }

    fn list_user_tables(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' AND table_name NOT LIKE 'd8a_monster_%' ORDER BY table_name",
            )
            .unwrap();
        stmt.query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect()
    }

    #[test]
    fn test_create_and_list() {
        let conn = setup();
        conn.execute("CREATE TABLE my_data AS SELECT 1 AS x", []).unwrap();
        register_table_metadata(&conn, "my_data", "source").unwrap();
        let tables = list_user_tables(&conn);
        assert_eq!(tables, vec!["my_data"]);
    }

    #[test]
    fn test_drop_table() {
        let conn = setup();
        conn.execute("CREATE TABLE t1 AS SELECT 1 AS x", []).unwrap();
        register_table_metadata(&conn, "t1", "source").unwrap();
        assert!(list_user_tables(&conn).contains(&"t1".to_string()));

        conn.execute("DROP TABLE IF EXISTS \"t1\"", []).unwrap();
        remove_table_metadata(&conn, "t1").unwrap();
        assert!(!list_user_tables(&conn).contains(&"t1".to_string()));
    }

    #[test]
    fn test_rename_table() {
        let conn = setup();
        conn.execute("CREATE TABLE old_name AS SELECT 1 AS x", []).unwrap();
        register_table_metadata(&conn, "old_name", "source").unwrap();

        conn.execute("ALTER TABLE \"old_name\" RENAME TO \"new_name\"", []).unwrap();
        rename_table_metadata(&conn, "old_name", "new_name").unwrap();

        let tables = list_user_tables(&conn);
        assert!(!tables.contains(&"old_name".to_string()));
        assert!(tables.contains(&"new_name".to_string()));
    }

    #[test]
    fn test_create_from_query() {
        let conn = setup();
        conn.execute("CREATE TABLE raw AS SELECT * FROM generate_series(1, 10) AS t(n)", []).unwrap();

        let sanitized = "filtered".replace('"', "\"\"");
        conn.execute(
            &format!("CREATE TABLE \"{}\" AS SELECT * FROM raw WHERE n > 5", sanitized),
            [],
        ).unwrap();
        register_table_metadata(&conn, "filtered", "model").unwrap();

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM filtered", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_table_name_with_special_chars() {
        let conn = setup();
        let name = "my table".replace('"', "\"\"");
        conn.execute(&format!("CREATE TABLE \"{}\" AS SELECT 1", name), []).unwrap();
        let tables = list_user_tables(&conn);
        assert!(tables.contains(&"my table".to_string()));
    }

    #[test]
    fn test_metadata_hidden_from_list() {
        let conn = setup();
        let tables = list_user_tables(&conn);
        assert!(!tables.iter().any(|t| t.starts_with("d8a_monster_")));
    }
}
