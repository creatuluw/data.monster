use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::metadata_helpers::{register_table_metadata, remove_table_metadata, rename_table_metadata};

#[tauri::command]
pub fn list_tables(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
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
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
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
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
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
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
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
