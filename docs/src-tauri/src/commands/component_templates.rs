use tauri::State;
use crate::state::DuckDbState;
use serde_json;

/// Generate a URL-safe slug from a component name
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

/// Create or update a component template
#[tauri::command]
pub fn save_component_template(
    state: State<DuckDbState>,
    component_name: String,
    component_type: String,
    html_code: String,
    css_code: Option<String>,
    js_code: Option<String>,
    config_schema: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    metrics: Option<String>,
    dimensions: Option<String>,
    sample_data: Option<String>,
    frameworks: Option<String>,
    existing_slug: Option<String>,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let _slug = if let Some(existing) = existing_slug {
        // Update existing component template
        db.execute(
            "UPDATE _warphead_component_templates 
             SET component_name = ?, component_type = ?, html_code = ?, css_code = ?, js_code = ?,
                 config_schema = ?, description = ?, tags = ?, metrics = ?, dimensions = ?, 
                 sample_data = ?, frameworks = ?, updated_at = CURRENT_TIMESTAMP
             WHERE slug = ?",
            [
                &component_name,
                &component_type,
                &html_code,
                &css_code.unwrap_or_default(),
                &js_code.unwrap_or_default(),
                &config_schema.unwrap_or_default(),
                &description.unwrap_or_default(),
                &tags.unwrap_or_default(),
                &metrics.unwrap_or_default(),
                &dimensions.unwrap_or_default(),
                &sample_data.unwrap_or_default(),
                &frameworks.unwrap_or_default(),
                &existing,
            ],
        )
        .map_err(|e| format!("Failed to update component template: {}", e))?;

        existing
    } else {
        // Create new component template
        let new_slug = generate_slug(&component_name);

        db.execute(
            "INSERT INTO _warphead_component_templates 
             (slug, component_name, component_type, html_code, css_code, js_code, config_schema, 
              description, tags, metrics, dimensions, sample_data, frameworks)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            [
                &new_slug,
                &component_name,
                &component_type,
                &html_code,
                &css_code.unwrap_or_default(),
                &js_code.unwrap_or_default(),
                &config_schema.unwrap_or_default(),
                &description.unwrap_or_default(),
                &tags.unwrap_or_default(),
                &metrics.unwrap_or_default(),
                &dimensions.unwrap_or_default(),
                &sample_data.unwrap_or_default(),
                &frameworks.unwrap_or_default(),
            ],
        )
        .map_err(|e| format!("Failed to create component template: {}", e))?;

        new_slug
    };

    Ok(format!("Component template '{}' saved successfully", component_name))
}

/// List all component templates
#[tauri::command]
pub fn list_component_templates(state: State<DuckDbState>) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            "SELECT slug, component_name, component_type, html_code, css_code, js_code,
                    config_schema, description, tags, metrics, dimensions, sample_data, frameworks,
                    min_metrics, max_metrics, min_dimensions, max_dimensions,
                    CAST(created_at AS VARCHAR) as created_at, 
                    CAST(updated_at AS VARCHAR) as updated_at
             FROM _warphead_component_templates 
             ORDER BY updated_at DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "slug": row.get::<_, String>(0)?,
                "component_name": row.get::<_, String>(1)?,
                "component_type": row.get::<_, String>(2)?,
                "html_code": row.get::<_, String>(3)?,
                "css_code": row.get::<_, Option<String>>(4)?,
                "js_code": row.get::<_, Option<String>>(5)?,
                "config_schema": row.get::<_, Option<String>>(6)?,
                "description": row.get::<_, Option<String>>(7)?,
                "tags": row.get::<_, Option<String>>(8)?,
                "metrics": row.get::<_, Option<String>>(9)?,
                "dimensions": row.get::<_, Option<String>>(10)?,
                "sample_data": row.get::<_, Option<String>>(11)?,
                "frameworks": row.get::<_, Option<String>>(12)?,
                "min_metrics": row.get::<_, Option<i32>>(13)?,
                "max_metrics": row.get::<_, Option<i32>>(14)?,
                "min_dimensions": row.get::<_, Option<i32>>(15)?,
                "max_dimensions": row.get::<_, Option<i32>>(16)?,
                "created_at": row.get::<_, String>(17)?,
                "updated_at": row.get::<_, String>(18)?,
            }))
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    let mut templates = Vec::new();
    for row in rows {
        templates.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
    }

    serde_json::to_string(&templates).map_err(|e| format!("Failed to serialize results: {}", e))
}

/// Get a single component template by slug
#[tauri::command]
pub fn get_component_template(
    state: State<DuckDbState>,
    slug: String,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            "SELECT slug, component_name, component_type, html_code, css_code, js_code,
                    config_schema, description, tags, metrics, dimensions, sample_data, frameworks,
                    min_metrics, max_metrics, min_dimensions, max_dimensions,
                    CAST(created_at AS VARCHAR) as created_at, 
                    CAST(updated_at AS VARCHAR) as updated_at
             FROM _warphead_component_templates 
             WHERE slug = ?",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let template = stmt
        .query_row([&slug], |row| {
            Ok(serde_json::json!({
                "slug": row.get::<_, String>(0)?,
                "component_name": row.get::<_, String>(1)?,
                "component_type": row.get::<_, String>(2)?,
                "html_code": row.get::<_, String>(3)?,
                "css_code": row.get::<_, Option<String>>(4)?,
                "js_code": row.get::<_, Option<String>>(5)?,
                "config_schema": row.get::<_, Option<String>>(6)?,
                "description": row.get::<_, Option<String>>(7)?,
                "tags": row.get::<_, Option<String>>(8)?,
                "metrics": row.get::<_, Option<String>>(9)?,
                "dimensions": row.get::<_, Option<String>>(10)?,
                "sample_data": row.get::<_, Option<String>>(11)?,
                "frameworks": row.get::<_, Option<String>>(12)?,
                "min_metrics": row.get::<_, Option<i32>>(13)?,
                "max_metrics": row.get::<_, Option<i32>>(14)?,
                "min_dimensions": row.get::<_, Option<i32>>(15)?,
                "max_dimensions": row.get::<_, Option<i32>>(16)?,
                "created_at": row.get::<_, String>(17)?,
                "updated_at": row.get::<_, String>(18)?,
            }))
        })
        .map_err(|e| format!("Failed to get component template: {}", e))?;

    serde_json::to_string(&template).map_err(|e| format!("Failed to serialize result: {}", e))
}

/// Delete a component template
#[tauri::command]
pub fn delete_component_template(
    state: State<DuckDbState>,
    slug: String,
) -> Result<String, String> {
    let conn_guard = state.conn.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
    let db = conn_guard.as_ref().ok_or("Database not initialized")?;

    db.execute(
        "DELETE FROM _warphead_component_templates WHERE slug = ?",
        [&slug],
    )
    .map_err(|e| format!("Failed to delete component template: {}", e))?;

    Ok(format!("Component template '{}' deleted successfully", slug))
}

