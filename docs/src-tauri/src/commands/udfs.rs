use tauri::{State, Manager};
use duckdb::{Connection, params};
use crate::state::DuckDbState;
use crate::utils::format_duckdb_value;

/// Create or update a user-defined function (macro)
#[tauri::command]
pub fn create_udf(
    state: State<DuckDbState>, 
    function_name: String, 
    parameters: String,
    return_type: String,
    function_body: String,
    description: String
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    if !function_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Function name can only contain letters, numbers, and underscores".to_string());
    }
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_udfs (
            function_name TEXT PRIMARY KEY,
            parameters TEXT NOT NULL,
            return_type TEXT NOT NULL,
            function_body TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create UDF metadata table: {}", e))?;
    
    let exists_query = "SELECT COUNT(*) FROM _warphead_udfs WHERE function_name = ?";
    let exists: i64 = conn
        .query_row(exists_query, params![&function_name], |row| row.get(0))
        .unwrap_or(0);
    let is_update = exists > 0;
    
    let create_query = format!(
        "CREATE OR REPLACE TEMP MACRO {}({}) AS {}",
        function_name,
        parameters,
        function_body
    );
    
    conn.execute(&create_query, []).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO _warphead_udfs (function_name, parameters, return_type, function_body, description, updated_at)
         VALUES (?, ?, ?, ?, ?, now())
         ON CONFLICT (function_name) DO UPDATE SET 
            parameters = EXCLUDED.parameters,
            return_type = EXCLUDED.return_type,
            function_body = EXCLUDED.function_body,
            description = EXCLUDED.description,
            updated_at = now()",
        params![&function_name, &parameters, &return_type, &function_body, &description]
    ).map_err(|e| format!("Failed to save UDF metadata: {}", e))?;
    
    Ok(format!(
        "Function '{}' {} successfully", 
        function_name,
        if is_update { "updated" } else { "created" }
    ))
}

/// Get all user-defined functions
#[tauri::command]
pub fn list_udfs(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_udfs (
            function_name TEXT PRIMARY KEY,
            parameters TEXT NOT NULL,
            return_type TEXT NOT NULL,
            function_body TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        []
    ).map_err(|e| format!("Failed to create UDF metadata table: {}", e))?;
    
    let query = "SELECT function_name, parameters, return_type, function_body, description, 
                        CAST(created_at AS VARCHAR) as created_at, 
                        CAST(updated_at AS VARCHAR) as updated_at
                 FROM _warphead_udfs 
                 ORDER BY function_name";
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let udfs_result = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "function_name": row.get::<_, String>(0)?,
            "parameters": row.get::<_, String>(1)?,
            "return_type": row.get::<_, String>(2)?,
            "function_body": row.get::<_, String>(3)?,
            "description": row.get::<_, Option<String>>(4)?,
            "created_at": row.get::<_, String>(5)?,
            "updated_at": row.get::<_, String>(6)?
        }))
    }).map_err(|e| e.to_string())?;
    
    let udfs: Vec<serde_json::Value> = udfs_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&udfs).map_err(|e| e.to_string())
}

/// Delete a user-defined function
#[tauri::command]
pub fn delete_udf(state: State<DuckDbState>, function_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let drop_query = format!("DROP MACRO IF EXISTS {}", function_name);
    conn.execute(&drop_query, []).map_err(|e| e.to_string())?;
    
    conn.execute(
        "DELETE FROM _warphead_udfs WHERE function_name = ?",
        [&function_name]
    ).map_err(|e| format!("Failed to remove UDF metadata: {}", e))?;
    
    Ok(format!("Function '{}' deleted successfully", function_name))
}

/// Test a user-defined function with sample inputs
#[tauri::command]
pub fn test_udf(
    state: State<DuckDbState>, 
    _function_name: String, 
    test_query: String
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn.prepare(&test_query).map_err(|e| e.to_string())?;
    
    let rows_result = stmt
        .query_map([], |row| {
            let column_count = row.as_ref().column_count();
            let mut row_map = serde_json::Map::new();
            
            for i in 0..column_count {
                let column_name = row.as_ref().column_name(i)
                    .map_or(format!("col_{}", i), |v| v.to_string());
                
                let value: serde_json::Value = match row.get_ref(i) {
                    Ok(val) => format_duckdb_value(val),
                    Err(_) => serde_json::Value::Null,
                };
                row_map.insert(column_name, value);
            }
            Ok(serde_json::Value::Object(row_map))
        })
        .map_err(|e| e.to_string())?;
    
    let mut rows = Vec::new();
    for row in rows_result {
        rows.push(row.map_err(|e: duckdb::Error| e.to_string())?);
    }
    
    serde_json::to_string(&rows).map_err(|e| e.to_string())
}

/// Reload all UDFs from metadata
#[tauri::command]
pub fn reload_udfs(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    reload_udfs_internal(conn)
}

/// Internal helper to reload UDFs
pub fn reload_udfs_internal(conn: &Connection) -> Result<String, String> {
    let query = "SELECT function_name, parameters, function_body FROM _warphead_udfs";
    let mut stmt = match conn.prepare(query) {
        Ok(s) => s,
        Err(_) => {
            return Ok("No UDFs to reload".to_string());
        }
    };
    
    let udfs_result = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?
        ))
    }).map_err(|e| e.to_string())?;
    
    let mut count = 0;
    let mut errors = Vec::new();
    
    for udf_result in udfs_result {
        let (function_name, parameters, function_body) = udf_result.map_err(|e| e.to_string())?;
        
        let create_query = format!(
            "CREATE OR REPLACE TEMP MACRO {}({}) AS {}",
            function_name,
            parameters,
            function_body
        );
        
        match conn.execute(&create_query, []) {
            Ok(_) => {
                count += 1;
            },
            Err(e) => {
                errors.push(format!("Function '{}': {}", function_name, e));
                eprintln!("Warning: Failed to reload UDF '{}': {}", function_name, e);
            }
        }
    }
    
    if errors.is_empty() {
        Ok(format!("Reloaded {} user-defined function(s)", count))
    } else {
        Ok(format!("Reloaded {} UDF(s), {} failed: {}", count, errors.len(), errors.join("; ")))
    }
}

/// Type inference commands
use std::collections::HashSet;
use crate::models::ColumnTypeInfo;
use crate::utils::detect_column_type;

/// Infer types for columns from sample data
#[tauri::command]
pub fn infer_column_types(data: String) -> Result<String, String> {
    let parsed: Vec<serde_json::Map<String, serde_json::Value>> = serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON data: {}", e))?;
    
    if parsed.is_empty() {
        return Ok(serde_json::to_string(&Vec::<ColumnTypeInfo>::new()).unwrap());
    }
    
    let first_row = &parsed[0];
    let column_names: Vec<String> = first_row.keys().cloned().collect();
    
    let mut column_types: Vec<ColumnTypeInfo> = Vec::new();
    
    for col_name in column_names {
        let mut values: Vec<String> = Vec::new();
        let mut null_count = 0;
        let mut distinct_values: HashSet<String> = HashSet::new();
        
        for row in &parsed {
            if let Some(value) = row.get(&col_name) {
                let str_value = match value {
                    serde_json::Value::Null => {
                        null_count += 1;
                        "NULL".to_string()
                    }
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                
                if str_value != "NULL" {
                    distinct_values.insert(str_value.clone());
                }
                values.push(str_value);
            }
        }
        
        let (detected_type, confidence) = detect_column_type(&values);
        
        let sample_values: Vec<String> = distinct_values
            .iter()
            .take(5)
            .cloned()
            .collect();
        
        column_types.push(ColumnTypeInfo {
            column_name: col_name.clone(),
            detected_type,
            nullable: null_count > 0,
            sample_values,
            distinct_count: distinct_values.len(),
            null_count,
            confidence,
        });
    }
    
    serde_json::to_string(&column_types).map_err(|e| e.to_string())
}

/// Infer types from a file (CSV, JSON, Parquet)
#[tauri::command]
pub fn infer_file_types(app: tauri::AppHandle, file_path: String, sample_size: Option<usize>) -> Result<String, String> {
    use duckdb::Connection;
    
    let app_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app directory: {}", e))?;
    let full_path = app_dir.join(&file_path);
    
    if !full_path.exists() {
        return Err(format!("File not found: {}", file_path));
    }
    
    let extension = full_path.extension()
        .and_then(|e| e.to_str())
        .ok_or("Could not determine file extension")?
        .to_lowercase();
    
    let sample_limit = sample_size.unwrap_or(100);
    
    let conn = Connection::open_in_memory()
        .map_err(|e| format!("Failed to create temp connection: {}", e))?;
    
    let query = match extension.as_str() {
        "csv" => format!(
            "SELECT * FROM read_csv_auto('{}') LIMIT {}",
            full_path.to_string_lossy().replace('\\', "/"),
            sample_limit
        ),
        "json" => format!(
            "SELECT * FROM read_json_auto('{}') LIMIT {}",
            full_path.to_string_lossy().replace('\\', "/"),
            sample_limit
        ),
        "parquet" => format!(
            "SELECT * FROM read_parquet('{}') LIMIT {}",
            full_path.to_string_lossy().replace('\\', "/"),
            sample_limit
        ),
        _ => return Err(format!("Unsupported file type: {}", extension)),
    };
    
    let stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let column_count = stmt.column_count();
    
    let mut column_types: Vec<ColumnTypeInfo> = Vec::new();
    
    for i in 0..column_count {
        let col_name = stmt.column_name(i)
            .map_err(|e| e.to_string())?
            .to_string();
        
        let type_query = format!(
            "SELECT typeof({}) as col_type FROM read_csv_auto('{}') LIMIT 1",
            col_name,
            full_path.to_string_lossy().replace('\\', "/")
        );
        
        let detected_type: String = conn.query_row(&type_query, [], |row| row.get(0))
            .unwrap_or_else(|_| "VARCHAR".to_string());
        
        let sample_query = format!(
            "SELECT DISTINCT {} FROM read_csv_auto('{}') WHERE {} IS NOT NULL LIMIT 5",
            col_name,
            full_path.to_string_lossy().replace('\\', "/"),
            col_name
        );
        
        let sample_values: Vec<String> = match conn.prepare(&sample_query) {
            Ok(mut stmt) => {
                stmt.query_map([], |row| {
                    let val: Result<String, _> = row.get(0);
                    Ok(val.unwrap_or_default())
                })
                .unwrap_or_else(|_| panic!("Query failed"))
                .filter_map(Result::ok)
                .collect()
            }
            Err(_) => Vec::new()
        };
        
        column_types.push(ColumnTypeInfo {
            column_name: col_name,
            detected_type,
            nullable: true,
            sample_values,
            distinct_count: 0,
            null_count: 0,
            confidence: 1.0,
        });
    }
    
    serde_json::to_string(&column_types).map_err(|e| e.to_string())
}

