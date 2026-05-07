use tauri::State;
use crate::state::DuckDbState;
use serde::{Deserialize, Serialize};
use duckdb::params;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableIngestionInfo {
    pub table_name: String,
    pub ingestion_strategy: String,
    pub is_ingested: bool,
    pub source_path: Option<String>,
    pub source_type: Option<String>,
    pub ingested_at: Option<String>,
    pub ingested_row_count: Option<i64>,
}

/// Get ingestion info for all tables
#[tauri::command]
pub fn get_tables_ingestion_info(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = "SELECT 
        table_name, 
        COALESCE(ingestion_strategy, 'manual') as ingestion_strategy,
        COALESCE(is_ingested, false) as is_ingested,
        source_path,
        source_type,
        ingested_at,
        ingested_row_count
    FROM _warphead_table_metadata 
    WHERE table_type IN ('source', 'model')
    ORDER BY table_name";
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    
    let mut tables = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        tables.push(TableIngestionInfo {
            table_name: row.get(0).map_err(|e| e.to_string())?,
            ingestion_strategy: row.get(1).map_err(|e| e.to_string())?,
            is_ingested: row.get(2).map_err(|e| e.to_string())?,
            source_path: row.get(3).ok(),
            source_type: row.get(4).ok(),
            ingested_at: row.get(5).ok(),
            ingested_row_count: row.get(6).ok(),
        });
    }
    
    serde_json::to_string(&tables).map_err(|e| e.to_string())
}

/// Set ingestion strategy for a table
#[tauri::command]
pub fn set_ingestion_strategy(
    state: State<DuckDbState>,
    table_name: String,
    strategy: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Validate strategy
    let valid_strategies = vec!["manual", "on_app_load", "on_query"];
    if !valid_strategies.contains(&strategy.as_str()) {
        return Err(format!("Invalid strategy: {}. Must be one of: manual, on_app_load, on_query", strategy));
    }
    
    conn.execute(
        "UPDATE _warphead_table_metadata SET ingestion_strategy = ? WHERE table_name = ?",
        params![&strategy, &table_name],
    ).map_err(|e| e.to_string())?;
    
    Ok(format!("Ingestion strategy for '{}' set to '{}'", table_name, strategy))
}

/// Ingest a table (load data from source)
#[tauri::command]
pub fn ingest_table(
    state: State<DuckDbState>,
    table_name: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Get source info from metadata
    let query = "SELECT source_path, source_type, creation_query FROM _warphead_table_metadata WHERE table_name = ?";
    let result: Result<(Option<String>, Option<String>, Option<String>), _> = conn.query_row(
        query,
        params![&table_name],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    );
    
    let (source_path, source_type, creation_query) = result.map_err(|e| format!("Table not found in metadata: {}", e))?;
    
    // Check if table already exists and drop it
    let table_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = ?",
        params![&table_name],
        |row| row.get(0)
    );
    
    if table_exists.map(|c| c > 0).unwrap_or(false) {
        conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), [])
            .map_err(|e| format!("Failed to drop existing table: {}", e))?;
    }
    
    let start_time = std::time::Instant::now();
    
    // Ingest based on source type
    let row_count = if let Some(source_path) = source_path {
        let source_type = source_type.as_deref().unwrap_or("csv");
        let normalized_path = source_path.replace("\\", "/");
        
        let ingest_query = match source_type {
            "csv" => format!("CREATE TABLE {} AS SELECT * FROM read_csv_auto('{}')", table_name, normalized_path),
            "parquet" => format!("CREATE TABLE {} AS SELECT * FROM read_parquet('{}')", table_name, normalized_path),
            "json" => format!("CREATE TABLE {} AS SELECT * FROM read_json_auto('{}')", table_name, normalized_path),
            _ => return Err(format!("Unsupported source type: {}", source_type)),
        };
        
        conn.execute(&ingest_query, []).map_err(|e| format!("Failed to ingest data: {}", e))?;
        
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        count
    } else if let Some(creation_query) = creation_query {
        // Use creation query if available (for model tables)
        let create_query = format!("CREATE TABLE {} AS {}", table_name, creation_query);
        conn.execute(&create_query, []).map_err(|e| format!("Failed to create table from query: {}", e))?;
        
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        count
    } else {
        return Err("No source path or creation query found for table".to_string());
    };
    
    let duration_ms = start_time.elapsed().as_millis() as i64;
    
    // Update metadata
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE _warphead_table_metadata SET is_ingested = true, ingested_at = ?, ingested_row_count = ? WHERE table_name = ?",
        params![&now, &row_count, &table_name],
    ).map_err(|e| format!("Failed to update metadata: {}", e))?;
    
    Ok(format!("Ingested {} rows into table '{}' in {}ms", row_count, table_name, duration_ms))
}

/// Register a source file for ingestion
#[tauri::command]
pub fn register_source_for_ingestion(
    state: State<DuckDbState>,
    table_name: String,
    source_path: String,
    source_type: String,
    ingestion_strategy: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Validate strategy
    let valid_strategies = vec!["manual", "on_app_load", "on_query"];
    if !valid_strategies.contains(&ingestion_strategy.as_str()) {
        return Err(format!("Invalid strategy: {}", ingestion_strategy));
    }
    
    // Insert or update metadata
    conn.execute(
        "INSERT INTO _warphead_table_metadata (table_name, table_type, source_path, source_type, ingestion_strategy, is_ingested) 
         VALUES (?, 'source', ?, ?, ?, false)
         ON CONFLICT (table_name) DO UPDATE SET 
            source_path = EXCLUDED.source_path, 
            source_type = EXCLUDED.source_type, 
            ingestion_strategy = EXCLUDED.ingestion_strategy",
        params![&table_name, &source_path, &source_type, &ingestion_strategy],
    ).map_err(|e| format!("Failed to register source: {}", e))?;
    
    Ok(format!("Source registered for table '{}' with strategy '{}'", table_name, ingestion_strategy))
}

/// Load all tables with 'on_app_load' strategy
#[tauri::command]
pub fn load_on_app_load_tables(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Get all tables with on_app_load strategy
    let query = "SELECT table_name FROM _warphead_table_metadata WHERE ingestion_strategy = 'on_app_load'";
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    
    let mut tables = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        tables.push(row.get::<_, String>(0).map_err(|e| e.to_string())?);
    }
    
    drop(rows);
    drop(stmt);
    drop(state_conn);
    
    let mut loaded = Vec::new();
    let mut errors = Vec::new();
    
    for table in tables {
        match ingest_table(state.clone(), table.clone()) {
            Ok(msg) => loaded.push(msg),
            Err(e) => errors.push(format!("{}: {}", table, e)),
        }
    }
    
    let mut result = format!("Loaded {} tables on app load", loaded.len());
    if !errors.is_empty() {
        result.push_str(&format!("\nErrors: {}", errors.join(", ")));
    }
    
    Ok(result)
}

/// Check if a table needs ingestion before query
#[tauri::command]
pub fn ensure_table_ingested(
    state: State<DuckDbState>,
    table_name: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Check if table is already ingested
    let query = "SELECT is_ingested, ingestion_strategy FROM _warphead_table_metadata WHERE table_name = ?";
    let result: Result<(bool, String), _> = conn.query_row(
        query,
        params![&table_name],
        |row| Ok((row.get(0)?, row.get(1)?))
    );
    
    match result {
        Ok((is_ingested, strategy)) => {
            if !is_ingested && strategy == "on_query" {
                drop(state_conn);
                ingest_table(state, table_name.clone())?;
                Ok(format!("Table '{}' ingested on query", table_name))
            } else if !is_ingested {
                Err(format!("Table '{}' is not ingested. Please ingest it manually or set ingestion strategy.", table_name))
            } else {
                Ok(format!("Table '{}' is already ingested", table_name))
            }
        },
        Err(_) => Ok(format!("Table '{}' not found in metadata (assuming pre-existing table)", table_name))
    }
}

