use duckdb::{Connection, params};

/// Helper function to register or update a table in metadata
pub fn register_table_metadata(conn: &Connection, table_name: &str, table_type: &str) -> Result<(), String> {
    register_table_metadata_with_query(conn, table_name, table_type, None)
}

/// Helper function to register a table in the metadata table with optional creation query
pub fn register_table_metadata_with_query(conn: &Connection, table_name: &str, table_type: &str, creation_query: Option<&str>) -> Result<(), String> {
    // First try to get existing created_at
    let existing_created_at: Option<String> = conn.query_row(
        "SELECT created_at FROM _warphead_table_metadata WHERE table_name = ?",
        [table_name],
        |row| row.get(0)
    ).ok();
    
    // Use INSERT ... ON CONFLICT for DuckDB compatibility
    if let Some(created_at) = existing_created_at {
        if let Some(query) = creation_query {
            conn.execute(
                "INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query) VALUES (?, ?, ?, ?) 
                 ON CONFLICT (table_name) DO UPDATE SET table_type = EXCLUDED.table_type, creation_query = EXCLUDED.creation_query",
                params![table_name, table_type, &created_at, query]
            )
        } else {
            conn.execute(
                "INSERT INTO _warphead_table_metadata (table_name, table_type, created_at) VALUES (?, ?, ?) 
                 ON CONFLICT (table_name) DO UPDATE SET table_type = EXCLUDED.table_type",
                params![table_name, table_type, &created_at]
            )
        }
    } else {
        if let Some(query) = creation_query {
            conn.execute(
                "INSERT INTO _warphead_table_metadata (table_name, table_type, creation_query) VALUES (?, ?, ?) 
                 ON CONFLICT (table_name) DO UPDATE SET table_type = EXCLUDED.table_type, creation_query = EXCLUDED.creation_query",
                params![table_name, table_type, query]
            )
        } else {
            conn.execute(
                "INSERT INTO _warphead_table_metadata (table_name, table_type) VALUES (?, ?) 
                 ON CONFLICT (table_name) DO UPDATE SET table_type = EXCLUDED.table_type",
                params![table_name, table_type]
            )
        }
    }.map_err(|e| format!("Failed to register table metadata: {}", e))?;
    Ok(())
}

/// Helper function to remove a table from metadata
pub fn remove_table_metadata(conn: &Connection, table_name: &str) -> Result<(), String> {
    conn.execute(
        "DELETE FROM _warphead_table_metadata WHERE table_name = ?",
        [table_name]
    ).map_err(|e| format!("Failed to remove table metadata: {}", e))?;
    Ok(())
}

/// Helper function to clean up orphaned metadata
/// Removes metadata entries for tables that no longer exist in the database
pub fn cleanup_orphaned_metadata(conn: &Connection) -> Result<String, String> {
    // Get list of tables from metadata that are source or model types
    let mut stmt = conn.prepare(
        "SELECT table_name FROM _warphead_table_metadata WHERE table_type IN ('source', 'model')"
    ).map_err(|e| format!("Failed to prepare metadata query: {}", e))?;
    
    let metadata_tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("Failed to query metadata: {}", e))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| format!("Failed to collect metadata tables: {}", e))?;
    
    drop(stmt); // Release the statement before modifying
    
    let mut orphaned_count = 0;
    
    // Check each table in metadata to see if it exists in information_schema
    for table_name in metadata_tables {
        let exists: Result<i64, _> = conn.query_row(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = ?",
            [&table_name],
            |row| row.get(0)
        );
        
        match exists {
            Ok(count) if count == 0 => {
                // Table doesn't exist, remove from metadata
                conn.execute(
                    "DELETE FROM _warphead_table_metadata WHERE table_name = ?",
                    [&table_name]
                ).map_err(|e| format!("Failed to remove orphaned metadata for {}: {}", table_name, e))?;
                
                // Also remove from datamodels if exists
                conn.execute(
                    "DELETE FROM datamodels WHERE table_name = ?",
                    [&table_name]
                ).map_err(|e| format!("Failed to remove from datamodels for {}: {}", table_name, e))?;
                
                orphaned_count += 1;
                eprintln!("Cleaned up orphaned metadata for table: {}", table_name);
            },
            Ok(_) => {
                // Table exists, all good
            },
            Err(e) => {
                eprintln!("Warning: Failed to check existence of table {}: {}", table_name, e);
            }
        }
    }
    
    if orphaned_count > 0 {
        Ok(format!("Cleaned up {} orphaned table metadata entries", orphaned_count))
    } else {
        Ok("No orphaned metadata found".to_string())
    }
}

