use duckdb::Connection;

/// Generate a URL-safe slug from a name
/// Converts to lowercase, replaces spaces and special characters with hyphens
pub fn generate_slug(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace("--", "-")
        .replace("--", "-")  // Run twice to handle consecutive replacements
        .trim_matches('-')
        .to_string()
}

/// Check if a slug is unique in the metrics table
pub fn is_metric_slug_unique(conn: &Connection, slug: &str, exclude_slug: Option<&str>) -> bool {
    let query = if let Some(existing_slug) = exclude_slug {
        format!(
            "SELECT COUNT(*) FROM _warphead_metrics WHERE slug = '{}' AND slug != '{}'",
            slug, existing_slug
        )
    } else {
        format!("SELECT COUNT(*) FROM _warphead_metrics WHERE slug = '{}'", slug)
    };
    
    if let Ok(mut stmt) = conn.prepare(&query) {
        if let Ok(count) = stmt.query_row([], |row| row.get::<_, i64>(0)) {
            return count == 0;
        }
    }
    true // If we can't check, assume it's unique
}

/// Check if a slug is unique in the dimensions table
pub fn is_dimension_slug_unique(conn: &Connection, slug: &str, exclude_slug: Option<&str>) -> bool {
    let query = if let Some(existing_slug) = exclude_slug {
        format!(
            "SELECT COUNT(*) FROM _warphead_dimensions WHERE slug = '{}' AND slug != '{}'",
            slug, existing_slug
        )
    } else {
        format!("SELECT COUNT(*) FROM _warphead_dimensions WHERE slug = '{}'", slug)
    };
    
    if let Ok(mut stmt) = conn.prepare(&query) {
        if let Ok(count) = stmt.query_row([], |row| row.get::<_, i64>(0)) {
            return count == 0;
        }
    }
    true // If we can't check, assume it's unique
}

/// Generate a unique slug for a metric
pub fn generate_unique_metric_slug(conn: &Connection, name: &str, exclude_slug: Option<&str>) -> String {
    let base_slug = generate_slug(name);
    
    // If base slug is unique, use it
    if is_metric_slug_unique(conn, &base_slug, exclude_slug) {
        return base_slug;
    }
    
    // Otherwise, append a number
    for i in 2..=999 {
        let numbered_slug = format!("{}-{}", base_slug, i);
        if is_metric_slug_unique(conn, &numbered_slug, exclude_slug) {
            return numbered_slug;
        }
    }
    
    // Fallback: use timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}-{}", base_slug, timestamp)
}

/// Generate a unique slug for a dimension
pub fn generate_unique_dimension_slug(conn: &Connection, name: &str, exclude_slug: Option<&str>) -> String {
    let base_slug = generate_slug(name);
    
    // If base slug is unique, use it
    if is_dimension_slug_unique(conn, &base_slug, exclude_slug) {
        return base_slug;
    }
    
    // Otherwise, append a number
    for i in 2..=999 {
        let numbered_slug = format!("{}-{}", base_slug, i);
        if is_dimension_slug_unique(conn, &numbered_slug, exclude_slug) {
            return numbered_slug;
        }
    }
    
    // Fallback: use timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}-{}", base_slug, timestamp)
}

