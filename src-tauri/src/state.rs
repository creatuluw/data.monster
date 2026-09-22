use duckdb::Connection;
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct DuckDbState {
    /// Arc so hot-path commands (execute_query) can clone it into
    /// spawn_blocking — keeps query work off the main/UI thread
    pub conn: Arc<Mutex<Option<Connection>>>,
    pub query_cancelled: Arc<AtomicBool>,
    pub workspace_path: Mutex<Option<String>>,
}

impl Default for DuckDbState {
    fn default() -> Self {
        Self {
            conn: Arc::new(Mutex::new(None)),
            query_cancelled: Arc::new(AtomicBool::new(false)),
            workspace_path: Mutex::new(None),
        }
    }
}
