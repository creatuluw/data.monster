use duckdb::Connection;
use std::fs;
use std::path::Path;
use tauri::{Emitter, State, AppHandle};

use crate::state::DuckDbState;
use crate::utils::metadata_helpers::cleanup_orphaned_metadata;

#[tauri::command]
pub fn initialize_duckdb(
    workspace_path: String,
    state: State<'_, DuckDbState>,
    app: AppHandle,
) -> Result<String, String> {
    eprintln!("[database] Initializing DuckDB at {}", workspace_path);
    let mut state_conn = state.conn.lock();

    if state_conn.is_some() {
        let _ = app.emit("db-init-progress", "Already initialized");
        return Ok("DuckDB already initialized".to_string());
    }

    let workspace = Path::new(&workspace_path);
    if !workspace.exists() {
        fs::create_dir_all(workspace)
            .map_err(|e| format!("Failed to create workspace directory: {}", e))?;
    }

    let data_dir = workspace.join("data").join("main");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }

    let db_path = workspace.join("d8a_monster.duckdb");
    let wal_path = workspace.join("d8a_monster.duckdb.wal");

    let had_wal = wal_path.exists();
    if had_wal {
        let _ = app.emit("db-init-progress", "Previous session detected — recovering...");
        eprintln!("[database] WAL file found from previous session, DuckDB will replay it");
    }

    let conn = open_with_retry(&db_path, &wal_path, &app)?;

    if had_wal {
        let _ = app.emit("db-init-progress", "WAL replayed — flushing to disk...");
        eprintln!("[database] WAL was replayed, running FORCE CHECKPOINT to flush recovered data");
        if let Err(e) = conn.execute_batch("FORCE CHECKPOINT") {
            eprintln!("[database] Warning: FORCE CHECKPOINT after WAL recovery failed: {}", e);
            if let Err(e2) = conn.execute_batch("CHECKPOINT") {
                eprintln!("[database] Warning: CHECKPOINT also failed: {}", e2);
            }
        }
    }

    let _ = app.emit("db-init-progress", "Configuring database...");
    if let Err(e) = conn.execute_batch("SET checkpoint_threshold = '4MB'") {
        eprintln!("[database] Warning: failed to set checkpoint_threshold: {}", e);
    }
    if let Err(e) = conn.execute_batch("SET wal_autocheckpoint_entries = 5000") {
        eprintln!("[database] Warning: failed to set wal_autocheckpoint_entries: {}", e);
    }

    let _ = app.emit("db-init-progress", "Creating schema...");
    initialize_schema(&conn)?;

    // workspace-file-first: export content tables to dm/ files (one-time, crash-safe,
    // idempotent) and bootstrap the git-safe workspace (.gitignore when missing).
    let _ = app.emit("db-init-progress", "Migrating content to dm/ files...");
    if let Err(e) = crate::commands::migration::migrate_content_tables(&conn, workspace) {
        eprintln!("[database] Warning: dm/ migration failed: {}", e);
    }
    if let Err(e) = crate::commands::migration::bootstrap_gitignore(workspace) {
        eprintln!("[database] Warning: .gitignore bootstrap failed: {}", e);
    }

    // live dm/ watcher — a workspace switch replaces the previous one
    if let Err(e) = crate::commands::dm_watch::start(workspace.to_path_buf(), app.clone()) {
        eprintln!("[database] Warning: dm/ watcher failed to start: {}", e);
    }

    let _ = app.emit("db-init-progress", "Cleaning up metadata...");
    let _ = cleanup_orphaned_metadata(&conn);

    *state_conn = Some(conn);

    let mut wp = state.workspace_path.lock();
    *wp = Some(workspace_path.clone());

    let _ = app.emit("db-init-progress", "Database ready");
    Ok("DuckDB initialized successfully".to_string())
}

#[tauri::command]
pub fn shutdown_duckdb(state: State<'_, DuckDbState>) -> Result<(), String> {
    eprintln!("[database] Shutting down DuckDB...");
    let mut state_conn = state.conn.lock();

    if let Some(conn) = state_conn.take() {
        eprintln!("[database] Running CHECKPOINT before shutdown...");
        if let Err(e) = conn.execute_batch("CHECKPOINT") {
            eprintln!("[database] Warning: CHECKPOINT failed during shutdown: {}", e);
        }

        match conn.close() {
            Ok(()) => {
                eprintln!("[database] Connection closed cleanly");
            },
            Err((conn, e)) => {
                eprintln!("[database] Close returned error: {}, retrying...", e);
                match conn.close() {
                    Ok(()) => eprintln!("[database] Close succeeded on retry"),
                    Err((_, e2)) => {
                        eprintln!("[database] Close failed again: {}. Dropping connection.", e2);
                        return Err(format!("Failed to close database cleanly: {} / {}", e, e2));
                    }
                }
            }
        }
    }

    let mut wp = state.workspace_path.lock();
    *wp = None;

    eprintln!("[database] Shutdown complete");
    Ok(())
}

#[tauri::command]
pub fn reset_all_data(state: State<'_, DuckDbState>) -> Result<(), String> {
    eprintln!("[database] Resetting all data");
    let state_conn = state.conn.lock();
    let conn = state_conn
        .as_ref()
        .ok_or("DuckDB not initialized")?;

    let mut stmt = conn
        .prepare("SELECT table_name FROM information_schema.tables WHERE table_schema = 'main'")
        .map_err(|e| e.to_string())?;

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    for table in &tables {
        let sanitized = table.replace('"', "\"\"");
        let _ = conn.execute(&format!("DROP TABLE IF EXISTS \"{}\"", sanitized), []);
        let _ = conn.execute(&format!("DROP VIEW IF EXISTS \"{}\"", sanitized), []);
    }

    initialize_schema(conn)?;

    Ok(())
}

fn open_with_retry(db_path: &Path, wal_path: &Path, app: &AppHandle) -> Result<Connection, String> {
    let max_retries = 10;
    let mut attempt = 0;
    let mut wal_removed = false;

    loop {
        attempt += 1;
        if attempt > max_retries {
            return Err(format!(
                "Failed to open database after {} attempts. Try deleting: {}",
                max_retries,
                db_path.display()
            ));
        }

        let _ = app.emit("db-init-progress", format!("Opening database (attempt {}/{})...", attempt, max_retries));

        match Connection::open(db_path) {
            Ok(conn) => {
                eprintln!("[database] Opened database successfully on attempt {}", attempt);
                return Ok(conn);
            },
            Err(e) => {
                let error_msg = e.to_string();
                eprintln!("[database] Open error (attempt {}): {}", attempt, error_msg);

                let is_wal_error = error_msg.contains("Failure while replaying WAL file")
                    || error_msg.contains("INTERNAL Error")
                    || error_msg.contains("DatabaseManager::GetDefaultDatabase");

                let is_lock_error = error_msg.contains("database is locked")
                    || error_msg.contains("unable to open database file")
                    || error_msg.contains("unable to get a read lock");

                if is_wal_error && !wal_removed {
                    eprintln!("[database] WAL replay failed, WAL file may be corrupted");
                    let _ = app.emit("db-init-progress", "WAL replay failed — removing corrupted WAL...");
                    if wal_path.exists() {
                        match fs::remove_file(wal_path) {
                            Ok(_) => {
                                eprintln!("[database] WAL removed, retrying open (data since last checkpoint may be lost)");
                                wal_removed = true;
                                continue;
                            }
                            Err(remove_err) => {
                                let is_file_locked = remove_err.raw_os_error() == Some(32)
                                    || remove_err
                                        .to_string()
                                        .contains("being used by another process");
                                if is_file_locked {
                                    let _ = app.emit("db-init-progress", format!("WAL file locked, retrying ({}/{})...", attempt, max_retries));
                                    eprintln!("[database] WAL locked (attempt {}/{})", attempt, max_retries);
                                    std::thread::sleep(std::time::Duration::from_millis(500));
                                    continue;
                                } else {
                                    return Err(format!(
                                        "Failed to remove corrupted WAL file: {}. Try manually deleting: {}",
                                        remove_err,
                                        wal_path.display()
                                    ));
                                }
                            }
                        }
                    } else {
                        wal_removed = true;
                        eprintln!("[database] WAL already gone, retrying open");
                        continue;
                    }
                } else if is_lock_error {
                    let _ = app.emit("db-init-progress", format!("Database locked, retrying ({}/{})...", attempt, max_retries));
                    eprintln!("[database] Locked (attempt {}/{})", attempt, max_retries);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                } else if is_wal_error && wal_removed {
                    return Err(format!(
                        "Database file may be corrupted (WAL was already removed). Try deleting: {}",
                        db_path.display()
                    ));
                } else {
                    return Err(format!(
                        "Failed to open database: {}. Try deleting: {}",
                        error_msg,
                        db_path.display()
                    ));
                }
            }
        }
    }
}

pub(crate) fn initialize_schema(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_table_metadata (
            table_name VARCHAR PRIMARY KEY,
            table_type VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            creation_query TEXT,
            source_path TEXT,
            source_type VARCHAR,
            original_source TEXT
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_table_metadata: {}", e))?;

    let _ = conn.execute("ALTER TABLE d8a_monster_table_metadata ADD COLUMN source_type VARCHAR", []);
    let _ = conn.execute("ALTER TABLE d8a_monster_table_metadata ADD COLUMN original_source TEXT", []);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_table_labels (
            table_name VARCHAR PRIMARY KEY,
            tags TEXT,
            \"group\" VARCHAR
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_table_labels: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_saved_queries (
            slug VARCHAR PRIMARY KEY,
            query_name VARCHAR NOT NULL,
            query_sql TEXT NOT NULL,
            description TEXT,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_saved_queries: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_pages (
            slug VARCHAR PRIMARY KEY,
            title VARCHAR NOT NULL,
            spec TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_pages: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_items (
            id VARCHAR PRIMARY KEY,
            kind VARCHAR NOT NULL,
            table_name VARCHAR NOT NULL,
            expr TEXT NOT NULL,
            label VARCHAR NOT NULL,
            fmt VARCHAR,
            description VARCHAR,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_items: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_relationships (
            id VARCHAR PRIMARY KEY,
            from_table VARCHAR NOT NULL,
            from_column VARCHAR NOT NULL,
            to_table VARCHAR NOT NULL,
            to_column VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_relationships: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_field_functions (
            id VARCHAR PRIMARY KEY,
            label VARCHAR NOT NULL,
            description TEXT,
            sql_template TEXT NOT NULL,
            applies_to TEXT NOT NULL,
            output_type VARCHAR NOT NULL DEFAULT ''
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_field_functions: {}", e))?;

    let has_output_type: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('d8a_monster_field_functions') WHERE name = 'output_type'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);
    if !has_output_type {
        let _ = conn.execute("ALTER TABLE d8a_monster_field_functions ADD COLUMN output_type VARCHAR NOT NULL DEFAULT ''", []);
    }

    conn.execute(
        "INSERT OR IGNORE INTO d8a_monster_field_functions (id, label, description, sql_template, applies_to, output_type) VALUES
            ('week_end', 'Week end', 'Last day of the week (Sunday)', 'date_trunc(''week'', {column}) + INTERVAL 6 DAYS', 'DATE,TIMESTAMP', 'DATE'),
            ('month_end', 'Month end', 'Last day of the month', '(date_trunc(''month'', {column}) + INTERVAL 1 MONTH) - INTERVAL 1 DAY', 'DATE,TIMESTAMP', 'DATE'),
            ('quarter_end', 'Quarter end', 'Last day of the quarter', '(date_trunc(''quarter'', {column}) + INTERVAL 3 MONTHS) - INTERVAL 1 DAY', 'DATE,TIMESTAMP', 'DATE')",
        [],
    )
    .map_err(|e| format!("Failed to seed d8a_monster_field_functions: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use duckdb::Connection;

    #[test]
    fn test_initialize_schema_creates_tables() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'main' AND table_name LIKE 'd8a_monster_%'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 7);
    }

    #[test]
    fn test_initialize_schema_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        initialize_schema(&conn).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'main' AND table_name LIKE 'd8a_monster_%'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 7);
    }

    #[test]
    fn test_field_functions_seeded() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM d8a_monster_field_functions",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_metadata_table_has_columns() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        let mut stmt = conn.prepare("SELECT column_name FROM (DESCRIBE d8a_monster_table_metadata)").unwrap();
        let cols: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(cols.contains(&"table_name".to_string()));
        assert!(cols.contains(&"table_type".to_string()));
        assert!(cols.contains(&"created_at".to_string()));
    }
}
