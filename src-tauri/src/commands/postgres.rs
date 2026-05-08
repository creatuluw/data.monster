use serde_json::json;
use tauri::State;

use crate::state::DuckDbState;

const PG_ATTACH_NAME: &str = "__pg_source";

#[tauri::command]
pub fn connect_postgres(
    url: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let _ = conn.execute(&format!("DETACH {}", PG_ATTACH_NAME), []);

    conn.execute("INSTALL postgres", [])
        .map_err(|e| format!("Failed to install postgres extension: {}", e))?;
    conn.execute("LOAD postgres", [])
        .map_err(|e| format!("Failed to load postgres extension: {}", e))?;

    let escaped_url = url.replace('\'', "''");
    conn.execute(
        &format!(
            "ATTACH '{}' AS {} (TYPE postgres)",
            escaped_url, PG_ATTACH_NAME
        ),
        [],
    )
    .map_err(|e| format!("Failed to connect to PostgreSQL: {}", e))?;

    let sql = format!(
        "SELECT schema_name FROM {}.information_schema.schemata \
         WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast') \
         ORDER BY schema_name",
        PG_ATTACH_NAME
    );
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("Connected but failed to query schemas: {}", e))?;

    let schemas: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "schemas": schemas }))
}

#[tauri::command]
pub fn list_postgres_tables(
    schema: String,
    state: State<'_, DuckDbState>,
) -> Result<serde_json::Value, String> {
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let sql = format!(
        "SELECT table_name FROM {}.information_schema.tables \
         WHERE table_schema = '{}' \
         ORDER BY table_name",
        PG_ATTACH_NAME,
        schema.replace('\'', "''")
    );

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("Failed to query tables: {}", e))?;

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "tables": tables }))
}

#[tauri::command]
pub fn generate_pg_ingest_sql(
    url: String,
    schema: String,
    table_names: Vec<String>,
) -> Result<serde_json::Value, String> {
    let escaped_url = url.replace('\'', "''");
    let statements: Vec<serde_json::Value> = table_names.iter().map(|table_name| {
        let sanitized = table_name.replace('"', "\"\"");
        let schema_sanitized = schema.replace('"', "\"\"");
        let sql = format!(
            "CREATE TABLE \"{}\" AS SELECT * FROM {}.\"{}\".\"{}\"",
            sanitized, PG_ATTACH_NAME, schema_sanitized, sanitized
        );
        json!({
            "tableName": table_name,
            "sql": sql,
            "sourcePath": format!("postgres://{}/{}.{}", escaped_url, schema, table_name),
            "sourceType": "postgres",
            "originalSource": format!("postgres://{}/{}.{}", url, schema, table_name),
        })
    }).collect();

    Ok(json!({ "statements": statements }))
}
