use duckdb::Connection;
use std::fs;
use std::path::Path;
use tauri::State;

use crate::state::DuckDbState;
use crate::utils::metadata_helpers::cleanup_orphaned_metadata;

#[tauri::command]
pub fn initialize_duckdb(workspace_path: String, state: State<'_, DuckDbState>) -> Result<String, String> {
    let mut state_conn = state.conn.lock().map_err(|e| e.to_string())?;

    if state_conn.is_some() {
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

    let conn = open_with_retry(&db_path, &wal_path)?;

    initialize_schema(&conn)?;

    let _ = cleanup_orphaned_metadata(&conn);

    *state_conn = Some(conn);

    let mut wp = state.workspace_path.lock().map_err(|e| e.to_string())?;
    *wp = Some(workspace_path.clone());

    Ok("DuckDB initialized successfully".to_string())
}

fn open_with_retry(db_path: &Path, wal_path: &Path) -> Result<Connection, String> {
    let max_retries = 10;
    let mut retry_count = 0;

    loop {
        match Connection::open(db_path) {
            Ok(conn) => return Ok(conn),
            Err(e) => {
                let error_msg = e.to_string();

                let is_lock_error = error_msg.contains("database is locked")
                    || error_msg.contains("unable to open database file")
                    || error_msg.contains("unable to get a read lock");

                let is_wal_error = error_msg.contains("Failure while replaying WAL file")
                    || error_msg.contains("INTERNAL Error")
                    || error_msg.contains("DatabaseManager::GetDefaultDatabase");

                if is_lock_error {
                    retry_count += 1;
                    if retry_count <= max_retries {
                        eprintln!(
                            "Database is locked (attempt {}/{})",
                            retry_count, max_retries
                        );
                        std::thread::sleep(std::time::Duration::from_millis(
                            300 * retry_count as u64,
                        ));
                        continue;
                    } else {
                        return Err(format!(
                            "Database is locked after {} retry attempts",
                            max_retries
                        ));
                    }
                } else if is_wal_error {
                    eprintln!("Detected corrupted WAL file, attempting recovery...");
                    if wal_path.exists() {
                        match fs::remove_file(wal_path) {
                            Ok(_) => {
                                eprintln!("Removed corrupted WAL file");
                                continue;
                            }
                            Err(remove_err) => {
                                let is_file_locked = remove_err.raw_os_error() == Some(32)
                                    || remove_err
                                        .to_string()
                                        .contains("being used by another process");
                                if is_file_locked {
                                    retry_count += 1;
                                    if retry_count <= max_retries {
                                        eprintln!(
                                            "WAL file locked (attempt {}/{})",
                                            retry_count, max_retries
                                        );
                                        std::thread::sleep(std::time::Duration::from_millis(
                                            300 * retry_count as u64,
                                        ));
                                        continue;
                                    } else {
                                        return Err(format!(
                                            "WAL file locked after {} attempts",
                                            max_retries
                                        ));
                                    }
                                } else {
                                    return Err(format!(
                                        "Failed to remove corrupted WAL file: {}",
                                        remove_err
                                    ));
                                }
                            }
                        }
                    } else {
                        continue;
                    }
                } else {
                    return Err(error_msg);
                }
            }
        }
    }
}

fn initialize_schema(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS d8a_monster_table_metadata (
            table_name VARCHAR PRIMARY KEY,
            table_type VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            creation_query TEXT,
            source_path TEXT
        )",
        [],
    )
    .map_err(|e| format!("Failed to create d8a_monster_table_metadata: {}", e))?;

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

    Ok(())
}
