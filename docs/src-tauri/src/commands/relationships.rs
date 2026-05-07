use tauri::State;
use crate::state::DuckDbState;
use crate::models::RelationshipInfo;

/// Detect relationships between a newly created table and existing tables
#[tauri::command]
pub fn detect_table_relationships(state: State<DuckDbState>, table_name: String) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    // Get columns from the new table
    let columns_query = format!(
        "SELECT column_name FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",
        table_name
    );
    let mut stmt = conn.prepare(&columns_query).map_err(|e| e.to_string())?;
    let new_table_columns: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;
    
    // Get all other MODEL tables only
    let tables_query = "SELECT table_name FROM _warphead_table_metadata WHERE table_type = 'model' AND table_name != ?";
    let mut tables_stmt = conn.prepare(tables_query).map_err(|e| e.to_string())?;
    let other_tables: Vec<String> = tables_stmt
        .query_map([&table_name], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;
    
    let mut relationships = Vec::new();
    
    for other_table in other_tables {
        let other_columns_query = format!(
            "SELECT column_name FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",
            other_table
        );
        let mut other_stmt = conn.prepare(&other_columns_query).map_err(|e| e.to_string())?;
        let other_table_columns: Vec<String> = other_stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| e.to_string())?;
        
        for col in &new_table_columns {
            if other_table_columns.contains(col) {
                relationships.push(RelationshipInfo {
                    from_table: table_name.clone(),
                    from_column: col.clone(),
                    to_table: other_table.clone(),
                    to_column: col.clone(),
                });
            }
        }
    }
    
    if !relationships.is_empty() {
        for rel in &relationships {
            conn.execute(
                "INSERT INTO _warphead_relationships (from_table, from_column, to_table, to_column, relationship_type, created_at) 
                 VALUES (?, ?, ?, ?, 'inferred', now())
                 ON CONFLICT (from_table, from_column, to_table, to_column) 
                 DO UPDATE SET relationship_type = 'inferred', created_at = now()",
                [&rel.from_table, &rel.from_column, &rel.to_table, &rel.to_column]
            ).map_err(|e| format!("Failed to store relationship: {}", e))?;
        }
    }
    
    serde_json::to_string(&relationships).map_err(|e| e.to_string())
}

/// Scan all existing tables and detect all relationships
#[tauri::command]
pub fn scan_all_relationships(state: State<DuckDbState>) -> Result<String, String> {
    let state_conn = state.conn.lock().map_err(|e| e.to_string())?;
    let conn = state_conn.as_ref().ok_or("DuckDB not initialized")?;
    
    let tables_query = "SELECT table_name FROM _warphead_table_metadata WHERE table_type = 'model' ORDER BY table_name";
    let mut tables_stmt = conn.prepare(tables_query).map_err(|e| e.to_string())?;
    let all_tables: Vec<String> = tables_stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;
    
    let mut all_relationships = Vec::new();
    
    for i in 0..all_tables.len() {
        let table1 = &all_tables[i];
        
        let columns_query1 = format!(
            "SELECT column_name FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",
            table1
        );
        let mut stmt1 = conn.prepare(&columns_query1).map_err(|e| e.to_string())?;
        let table1_columns: Vec<String> = stmt1
            .query_map([], |row| row.get(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| e.to_string())?;
        
        for j in (i + 1)..all_tables.len() {
            let table2 = &all_tables[j];
            
            let columns_query2 = format!(
                "SELECT column_name FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",
                table2
            );
            let mut stmt2 = conn.prepare(&columns_query2).map_err(|e| e.to_string())?;
            let table2_columns: Vec<String> = stmt2
                .query_map([], |row| row.get(0))
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<String>, _>>()
                .map_err(|e| e.to_string())?;
            
            for col in &table1_columns {
                if table2_columns.contains(col) {
                    all_relationships.push(RelationshipInfo {
                        from_table: table1.clone(),
                        from_column: col.clone(),
                        to_table: table2.clone(),
                        to_column: col.clone(),
                    });
                }
            }
        }
    }
    
    conn.execute(
        "DELETE FROM _warphead_relationships WHERE relationship_type = 'inferred'",
        []
    ).map_err(|e| format!("Failed to clear existing relationships: {}", e))?;
    
    if !all_relationships.is_empty() {
        for rel in &all_relationships {
            conn.execute(
                "INSERT INTO _warphead_relationships (from_table, from_column, to_table, to_column, relationship_type, created_at) 
                 VALUES (?, ?, ?, ?, 'inferred', now())",
                [&rel.from_table, &rel.from_column, &rel.to_table, &rel.to_column]
            ).map_err(|e| format!("Failed to store relationship: {}", e))?;
        }
    }
    
    serde_json::to_string(&all_relationships).map_err(|e| e.to_string())
}

