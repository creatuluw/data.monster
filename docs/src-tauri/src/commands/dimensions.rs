use tauri::State;
use duckdb::params;
use crate::state::DuckDbState;
use crate::utils::generate_unique_dimension_slug;

/// Create a new dimension
#[tauri::command]
pub fn create_dimension(
    state: State<DuckDbState>,
    dimension_name: String,
    field_name: String,
    source_table: String,
    description: String,
    tags: String,
    existing_slug: Option<String>
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    if dimension_name.trim().is_empty() {
        return Err("Dimension name cannot be empty".to_string());
    }
    
    if field_name.trim().is_empty() {
        return Err("Field name cannot be empty".to_string());
    }
    
    if source_table.trim().is_empty() {
        return Err("Source table cannot be empty".to_string());
    }
    
    let table_check = conn.prepare(&format!(
        "SELECT COUNT(*) FROM _warphead_table_metadata WHERE table_name = '{}' AND table_type IN ('source', 'model')",
        source_table.trim()
    ));
    
    if let Ok(mut stmt) = table_check {
        let count: i64 = stmt.query_row([], |row| row.get(0))
            .unwrap_or(0);
        if count == 0 {
            return Err(format!("Source table '{}' not found in data model", source_table.trim()));
        }
    }
    
    let slug = if let Some(existing) = existing_slug {
        existing
    } else {
        generate_unique_dimension_slug(conn, dimension_name.trim(), None)
    };
    
    conn.execute(
        "INSERT INTO _warphead_dimensions (slug, dimension_name, field_name, source_table, description, tags, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, now())
         ON CONFLICT (slug) DO UPDATE SET
            dimension_name = EXCLUDED.dimension_name,
            field_name = EXCLUDED.field_name,
            source_table = EXCLUDED.source_table,
            description = EXCLUDED.description,
            tags = EXCLUDED.tags,
            updated_at = now()",
        params![
            &slug,
            dimension_name.trim(),
            field_name.trim(),
            source_table.trim(),
            description.trim(),
            tags.trim()
        ]
    ).map_err(|e| format!("Failed to save dimension: {}", e))?;
    
    Ok(format!("Dimension '{}' saved successfully", dimension_name.trim()))
}

/// List all dimensions
#[tauri::command]
pub fn list_dimensions(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = "SELECT slug, dimension_name, field_name, source_table, description, tags, 
                        strftime(created_at, '%Y-%m-%d %H:%M:%S') as created_at,
                        strftime(updated_at, '%Y-%m-%d %H:%M:%S') as updated_at
                 FROM _warphead_dimensions
                 ORDER BY dimension_name";
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let dimensions_result = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "slug": row.get::<_, String>(0)?,
            "dimension_name": row.get::<_, String>(1)?,
            "field_name": row.get::<_, String>(2)?,
            "source_table": row.get::<_, String>(3)?,
            "description": row.get::<_, Option<String>>(4)?,
            "tags": row.get::<_, Option<String>>(5)?,
            "created_at": row.get::<_, String>(6)?,
            "updated_at": row.get::<_, String>(7)?
        }))
    }).map_err(|e| e.to_string())?;
    
    let dimensions: Vec<serde_json::Value> = dimensions_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&dimensions).map_err(|e| e.to_string())
}

/// Delete a dimension
#[tauri::command]
pub fn delete_dimension(state: State<DuckDbState>, dimension_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "DELETE FROM _warphead_dimensions WHERE dimension_name = ?",
        [&dimension_name]
    ).map_err(|e| format!("Failed to remove dimension metadata: {}", e))?;
    
    Ok(format!("Dimension '{}' deleted successfully", dimension_name))
}

