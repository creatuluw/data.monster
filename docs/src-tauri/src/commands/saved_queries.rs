use tauri::State;
use crate::state::DuckDbState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedQuery {
    pub slug: String,
    pub query_name: String,
    pub query_sql: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Initialize the saved queries table
pub fn init_saved_queries_table(conn: &duckdb::Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _warphead_saved_queries (
            slug VARCHAR PRIMARY KEY,
            query_name VARCHAR NOT NULL,
            query_sql TEXT NOT NULL,
            description TEXT,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// List all saved queries
#[tauri::command]
pub fn list_saved_queries(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn
        .prepare("SELECT slug, query_name, query_sql, description, tags, 
                  strftime(created_at, '%Y-%m-%d %H:%M:%S') as created_at,
                  strftime(updated_at, '%Y-%m-%d %H:%M:%S') as updated_at 
                  FROM _warphead_saved_queries ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;
    
    let queries: Vec<SavedQuery> = stmt
        .query_map([], |row| {
            Ok(SavedQuery {
                slug: row.get(0)?,
                query_name: row.get(1)?,
                query_sql: row.get(2)?,
                description: row.get(3)?,
                tags: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&queries).map_err(|e| e.to_string())
}

/// Save a new query
#[tauri::command]
pub fn save_query(
    state: State<DuckDbState>,
    slug: String,
    query_name: String,
    query_sql: String,
    description: Option<String>,
    tags: Option<String>,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "INSERT INTO _warphead_saved_queries (slug, query_name, query_sql, description, tags) 
         VALUES (?, ?, ?, ?, ?)",
        duckdb::params![slug, query_name, query_sql, description, tags],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(format!("Query '{}' saved successfully", query_name))
}

/// Update an existing query
#[tauri::command]
pub fn update_saved_query(
    state: State<DuckDbState>,
    slug: String,
    query_name: String,
    query_sql: String,
    description: Option<String>,
    tags: Option<String>,
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "UPDATE _warphead_saved_queries 
         SET query_name = ?, query_sql = ?, description = ?, tags = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE slug = ?",
        duckdb::params![query_name, query_sql, description, tags, slug],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(format!("Query '{}' updated successfully", query_name))
}

/// Delete a saved query
#[tauri::command]
pub fn delete_saved_query(state: State<DuckDbState>, slug: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "DELETE FROM _warphead_saved_queries WHERE slug = ?",
        [&slug],
    )
    .map_err(|e| e.to_string())?;
    
    Ok("Query deleted successfully".to_string())
}

/// Get a single saved query by slug
#[tauri::command]
pub fn get_saved_query(state: State<DuckDbState>, slug: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let mut stmt = conn
        .prepare("SELECT slug, query_name, query_sql, description, tags, 
                  strftime(created_at, '%Y-%m-%d %H:%M:%S') as created_at,
                  strftime(updated_at, '%Y-%m-%d %H:%M:%S') as updated_at 
                  FROM _warphead_saved_queries WHERE slug = ?")
        .map_err(|e| e.to_string())?;
    
    let query = stmt
        .query_row([&slug], |row| {
            Ok(SavedQuery {
                slug: row.get(0)?,
                query_name: row.get(1)?,
                query_sql: row.get(2)?,
                description: row.get(3)?,
                tags: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&query).map_err(|e| e.to_string())
}

