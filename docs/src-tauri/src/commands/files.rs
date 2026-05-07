use tauri::{State, AppHandle, Manager};
use std::path::PathBuf;
use std::fs;
use crate::state::DuckDbState;
use crate::models::{FileMetadata};
use crate::utils::register_table_metadata;
use std::time::SystemTime;

/// Get the app's data directory path
pub fn get_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let data_dir = app_data_dir.join("data");
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }
    
    Ok(data_dir)
}

/// Load CSV file into DuckDB
#[tauri::command]
pub fn load_csv_file(state: State<DuckDbState>, file_path: String, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_csv_auto('{}')",
        table_name,
        file_path.replace("\\", "\\\\")
    );
    
    conn.execute(&query, []).map_err(|e| e.to_string())?;
    register_table_metadata(conn, &table_name, "source")?;
    
    let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
    let count: i64 = conn
        .query_row(&count_query, [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Loaded {} rows into table '{}'", count, table_name))
}

/// Load Parquet file into DuckDB
#[tauri::command]
pub fn load_parquet_file(state: State<DuckDbState>, file_path: String, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_parquet('{}')",
        table_name,
        file_path.replace("\\", "\\\\")
    );
    
    conn.execute(&query, []).map_err(|e| e.to_string())?;
    register_table_metadata(conn, &table_name, "source")?;
    
    let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
    let count: i64 = conn
        .query_row(&count_query, [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Loaded {} rows into table '{}'", count, table_name))
}

/// Load JSON file into DuckDB
#[tauri::command]
pub fn load_json_file(state: State<DuckDbState>, file_path: String, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM read_json_auto('{}')",
        table_name,
        file_path.replace("\\", "\\\\")
    );
    
    conn.execute(&query, []).map_err(|e| e.to_string())?;
    register_table_metadata(conn, &table_name, "source")?;
    
    let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
    let count: i64 = conn
        .query_row(&count_query, [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Loaded {} rows into table '{}'", count, table_name))
}

/// Get column names and types from a file
#[tauri::command]
pub fn get_file_columns(app: AppHandle, state: State<DuckDbState>, folder: String, filename: String) -> Result<String, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join(&folder).join(&filename);
    
    if !file_path.exists() {
        return Err(format!("File not found: {}/{}", folder, filename));
    }
    
    let file_path_str = file_path.to_str()
        .ok_or("Invalid file path")?
        .replace("\\", "\\\\");
    
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!("DESCRIBE SELECT * FROM '{}' LIMIT 0", file_path_str);
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    let columns_result = stmt.query_map([], |row| {
        let column_name: String = row.get(0)?;
        let column_type: String = row.get(1)?;
        Ok(serde_json::json!({
            "name": column_name,
            "type": column_type
        }))
    }).map_err(|e| e.to_string())?;
    
    let columns: Vec<serde_json::Value> = columns_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&columns).map_err(|e| e.to_string())
}

/// Get file metadata
pub fn get_file_metadata_internal(app: &AppHandle, state: &State<DuckDbState>, folder_name: String, filename: String) -> Result<FileMetadata, String> {
    let data_dir = get_data_dir(app)?;
    let file_path = data_dir.join(&folder_name).join(&filename);
    
    if !file_path.exists() {
        return Err(format!("File not found: {}", filename));
    }
    
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
    
    let size_bytes = metadata.len();
    
    let uploaded_at = metadata.modified()
        .ok()
        .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase();
    
    let file_type = match extension.as_str() {
        "csv" => "CSV",
        "parquet" => "Parquet",
        "json" | "jsonl" => "JSON",
        _ => "Unknown"
    }.to_string();
    
    let table_name = filename.replace('.', "_").replace('-', "_");
    let row_count = get_row_count_for_table(state, &table_name);
    
    let source_type = if folder_name.to_lowercase().contains("remote") {
        "remote"
    } else if folder_name.to_lowercase().contains("database") || folder_name.to_lowercase().contains("connect") {
        "database"
    } else if folder_name.to_lowercase().contains("created") || folder_name.to_lowercase().contains("create") {
        "created"
    } else {
        "file"
    }.to_string();
    
    let source_format = if source_type == "database" {
        let folder_lower = folder_name.to_lowercase();
        if folder_lower.contains("mysql") {
            "mysql"
        } else if folder_lower.contains("postgres") || folder_lower.contains("postgresql") {
            "postgres"
        } else if folder_lower.contains("sqlite") {
            "sqlite"
        } else if folder_lower.contains("mssql") || folder_lower.contains("sqlserver") {
            "mssql"
        } else if folder_lower.contains("oracle") {
            "oracle"
        } else if folder_lower.contains("mongodb") || folder_lower.contains("mongo") {
            "mongodb"
        } else {
            "database"
        }
    } else {
        match extension.as_str() {
            "csv" => "csv",
            "tsv" => "tsv",
            "parquet" => "parquet",
            "json" => "json",
            "jsonl" => "jsonl",
            "xml" => "xml",
            "xlsx" | "xls" => "excel",
            "arrow" => "arrow",
            "avro" => "avro",
            _ => "unknown"
        }
    }.to_string();
    
    Ok(FileMetadata {
        filename: filename.clone(),
        folder: folder_name,
        file_path: file_path.to_str().unwrap_or("").to_string(),
        file_type,
        size_bytes,
        uploaded_at,
        row_count,
        source_type,
        source_format,
    })
}

/// Get metadata for a specific file
#[tauri::command]
pub fn get_file_metadata(app: AppHandle, state: State<DuckDbState>, folder_name: String, filename: String) -> Result<FileMetadata, String> {
    get_file_metadata_internal(&app, &state, folder_name, filename)
}

fn get_row_count_for_table(state: &State<DuckDbState>, table_name: &str) -> Option<i64> {
    let state_conn = state.conn.lock().ok()?;
    let conn = state_conn.as_ref()?;
    
    let query = format!("SELECT COUNT(*) FROM {}", table_name);
    conn.query_row(&query, [], |row| row.get(0)).ok()
}

/// Get all files across all folders with metadata
#[tauri::command]
pub fn get_all_files_metadata(app: AppHandle, state: State<DuckDbState>) -> Result<Vec<FileMetadata>, String> {
    use crate::commands::folders::list_folders;
    use crate::commands::folders::list_files_in_folder;
    use chrono::Utc;
    
    let folders = list_folders(app.clone())?;
    let mut all_files = Vec::new();
    
    for folder in folders {
        let files = list_files_in_folder(app.clone(), folder.clone())?;
        
        for filename in files {
            if let Some(ext) = filename.split('.').last() {
                let ext_lower = ext.to_lowercase();
                if matches!(ext_lower.as_str(), "csv" | "parquet" | "json" | "jsonl") {
                    if let Ok(metadata) = get_file_metadata_internal(&app, &state, folder.clone(), filename) {
                        all_files.push(metadata);
                    }
                }
            }
        }
    }
    
    // Also include imported database tables
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    if let Some(conn) = state_conn.as_ref() {
        let query = "SELECT local_table_name, CAST(cached_at AS VARCHAR) as cached_at, cache_row_count, full_name, access_mode 
                     FROM _warphead_attached_tables 
                     WHERE access_mode IN ('cached', 'imported') AND local_table_name IS NOT NULL
                     ORDER BY COALESCE(cached_at, '1970-01-01 00:00:00') DESC";
        
        if let Ok(mut stmt) = conn.prepare(query) {
            if let Ok(rows) = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            }) {
                for row_result in rows {
                    if let Ok((table_name, cached_at, row_count, full_name, access_mode)) = row_result {
                        let uploaded_at = if let Some(ts) = cached_at {
                            chrono::NaiveDateTime::parse_from_str(&ts, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .and_then(|dt| dt.and_utc().timestamp().try_into().ok())
                                .unwrap_or_else(|| Utc::now().timestamp() as u64)
                        } else {
                            Utc::now().timestamp() as u64
                        };
                        
                        let source_format = if full_name.starts_with("pg") || full_name.contains("postgres") {
                            "postgres"
                        } else if full_name.starts_with("my") || full_name.contains("mysql") {
                            "mysql"
                        } else {
                            "database"
                        }.to_string();
                        
                        all_files.push(FileMetadata {
                            filename: format!("{}.db", table_name),
                            folder: "database".to_string(),
                            file_path: format!("duckdb://{}", table_name),
                            file_type: access_mode.to_uppercase(),
                            size_bytes: 0,
                            uploaded_at: uploaded_at as u64,
                            row_count,
                            source_type: "database".to_string(),
                            source_format,
                        });
                    }
                }
            }
        }
    }
    
    all_files.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));
    
    Ok(all_files)
}

/// Get recent files
#[tauri::command]
pub fn get_recent_files(app: AppHandle, state: State<DuckDbState>, limit: usize) -> Result<Vec<FileMetadata>, String> {
    let mut all_files = get_all_files_metadata(app, state)?;
    all_files.truncate(limit);
    Ok(all_files)
}

/// Load a specific file into DuckDB by filename and folder
#[tauri::command]
pub fn load_file_by_name(app: AppHandle, state: State<DuckDbState>, filename: String, folder: String) -> Result<String, String> {
    use crate::commands::folders::get_file_path_in_folder;
    
    let file_path = get_file_path_in_folder(app.clone(), folder.clone(), filename.clone())?;
    
    let extension = filename.split('.').last()
        .ok_or("Invalid filename")?
        .to_lowercase();
    
    let table_name = filename
        .trim_end_matches(".csv")
        .trim_end_matches(".parquet")
        .trim_end_matches(".json")
        .trim_end_matches(".jsonl")
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect::<String>();
    
    let result = match extension.as_str() {
        "csv" => load_csv_file(state, file_path, table_name.clone()),
        "parquet" => load_parquet_file(state, file_path, table_name.clone()),
        "json" | "jsonl" => load_json_file(state, file_path, table_name.clone()),
        _ => return Err(format!("Unsupported file type: {}", extension)),
    }?;
    
    Ok(format!("{} (Table: {})", result, table_name))
}

/// Reload all saved files into DuckDB
#[tauri::command]
pub fn reload_all_files(app: AppHandle, state: State<DuckDbState>) -> Result<String, String> {
    use crate::commands::folders::{list_folders, list_files_in_folder, get_file_path_in_folder};
    
    let folders = list_folders(app.clone())?;
    let mut loaded_count = 0;
    let mut error_count = 0;
    
    for folder in folders {
        let files = list_files_in_folder(app.clone(), folder.clone())?;
        
        for filename in files {
            if let Some(ext) = filename.split('.').last() {
                let ext_lower = ext.to_lowercase();
                if matches!(ext_lower.as_str(), "csv" | "parquet" | "json" | "jsonl") {
                    let file_path = match get_file_path_in_folder(app.clone(), folder.clone(), filename.clone()) {
                        Ok(path) => path,
                        Err(_) => {
                            error_count += 1;
                            continue;
                        }
                    };
                    
                    let table_name = filename
                        .trim_end_matches(".csv")
                        .trim_end_matches(".parquet")
                        .trim_end_matches(".json")
                        .trim_end_matches(".jsonl")
                        .chars()
                        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                        .collect::<String>();
                    
                    let result = match ext_lower.as_str() {
                        "csv" => load_csv_file(state.clone(), file_path, table_name),
                        "parquet" => load_parquet_file(state.clone(), file_path, table_name),
                        "json" | "jsonl" => load_json_file(state.clone(), file_path, table_name),
                        _ => continue,
                    };
                    
                    match result {
                        Ok(_) => loaded_count += 1,
                        Err(_) => error_count += 1,
                    }
                }
            }
        }
    }
    
    if error_count > 0 {
        Ok(format!("Loaded {} files into DuckDB ({} errors)", loaded_count, error_count))
    } else {
        Ok(format!("Loaded {} files into DuckDB", loaded_count))
    }
}

/// Delete a file
#[tauri::command]
pub fn delete_file(app: AppHandle, state: State<DuckDbState>, filename: String, folder: String) -> Result<String, String> {
    use crate::utils::remove_table_metadata;
    
    // Check if this is a database table
    if folder == "database" {
        let table_name = filename.trim_end_matches(".db");
        
        let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
        if let Some(conn) = state_conn.as_ref() {
            let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
            conn.execute(&drop_query, [])
                .map_err(|e| format!("Failed to drop table: {}", e))?;
            
            let _ = remove_table_metadata(conn, table_name);
            
            let _ = conn.execute(
                "DELETE FROM _warphead_attached_tables WHERE local_table_name = ?",
                [table_name]
            );
        }
        
        return Ok(format!("Table '{}' dropped successfully", table_name));
    }
    
    // Otherwise, handle as physical file
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join(&folder).join(&filename);
    
    if !file_path.exists() {
        return Err(format!("File not found: {}", filename));
    }
    
    fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete file: {}", e))?;
    
    let table_name = filename
        .trim_end_matches(".csv")
        .trim_end_matches(".parquet")
        .trim_end_matches(".json")
        .trim_end_matches(".jsonl")
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect::<String>();
    
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    if let Some(conn) = state_conn.as_ref() {
        let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
        conn.execute(&drop_query, [])
            .map_err(|e| format!("Failed to drop table: {}", e))?;
        
        let _ = remove_table_metadata(conn, &table_name);
    }
    
    Ok(format!("File '{}' deleted successfully", filename))
}

/// Rename a file
#[tauri::command]
pub fn rename_file(app: AppHandle, filename: String, folder: String, new_filename: String) -> Result<String, String> {
    if new_filename.is_empty() {
        return Err("New filename cannot be empty".to_string());
    }
    
    let data_dir = get_data_dir(&app)?;
    let old_path = data_dir.join(&folder).join(&filename);
    let new_path = data_dir.join(&folder).join(&new_filename);
    
    if !old_path.exists() {
        return Err(format!("File not found: {}", filename));
    }
    
    if new_path.exists() {
        return Err(format!("File already exists: {}", new_filename));
    }
    
    fs::rename(&old_path, &new_path)
        .map_err(|e| format!("Failed to rename file: {}", e))?;
    
    Ok(format!("File renamed from '{}' to '{}'", filename, new_filename))
}

