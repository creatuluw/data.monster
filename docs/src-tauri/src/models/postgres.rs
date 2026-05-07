use serde::Serialize;

#[derive(Serialize)]
pub struct PostgresConnection {
    pub connection_id: String,
    pub connection_name: String,
    pub connection_type: String,
    pub secret_name: String,
    pub is_attached: bool,
    pub attach_mode: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

#[derive(Serialize)]
pub struct PostgresTable {
    pub table_name: String,
    pub schema_name: String,
    pub row_count: Option<i64>,
}

