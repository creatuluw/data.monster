use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::metadata_helpers::register_table_metadata;

#[tauri::command]
pub fn load_csv_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_csv_auto", &state)
}

#[tauri::command]
pub fn load_parquet_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_parquet", &state)
}

#[tauri::command]
pub fn load_json_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_json_auto", &state)
}

#[tauri::command]
pub fn get_file_columns(
    path: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let ext = Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let read_fn = match ext.as_str() {
        "parquet" => "read_parquet",
        "json" | "jsonl" => "read_json_auto",
        _ => "read_csv_auto",
    };

    let query = format!(
        "SELECT column_name, data_type FROM (DESCRIBE SELECT * FROM {}('{}'))",
        read_fn, path
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let columns: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let dtype: String = row.get(1)?;
            Ok(json!({ "name": name, "type": dtype }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "columns": columns }))
}

fn load_file(
    path: &str,
    table_name: &str,
    read_fn: &str,
    state: &State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let wp = state.workspace_path.lock().map_err(|e| e.to_string())?;
    let workspace = wp.as_ref().ok_or("No workspace folder set")?;

    let source_path = Path::new(path);
    let file_name = source_path
        .file_name()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Invalid file name")?;

    let dest_dir = Path::new(workspace).join("data").join("main");
    let dest_path = dest_dir.join(file_name);

    if source_path != dest_path {
        fs::copy(source_path, &dest_path)
            .map_err(|e| format!("Failed to copy file to workspace: {}", e))?;
    }

    let sanitized = table_name.replace('"', "\"\"");
    let escaped_dest = dest_path.to_str().ok_or("Invalid destination path")?;
    let query = format!(
        "CREATE TABLE \"{}\" AS SELECT * FROM {}('{}')",
        sanitized, read_fn, escaped_dest
    );

    conn.execute(&query, [])
        .map_err(|e| format!("Failed to load file into table: {}", e))?;

    let row_count: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM \"{}\"", sanitized),
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    register_table_metadata(conn, table_name, "source")?;

    Ok(json!({
        "tableName": table_name,
        "rowCount": row_count
    }))
}
