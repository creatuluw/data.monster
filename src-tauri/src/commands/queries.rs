use serde_json::json;
use std::sync::atomic::Ordering;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::formatting::format_duckdb_value;

#[tauri::command]
pub fn execute_query(sql: String, state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized. Please select a workspace folder.")?;

    state.query_cancelled.store(false, Ordering::SeqCst);

    let trimmed = sql.trim().to_uppercase();

    if trimmed.starts_with("SELECT")
        || trimmed.starts_with("WITH")
        || trimmed.starts_with("SHOW")
        || trimmed.starts_with("DESCRIBE")
        || trimmed.starts_with("EXPLAIN")
        || trimmed.starts_with("PRAGMA")
    {
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

        let mut column_names: Vec<String> = Vec::new();
        let mut first_row = true;

        let rows_result = stmt
            .query_map([], |row| {
                let column_count = row.as_ref().column_count();

                let mut cols_with_values: Vec<(String, serde_json::Value)> = Vec::new();

                for i in 0..column_count {
                    let column_name = row
                        .as_ref()
                        .column_name(i)
                        .map_or(format!("col_{}", i), |v| v.to_string());

                    let value: serde_json::Value = match row.get_ref(i) {
                        Ok(val) => format_duckdb_value(val),
                        Err(_) => serde_json::Value::Null,
                    };

                    cols_with_values.push((column_name, value));
                }

                Ok((
                    cols_with_values
                        .iter()
                        .map(|(name, _)| name.clone())
                        .collect::<Vec<String>>(),
                    cols_with_values,
                ))
            })
            .map_err(|e| e.to_string())?;

        let mut data: Vec<Vec<serde_json::Value>> = Vec::new();
        for row_result in rows_result {
            if state.query_cancelled.load(Ordering::SeqCst) {
                return Err("Query cancelled".to_string());
            }

            let (col_names, col_values) = row_result.map_err(|e: duckdb::Error| e.to_string())?;

            if first_row {
                column_names = col_names;
                first_row = false;
            }

            let values: Vec<serde_json::Value> =
                col_values.into_iter().map(|(_, v)| v).collect();
            data.push(values);
        }

        Ok(json!({
            "columns": column_names,
            "data": data,
            "rowCount": data.len()
        }))
    } else {
        let affected = conn.execute(&sql, []).map_err(|e| e.to_string())?;
        Ok(json!({
            "type": "dml",
            "affectedRows": affected
        }))
    }
}

#[tauri::command]
pub fn cancel_query(state: State<'_, DuckDbState>) -> Result<(), String> {
    state.query_cancelled.store(true, Ordering::SeqCst);
    Ok(())
}
