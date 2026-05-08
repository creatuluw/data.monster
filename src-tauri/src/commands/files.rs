use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::formatting::format_duckdb_value;
use crate::utils::metadata_helpers::register_table_metadata;

#[tauri::command]
pub fn load_csv_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_csv_auto", &state)
}

#[tauri::command]
pub fn load_parquet_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_parquet", &state)
}

#[tauri::command]
pub fn load_json_file(
    path: String,
    table_name: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    load_file(&path, &table_name, "read_json_auto", &state)
}

#[tauri::command]
pub fn get_file_columns(
    path: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let ext = Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let read_fn = match ext.as_str() {
        "parquet" => "read_parquet",
        "json" | "jsonl" => "read_json_auto",
        _ => "read_csv_auto",
    };

    let safe_path = path.replace('\\', "/");
    let query = format!(
        "SELECT column_name, column_type FROM (DESCRIBE SELECT * FROM {}('{}'))",
        read_fn, safe_path
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let columns: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let dtype: String = row.get(1)?;
            Ok(json!({ "name": name, "type": dtype }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "columns": columns }))
}

#[tauri::command]
pub fn preview_file(
    path: String,
    limit: i64,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let wp = state.workspace_path.lock().map_err(|e| e.to_string())?;
    let workspace = wp.as_ref().ok_or("No workspace folder set")?.clone();
    drop(wp);

    let source_path = Path::new(&path);
    let file_name = source_path
        .file_name()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Invalid file name")?;

    let dest_dir = Path::new(&workspace).join("data").join("main");
    let dest_path = dest_dir.join(file_name);

    if !dest_path.exists() {
        fs::create_dir_all(&dest_dir).map_err(|e| format!("Failed to create data dir: {}", e))?;
        fs::copy(source_path, &dest_path)
            .map_err(|e| format!("Failed to copy file to workspace: {}", e))?;
    }

    let ext = Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let read_fn = match ext.as_str() {
        "parquet" => "read_parquet",
        "json" | "jsonl" => "read_json_auto",
        _ => "read_csv_auto",
    };

    let safe_path = dest_path.to_str().ok_or("Invalid destination path")?.replace('\\', "/");

    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let desc_sql = format!(
        "SELECT column_name, column_type FROM (DESCRIBE SELECT * FROM {}('{}'))",
        read_fn, safe_path
    );
    let mut desc_stmt = conn.prepare(&desc_sql).map_err(|e| e.to_string())?;
    let column_types: Vec<serde_json::Value> = desc_stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let dtype: String = row.get(1)?;
            Ok(json!({ "name": name, "type": dtype }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
    drop(desc_stmt);

    let count_sql = format!("SELECT COUNT(*) FROM {}('{}')", read_fn, safe_path);
    let total_rows: i64 = conn
        .query_row(&count_sql, [], |row| row.get(0))
        .map_err(|e| format!("Failed to count rows: {}", e))?;

    let data_sql = format!(
        "SELECT * FROM {}('{}') LIMIT {}",
        read_fn, safe_path, limit
    );
    let mut stmt = conn.prepare(&data_sql).map_err(|e| e.to_string())?;

    let mut column_names: Vec<String> = Vec::new();
    let mut first_row = true;

    let row_results: Vec<Vec<serde_json::Value>> = stmt
        .query_map([], |row| {
            let col_count = row.as_ref().column_count();
            let mut vals: Vec<serde_json::Value> = Vec::new();
            for i in 0..col_count {
                if first_row {
                    let name = row.as_ref().column_name(i)
                        .map_or(format!("col_{}", i), |v| v.to_string());
                    column_names.push(name);
                }
                let value = match row.get_ref(i) {
                    Ok(val) => format_duckdb_value(val),
                    Err(_) => serde_json::Value::Null,
                };
                vals.push(value);
            }
            if first_row {
                first_row = false;
            }
            Ok(vals)
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let row_values: Vec<serde_json::Value> = row_results
        .into_iter()
        .map(|row| {
            let obj: serde_json::Map<String, serde_json::Value> = column_names
                .iter()
                .zip(row.into_iter())
                .map(|(name, val)| (name.clone(), val))
                .collect();
            serde_json::Value::Object(obj)
        })
        .collect();

    Ok(json!({
        "columns": column_names,
        "rows": row_values,
        "totalRows": total_rows,
        "columnTypes": column_types
    }))
}

fn load_file(
    path: &str,
    table_name: &str,
    read_fn: &str,
    state: &State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let wp = state.workspace_path.lock().map_err(|e| e.to_string())?;
    let workspace = wp.as_ref().ok_or("No workspace folder set")?;

    let source_path = Path::new(path);
    let file_name = source_path
        .file_name()
        .ok_or("Invalid file path")?
        .to_str()
        .ok_or("Invalid file name")?;

    let dest_dir = Path::new(workspace).join("data").join("main");
    let dest_path = dest_dir.join(file_name);

    if source_path != dest_path {
        fs::copy(source_path, &dest_path)
            .map_err(|e| format!("Failed to copy file to workspace: {}", e))?;
    }

    let sanitized = table_name.replace('"', "\"\"");
    let escaped_dest = dest_path.to_str().ok_or("Invalid destination path")?.replace('\\', "/");
    let query = format!(
        "CREATE TABLE \"{}\" AS SELECT * FROM {}('{}')",
        sanitized, read_fn, escaped_dest
    );

    conn.execute(&query, [])
        .map_err(|e| format!("Failed to load file into table: {}", e))?;

    let row_count: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM \"{}\"", sanitized),
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    register_table_metadata(conn, table_name, "source")?;

    Ok(json!({
        "tableName": table_name,
        "rowCount": row_count
    }))
}

#[cfg(test)]
mod tests {
    use duckdb::Connection;
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::utils::formatting::format_duckdb_value;

    fn create_test_csv(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_preview_csv_basic() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(
            dir.path(),
            "people.csv",
            "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,SF\n",
        );

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = csv_path.to_str().unwrap().replace('\\', "/");

        let count_sql = format!("SELECT COUNT(*) FROM read_csv_auto('{}')", safe_path);
        let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0)).unwrap();
        assert_eq!(total_rows, 3);

        let data_sql = format!("SELECT * FROM read_csv_auto('{}') LIMIT 100", safe_path);
        let mut stmt = conn.prepare(&data_sql).unwrap();

        let mut column_names: Vec<String> = Vec::new();
        let mut first_row = true;

        let rows: Vec<Vec<serde_json::Value>> = stmt
            .query_map([], |row| {
                let col_count = row.as_ref().column_count();
                let mut vals = Vec::new();
                for i in 0..col_count {
                    if first_row {
                        let name = row.as_ref().column_name(i)
                            .map_or(format!("col_{}", i), |v| v.to_string());
                        column_names.push(name);
                    }
                    let value = match row.get_ref(i) {
                        Ok(val) => format_duckdb_value(val),
                        Err(_) => serde_json::Value::Null,
                    };
                    vals.push(value);
                }
                if first_row { first_row = false; }
                Ok(vals)
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert_eq!(column_names, vec!["name", "age", "city"]);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0], json!("Alice"));
        assert_eq!(rows[0][1], json!(30));
        assert_eq!(rows[0][2], json!("NYC"));
        assert_eq!(rows[2][0], json!("Charlie"));
    }

    #[test]
    fn test_preview_csv_with_limit() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(
            dir.path(),
            "big.csv",
            "id\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n",
        );

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = csv_path.to_str().unwrap().replace('\\', "/");

        let count_sql = format!("SELECT COUNT(*) FROM read_csv_auto('{}')", safe_path);
        let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0)).unwrap();
        assert_eq!(total_rows, 10);

        let data_sql = format!("SELECT * FROM read_csv_auto('{}') LIMIT 3", safe_path);
        let mut stmt = conn.prepare(&data_sql).unwrap();
        let rows: Vec<_> = stmt.query_map([], |row| row.get::<_, i64>(0)).unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn test_preview_json_file() {
        let dir = TempDir::new().unwrap();
        let json_path = create_test_csv(
            dir.path(),
            "data.json",
            "[{\"x\":1,\"y\":\"a\"},{\"x\":2,\"y\":\"b\"}]",
        );

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = json_path.to_str().unwrap().replace('\\', "/");

        let data_sql = format!("SELECT * FROM read_json_auto('{}')", safe_path);
        let mut stmt = conn.prepare(&data_sql).unwrap();

        let mut col_names: Vec<String> = Vec::new();
        let mut first = true;
        let count = stmt.query_map([], |row| {
            let mut cols = Vec::new();
            for i in 0..row.as_ref().column_count() {
                let name = row.as_ref().column_name(i).unwrap().to_string();
                cols.push(name);
            }
            Ok(cols)
        }).unwrap()
        .filter_map(|r| r.ok())
        .inspect(|cols| {
            if first {
                col_names = cols.clone();
                first = false;
            }
        })
        .count();

        assert_eq!(col_names, vec!["x", "y"]);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_preview_csv_with_backslash_path() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(
            dir.path(),
            "test.csv",
            "a,b\n1,2\n",
        );

        let conn = Connection::open_in_memory().unwrap();
        let raw_path = csv_path.to_str().unwrap();
        let safe_path = raw_path.replace('\\', "/");

        let sql = format!("SELECT * FROM read_csv_auto('{}')", safe_path);
        let result: Result<i64, _> = conn.query_row(&sql, [], |row| row.get(0));
        assert!(result.is_ok(), "Forward-slash path should work");
    }

    #[test]
    fn test_preview_empty_csv() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(dir.path(), "empty.csv", "col1,col2\n");

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = csv_path.to_str().unwrap().replace('\\', "/");

        let count_sql = format!("SELECT COUNT(*) FROM read_csv_auto('{}')", safe_path);
        let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0)).unwrap();
        assert_eq!(total_rows, 0);

        let desc_sql = format!(
            "SELECT column_name FROM (DESCRIBE SELECT * FROM read_csv_auto('{}'))",
            safe_path
        );
        let mut desc_stmt = conn.prepare(&desc_sql).unwrap();
        let col_names: Vec<String> = desc_stmt
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        drop(desc_stmt);
        assert_eq!(col_names, vec!["col1", "col2"]);
    }

    #[test]
    fn test_preview_csv_null_values() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(
            dir.path(),
            "nulls.csv",
            "a,b\n1,hello\n,\n3,\n",
        );

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = csv_path.to_str().unwrap().replace('\\', "/");

        let data_sql = format!("SELECT * FROM read_csv_auto('{}')", safe_path);
        let mut stmt = conn.prepare(&data_sql).unwrap();

        let rows: Vec<Vec<serde_json::Value>> = stmt
            .query_map([], |row| {
                let col_count = row.as_ref().column_count();
                let mut vals = Vec::new();
                for i in 0..col_count {
                    let val = match row.get_ref(i) {
                        Ok(v) => format_duckdb_value(v),
                        Err(_) => serde_json::Value::Null,
                    };
                    vals.push(val);
                }
                Ok(vals)
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1][0], serde_json::Value::Null);
    }

    #[test]
    fn test_sequential_queries_same_connection() {
        let dir = TempDir::new().unwrap();
        let csv_path = create_test_csv(
            dir.path(),
            "seq.csv",
            "id\n1\n2\n3\n",
        );

        let conn = Connection::open_in_memory().unwrap();
        let safe_path = csv_path.to_str().unwrap().replace('\\', "/");

        let count_sql = format!("SELECT COUNT(*) FROM read_csv_auto('{}')", safe_path);
        let total: i64 = conn.query_row(&count_sql, [], |row| row.get(0)).unwrap();
        assert_eq!(total, 3);

        let data_sql = format!("SELECT * FROM read_csv_auto('{}') LIMIT 100", safe_path);
        let mut stmt = conn.prepare(&data_sql).unwrap();
        let rows: Vec<_> = stmt.query_map([], |row| row.get::<_, i64>(0)).unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(rows.len(), 3);

        let count2: i64 = conn.query_row(&count_sql, [], |row| row.get(0)).unwrap();
        assert_eq!(count2, 3);
    }
}
