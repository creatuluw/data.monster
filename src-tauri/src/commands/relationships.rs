use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::slugs::generate_slug;

#[tauri::command]
pub fn list_relationships(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    eprintln!("[relationships] Listing");
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT id, from_table, from_column, to_table, to_column, CAST(created_at AS VARCHAR) FROM d8a_monster_relationships ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let relationships: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let from_table: String = row.get(1)?;
            let from_column: String = row.get(2)?;
            let to_table: String = row.get(3)?;
            let to_column: String = row.get(4)?;
            let created_at: Option<String> = row.get(5)?;
            Ok(json!({
                "id": id,
                "fromTable": from_table,
                "fromColumn": from_column,
                "toTable": to_table,
                "toColumn": to_column,
                "createdAt": created_at
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "relationships": relationships }))
}

#[tauri::command]
pub fn save_relationship(
    from_table: String,
    from_column: String,
    to_table: String,
    to_column: String,
    id: Option<String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let id = match id {
        Some(id) if !id.is_empty() => id,
        _ => generate_slug(&format!("rel {} {}", from_table, to_table)),
    };
    eprintln!("[relationships] Saving '{}'", id);
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "INSERT OR REPLACE INTO d8a_monster_relationships (id, from_table, from_column, to_table, to_column, created_at)
         VALUES (?, ?, ?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_relationships WHERE id = ?), CURRENT_TIMESTAMP))",
        duckdb::params![&id, &from_table, &from_column, &to_table, &to_column, &id],
    )
    .map_err(|e| format!("Failed to save relationship: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_relationship(id: String, state: State<'_, DuckDbState>) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    conn.execute(
        "DELETE FROM d8a_monster_relationships WHERE id = ?",
        duckdb::params![&id],
    )
    .map_err(|e| format!("Failed to delete relationship: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
    use crate::utils::slugs::generate_slug;
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        conn
    }

    #[test]
    fn test_generated_id_format() {
        assert_eq!(
            generate_slug(&format!("rel {} {}", "orders", "customers")),
            "rel-orders-customers"
        );
    }

    #[test]
    fn test_upsert_reuses_created_at() {
        let conn = setup();
        let id = generate_slug(&format!("rel {} {}", "orders", "customers"));
        conn.execute(
            "INSERT INTO d8a_monster_relationships (id, from_table, from_column, to_table, to_column, created_at)
             VALUES (?, ?, ?, ?, ?, TIMESTAMP '2020-01-01 00:00:00')",
            duckdb::params![&id, "orders", "customer_id", "customers", "id"],
        ).unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO d8a_monster_relationships (id, from_table, from_column, to_table, to_column, created_at)
             VALUES (?, ?, ?, ?, ?, COALESCE((SELECT created_at FROM d8a_monster_relationships WHERE id = ?), CURRENT_TIMESTAMP))",
            duckdb::params![&id, "orders", "cust_id", "customers", "id", &id],
        ).unwrap();

        let (from_column, created_at): (String, String) = conn.query_row(
            "SELECT from_column, CAST(created_at AS VARCHAR) FROM d8a_monster_relationships WHERE id = ?",
            duckdb::params![&id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        assert_eq!(from_column, "cust_id");
        assert!(created_at.starts_with("2020-01-01"), "created_at should be reused, got {}", created_at);
    }
}
