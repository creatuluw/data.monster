use serde::{Deserialize, Serialize};
use tauri::State;

use crate::state::DuckDbState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldFunction {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub sql_template: String,
    pub applies_to: String,
    pub output_type: String,
}

#[tauri::command]
pub fn list_field_functions(state: State<'_, DuckDbState>) -> Result<Vec<FieldFunction>, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare("SELECT id, label, description, sql_template, applies_to, output_type FROM d8a_monster_field_functions ORDER BY id")
        .map_err(|e| e.to_string())?;

    let functions: Vec<FieldFunction> = stmt
        .query_map([], |row| {
            Ok(FieldFunction {
                id: row.get(0)?,
                label: row.get(1)?,
                description: row.get(2)?,
                sql_template: row.get(3)?,
                applies_to: row.get(4)?,
                output_type: row.get::<_, String>(5).unwrap_or_default(),
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(functions)
}

#[tauri::command]
pub fn create_field_function(
    id: String,
    label: String,
    description: Option<String>,
    sql_template: String,
    applies_to: String,
    output_type: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    if label.trim().is_empty() {
        return Err("Label is required".to_string());
    }
    if sql_template.trim().is_empty() {
        return Err("SQL template is required".to_string());
    }
    if applies_to.trim().is_empty() {
        return Err("Applies to type patterns are required".to_string());
    }

    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let existing: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM d8a_monster_field_functions WHERE id = ?",
            duckdb::params![&id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    if existing > 0 {
        return Err(format!("A function with id '{}' already exists", id));
    }

    conn.execute(
        "INSERT INTO d8a_monster_field_functions (id, label, description, sql_template, applies_to, output_type) VALUES (?, ?, ?, ?, ?, ?)",
        duckdb::params![&id, &label.trim(), &description, &sql_template.trim(), &applies_to.trim(), &output_type.trim()],
    )
    .map_err(|e| format!("Failed to create field function: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn update_field_function(
    id: String,
    label: String,
    description: Option<String>,
    sql_template: String,
    applies_to: String,
    output_type: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    if label.trim().is_empty() {
        return Err("Label is required".to_string());
    }
    if sql_template.trim().is_empty() {
        return Err("SQL template is required".to_string());
    }
    if applies_to.trim().is_empty() {
        return Err("Applies to type patterns are required".to_string());
    }

    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let affected = conn.execute(
        "UPDATE d8a_monster_field_functions SET label = ?, description = ?, sql_template = ?, applies_to = ?, output_type = ? WHERE id = ?",
        duckdb::params![&label.trim(), &description, &sql_template.trim(), &applies_to.trim(), &output_type.trim(), &id],
    )
    .map_err(|e| format!("Failed to update field function: {}", e))?;

    if affected == 0 {
        return Err(format!("No function found with id '{}'", id));
    }

    Ok(())
}

#[tauri::command]
pub fn delete_field_function(
    id: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let affected = conn
        .execute(
            "DELETE FROM d8a_monster_field_functions WHERE id = ?",
            duckdb::params![&id],
        )
        .map_err(|e| format!("Failed to delete field function: {}", e))?;

    if affected == 0 {
        return Err(format!("No function found with id '{}'", id));
    }

    Ok(())
}
