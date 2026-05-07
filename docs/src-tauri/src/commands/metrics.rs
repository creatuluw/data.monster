use tauri::State;
use duckdb::params;
use std::collections::HashSet;
use crate::state::DuckDbState;
use crate::utils::{format_duckdb_value, generate_unique_metric_slug, build_join_path, parse_dimension};

/// Create or update a metric definition
#[tauri::command]
pub fn create_metric(
    state: State<DuckDbState>,
    metric_name: String,
    formula: String,
    source_table: String,
    description: String,
    tags: String,
    existing_slug: Option<String>
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    if metric_name.trim().is_empty() {
        return Err("Metric name cannot be empty".to_string());
    }
    
    if formula.trim().is_empty() {
        return Err("Formula cannot be empty".to_string());
    }
    
    if source_table.trim().is_empty() {
        return Err("Source table cannot be empty".to_string());
    }
    
    let table_check = conn.prepare(&format!(
        "SELECT COUNT(*) FROM _warphead_table_metadata WHERE table_name = '{}' AND table_type IN ('source', 'model')",
        source_table.trim()
    ));
    
    match table_check {
        Ok(mut stmt) => {
            let count: i64 = stmt.query_row([], |row| row.get(0)).map_err(|e| e.to_string())?;
            if count == 0 {
                return Err(format!("Table '{}' not found in data model", source_table.trim()));
            }
        },
        Err(e) => return Err(format!("Failed to validate source table: {}", e))
    }
    
    let slug = if let Some(existing) = existing_slug {
        existing
    } else {
        generate_unique_metric_slug(conn, metric_name.trim(), None)
    };
    
    conn.execute(
        "INSERT INTO _warphead_metrics (slug, metric_name, formula, source_table, description, tags, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, now())
         ON CONFLICT (slug) DO UPDATE SET
            metric_name = EXCLUDED.metric_name,
            formula = EXCLUDED.formula,
            source_table = EXCLUDED.source_table,
            description = EXCLUDED.description,
            tags = EXCLUDED.tags,
            updated_at = now()",
        params![&slug, metric_name.trim(), formula.trim(), source_table.trim(), description.trim(), tags.trim()]
    ).map_err(|e| format!("Failed to save metric metadata: {}", e))?;
    
    Ok(format!("Metric '{}' saved successfully", metric_name))
}

/// Get all metrics
#[tauri::command]
pub fn list_metrics(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = "SELECT slug, metric_name, formula, source_table, description, tags, 
                        strftime(created_at, '%Y-%m-%d %H:%M:%S') as created_at,
                        strftime(updated_at, '%Y-%m-%d %H:%M:%S') as updated_at
                 FROM _warphead_metrics
                 ORDER BY metric_name";
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let metrics_result = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "slug": row.get::<_, String>(0)?,
            "metric_name": row.get::<_, String>(1)?,
            "formula": row.get::<_, String>(2)?,
            "source_table": row.get::<_, String>(3)?,
            "description": row.get::<_, Option<String>>(4)?,
            "tags": row.get::<_, Option<String>>(5)?,
            "created_at": row.get::<_, String>(6)?,
            "updated_at": row.get::<_, String>(7)?
        }))
    }).map_err(|e| e.to_string())?;
    
    let metrics: Vec<serde_json::Value> = metrics_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    serde_json::to_string(&metrics).map_err(|e| e.to_string())
}

/// Delete a metric
#[tauri::command]
pub fn delete_metric(state: State<DuckDbState>, metric_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    conn.execute(
        "DELETE FROM _warphead_metrics WHERE metric_name = ?",
        [&metric_name]
    ).map_err(|e| format!("Failed to remove metric metadata: {}", e))?;
    
    Ok(format!("Metric '{}' deleted successfully", metric_name))
}

/// Test a metric formula
#[tauri::command]
pub fn test_metric(
    state: State<DuckDbState>,
    formula: String,
    test_query: String
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let query = if !test_query.trim().is_empty() {
        test_query.trim().to_string()
    } else {
        format!("SELECT {} AS result", formula.trim())
    };
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    let rows_result = stmt
        .query_map([], |row| {
            let column_count = row.as_ref().column_count();
            let mut row_map = serde_json::Map::new();
            
            for i in 0..column_count {
                let column_name = row.as_ref().column_name(i)
                    .map_or(format!("col_{}", i), |v| v.to_string());
                
                let value: serde_json::Value = match row.get_ref(i) {
                    Ok(val) => format_duckdb_value(val),
                    Err(_) => serde_json::Value::Null,
                };
                row_map.insert(column_name, value);
            }
            Ok(serde_json::Value::Object(row_map))
        })
        .map_err(|e| format!("Failed to execute test query: {}", e))?;
    
    let mut rows = Vec::new();
    for row in rows_result {
        rows.push(row.map_err(|e: duckdb::Error| e.to_string())?);
    }
    
    serde_json::to_string(&rows).map_err(|e| e.to_string())
}

/// Execute a metric with specified dimensions and optional filters
#[tauri::command]
pub fn execute_metric_with_dimensions(
    state: State<DuckDbState>,
    metric_name: String,
    dimensions: Vec<String>,
    filters: Option<String>
) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Load metric definition
    let metric_query = "SELECT formula, source_table FROM _warphead_metrics WHERE metric_name = ?";
    let mut metric_stmt = conn.prepare(metric_query).map_err(|e| e.to_string())?;
    
    let (formula, source_table): (String, String) = metric_stmt
        .query_row([&metric_name], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .map_err(|e| format!("Metric '{}' not found: {}", metric_name, e))?;
    
    // Parse dimensions
    let mut dimension_tables: HashSet<String> = HashSet::new();
    let mut dimension_columns: Vec<(String, String)> = Vec::new();
    
    for dim in &dimensions {
        let (table, column) = parse_dimension(dim)?;
        dimension_tables.insert(table.clone());
        dimension_columns.push((table, column));
    }
    
    // Build JOIN paths
    let mut join_paths = Vec::new();
    let mut joined_tables: HashSet<String> = HashSet::new();
    joined_tables.insert(source_table.clone());
    
    for dim_table in &dimension_tables {
        if dim_table != &source_table {
            let path = build_join_path(conn, &source_table, dim_table)?;
            for join in path {
                if !joined_tables.contains(&join.table) {
                    join_paths.push(join.clone());
                    joined_tables.insert(join.table.clone());
                }
            }
        }
    }
    
    // Build SQL query
    let mut sql = String::from("SELECT ");
    
    let dimension_list: Vec<String> = dimension_columns
        .iter()
        .map(|(table, column)| format!("{}.{}", table, column))
        .collect();
    
    if !dimension_list.is_empty() {
        sql.push_str(&dimension_list.join(", "));
        sql.push_str(", ");
    }
    
    sql.push_str(&format!("{} AS \"{}\"", formula, metric_name));
    sql.push_str(&format!(" FROM {}", source_table));
    
    // Add JOINs
    let mut previous_table = source_table.clone();
    for join_path in &join_paths {
        sql.push_str(&format!(
            " JOIN {} ON {}.{} = {}.{}",
            join_path.table,
            previous_table,
            join_path.source_column,
            join_path.table,
            join_path.join_column
        ));
        previous_table = join_path.table.clone();
    }
    
    // Add WHERE clause
    if let Some(filter_str) = filters {
        if !filter_str.trim().is_empty() {
            sql.push_str(&format!(" WHERE {}", filter_str.trim()));
        }
    }
    
    // Add GROUP BY
    if !dimension_list.is_empty() {
        sql.push_str(" GROUP BY ");
        sql.push_str(&dimension_list.join(", "));
    }
    
    // Execute query
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("SQL error: {} | Query: {}", e, sql))?;
    
    let rows_result = stmt
        .query_map([], |row| {
            let column_count = row.as_ref().column_count();
            let mut row_map = serde_json::Map::new();
            
            for i in 0..column_count {
                let column_name = row.as_ref().column_name(i)
                    .map_or(format!("col_{}", i), |v| v.to_string());
                
                let value: serde_json::Value = match row.get_ref(i) {
                    Ok(val) => format_duckdb_value(val),
                    Err(_) => serde_json::Value::Null,
                };
                row_map.insert(column_name, value);
            }
            Ok(serde_json::Value::Object(row_map))
        })
        .map_err(|e| format!("Failed to execute metric query: {}", e))?;
    
    let mut rows = Vec::new();
    for row in rows_result {
        rows.push(row.map_err(|e: duckdb::Error| e.to_string())?);
    }
    
    let response = serde_json::json!({
        "query": sql,
        "results": rows
    });
    
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

