use tauri::{State, Emitter};
use crate::state::DuckDbState;
use crate::utils::{format_duckdb_value};
use std::sync::atomic::Ordering;
use regex::Regex;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryProgress {
    pub rows_loaded: usize,
    pub message: String,
}

/// Extract table names from a SQL query
fn extract_table_names(query: &str) -> Vec<String> {
    let mut tables = Vec::new();
    
    // Simple regex to find table names after FROM and JOIN keywords
    // This is a basic implementation - might need enhancement for complex queries
    let re = Regex::new(r"(?i)\b(?:FROM|JOIN)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    
    for cap in re.captures_iter(query) {
        if let Some(table) = cap.get(1) {
            let table_name = table.as_str().to_string();
            // Filter out common SQL keywords
            if !matches!(table_name.to_lowercase().as_str(), 
                "select" | "where" | "order" | "group" | "having" | "limit" | "offset") {
                tables.push(table_name);
            }
        }
    }
    
    tables
}

/// Execute a SQL query and return results as JSON
#[tauri::command]
pub fn execute_query(app: tauri::AppHandle, state: State<DuckDbState>, query: String) -> Result<String, String> {
    // Reset cancellation flag at the start
    state.query_cancelled.store(false, Ordering::SeqCst);
    
    // Check if any tables in the query need ingestion with 'on_query' strategy
    let table_names = extract_table_names(&query);
    let mut tables_to_ingest = Vec::new();
    
    {
        let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
        let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
        
        for table_name in table_names {
            // Check if table has on_query strategy and is not ingested
            let check_query = "SELECT ingestion_strategy, is_ingested FROM _warphead_table_metadata WHERE table_name = ?";
            if let Ok((strategy, is_ingested)) = conn.query_row(
                check_query,
                [&table_name],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, bool>(1)?))
            ) {
                if strategy == "on_query" && !is_ingested {
                    tables_to_ingest.push(table_name);
                }
            }
        }
        // state_conn lock is released here
    }
    
    // Ingest tables that need ingestion (outside the lock)
    for table_name in tables_to_ingest {
        eprintln!("🔄 Auto-ingesting table '{}' on query...", table_name);
        let ingest_result = crate::commands::ingestion::ingest_table(state.clone(), table_name.clone());
        match ingest_result {
            Ok(msg) => eprintln!("✅ {}", msg),
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to auto-ingest table '{}': {}", table_name, e);
                // Don't fail the query, just warn
            }
        }
    }
    
    // Now execute the actual query
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Store the interrupt handle so cancel_query can use it without locking conn
    let interrupt = conn.interrupt_handle();
    {
        let mut interrupt_handle = state.interrupt_handle.lock().map_err(|e| e.to_string())?;
        *interrupt_handle = Some(interrupt);
    }
    
    // Check if cancelled before starting
    if state.query_cancelled.load(Ordering::SeqCst) {
        return Err("Query cancelled before execution".to_string());
    }
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    let mut column_names: Vec<String> = Vec::new();
    let mut first_row = true;
    
    let rows_result = stmt
        .query_map([], |row| {
            let column_count = row.as_ref().column_count();
            
            let mut cols_with_values: Vec<(String, serde_json::Value)> = Vec::new();
            
            for i in 0..column_count {
                let column_name = row.as_ref().column_name(i)
                    .map_or(format!("col_{}", i), |v| v.to_string());
                
                let value: serde_json::Value = match row.get_ref(i) {
                    Ok(val) => format_duckdb_value(val),
                    Err(_) => serde_json::Value::Null,
                };
                
                cols_with_values.push((column_name, value));
            }
            
            Ok((cols_with_values.iter().map(|(name, _)| name.clone()).collect::<Vec<String>>(), cols_with_values))
        })
        .map_err(|e| e.to_string())?;
    
    let mut rows = Vec::new();
    let mut row_count = 0;
    const PROGRESS_INTERVAL: usize = 1000; // Emit progress every 1000 rows
    
    for row_result in rows_result {
        // Check for cancellation during row processing
        if state.query_cancelled.load(Ordering::SeqCst) {
            return Err("Query cancelled during execution".to_string());
        }
        
        let (col_names, col_values) = row_result.map_err(|e: duckdb::Error| e.to_string())?;
        
        if first_row {
            column_names = col_names;
            first_row = false;
        }
        
        let mut row_map = serde_json::Map::new();
        for (col_name, col_value) in col_values {
            row_map.insert(col_name, col_value);
        }
        
        rows.push(serde_json::Value::Object(row_map));
        row_count += 1;
        
        // Emit progress event every PROGRESS_INTERVAL rows or for the first few updates
        if row_count % PROGRESS_INTERVAL == 0 || (row_count <= 10000 && row_count % 100 == 0) {
            let _ = app.emit("query-progress", QueryProgress {
                rows_loaded: row_count,
                message: format!("Loading... {} rows", row_count),
            });
        }
    }
    
    // Notify that we're processing the results
    if row_count > 10000 {
        let _ = app.emit("query-progress", QueryProgress {
            rows_loaded: row_count,
            message: format!("Processing {} rows...", row_count),
        });
    }
    
    let result = serde_json::json!({
        "columns": column_names,
        "data": rows
    });
    
    // Notify that we're serializing
    if row_count > 10000 {
        let _ = app.emit("query-progress", QueryProgress {
            rows_loaded: row_count,
            message: format!("Serializing {} rows...", row_count),
        });
    }
    
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

/// Cancel the currently running query
#[tauri::command]
pub fn cancel_query(state: State<DuckDbState>) -> Result<String, String> {
    // Set the cancellation flag
    state.query_cancelled.store(true, Ordering::SeqCst);
    
    // Use the stored interrupt handle to cancel without locking the connection
    let interrupt_handle = state.interrupt_handle.lock().map_err(|e| e.to_string())?;
    if let Some(handle) = interrupt_handle.as_ref() {
        handle.interrupt();
        Ok("Query cancellation requested".to_string())
    } else {
        Err("No active query to cancel".to_string())
    }
}

