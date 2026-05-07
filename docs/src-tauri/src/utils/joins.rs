use duckdb::Connection;
use std::collections::{HashMap, HashSet, VecDeque};

/// Struct to represent a JOIN path between tables
#[derive(Debug, Clone)]
pub struct JoinPath {
    pub table: String,
    pub join_column: String,
    pub source_column: String,
}

/// Build JOIN path from source_table to target_table using relationships
/// Uses breadth-first search to find shortest path
pub fn build_join_path(
    conn: &Connection,
    source_table: &str,
    target_table: &str,
) -> Result<Vec<JoinPath>, String> {
    if source_table == target_table {
        return Ok(Vec::new()); // No JOIN needed for same table
    }
    
    // Get all relationships from the database
    let relationships_query = "SELECT from_table, from_column, to_table, to_column FROM _warphead_relationships";
    let mut stmt = conn.prepare(relationships_query).map_err(|e| e.to_string())?;
    
    let relationships: Vec<(String, String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    // Build adjacency list (bidirectional graph)
    let mut graph: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    
    for (from_table, from_col, to_table, to_col) in relationships {
        // Add forward edge
        graph.entry(from_table.clone())
            .or_insert_with(Vec::new)
            .push((to_table.clone(), from_col.clone(), to_col.clone()));
        
        // Add reverse edge (relationships are bidirectional)
        graph.entry(to_table.clone())
            .or_insert_with(Vec::new)
            .push((from_table.clone(), to_col.clone(), from_col.clone()));
    }
    
    // BFS to find shortest path
    let mut queue: VecDeque<(String, Vec<JoinPath>)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    
    queue.push_back((source_table.to_string(), Vec::new()));
    visited.insert(source_table.to_string());
    
    while let Some((current_table, path)) = queue.pop_front() {
        if current_table == target_table {
            return Ok(path); // Found path!
        }
        
        if let Some(neighbors) = graph.get(&current_table) {
            for (next_table, source_col, target_col) in neighbors {
                if !visited.contains(next_table) {
                    visited.insert(next_table.clone());
                    
                    let mut new_path = path.clone();
                    new_path.push(JoinPath {
                        table: next_table.clone(),
                        join_column: target_col.clone(),
                        source_column: source_col.clone(),
                    });
                    
                    queue.push_back((next_table.clone(), new_path));
                }
            }
        }
    }
    
    Err(format!("No relationship path found from '{}' to '{}'", source_table, target_table))
}

/// Parse dimension string into table and column (e.g., "customers.customer_name" -> ("customers", "customer_name"))
pub fn parse_dimension(dimension: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = dimension.split('.').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid dimension format: '{}'. Expected format: 'table.column'", dimension));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

