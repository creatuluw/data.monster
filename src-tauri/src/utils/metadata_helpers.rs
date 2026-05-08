use duckdb::Connection;

pub fn register_table_metadata(
    conn: &Connection,
    table_name: &str,
    table_type: &str,
) -> Result<(), String> {
    conn.execute(
        "INSERT INTO d8a_monster_table_metadata (table_name, table_type, created_at)
         VALUES (?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT (table_name) DO UPDATE SET table_type = EXCLUDED.table_type",
        duckdb::params![table_name, table_type],
    )
    .map_err(|e| format!("Failed to register table metadata: {}", e))?;

    Ok(())
}

pub fn remove_table_metadata(conn: &Connection, table_name: &str) -> Result<(), String> {
    conn.execute(
        "DELETE FROM d8a_monster_table_metadata WHERE table_name = ?",
        duckdb::params![table_name],
    )
    .map_err(|e| format!("Failed to remove table metadata: {}", e))?;

    Ok(())
}

pub fn rename_table_metadata(
    conn: &Connection,
    old_name: &str,
    new_name: &str,
) -> Result<(), String> {
    conn.execute(
        "UPDATE d8a_monster_table_metadata SET table_name = ? WHERE table_name = ?",
        duckdb::params![new_name, old_name],
    )
    .map_err(|e| format!("Failed to rename table metadata: {}", e))?;

    Ok(())
}

pub fn cleanup_orphaned_metadata(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "DELETE FROM d8a_monster_table_metadata WHERE table_name NOT IN (
            SELECT table_name FROM information_schema.tables WHERE table_schema = 'main'
        ) AND table_type != 'system'",
        [],
    )
    .map_err(|e| format!("Failed to cleanup orphaned metadata: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use duckdb::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS d8a_monster_table_metadata (
                table_name VARCHAR PRIMARY KEY,
                table_type VARCHAR NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                creation_query TEXT,
                source_path TEXT
            )",
            [],
        ).unwrap();
        conn
    }

    fn count_metadata(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM d8a_monster_table_metadata", [], |row| row.get(0)).unwrap()
    }

    fn has_metadata(conn: &Connection, name: &str) -> bool {
        conn.query_row(
            "SELECT COUNT(*) FROM d8a_monster_table_metadata WHERE table_name = ?",
            duckdb::params![name],
            |row| row.get::<_, i64>(0),
        ).unwrap() > 0
    }

    #[test]
    fn test_register() {
        let conn = setup();
        register_table_metadata(&conn, "my_table", "source").unwrap();
        assert!(has_metadata(&conn, "my_table"));
    }

    #[test]
    fn test_register_upsert() {
        let conn = setup();
        register_table_metadata(&conn, "t1", "source").unwrap();
        register_table_metadata(&conn, "t1", "model").unwrap();
        let typ: String = conn.query_row(
            "SELECT table_type FROM d8a_monster_table_metadata WHERE table_name = 't1'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(typ, "model");
        assert_eq!(count_metadata(&conn), 1);
    }

    #[test]
    fn test_remove() {
        let conn = setup();
        register_table_metadata(&conn, "t1", "source").unwrap();
        assert!(has_metadata(&conn, "t1"));
        remove_table_metadata(&conn, "t1").unwrap();
        assert!(!has_metadata(&conn, "t1"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let conn = setup();
        assert!(remove_table_metadata(&conn, "nope").is_ok());
    }

    #[test]
    fn test_rename() {
        let conn = setup();
        register_table_metadata(&conn, "old_name", "source").unwrap();
        rename_table_metadata(&conn, "old_name", "new_name").unwrap();
        assert!(!has_metadata(&conn, "old_name"));
        assert!(has_metadata(&conn, "new_name"));
    }

    #[test]
    fn test_cleanup_orphans() {
        let conn = setup();
        conn.execute("CREATE TABLE real_table AS SELECT 1 AS x", []).unwrap();
        register_table_metadata(&conn, "real_table", "source").unwrap();
        register_table_metadata(&conn, "ghost_table", "source").unwrap();
        assert_eq!(count_metadata(&conn), 2);
        cleanup_orphaned_metadata(&conn).unwrap();
        assert!(has_metadata(&conn, "real_table"));
        assert!(!has_metadata(&conn, "ghost_table"));
    }
}
