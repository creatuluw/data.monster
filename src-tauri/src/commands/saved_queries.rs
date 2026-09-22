//! Saved queries: file-backed storage under `dm/saved-queries/<slug>.sql`
//! (workspace-file-first FR-4). File = optional single-line `-- dm: {json}` meta header
//! (name/description/tags — app always writes it; agent-written headerless files default
//! to name=stem) + the SQL body verbatim. No timestamps in files (git-safe).
//! `tags` is a JSON array in the file, comma-separated string at the invoke boundary
//! (frontend splits on ','). Invoke names/params unchanged.

use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use tauri::AppHandle;

use crate::commands::dm_store;
use crate::commands::workspace;
use crate::utils::slugs;

/// Resolve the active workspace directory (commands only; cores take `ws` directly).
fn ws_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let ws = workspace::get_workspace_path(app.clone())?;
    ws.map(std::path::PathBuf::from)
        .ok_or_else(|| "No workspace open".to_string())
}

const HEADER_PREFIX: &str = "-- dm:";

/// comma-separated invoke string → JSON array (or null).
fn tags_to_array(tags: Option<&str>) -> serde_json::Value {
    match tags {
        Some(t) if !t.trim().is_empty() => json!(t
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()),
        _ => serde_json::Value::Null,
    }
}

/// JSON array → comma-separated invoke string (or null).
fn tags_to_string(v: &serde_json::Value) -> Option<String> {
    v.as_array().map(|a| {
        a.iter()
            .filter_map(|t| t.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    })
}

/// Serialize one query to file bytes: header line (always) + body verbatim.
fn serialize(name: &str, sql: &str, description: Option<&str>, tags: Option<&str>) -> String {
    let meta = json!({
        "name": name,
        "description": description,
        "tags": tags_to_array(tags),
    });
    format!(
        "{} {}\n{}",
        HEADER_PREFIX,
        serde_json::to_string(&meta).unwrap(),
        sql
    )
}

/// Parse file bytes → (name, sql, description, tags-as-invoke-string). Headerless
/// files: name = file stem, rest null, body = whole file.
fn parse(bytes: &[u8], slug: &str) -> Result<(String, String, Option<String>, Option<String>), String> {
    let text = String::from_utf8_lossy(bytes);
    let (first, body) = match text.split_once('\n') {
        Some((f, rest)) => (f, rest),
        None => (text.trim_end(), ""),
    };
    let trimmed = first.trim_start();
    if !trimmed.starts_with(HEADER_PREFIX) {
        // headerless: entire file is SQL
        return Ok((slug.to_string(), text.to_string(), None, None));
    }
    let meta: serde_json::Value = serde_json::from_str(trimmed[HEADER_PREFIX.len()..].trim())
        .map_err(|e| format!("malformed {} header: {e}", HEADER_PREFIX))?;
    let name = meta
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(slug)
        .to_string();
    let description = meta
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);
    let tags = meta.get("tags").and_then(tags_to_string);
    Ok((name, body.to_string(), description, tags))
}

fn mtime_iso(path: &Path) -> Option<String> {
    let t: SystemTime = fs::metadata(path).ok()?.modified().ok()?;
    Some(chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
}

pub fn list_saved_queries_in(ws: &Path) -> Result<serde_json::Value, String> {
    let dir = dm_store::saved_queries_dir(ws);
    if !dir.exists() {
        return Ok(json!({ "queries": [], "errors": [] }));
    }

    let mut queries = Vec::new();
    let mut errors = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| format!("Failed to read {}: {e}", dir.display()))? {
        let path = match entry {
            Ok(e) if e.path().is_file() => e.path(),
            Ok(_) => continue,
            Err(e) => {
                errors.push(json!({ "slug": null, "error": e.to_string() }));
                continue;
            }
        };
        let Some(slug) = dm_store::saved_query_slug_from_path(&path) else {
            continue; // non-.sql — not ours
        };
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                errors.push(json!({ "slug": slug, "error": format!("unreadable: {e}") }));
                continue;
            }
        };
        match parse(&bytes, &slug) {
            Ok((name, sql, description, tags)) => queries.push(json!({
                "slug": slug,
                "name": name,
                "sql": sql,
                "description": description,
                "tags": tags,
                "createdAt": null,
                "updatedAt": mtime_iso(&path),
            })),
            Err(e) => errors.push(json!({ "slug": slug, "error": e })),
        }
    }

    queries.sort_by(|a, b| {
        let key = |v: &serde_json::Value| {
            v.get("updatedAt")
                .and_then(|u| u.as_str())
                .unwrap_or("")
                .to_string()
        };
        key(b).cmp(&key(a)) // mtime desc, matching the old ORDER BY updated_at DESC
    });

    Ok(json!({ "queries": queries, "errors": errors }))
}

/// Upsert by generated slug — saving the same name twice overwrites (old semantics).
pub fn save_query_in(
    ws: &Path,
    name: &str,
    sql: &str,
    description: Option<&str>,
    tags: Option<&str>,
) -> Result<(), String> {
    let slug = slugs::generate_slug(name);
    write_query_file(ws, &slug, name, sql, description, tags)
}

/// Write to an EXACT slug (migration: the DB slug may predate a rename).
pub(crate) fn write_query_file(
    ws: &Path,
    slug: &str,
    name: &str,
    sql: &str,
    description: Option<&str>,
    tags: Option<&str>,
) -> Result<(), String> {
    let path = dm_store::saved_query_path(ws, slug)?;
    dm_store::atomic_write(&path, serialize(name, sql, description, tags).as_bytes())
}

/// Partial merge (None = keep existing), matching the old UPDATE semantics.
pub fn update_saved_query_in(
    ws: &Path,
    slug: &str,
    name: Option<&str>,
    sql: Option<&str>,
    description: Option<&str>,
    tags: Option<&str>,
) -> Result<(), String> {
    let path = dm_store::saved_query_path(ws, slug)?;
    let bytes = fs::read(&path).map_err(|_| "Query not found".to_string())?;
    let (cur_name, cur_sql, cur_desc, cur_tags) = parse(&bytes, slug)?;

    let new_name = name.unwrap_or(&cur_name);
    let new_sql = sql.unwrap_or(&cur_sql);
    let new_desc = description.map(String::from).or(cur_desc);
    let new_tags = tags.map(String::from).or(cur_tags);

    dm_store::atomic_write(
        &path,
        serialize(new_name, new_sql, new_desc.as_deref(), new_tags.as_deref()).as_bytes(),
    )
}

/// Idempotent.
pub fn delete_saved_query_in(ws: &Path, slug: &str) -> Result<(), String> {
    let path = dm_store::saved_query_path(ws, slug)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete query: {e}"))?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_saved_queries(app: AppHandle) -> Result<serde_json::Value, String> {
    list_saved_queries_in(&ws_dir(&app)?)
}

#[tauri::command]
pub fn save_query(
    app: AppHandle,
    name: String,
    sql: String,
    description: Option<String>,
    tags: Option<String>,
) -> Result<(), String> {
    save_query_in(&ws_dir(&app)?, &name, &sql, description.as_deref(), tags.as_deref())
}

#[tauri::command]
pub fn update_saved_query(
    app: AppHandle,
    slug: String,
    name: Option<String>,
    sql: Option<String>,
    description: Option<String>,
    tags: Option<String>,
) -> Result<(), String> {
    update_saved_query_in(
        &ws_dir(&app)?,
        &slug,
        name.as_deref(),
        sql.as_deref(),
        description.as_deref(),
        tags.as_deref(),
    )
}

#[tauri::command]
pub fn delete_saved_query(app: AppHandle, slug: String) -> Result<(), String> {
    delete_saved_query_in(&ws_dir(&app)?, &slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("sq-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn save_writes_header_and_body_round_trips() {
        let ws = tmp_ws("roundtrip");
        save_query_in(&ws, "My Query", "SELECT 1\n-- note\nFROM t", Some("demo"), Some("sales, revenue")).unwrap();

        let path = dm_store::saved_query_path(&ws, "my-query").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("-- dm: {"));
        assert!(content.contains("\"name\":\"My Query\""));
        assert!(!content.contains("createdAt"));

        let out = list_saved_queries_in(&ws).unwrap();
        let q = &out["queries"].as_array().unwrap()[0];
        assert_eq!(q["slug"].as_str().unwrap(), "my-query");
        assert_eq!(q["name"].as_str().unwrap(), "My Query");
        assert_eq!(q["sql"].as_str().unwrap(), "SELECT 1\n-- note\nFROM t");
        assert_eq!(q["description"].as_str().unwrap(), "demo");
        assert_eq!(q["tags"].as_str().unwrap(), "sales, revenue"); // invoke boundary: comma string
        assert!(q["updatedAt"].as_str().is_some());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn same_name_overwrites_by_slug() {
        let ws = tmp_ws("collision");
        save_query_in(&ws, "My Query", "SELECT 1", None, None).unwrap();
        save_query_in(&ws, "My Query", "SELECT 2", None, None).unwrap();
        let out = list_saved_queries_in(&ws).unwrap();
        let queries = out["queries"].as_array().unwrap();
        assert_eq!(queries.len(), 1);
        assert_eq!(queries[0]["sql"].as_str().unwrap(), "SELECT 2");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn headerless_agent_file_defaults_to_stem_name() {
        let ws = tmp_ws("headerless");
        let path = dm_store::saved_query_path(&ws, "top-customers").unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "-- a plain comment\nSELECT * FROM orders").unwrap();

        let out = list_saved_queries_in(&ws).unwrap();
        let q = &out["queries"].as_array().unwrap()[0];
        assert_eq!(q["name"].as_str().unwrap(), "top-customers");
        assert_eq!(q["sql"].as_str().unwrap(), "-- a plain comment\nSELECT * FROM orders");
        assert!(q["description"].is_null());
        assert!(q["tags"].is_null());
        assert_eq!(out["errors"].as_array().unwrap().len(), 0);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn malformed_header_is_a_reported_error_not_silent_sql() {
        let ws = tmp_ws("malformed");
        let path = dm_store::saved_query_path(&ws, "broken").unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "-- dm: {oops\nSELECT 1").unwrap();

        let out = list_saved_queries_in(&ws).unwrap();
        assert_eq!(out["queries"].as_array().unwrap().len(), 0);
        let errors = out["errors"].as_array().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0]["slug"].as_str().unwrap(), "broken");
        assert!(update_saved_query_in(&ws, "broken", None, Some("SELECT 2"), None, None).is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn update_partial_merge_keeps_unspecified_fields() {
        let ws = tmp_ws("update");
        save_query_in(&ws, "My Query", "SELECT 1", Some("demo"), Some("sales")).unwrap();
        update_saved_query_in(&ws, "my-query", Some("Renamed"), Some("SELECT 2"), None, None).unwrap();

        let out = list_saved_queries_in(&ws).unwrap();
        let q = &out["queries"].as_array().unwrap()[0];
        assert_eq!(q["slug"].as_str().unwrap(), "my-query"); // slug (filename) unchanged
        assert_eq!(q["name"].as_str().unwrap(), "Renamed");
        assert_eq!(q["sql"].as_str().unwrap(), "SELECT 2");
        assert_eq!(q["description"].as_str().unwrap(), "demo"); // kept
        assert_eq!(q["tags"].as_str().unwrap(), "sales"); // kept
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn update_missing_is_query_not_found() {
        let ws = tmp_ws("missing");
        assert_eq!(
            update_saved_query_in(&ws, "nope", Some("n"), None, None, None).unwrap_err(),
            "Query not found"
        );
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn delete_idempotent_and_list_on_missing_dir_empty() {
        let ws = tmp_ws("delete");
        assert_eq!(list_saved_queries_in(&ws).unwrap()["queries"].as_array().unwrap().len(), 0);
        save_query_in(&ws, "Q", "SELECT 1", None, None).unwrap();
        delete_saved_query_in(&ws, "q").unwrap();
        delete_saved_query_in(&ws, "q").unwrap(); // still Ok
        assert_eq!(list_saved_queries_in(&ws).unwrap()["queries"].as_array().unwrap().len(), 0);
        let _ = fs::remove_dir_all(&ws);
    }
}
