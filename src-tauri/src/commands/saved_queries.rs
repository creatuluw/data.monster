use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::slugs::generate_slug;

#[tauri::command]
pub fn list_saved_queries(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT slug, query_name, query_sql, description, tags, created_at, updated_at FROM d8a_monster_saved_queries ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let queries: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let slug: String = row.get(0)?;
            let name: String = row.get(1)?;
            let sql: String = row.get(2)?;
            let description: Option<String> = row.get(3)?;
            let tags: Option<String> = row.get(4)?;
            let created_at: Option<String> = row.get(5)?;
            let updated_at: Option<String> = row.get(6)?;
            Ok(json!({
                "slug": slug,
                "name": name,
                "sql": sql,
                "description": description,
                "tags": tags,
                "createdAt": created_at,
                "updatedAt": updated_at
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "queries": queries }))
}

#[tauri::command]
pub fn save_query(
    name: String,
    sql: String,
    description: Option<String>,
    tags: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let slug = generate_slug(&name);

    conn.execute(
        "INSERT INTO d8a_monster_saved_queries (slug, query_name, query_sql, description, tags, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        duckdb::params![&slug, &name, &sql, &description, &tags],
    )
    .map_err(|e| format!("Failed to save query: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn update_saved_query(
    slug: String,
    name: Option<String>,
    sql: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let existing: (String, String, Option<String>, Option<String>) = conn
        .query_row(
            "SELECT query_name, query_sql, description, tags FROM d8a_monster_saved_queries WHERE slug = ?",
            duckdb::params![&slug],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|e| format!("Query not found: {}", e))?;

    let new_name = name.unwrap_or(existing.0);
    let new_sql = sql.unwrap_or(existing.1);
    let new_desc = description.or(existing.2);
    let new_tags = tags.or(existing.3);

    conn.execute(
        "UPDATE d8a_monster_saved_queries SET query_name = ?, query_sql = ?, description = ?, tags = ?, updated_at = CURRENT_TIMESTAMP WHERE slug = ?",
        duckdb::params![&new_name, &new_sql, &new_desc, &new_tags, &slug],
    )
    .map_err(|e| format!("Failed to update query: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_saved_query(slug: String, state: State<'_, DuckDbState>) -> Result<(), String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "DELETE FROM d8a_monster_saved_queries WHERE slug = ?",
        duckdb::params![&slug],
    )
    .map_err(|e| format!("Failed to delete query: {}", e))?;

    Ok(())
}
