use tauri::State;
use duckdb::params;
use chrono::Utc;
use crate::state::DuckDbState;
use crate::utils::remove_table_metadata;

/// Debug command: List all entries in _warphead_attached_tables
#[tauri::command]
pub fn debug_list_attached_tables(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = "SELECT table_id, connection_id, schema_name, table_name, full_name, access_mode, local_table_name, cached_at, cache_row_count 
                 FROM _warphead_attached_tables 
                 ORDER BY cached_at DESC";
    
    let mut stmt = conn.prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let rows = stmt.query_map([], |row| {
        Ok(format!(
            "table_id: {}, connection_id: {}, schema: {}, table: {}, full_name: {}, access_mode: {}, local_table_name: {}, cached_at: {}, row_count: {}",
            row.get::<_, String>(0).unwrap_or_default(),
            row.get::<_, String>(1).unwrap_or_default(),
            row.get::<_, String>(2).unwrap_or_default(),
            row.get::<_, String>(3).unwrap_or_default(),
            row.get::<_, String>(4).unwrap_or_default(),
            row.get::<_, String>(5).unwrap_or_default(),
            row.get::<_, Option<String>>(6).unwrap_or_default().unwrap_or("NULL".to_string()),
            row.get::<_, Option<String>>(7).unwrap_or_default().unwrap_or("NULL".to_string()),
            row.get::<_, Option<i64>>(8).unwrap_or_default().map(|n| n.to_string()).unwrap_or("NULL".to_string()),
        ))
    })
    .map_err(|e| format!("Failed to query: {}", e))?;
    
    let mut result = Vec::new();
    for row in rows {
        if let Ok(data) = row {
            result.push(data);
        }
    }
    
    if result.is_empty() {
        Ok("No attached tables found in _warphead_attached_tables".to_string())
    } else {
        Ok(format!("Found {} attached tables:\n{}", result.len(), result.join("\n")))
    }
}

/// Debug command: Fix NULL timestamps in attached tables
#[tauri::command]
pub fn debug_fix_null_timestamps(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = conn.execute(
        "UPDATE _warphead_attached_tables SET cached_at = ? WHERE cached_at IS NULL",
        params![&current_time]
    ).map_err(|e| format!("Failed to update timestamps: {}", e))?;
    
    Ok(format!("Fixed {} table(s) with NULL timestamps", result))
}

/// Debug command: Delete table by local_table_name
#[tauri::command]
pub fn debug_delete_table(state: State<DuckDbState>, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])
        .map_err(|e| format!("Failed to drop table: {}", e))?;
    
    let _ = remove_table_metadata(conn, &table_name);
    
    conn.execute(
        "DELETE FROM _warphead_attached_tables WHERE local_table_name = ?",
        params![&table_name]
    ).map_err(|e| format!("Failed to remove from tracking: {}", e))?;
    
    Ok(format!("Successfully deleted table '{}'", table_name))
}

