use std::collections::HashMap;

/// Detect the appropriate DuckDB type for a column based on sample values
pub fn detect_column_type(values: &[String]) -> (String, f64) {
    if values.is_empty() {
        return ("VARCHAR".to_string(), 0.0);
    }
    
    let mut type_scores: HashMap<String, usize> = HashMap::new();
    let total = values.len();
    
    for value in values {
        let trimmed = value.trim();
        
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
            continue; // Skip null values in type detection
        }
        
        // Try Boolean
        if trimmed.eq_ignore_ascii_case("true") || 
           trimmed.eq_ignore_ascii_case("false") || 
           trimmed == "0" || trimmed == "1" {
            *type_scores.entry("BOOLEAN".to_string()).or_insert(0) += 1;
        }
        
        // Try Integer
        if trimmed.parse::<i64>().is_ok() {
            *type_scores.entry("BIGINT".to_string()).or_insert(0) += 1;
        }
        
        // Try Float/Double
        if trimmed.parse::<f64>().is_ok() {
            *type_scores.entry("DOUBLE".to_string()).or_insert(0) += 1;
        }
        
        // Try Date (YYYY-MM-DD or similar)
        if trimmed.len() == 10 && trimmed.chars().filter(|c| *c == '-').count() == 2 {
            if chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").is_ok() {
                *type_scores.entry("DATE".to_string()).or_insert(0) += 1;
            }
        }
        
        // Try Timestamp (ISO 8601)
        if trimmed.len() >= 19 && (trimmed.contains('T') || trimmed.contains(' ')) {
            if chrono::NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S").is_ok() ||
               chrono::NaiveDateTime::parse_from_str(&trimmed.replace('T', " "), "%Y-%m-%d %H:%M:%S").is_ok() {
                *type_scores.entry("TIMESTAMP".to_string()).or_insert(0) += 1;
            }
        }
        
        // Try Time
        if trimmed.len() >= 5 && trimmed.len() <= 12 && trimmed.contains(':') {
            let time_parts: Vec<&str> = trimmed.split(':').collect();
            if time_parts.len() >= 2 && time_parts.iter().all(|p| p.parse::<u32>().is_ok()) {
                *type_scores.entry("TIME".to_string()).or_insert(0) += 1;
            }
        }
        
        // Try JSON
        if (trimmed.starts_with('{') && trimmed.ends_with('}')) ||
           (trimmed.starts_with('[') && trimmed.ends_with(']')) {
            if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                *type_scores.entry("JSON".to_string()).or_insert(0) += 1;
            }
        }
        
        // Default to VARCHAR
        *type_scores.entry("VARCHAR".to_string()).or_insert(0) += 1;
    }
    
    // Determine best type by highest score
    // Priority: TIMESTAMP > DATE > TIME > BOOLEAN > BIGINT > DOUBLE > JSON > VARCHAR
    let type_priority = vec![
        "TIMESTAMP", "DATE", "TIME", "BOOLEAN", "BIGINT", "DOUBLE", "JSON", "VARCHAR"
    ];
    
    let mut best_type = "VARCHAR".to_string();
    let mut best_score = 0;
    
    for type_name in type_priority {
        if let Some(&score) = type_scores.get(type_name) {
            if score > best_score {
                best_score = score;
                best_type = type_name.to_string();
            }
        }
    }
    
    // Calculate confidence (percentage of values matching the type)
    let confidence = best_score as f64 / total as f64;
    
    (best_type, confidence)
}

