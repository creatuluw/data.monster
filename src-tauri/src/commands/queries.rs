use serde_json::json;
use std::sync::atomic::Ordering;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::formatting::format_duckdb_value;

#[tauri::command]
pub fn execute_query(sql: String, state: State<'_, DuckDbState>) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
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
        eprintln!("[queries] DML start: {}", if sql.len() > 200 { &sql[..200] } else { &sql });
        let affected = conn.execute(&sql, []).map_err(|e| {
            eprintln!("[queries] DML error: {}", e);
            e.to_string()
        })?;
        eprintln!("[queries] DML done ({} rows affected)", affected);
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

#[cfg(test)]
mod tests {
    use crate::utils::formatting::format_duckdb_value;
    use duckdb::Connection;
    use serde_json::json;

    fn exec_select(conn: &Connection, sql: &str) -> (Vec<String>, Vec<Vec<serde_json::Value>>) {
        let mut stmt = conn.prepare(sql).unwrap();
        let mut column_names: Vec<String> = Vec::new();
        let mut first_row = true;

        let data: Vec<Vec<serde_json::Value>> = stmt
            .query_map([], |row| {
                let col_count = row.as_ref().column_count();
                let mut cols: Vec<(String, serde_json::Value)> = Vec::new();
                for i in 0..col_count {
                    let name = row.as_ref().column_name(i)
                        .map_or(format!("col_{}", i), |v| v.to_string());
                    let value = match row.get_ref(i) {
                        Ok(val) => format_duckdb_value(val),
                        Err(_) => serde_json::Value::Null,
                    };
                    cols.push((name, value));
                }
                Ok(cols)
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .map(|cols| {
                if first_row {
                    column_names = cols.iter().map(|(n, _)| n.clone()).collect();
                    first_row = false;
                }
                cols.into_iter().map(|(_, v)| v).collect()
            })
            .collect();

        (column_names, data)
    }

    #[test]
    fn test_select_basic() {
        let conn = Connection::open_in_memory().unwrap();
        let (cols, rows) = exec_select(&conn, "SELECT 1 AS x, 'hello' AS y");
        assert_eq!(cols, vec!["x", "y"]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], json!(1));
        assert_eq!(rows[0][1], json!("hello"));
    }

    #[test]
    fn test_select_with() {
        let conn = Connection::open_in_memory().unwrap();
        let (cols, rows) = exec_select(&conn, "WITH cte AS (SELECT 1 AS v) SELECT * FROM cte");
        assert_eq!(cols, vec!["v"]);
        assert_eq!(rows[0][0], json!(1));
    }

    #[test]
    fn test_dml_create() {
        let conn = Connection::open_in_memory().unwrap();
        let affected = conn.execute("CREATE TABLE t1 AS SELECT 1 AS x", []).unwrap();
        assert_eq!(affected, 0);
    }

    #[test]
    fn test_dml_insert() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE t1 (x INT)", []).unwrap();
        let affected = conn.execute("INSERT INTO t1 VALUES (1), (2), (3)", []).unwrap();
        assert_eq!(affected, 3);
    }

    #[test]
    fn test_dml_drop() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE t1 AS SELECT 1 AS x", []).unwrap();
        let affected = conn.execute("DROP TABLE t1", []).unwrap();
        assert_eq!(affected, 0);
    }

    #[test]
    fn test_select_multiple_rows() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE nums AS SELECT * FROM generate_series(1, 5) AS t(n)", []).unwrap();
        let (_, rows) = exec_select(&conn, "SELECT * FROM nums ORDER BY n");
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0][0], json!(1));
        assert_eq!(rows[4][0], json!(5));
    }

    #[test]
    fn test_select_null_handling() {
        let conn = Connection::open_in_memory().unwrap();
        let (_, rows) = exec_select(&conn, "SELECT NULL AS a, 1 AS b");
        assert_eq!(rows[0][0], serde_json::Value::Null);
        assert_eq!(rows[0][1], json!(1));
    }
}
