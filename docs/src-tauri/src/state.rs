use duckdb::{Connection, InterruptHandle};
use std::sync::{Mutex, Arc};
use std::sync::atomic::AtomicBool;

/// DuckDB state management for Tauri
pub struct DuckDbState {
    pub conn: Mutex<Option<Connection>>,
    pub query_cancelled: Arc<AtomicBool>,
    pub interrupt_handle: Mutex<Option<Arc<InterruptHandle>>>,
}

impl DuckDbState {
    pub fn new() -> Self {
        Self {
            conn: Mutex::new(None),
            query_cancelled: Arc::new(AtomicBool::new(false)),
            interrupt_handle: Mutex::new(None),
        }
    }
}

