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
