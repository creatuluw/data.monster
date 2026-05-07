use tauri::State;
use duckdb::params;
use uuid::Uuid;
use chrono::Utc;
use crate::state::DuckDbState;
use crate::models::{PostgresConnection, PostgresTable};
use crate::utils::register_table_metadata;

/// Create a new PostgreSQL connection using DuckDB secrets
#[tauri::command]
pub fn create_postgres_connection(
    state: State<DuckDbState>,
    connection_name: String,
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    _ssl_mode: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let connection_id = Uuid::new_v4().to_string();
    let secret_name = format!("pg_{}", Uuid::new_v4().to_string().replace("-", "_"));
    
    // Create persistent DuckDB secret
    let create_secret_sql = format!(
        "CREATE PERSISTENT SECRET {} (TYPE POSTGRES, HOST '{}', PORT {}, DATABASE '{}', USER '{}', PASSWORD '{}')",
        secret_name, host, port, database, username, password
    );
    
    conn.execute(&create_secret_sql, [])
        .map_err(|e| format!("Failed to create secret: {}", e))?;
    
    // Test connection
    let test_attach_sql = format!(
        "ATTACH 'dbname={}' AS test_pg (TYPE postgres, SECRET {})",
        database, secret_name
    );
    conn.execute(&test_attach_sql, [])
        .map_err(|e| {
            let _ = conn.execute(&format!("DROP SECRET IF EXISTS {}", secret_name), []);
            format!("Connection test failed: {}", e)
        })?;
    
    let _ = conn.execute("DETACH test_pg", []);
    
    // Save connection metadata
    conn.execute(
        "INSERT INTO _warphead_connections 
         (connection_id, connection_name, connection_type, secret_name, is_attached, attach_mode, created_at) 
         VALUES (?, ?, 'postgresql', ?, false, 'hybrid', CURRENT_TIMESTAMP)",
        params![&connection_id, &connection_name, &secret_name]
    ).map_err(|e| {
        let _ = conn.execute(&format!("DROP SECRET IF EXISTS {}", secret_name), []);
        format!("Failed to save connection: {}", e)
    })?;
    
    Ok(connection_id)
}

/// List all saved PostgreSQL connections
#[tauri::command]
pub fn list_postgres_connections(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn.prepare(
        "SELECT connection_id, connection_name, connection_type, secret_name, is_attached, 
                attach_mode, strftime(created_at, '%Y-%m-%d %H:%M:%S'), 
                strftime(last_used_at, '%Y-%m-%d %H:%M:%S')
         FROM _warphead_connections 
         ORDER BY created_at DESC"
    ).map_err(|e| e.to_string())?;
    
    let connections_result = stmt.query_map([], |row| {
        Ok(PostgresConnection {
            connection_id: row.get(0)?,
            connection_name: row.get(1)?,
            connection_type: row.get(2)?,
            secret_name: row.get(3)?,
            is_attached: row.get(4)?,
            attach_mode: row.get(5)?,
            created_at: row.get(6)?,
            last_used_at: row.get(7)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let connections: Vec<PostgresConnection> = connections_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&connections).map_err(|e| e.to_string())
}

/// Test a PostgreSQL connection
#[tauri::command]
pub fn test_postgres_connection(state: State<DuckDbState>, connection_id: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let (secret_name, connection_name): (String, String) = conn.query_row(
        "SELECT secret_name, connection_name FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id],
        |row| Ok((row.get(0)?, row.get(1)?))
    ).map_err(|e| format!("Connection not found: {}", e))?;
    
    let test_attach_sql = format!(
        "ATTACH '' AS test_pg (TYPE postgres, SECRET {})",
        secret_name
    );
    conn.execute(&test_attach_sql, [])
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("Secret") && err_str.contains("not found") {
                format!("Connection credentials expired or lost. Please delete this connection and create a new one.")
            } else {
                format!("Connection test failed: {}", err_str)
            }
        })?;
    
    let _ = conn.execute("DETACH test_pg", []);
    
    conn.execute(
        "UPDATE _warphead_connections SET last_used_at = CURRENT_TIMESTAMP WHERE connection_id = ?",
        [&connection_id]
    ).map_err(|e| format!("Failed to update connection: {}", e))?;
    
    Ok(format!("Connection '{}' tested successfully", connection_name))
}

/// Delete a PostgreSQL connection
#[tauri::command]
pub fn delete_postgres_connection(state: State<DuckDbState>, connection_id: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let (secret_name, connection_name, is_attached): (String, String, bool) = conn.query_row(
        "SELECT secret_name, connection_name, is_attached FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    ).map_err(|e| format!("Connection not found: {}", e))?;
    
    if is_attached {
        let _ = conn.execute(&format!("DETACH {}", connection_name), []);
    }
    
    let _ = conn.execute(&format!("DROP SECRET IF EXISTS {}", secret_name), []);
    
    conn.execute(
        "DELETE FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id]
    ).map_err(|e| format!("Failed to delete connection: {}", e))?;
    
    Ok(format!("Connection '{}' deleted successfully", connection_name))
}

/// Attach a PostgreSQL database
#[tauri::command]
pub fn attach_postgres_database(state: State<DuckDbState>, connection_id: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let (secret_name, connection_name, is_attached): (String, String, bool) = conn.query_row(
        "SELECT secret_name, connection_name, is_attached FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    ).map_err(|e| format!("Connection not found: {}", e))?;
    
    let actually_attached: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM duckdb_databases() WHERE database_name = ?",
        [&connection_name],
        |row| row.get(0)
    ).unwrap_or(false);
    
    if is_attached && actually_attached {
        return Ok(format!("Connection '{}' is already attached", connection_name));
    }
    
    let attach_sql = format!(
        "ATTACH '' AS {} (TYPE postgres, SECRET {})",
        connection_name, secret_name
    );
    conn.execute(&attach_sql, [])
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("Secret") && err_str.contains("not found") {
                format!("Connection credentials expired or lost. Please delete this connection and create a new one.")
            } else {
                format!("Failed to attach database: {}", err_str)
            }
        })?;
    
    conn.execute(
        "UPDATE _warphead_connections SET is_attached = true, last_used_at = CURRENT_TIMESTAMP WHERE connection_id = ?",
        [&connection_id]
    ).map_err(|e| format!("Failed to update connection: {}", e))?;
    
    Ok(format!("Database '{}' attached successfully", connection_name))
}

/// Detach a PostgreSQL database
#[tauri::command]
pub fn detach_postgres_database(state: State<DuckDbState>, connection_id: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let (connection_name, is_attached): (String, bool) = conn.query_row(
        "SELECT connection_name, is_attached FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id],
        |row| Ok((row.get(0)?, row.get(1)?))
    ).map_err(|e| format!("Connection not found: {}", e))?;
    
    if !is_attached {
        return Ok(format!("Connection '{}' is not attached", connection_name));
    }
    
    conn.execute(&format!("DETACH {}", connection_name), [])
        .map_err(|e| format!("Failed to detach database: {}", e))?;
    
    conn.execute(
        "UPDATE _warphead_connections SET is_attached = false WHERE connection_id = ?",
        [&connection_id]
    ).map_err(|e| format!("Failed to update connection: {}", e))?;
    
    Ok(format!("Database '{}' detached successfully", connection_name))
}

/// Browse schemas in attached PostgreSQL database
#[tauri::command]
pub fn browse_postgres_schemas(state: State<DuckDbState>, connection_id: String) -> Result<String, String> {
    let connection_name: String = {
        let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
        let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
        
        conn.query_row(
            "SELECT connection_name FROM _warphead_connections WHERE connection_id = ?",
            [&connection_id],
            |row| row.get(0)
        ).map_err(|e| format!("Connection not found: {}", e))?
    };
    
    attach_postgres_database(state.clone(), connection_id.clone())?;
    
    let query = format!(
        "SELECT schema_name FROM {}.information_schema.schemata 
         WHERE schema_name NOT IN ('information_schema', 'pg_catalog', 'pg_toast', 'pg_temp_1', 'pg_toast_temp_1')
         ORDER BY schema_name",
        connection_name
    );
    
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let schemas: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&schemas).map_err(|e| e.to_string())
}

/// Browse tables in a PostgreSQL schema
#[tauri::command]
pub fn browse_postgres_tables(
    state: State<DuckDbState>,
    connection_id: String,
    schema_name: String
) -> Result<String, String> {
    let connection_name: String = {
        let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
        let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
        
        conn.query_row(
            "SELECT connection_name FROM _warphead_connections WHERE connection_id = ?",
            [&connection_id],
            |row| row.get(0)
        ).map_err(|e| format!("Connection not found: {}", e))?
    };
    
    attach_postgres_database(state.clone(), connection_id.clone())?;
    
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = format!(
        "SELECT table_name FROM {}.information_schema.tables 
         WHERE table_schema = '{}'
         AND table_type = 'BASE TABLE'
         ORDER BY table_name",
        connection_name, schema_name
    );
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    let tables: Vec<PostgresTable> = table_names.iter().map(|name| {
        PostgresTable {
            table_name: name.clone(),
            schema_name: schema_name.clone(),
            row_count: None,
        }
    }).collect();
    
    serde_json::to_string(&tables).map_err(|e| e.to_string())
}

/// Import a PostgreSQL table into DuckDB
#[tauri::command]
pub fn import_postgres_table(
    state: State<DuckDbState>,
    connection_id: String,
    schema_name: String,
    table_name: String,
    local_table_name: String,
    access_mode: String,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let connection_name: String = conn.query_row(
        "SELECT connection_name FROM _warphead_connections WHERE connection_id = ?",
        [&connection_id],
        |row| row.get(0)
    ).map_err(|e| format!("Connection not found: {}", e))?;
    
    let import_id = Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();
    
    let full_source = format!("{}.{}.{}", connection_name, schema_name, table_name);
    
    match access_mode.as_str() {
        "live" => {
            let table_id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO _warphead_attached_tables 
                 (table_id, connection_id, schema_name, table_name, full_name, access_mode) 
                 VALUES (?, ?, ?, ?, ?, 'live')",
                params![&table_id, &connection_id, &schema_name, &table_name, &full_source]
            ).map_err(|e| format!("Failed to register table: {}", e))?;
            
            Ok(format!("Table '{}' registered in live mode (no data imported)", local_table_name))
        },
        "cached" | "imported" => {
            let import_sql = format!("CREATE TABLE {} AS SELECT * FROM {}", local_table_name, full_source);
            conn.execute(&import_sql, [])
                .map_err(|e| format!("Failed to import table: {}", e))?;
            
            let row_count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", local_table_name),
                [],
                |row| row.get(0)
            ).unwrap_or(0);
            
            let duration_ms = start_time.elapsed().as_millis() as i64;
            
            register_table_metadata(conn, &local_table_name, "source")?;
            
            let table_id = Uuid::new_v4().to_string();
            let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            
            conn.execute(
                "INSERT INTO _warphead_attached_tables 
                 (table_id, connection_id, schema_name, table_name, full_name, access_mode, local_table_name, cached_at, cache_row_count) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![&table_id, &connection_id, &schema_name, &table_name, &full_source, &access_mode, &local_table_name, &current_time, &row_count]
            ).map_err(|e| format!("Failed to register table: {}", e))?;
            
            conn.execute(
                "INSERT INTO _warphead_import_log 
                 (import_id, connection_id, source_table, target_table, rows_imported, import_duration_ms, status) 
                 VALUES (?, ?, ?, ?, ?, ?, 'success')",
                params![&import_id, &connection_id, &full_source, &local_table_name, &row_count, &duration_ms]
            ).map_err(|e| format!("Failed to log import: {}", e))?;
            
            Ok(format!("Imported {} rows into '{}' in {}ms", row_count, local_table_name, duration_ms))
        },
        _ => Err(format!("Invalid access mode: {}", access_mode))
    }
}

/// Get all attached tables as JSON
#[tauri::command]
pub fn get_attached_tables(state: State<DuckDbState>) -> Result<String, String> {
    use crate::models::AttachedTableInfo;
    
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = "SELECT table_id, connection_id, schema_name, table_name, full_name, access_mode, local_table_name, 
                        CAST(cached_at AS VARCHAR) as cached_at, cache_row_count 
                 FROM _warphead_attached_tables 
                 ORDER BY COALESCE(cached_at, '1970-01-01 00:00:00') DESC";
    
    let mut stmt = conn.prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let tables: Vec<AttachedTableInfo> = stmt.query_map([], |row| {
        Ok(AttachedTableInfo {
            table_id: row.get(0)?,
            connection_id: row.get(1)?,
            schema_name: row.get(2)?,
            table_name: row.get(3)?,
            full_name: row.get(4)?,
            access_mode: row.get(5)?,
            local_table_name: row.get(6)?,
            cached_at: row.get(7)?,
            cache_row_count: row.get(8)?,
        })
    })
    .map_err(|e| format!("Failed to query: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect results: {}", e))?;
    
    serde_json::to_string(&tables).map_err(|e| format!("Failed to serialize: {}", e))
}

