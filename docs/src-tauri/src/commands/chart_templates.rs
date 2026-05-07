use tauri::State;
use crate::state::DuckDbState;
use serde_json;

/// Generate a URL-safe slug from a chart name
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

/// Create or update a chart template
#[tauri::command]
pub fn save_chart_template(
    state: State<DuckDbState>,
    chart_name: String,
    chart_type: String,
    chart_code: String,
    config_schema: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    metrics: Option<String>,
    dimensions: Option<String>,
    sample_data: Option<String>,
    existing_slug: Option<String>,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let _slug = if let Some(existing) = existing_slug {
        // Update existing chart template
        db.execute(
            "UPDATE _warphead_chart_templates 
             SET chart_name = ?, chart_type = ?, chart_code = ?, config_schema = ?, 
                 description = ?, tags = ?, metrics = ?, dimensions = ?, sample_data = ?,
                 updated_at = CURRENT_TIMESTAMP
             WHERE slug = ?",
            [
                &chart_name,
                &chart_type,
                &chart_code,
                &config_schema.unwrap_or_default(),
                &description.unwrap_or_default(),
                &tags.unwrap_or_default(),
                &metrics.unwrap_or_default(),
                &dimensions.unwrap_or_default(),
                &sample_data.unwrap_or_default(),
                &existing,
            ],
        )
        .map_err(|e| format!("Failed to update chart template: {}", e))?;

        existing
    } else {
        // Create new chart template
        let new_slug = generate_slug(&chart_name);

        db.execute(
            "INSERT INTO _warphead_chart_templates 
             (slug, chart_name, chart_type, chart_code, config_schema, description, tags, metrics, dimensions, sample_data)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            [
                &new_slug,
                &chart_name,
                &chart_type,
                &chart_code,
                &config_schema.unwrap_or_default(),
                &description.unwrap_or_default(),
                &tags.unwrap_or_default(),
                &metrics.unwrap_or_default(),
                &dimensions.unwrap_or_default(),
                &sample_data.unwrap_or_default(),
            ],
        )
        .map_err(|e| format!("Failed to create chart template: {}", e))?;

        new_slug
    };

    Ok(format!("Chart template '{}' saved successfully", chart_name))
}

/// List all chart templates
#[tauri::command]
pub fn list_chart_templates(state: State<DuckDbState>) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            "SELECT slug, chart_name, chart_type, chart_code, config_schema, description, tags, 
                    metrics, dimensions, sample_data, 
                    min_metrics, max_metrics, min_dimensions, max_dimensions,
                    CAST(created_at AS VARCHAR) as created_at, 
                    CAST(updated_at AS VARCHAR) as updated_at
             FROM _warphead_chart_templates 
             ORDER BY updated_at DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "slug": row.get::<_, String>(0)?,
                "chart_name": row.get::<_, String>(1)?,
                "chart_type": row.get::<_, String>(2)?,
                "chart_code": row.get::<_, String>(3)?,
                "config_schema": row.get::<_, Option<String>>(4)?,
                "description": row.get::<_, Option<String>>(5)?,
                "tags": row.get::<_, Option<String>>(6)?,
                "metrics": row.get::<_, Option<String>>(7)?,
                "dimensions": row.get::<_, Option<String>>(8)?,
                "sample_data": row.get::<_, Option<String>>(9)?,
                "min_metrics": row.get::<_, Option<i32>>(10)?,
                "max_metrics": row.get::<_, Option<i32>>(11)?,
                "min_dimensions": row.get::<_, Option<i32>>(12)?,
                "max_dimensions": row.get::<_, Option<i32>>(13)?,
                "created_at": row.get::<_, String>(14)?,
                "updated_at": row.get::<_, String>(15)?,
            }))
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    let mut templates = Vec::new();
    for row in rows {
        templates.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
    }

    serde_json::to_string(&templates).map_err(|e| format!("Failed to serialize results: {}", e))
}

/// Get a single chart template by slug
#[tauri::command]
pub fn get_chart_template(
    state: State<DuckDbState>,
    slug: String,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            "SELECT slug, chart_name, chart_type, chart_code, config_schema, description, tags,
                    metrics, dimensions, sample_data, 
                    min_metrics, max_metrics, min_dimensions, max_dimensions,
                    CAST(created_at AS VARCHAR) as created_at, 
                    CAST(updated_at AS VARCHAR) as updated_at
             FROM _warphead_chart_templates 
             WHERE slug = ?",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let template = stmt
        .query_row([&slug], |row| {
            Ok(serde_json::json!({
                "slug": row.get::<_, String>(0)?,
                "chart_name": row.get::<_, String>(1)?,
                "chart_type": row.get::<_, String>(2)?,
                "chart_code": row.get::<_, String>(3)?,
                "config_schema": row.get::<_, Option<String>>(4)?,
                "description": row.get::<_, Option<String>>(5)?,
                "tags": row.get::<_, Option<String>>(6)?,
                "metrics": row.get::<_, Option<String>>(7)?,
                "dimensions": row.get::<_, Option<String>>(8)?,
                "sample_data": row.get::<_, Option<String>>(9)?,
                "min_metrics": row.get::<_, Option<i32>>(10)?,
                "max_metrics": row.get::<_, Option<i32>>(11)?,
                "min_dimensions": row.get::<_, Option<i32>>(12)?,
                "max_dimensions": row.get::<_, Option<i32>>(13)?,
                "created_at": row.get::<_, String>(14)?,
                "updated_at": row.get::<_, String>(15)?,
            }))
        })
        .map_err(|e| format!("Failed to get chart template: {}", e))?;

    serde_json::to_string(&template).map_err(|e| format!("Failed to serialize result: {}", e))
}

/// Delete a chart template
#[tauri::command]
pub fn delete_chart_template(
    state: State<DuckDbState>,
    slug: String,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    db.execute(
        "DELETE FROM _warphead_chart_templates WHERE slug = ?",
        [&slug],
    )
    .map_err(|e| format!("Failed to delete chart template: {}", e))?;

    Ok(format!("Chart template '{}' deleted successfully", slug))
}

