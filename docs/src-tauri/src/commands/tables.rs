use tauri::State;
use crate::state::DuckDbState;
use crate::utils::{register_table_metadata_with_query, remove_table_metadata};

/// Create a table from a SQL query (CTAS - Create Table As Select)
#[tauri::command]
pub fn create_table_from_query(state: State<DuckDbState>, table_name: String, query: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Validate table name
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Table name can only contain letters, numbers, and underscores".to_string());
    }
    
    // Create table from query
    let create_query = format!("CREATE TABLE {} AS {}", table_name, query);
    conn.execute(&create_query, []).map_err(|e| e.to_string())?;
    
    // Register in datamodels table
    conn.execute(
        "INSERT INTO datamodels (table_name) VALUES (?) ON CONFLICT DO NOTHING",
        [&table_name]
    ).map_err(|e| format!("Failed to register in datamodels: {}", e))?;
    
    // Register table in metadata as 'model' type
    register_table_metadata_with_query(conn, &table_name, "model", Some(&query))?;
    
    // Get row count
    let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
    let count: i64 = conn
        .query_row(&count_query, [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Table '{}' created successfully with {} rows", table_name, count))
}

/// List all tables in the database
#[tauri::command]
pub fn list_tables(state: State<DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn
        .prepare("SELECT table_name FROM information_schema.tables WHERE table_schema = 'main'")
        .map_err(|e| e.to_string())?;
    
    let tables = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(tables)
}

/// Get list of saved/created model tables
#[tauri::command]
pub fn get_saved_tables(state: State<DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn
        .prepare("SELECT table_name FROM _warphead_table_metadata WHERE table_type = 'model' ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    
    let tables = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(tables)
}

/// Get the original creation query for a model table
#[tauri::command]
pub fn get_table_creation_query(state: State<DuckDbState>, table_name: String) -> Result<Option<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query_result: Option<String> = conn.query_row(
        "SELECT creation_query FROM _warphead_table_metadata WHERE table_name = ? AND table_type = 'model'",
        [&table_name],
        |row| row.get(0)
    ).ok();
    
    Ok(query_result)
}

/// Get table size in bytes
#[tauri::command]
pub fn get_table_size(state: State<DuckDbState>, table_name: String) -> Result<i64, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!(
        "SELECT SUM(estimated_size) FROM duckdb_tables() WHERE table_name = '{}'",
        table_name
    );
    
    let size: i64 = conn
        .query_row(&query, [], |row| row.get(0))
        .unwrap_or(0);
    
    Ok(size)
}

/// Drop (delete) a table
#[tauri::command]
pub fn drop_table(state: State<DuckDbState>, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Validate table name
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Invalid table name".to_string());
    }
    
    // Prevent deletion of system tables
    if table_name == "datamodels" || table_name == "_warphead_table_metadata" {
        return Err("Cannot delete system tables".to_string());
    }
    
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, []).map_err(|e| e.to_string())?;
    
    // Remove from datamodels table
    conn.execute(
        "DELETE FROM datamodels WHERE table_name = ?",
        [&table_name]
    ).map_err(|e| format!("Failed to remove from datamodels: {}", e))?;
    
    // Remove from metadata table
    remove_table_metadata(conn, &table_name)?;
    
    Ok(format!("Table '{}' deleted successfully", table_name))
}

