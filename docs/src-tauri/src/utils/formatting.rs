/// Helper function to convert DuckDB ValueRef to a properly formatted JSON value
/// Handles complex types like Timestamp, Date, and Lists by converting them to human-readable strings
pub fn format_duckdb_value(val: duckdb::types::ValueRef) -> serde_json::Value {
    match val {
        duckdb::types::ValueRef::Null => serde_json::Value::Null,
        duckdb::types::ValueRef::Boolean(b) => serde_json::Value::Bool(b),
        duckdb::types::ValueRef::TinyInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::SmallInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::Int(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::BigInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::HugeInt(i) => {
            // Try to convert i128 to i64, fall back to string if too large
            match i64::try_from(i) {
                Ok(num) => serde_json::Value::Number(num.into()),
                Err(_) => serde_json::Value::String(i.to_string()),
            }
        }
        duckdb::types::ValueRef::UTinyInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::USmallInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::UInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::UBigInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::Float(f) => {
            serde_json::Number::from_f64(f as f64)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        }
        duckdb::types::ValueRef::Double(f) => {
            serde_json::Number::from_f64(f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        }
        duckdb::types::ValueRef::Decimal(d) => {
            // Convert Decimal to string to preserve precision
            serde_json::Value::String(d.to_string())
        }
        duckdb::types::ValueRef::Timestamp(unit, i) => {
            // Convert timestamp to ISO 8601 format
            match unit {
                duckdb::types::TimeUnit::Second => {
                    if let Some(dt) = chrono::DateTime::from_timestamp(i, 0) {
                        serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    } else {
                        serde_json::Value::String(format!("Timestamp({})", i))
                    }
                }
                duckdb::types::TimeUnit::Millisecond => {
                    if let Some(dt) = chrono::DateTime::from_timestamp_millis(i) {
                        serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
                    } else {
                        serde_json::Value::String(format!("Timestamp({})", i))
                    }
                }
                duckdb::types::TimeUnit::Microsecond => {
                    let secs = i / 1_000_000;
                    let nanos = ((i % 1_000_000) * 1000) as u32;
                    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
                        serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string())
                    } else {
                        serde_json::Value::String(format!("Timestamp({})", i))
                    }
                }
                duckdb::types::TimeUnit::Nanosecond => {
                    let secs = i / 1_000_000_000;
                    let nanos = (i % 1_000_000_000) as u32;
                    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
                        serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S%.9f").to_string())
                    } else {
                        serde_json::Value::String(format!("Timestamp({})", i))
                    }
                }
            }
        }
        duckdb::types::ValueRef::Date32(days) => {
            // Date32 is days since Unix epoch (1970-01-01)
            let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
            if let Some(date) = epoch.checked_add_signed(chrono::Duration::days(days as i64)) {
                serde_json::Value::String(date.format("%Y-%m-%d").to_string())
            } else {
                serde_json::Value::String(format!("Date({})", days))
            }
        }
        duckdb::types::ValueRef::Time64(unit, t) => {
            // Time64 represents time of day
            let time_str = match unit {
                duckdb::types::TimeUnit::Microsecond => {
                    let total_seconds = t / 1_000_000;
                    let micros = t % 1_000_000;
                    let hours = total_seconds / 3600;
                    let minutes = (total_seconds % 3600) / 60;
                    let seconds = total_seconds % 60;
                    format!("{:02}:{:02}:{:02}.{:06}", hours, minutes, seconds, micros)
                }
                duckdb::types::TimeUnit::Nanosecond => {
                    let total_seconds = t / 1_000_000_000;
                    let nanos = t % 1_000_000_000;
                    let hours = total_seconds / 3600;
                    let minutes = (total_seconds % 3600) / 60;
                    let seconds = total_seconds % 60;
                    format!("{:02}:{:02}:{:02}.{:09}", hours, minutes, seconds, nanos)
                }
                _ => format!("Time({})", t),
            };
            serde_json::Value::String(time_str)
        }
        duckdb::types::ValueRef::Interval { months, days, nanos } => {
            // Format interval in a readable way
            let mut parts = Vec::new();
            if months != 0 {
                parts.push(format!("{} months", months));
            }
            if days != 0 {
                parts.push(format!("{} days", days));
            }
            if nanos != 0 {
                let seconds = nanos / 1_000_000_000;
                if seconds != 0 {
                    parts.push(format!("{} seconds", seconds));
                }
            }
            serde_json::Value::String(if parts.is_empty() {
                "0 seconds".to_string()
            } else {
                parts.join(", ")
            })
        }
        duckdb::types::ValueRef::Text(s) => {
            serde_json::Value::String(String::from_utf8_lossy(s).to_string())
        }
        duckdb::types::ValueRef::Blob(b) => {
            serde_json::Value::String(format!("<blob {} bytes>", b.len()))
        }
        // For complex types like Lists, Structs, and Maps, convert to JSON
        _ => {
            // Try to serialize as JSON string for complex types
            let debug_str = format!("{:?}", val);
            
            // If it's a List or Array, try to parse and format nicely
            if debug_str.starts_with("List(") {
                // Extract the content and try to make it more readable
                if let Some(content) = extract_list_content(&debug_str) {
                    return serde_json::Value::String(content);
                }
            }
            
            // Otherwise, return the debug representation
            serde_json::Value::String(debug_str)
        }
    }
}

/// Helper to extract and format list content from debug string
fn extract_list_content(debug_str: &str) -> Option<String> {
    // Try to extract string array content
    if debug_str.contains("StringArray") {
        let start = debug_str.find('[')?;
        let end = debug_str.rfind(']')?;
        let content = &debug_str[start..=end];
        
        // Clean up the content: remove extra quotes and format
        let cleaned = content
            .replace("StringArray", "")
            .replace("  ", " ")
            .trim()
            .to_string();
        
        return Some(cleaned);
    }
    
    // For other list types, try to extract content between brackets
    if let Some(start) = debug_str.find('[') {
        if let Some(end) = debug_str.rfind(']') {
            return Some(debug_str[start..=end].to_string());
        }
    }
    
    None
}

