use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;

#[tauri::command]
pub fn list_master_items(
    kind: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    eprintln!("[items] Listing (kind: {:?})", kind);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(match &kind {
            Some(_) => "SELECT id, kind, table_name, expr, label, fmt, description, CAST(created_at AS VARCHAR), CAST(updated_at AS VARCHAR) FROM d8a_monster_items WHERE kind = ? ORDER BY updated_at DESC",
            None => "SELECT id, kind, table_name, expr, label, fmt, description, CAST(created_at AS VARCHAR), CAST(updated_at AS VARCHAR) FROM d8a_monster_items ORDER BY updated_at DESC",
        })
        .map_err(|e| e.to_string())?;

    let mapper = |row: &duckdb::Row| -> duckdb::Result<serde_json::Value> {
        let id: String = row.get(0)?;
        let kind: String = row.get(1)?;
        let table_name: String = row.get(2)?;
        let expr: String = row.get(3)?;
        let label: String = row.get(4)?;
        let fmt: Option<String> = row.get(5)?;
        let description: Option<String> = row.get(6)?;
        let created_at: Option<String> = row.get(7)?;
        let updated_at: Option<String> = row.get(8)?;
        Ok(json!({
            "id": id,
            "kind": kind,
            "tableName": table_name,
            "expr": expr,
            "label": label,
            "fmt": fmt,
            "description": description,
            "createdAt": created_at,
            "updatedAt": updated_at
        }))
    };

    let items: Vec<serde_json::Value> = match &kind {
        Some(k) => stmt
            .query_map(duckdb::params![k], mapper)
            .map_err(|e| e.to_string())?,
        None => stmt
            .query_map([], mapper)
            .map_err(|e| e.to_string())?,
    }
    .filter_map(|r| r.ok())
    .collect();

    Ok(json!({ "items": items }))
}

#[tauri::command]
pub fn get_master_item(
    id: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    eprintln!("[items] Getting '{}'", id);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.query_row(
        "SELECT id, kind, table_name, expr, label, fmt, description, CAST(created_at AS VARCHAR), CAST(updated_at AS VARCHAR) FROM d8a_monster_items WHERE id = ?",
        duckdb::params![&id],
        |row| {
            let id: String = row.get(0)?;
            let kind: String = row.get(1)?;
            let table_name: String = row.get(2)?;
            let expr: String = row.get(3)?;
            let label: String = row.get(4)?;
            let fmt: Option<String> = row.get(5)?;
            let description: Option<String> = row.get(6)?;
            let created_at: Option<String> = row.get(7)?;
            let updated_at: Option<String> = row.get(8)?;
            Ok(json!({
                "id": id,
                "kind": kind,
                "tableName": table_name,
                "expr": expr,
                "label": label,
                "fmt": fmt,
                "description": description,
                "createdAt": created_at,
                "updatedAt": updated_at
            }))
        },
    )
    .map_err(|_| "Master item not found".to_string())
}

#[tauri::command]
pub fn save_master_item(
    id: String,
    kind: String,
    table_name: String,
    expr: String,
    label: String,
    fmt: Option<String>,
    description: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    eprintln!("[items] Saving '{}' ({})", id, kind);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "INSERT OR REPLACE INTO d8a_monster_items (id, kind, table_name, expr, label, fmt, description, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_items WHERE id = ?), CURRENT_TIMESTAMP), CURRENT_TIMESTAMP)",
        duckdb::params![&id, &kind, &table_name, &expr, &label, &fmt, &description, &id],
    )
    .map_err(|e| format!("Failed to save master item: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_master_item(id: String, state: State<'_, DuckDbState>) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "DELETE FROM d8a_monster_items WHERE id = ?",
        duckdb::params![&id],
    )
    .map_err(|e| format!("Failed to delete master item: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn
    }

    #[test]
    fn test_upsert_preserves_created_at() {
        let conn = setup();
        conn.execute(
            "INSERT INTO d8a_monster_items (id, kind, table_name, expr, label, fmt, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, TIMESTAMP '2020-01-01 00:00:00', TIMESTAMP '2020-01-01 00:00:00')",
            duckdb::params!["rev", "measure", "orders", "SUM(amount)", "Revenue", Option::<String>::None, Option::<String>::None],
        ).unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO d8a_monster_items (id, kind, table_name, expr, label, fmt, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_items WHERE id = ?), CURRENT_TIMESTAMP), CURRENT_TIMESTAMP)",
            duckdb::params!["rev", "measure", "orders", "SUM(amount)", "Revenue v2", "#.##", "desc", "rev"],
        ).unwrap();

        let (label, created_at): (String, String) = conn.query_row(
            "SELECT label, CAST(created_at AS VARCHAR) FROM d8a_monster_items WHERE id = ?",
            duckdb::params!["rev"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        assert_eq!(label, "Revenue v2");
        assert!(created_at.starts_with("2020-01-01"), "created_at should be preserved, got {}", created_at);
    }
}
