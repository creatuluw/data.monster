use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::formatting::format_duckdb_value;

#[tauri::command]
pub fn list_internal_tables(state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' AND table_name NOT LIKE 'd8a_monster_%' ORDER BY table_name",
        )
        .map_err(|e| e.to_string())?;

    let user_tables: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(json!({ "name": name }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut stmt2 = conn
        .prepare(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' AND table_name LIKE 'd8a_monster_%' ORDER BY table_name",
        )
        .map_err(|e| e.to_string())?;

    let meta_tables: Vec<serde_json::Value> = stmt2
        .query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(json!({ "name": name }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let all_tables: Vec<serde_json::Value> = user_tables.into_iter().chain(meta_tables.into_iter()).collect();

    let mut result = Vec::new();
    for table_val in &all_tables {
        let table_name = table_val["name"].as_str().unwrap();
        let is_internal = table_name.starts_with("d8a_monster_");
        let count: i64 = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM \"{}\"", table_name.replace('"', "\"\"")),
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let mut col_stmt = conn
            .prepare(
                "SELECT column_name, data_type FROM information_schema.columns WHERE table_schema = 'main' AND table_name = ? ORDER BY ordinal_position",
            )
            .map_err(|e| e.to_string())?;

        let columns: Vec<serde_json::Value> = col_stmt
            .query_map([table_name], |row| {
                let name: String = row.get(0)?;
                let dtype: String = row.get(1)?;
                Ok(json!({ "name": name, "type": dtype }))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        result.push(json!({
            "name": table_name,
            "rowCount": count,
            "columns": columns,
            "isInternal": is_internal,
        }));
    }

    Ok(json!({ "tables": result }))
}

#[tauri::command]
pub fn query_internal_table(
    table_name: String,
    page: i64,
    page_size: i64,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sanitized = table_name.replace('"', "\"\"");
    let offset = (page - 1) * page_size;

    let count: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM \"{}\"", sanitized),
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(&format!("SELECT * FROM \"{}\" LIMIT ? OFFSET ?", sanitized))
        .map_err(|e| e.to_string())?;

    let mut column_names: Vec<String> = Vec::new();
    let mut first_row = true;
    let mut rows: Vec<serde_json::Value> = Vec::new();

    let result_iter = stmt
        .query_map(duckdb::params![page_size, offset], |row| {
            let column_count = row.as_ref().column_count();

            if first_row {
                for i in 0..column_count {
                    let name = row
                        .as_ref()
                        .column_name(i)
                        .map_or(format!("col_{}", i), |v| v.to_string());
                    column_names.push(name);
                }
                first_row = false;
            }

            let mut map = serde_json::Map::new();
            for i in 0..column_count {
                let val = match row.get_ref(i) {
                    Ok(v) => format_duckdb_value(v),
                    Err(_) => serde_json::Value::Null,
                };
                map.insert(column_names[i].clone(), val);
            }
            Ok(json!(map))
        })
        .map_err(|e| e.to_string())?;

    for r in result_iter {
        if let Ok(val) = r {
            rows.push(val);
        }
    }

    Ok(json!({
        "columns": column_names,
        "rows": rows,
        "totalRows": count,
        "page": page,
        "pageSize": page_size,
    }))
}

#[tauri::command]
pub fn update_internal_row(
    table_name: String,
    pk_column: String,
    pk_value: String,
    updates: std::collections::HashMap<String, String>,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sanitized_table = table_name.replace('"', "\"\"");
    let sanitized_pk = pk_column.replace('"', "\"\"");

    let set_clauses: Vec<String> = updates
        .keys()
        .filter(|k| *k != &pk_column)
        .map(|k| format!("\"{}\" = ?", k.replace('"', "\"\"")))
        .collect();

    if set_clauses.is_empty() {
        return Ok(());
    }

    let sql = format!(
        "UPDATE \"{}\" SET {} WHERE \"{}\" = ?",
        sanitized_table,
        set_clauses.join(", "),
        sanitized_pk,
    );

    let mut params: Vec<Box<dyn duckdb::types::ToSql>> = Vec::new();
    for (k, v) in &updates {
        if k != &pk_column {
            params.push(Box::new(v.clone()));
        }
    }
    params.push(Box::new(pk_value));

    let param_refs: Vec<&dyn duckdb::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, param_refs.as_slice())
        .map_err(|e| format!("Failed to update row: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_internal_row(
    table_name: String,
    pk_column: String,
    pk_value: String,
    state: State<'_, DuckDbState>,
) -> Result<(), String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sanitized_table = table_name.replace('"', "\"\"");
    let sanitized_pk = pk_column.replace('"', "\"\"");

    conn.execute(
        &format!("DELETE FROM \"{}\" WHERE \"{}\" = ?", sanitized_table, sanitized_pk),
        duckdb::params![&pk_value],
    )
    .map_err(|e| format!("Failed to delete row: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::database::initialize_schema;
    use duckdb::Connection;

    #[test]
    fn test_list_internal_tables() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'main' AND table_name LIKE 'd8a_monster_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 3);
    }
}
