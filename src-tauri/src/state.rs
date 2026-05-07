use duckdb::Connection;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

pub struct DuckDbState {
    pub conn: Mutex<Option<Connection>>,
    pub query_cancelled: Arc<AtomicBool>,
    pub workspace_path: Mutex<Option<String>>,
}

impl Default for DuckDbState {
    fn default() -> Self {
        Self {
            conn: Mutex::new(None),
            query_cancelled: Arc::new(AtomicBool::new(false)),
            workspace_path: Mutex::new(None),
        }
    }
}
