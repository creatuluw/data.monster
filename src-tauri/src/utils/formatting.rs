pub fn format_duckdb_value(val: duckdb::types::ValueRef) -> serde_json::Value {
    match val {
        duckdb::types::ValueRef::Null => serde_json::Value::Null,
        duckdb::types::ValueRef::Boolean(b) => serde_json::Value::Bool(b),
        duckdb::types::ValueRef::TinyInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::SmallInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::Int(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::BigInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::HugeInt(i) => match i64::try_from(i) {
            Ok(num) => serde_json::Value::Number(num.into()),
            Err(_) => serde_json::Value::String(i.to_string()),
        },
        duckdb::types::ValueRef::UTinyInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::USmallInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::UInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::UBigInt(i) => serde_json::Value::Number(i.into()),
        duckdb::types::ValueRef::Float(f) => serde_json::Number::from_f64(f as f64)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        duckdb::types::ValueRef::Double(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        duckdb::types::ValueRef::Decimal(d) => serde_json::Value::String(d.to_string()),
        duckdb::types::ValueRef::Timestamp(unit, i) => match unit {
            duckdb::types::TimeUnit::Second => {
                if let Some(dt) = chrono::DateTime::from_timestamp(i, 0) {
                    serde_json::Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                } else {
                    serde_json::Value::String(format!("Timestamp({})", i))
                }
            }
            duckdb::types::TimeUnit::Millisecond => {
                if let Some(dt) = chrono::DateTime::from_timestamp_millis(i) {
                    serde_json::Value::String(
                        dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    )
                } else {
                    serde_json::Value::String(format!("Timestamp({})", i))
                }
            }
            duckdb::types::TimeUnit::Microsecond => {
                let secs = i / 1_000_000;
                let nanos = ((i % 1_000_000) * 1000) as u32;
                if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
                    serde_json::Value::String(
                        dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string(),
                    )
                } else {
                    serde_json::Value::String(format!("Timestamp({})", i))
                }
            }
            duckdb::types::TimeUnit::Nanosecond => {
                let secs = i / 1_000_000_000;
                let nanos = (i % 1_000_000_000) as u32;
                if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
                    serde_json::Value::String(
                        dt.format("%Y-%m-%d %H:%M:%S%.9f").to_string(),
                    )
                } else {
                    serde_json::Value::String(format!("Timestamp({})", i))
                }
            }
        },
        duckdb::types::ValueRef::Date32(days) => {
            let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
            if let Some(date) = epoch.checked_add_signed(chrono::Duration::days(days as i64)) {
                serde_json::Value::String(date.format("%Y-%m-%d").to_string())
            } else {
                serde_json::Value::String(format!("Date({})", days))
            }
        }
        duckdb::types::ValueRef::Text(s) => {
            serde_json::Value::String(String::from_utf8_lossy(s).to_string())
        }
        duckdb::types::ValueRef::Blob(b) => {
            serde_json::Value::String(format!("<blob {} bytes>", b.len()))
        }
        _ => {
            let debug_str = format!("{:?}", val);
            serde_json::Value::String(debug_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use duckdb::Connection;

    fn query_val(conn: &Connection, sql: &str) -> serde_json::Value {
        let mut stmt = conn.prepare(sql).unwrap();
        let result: serde_json::Value = stmt
            .query_map([], |row| Ok(format_duckdb_value(row.get_ref(0).unwrap())))
            .unwrap()
            .filter_map(|r| r.ok())
            .next()
            .unwrap();
        result
    }

    #[test]
    fn test_null() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(query_val(&conn, "SELECT NULL"), serde_json::Value::Null);
    }

    #[test]
    fn test_boolean() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(query_val(&conn, "SELECT true"), serde_json::Value::Bool(true));
        assert_eq!(query_val(&conn, "SELECT false"), serde_json::Value::Bool(false));
    }

    #[test]
    fn test_integer() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT 42");
        assert_eq!(val, serde_json::json!(42));
    }

    #[test]
    fn test_bigint() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT 9999999999::BIGINT");
        assert_eq!(val, serde_json::json!(9999999999i64));
    }

    #[test]
    fn test_double() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT 3.14::DOUBLE");
        assert!(val.is_f64() || val.is_string());
        if let Some(f) = val.as_f64() {
            assert!((f - 3.14f64).abs() < 0.001);
        }
    }

    #[test]
    fn test_text() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT 'hello world'");
        assert_eq!(val, serde_json::json!("hello world"));
    }

    #[test]
    fn test_decimal() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT 1.5::DECIMAL(10,2)");
        assert!(val.is_string());
    }

    #[test]
    fn test_date() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT DATE '2024-01-15'");
        assert_eq!(val, serde_json::json!("2024-01-15"));
    }

    #[test]
    fn test_blob() {
        let conn = Connection::open_in_memory().unwrap();
        let val = query_val(&conn, "SELECT BLOB 'abc'");
        assert!(val.as_str().unwrap().contains("blob"));
    }
}
