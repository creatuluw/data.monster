use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;

#[tauri::command]
pub fn save_table_labels(
    table_name: String,
    tags: String,
    group: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "INSERT INTO d8a_monster_table_labels (table_name, tags, \"group\") VALUES (?, ?, ?)
         ON CONFLICT (table_name) DO UPDATE SET tags = EXCLUDED.tags, \"group\" = EXCLUDED.\"group\"",
        duckdb::params![&table_name, &tags, &group],
    )
    .map_err(|e| format!("Failed to save labels: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_table_labels(
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let result = conn.query_row(
        "SELECT tags, \"group\" FROM d8a_monster_table_labels WHERE table_name = ?",
        duckdb::params![&table_name],
        |row| {
            let tags_str: Option<String> = row.get(0)?;
            let group: Option<String> = row.get(1)?;
            Ok((tags_str, group))
        },
    );

    match result {
        Ok((tags_str, group)) => {
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            Ok(json!({
                "tableName": table_name,
                "tags": tags,
                "group": group
            }))
        }
        Err(_) => Ok(json!({
            "tableName": table_name,
            "tags": [],
            "group": null
        })),
    }
}

#[tauri::command]
pub fn get_all_tags(state: State<'_, DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare("SELECT tags FROM d8a_monster_table_labels WHERE tags IS NOT NULL AND tags != ''")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            let tags: String = row.get(0)?;
            Ok(tags)
        })
        .map_err(|e| e.to_string())?;

    let mut all_tags: Vec<String> = Vec::new();
    for row in rows {
        let tags_str = row.map_err(|e| e.to_string())?;
        for tag in tags_str.split(',') {
            let trimmed = tag.trim().to_string();
            if !trimmed.is_empty() && !all_tags.contains(&trimmed) {
                all_tags.push(trimmed);
            }
        }
    }

    all_tags.sort();
    Ok(all_tags)
}

#[tauri::command]
pub fn get_all_groups(state: State<'_, DuckDbState>) -> Result<Vec<String>, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT \"group\" FROM d8a_monster_table_labels WHERE \"group\" IS NOT NULL AND \"group\" != '' ORDER BY \"group\"",
        )
        .map_err(|e| e.to_string())?;

    let groups: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(groups)
}
