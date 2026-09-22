//! Master items: file-backed storage under `dm/master-items/{measures,dimensions}/<id>.json`
//! (workspace-file-first FR-3). Filename = stable id (charts reference `{ "ref": id }`);
//! `kind` picks the folder. No timestamps in files (git-safe; file mtime replaces them).
//! Command invoke names/params unchanged (frontend `central-api.ts` untouched).

use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

use crate::commands::dm_store;
use crate::commands::workspace;

/// Resolve the active workspace directory (commands only; cores take `ws` directly).
fn ws_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let ws = workspace::get_workspace_path(app.clone())?;
    ws.map(std::path::PathBuf::from)
        .ok_or_else(|| "No workspace open".to_string())
}

/// Read one item file; `kind`/`id` come from the folder/filename (authoritative).
fn read_item(path: &Path, kind: &str, id: &str) -> Result<serde_json::Value, String> {
    let bytes = fs::read(path).map_err(|e| format!("unreadable: {e}"))?;
    dm_store::check_json_object(&bytes, &["tableName", "expr"])?;
    let mut v: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("invalid JSON: {e}"))?;
    let obj = v.as_object_mut().ok_or("expected a JSON object")?;
    obj.insert("id".into(), json!(id));
    obj.insert("kind".into(), json!(kind));
    // timestamps are not stored in files
    obj.insert("createdAt".into(), serde_json::Value::Null);
    obj.insert("updatedAt".into(), serde_json::Value::Null);
    Ok(v)
}

pub fn list_items_in(ws: &Path, kind: Option<&str>) -> Result<serde_json::Value, String> {
    let kinds: &[&str] = match kind {
        Some("measure") => &["measure"],
        Some("dimension") => &["dimension"],
        Some(other) => return Err(format!("invalid master-item kind: {other:?}")),
        None => &["measure", "dimension"],
    };

    let mut items = Vec::new();
    let mut errors = Vec::new();
    for k in kinds {
        let dir = dm_store::item_dir(ws, k)?;
        if !dir.exists() {
            continue;
        }
        for entry in
            fs::read_dir(&dir).map_err(|e| format!("Failed to read {}: {e}", dir.display()))?
        {
            let path = match entry {
                Ok(e) if e.path().is_file() => e.path(),
                Ok(_) => continue,
                Err(e) => {
                    errors.push(json!({ "id": null, "error": e.to_string() }));
                    continue;
                }
            };
            let Some(id) = dm_store::item_from_path(&path).map(|(_, id)| id) else {
                continue; // not a .json item file — not ours
            };
            match read_item(&path, k, &id) {
                Ok(item) => items.push(item),
                Err(e) => errors.push(json!({ "id": id, "error": e })),
            }
        }
    }

    items.sort_by(|a, b| {
        let key = |v: &serde_json::Value| {
            (
                v["kind"].as_str().unwrap_or("").to_string(),
                v["id"].as_str().unwrap_or("").to_string(),
            )
        };
        key(a).cmp(&key(b)) // deterministic: kind, then id
    });

    Ok(json!({ "items": items, "errors": errors }))
}

pub fn get_item_in(ws: &Path, id: &str) -> Result<serde_json::Value, String> {
    for kind in ["measure", "dimension"] {
        let path = dm_store::item_path(ws, kind, id)?;
        if path.exists() {
            return read_item(&path, kind, id);
        }
    }
    Err("Master item not found".to_string())
}

/// Upsert one item. The file contains NO timestamps (git-safe diffs).
pub fn save_item_in(
    ws: &Path,
    id: &str,
    kind: &str,
    table_name: &str,
    expr: &str,
    label: &str,
    fmt: Option<&str>,
    description: Option<&str>,
) -> Result<(), String> {
    let doc = json!({
        "id": id,
        "kind": kind,
        "tableName": table_name,
        "expr": expr,
        "label": label,
        "fmt": fmt,
        "description": description,
    });
    let path = dm_store::item_path(ws, kind, id)?;
    dm_store::atomic_write(&path, serde_json::to_string_pretty(&doc).unwrap().as_bytes())
}

/// Idempotent across both folders (id may live in either).
pub fn delete_item_in(ws: &Path, id: &str) -> Result<(), String> {
    for kind in ["measure", "dimension"] {
        let path = dm_store::item_path(ws, kind, id)?;
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete master item: {e}"))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn list_master_items(app: AppHandle, kind: Option<String>) -> Result<serde_json::Value, String> {
    list_items_in(&ws_dir(&app)?, kind.as_deref())
}

#[tauri::command]
pub fn get_master_item(app: AppHandle, id: String) -> Result<serde_json::Value, String> {
    get_item_in(&ws_dir(&app)?, &id)
}

#[tauri::command]
pub fn save_master_item(
    app: AppHandle,
    id: String,
    kind: String,
    table_name: String,
    expr: String,
    label: String,
    fmt: Option<String>,
    description: Option<String>,
) -> Result<(), String> {
    save_item_in(
        &ws_dir(&app)?,
        &id,
        &kind,
        &table_name,
        &expr,
        &label,
        fmt.as_deref(),
        description.as_deref(),
    )
}

#[tauri::command]
pub fn delete_master_item(app: AppHandle, id: String) -> Result<(), String> {
    delete_item_in(&ws_dir(&app)?, &id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("items-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn save_total(ws: &Path, kind: &str) {
        save_item_in(ws, "total_revenue", kind, "orders", "sum(amount)", "Total revenue", Some("$.2f"), None).unwrap();
    }

    #[test]
    fn save_then_get_round_trips_measures() {
        let ws = tmp_ws("roundtrip");
        save_total(&ws, "measure");
        let got = get_item_in(&ws, "total_revenue").unwrap();
        assert_eq!(got["kind"].as_str().unwrap(), "measure");
        assert_eq!(got["tableName"].as_str().unwrap(), "orders");
        assert_eq!(got["expr"].as_str().unwrap(), "sum(amount)");
        assert_eq!(got["label"].as_str().unwrap(), "Total revenue");
        assert_eq!(got["fmt"].as_str().unwrap(), "$.2f");
        assert!(got["description"].is_null());
        assert!(got["createdAt"].is_null(), "timestamps are not stored");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn get_searches_both_folders() {
        let ws = tmp_ws("search-both");
        save_item_in(&ws, "region", "dimension", "orders", "region", "Region", None, None).unwrap();
        let got = get_item_in(&ws, "region").unwrap();
        assert_eq!(got["kind"].as_str().unwrap(), "dimension");
        assert!(get_item_in(&ws, "nope").is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn list_both_kinds_filtered_sorted_ids_from_filenames() {
        let ws = tmp_ws("list");
        save_total(&ws, "measure");
        save_item_in(&ws, "b_item", "measure", "t", "1", "B", None, None).unwrap();
        save_item_in(&ws, "region", "dimension", "t", "region", "Region", None, None).unwrap();
        // agent-written garbage lands in errors, never breaks the listing
        fs::write(dm_store::item_dir(&ws, "measure").unwrap().join("broken.json"), "{oops").unwrap();

        let all = list_items_in(&ws, None).unwrap();
        let items = all["items"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0]["kind"].as_str().unwrap(), "dimension"); // dimension < measure
        assert_eq!(items[1]["id"].as_str().unwrap(), "b_item");
        assert_eq!(items[2]["id"].as_str().unwrap(), "total_revenue");
        assert_eq!(all["errors"].as_array().unwrap().len(), 1);

        let measures = list_items_in(&ws, Some("measure")).unwrap();
        assert_eq!(measures["items"].as_array().unwrap().len(), 2);
        assert!(list_items_in(&ws, Some("bogus")).is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn file_has_no_timestamps_and_stable_key_order() {
        let ws = tmp_ws("git-safe");
        save_total(&ws, "measure");
        let path = dm_store::item_path(&ws, "measure", "total_revenue").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(!content.contains("createdAt") && !content.contains("updatedAt"));
        assert!(content.starts_with('{'));
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn delete_idempotent_across_folders() {
        let ws = tmp_ws("delete");
        save_total(&ws, "measure");
        delete_item_in(&ws, "total_revenue").unwrap();
        assert!(get_item_in(&ws, "total_revenue").is_err());
        delete_item_in(&ws, "total_revenue").unwrap(); // still Ok
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn guards_reject_bad_kind_and_bad_id() {
        let ws = tmp_ws("guards");
        assert!(save_item_in(&ws, "x", "fact", "t", "1", "x", None, None).is_err());
        assert!(save_item_in(&ws, "../evil", "measure", "t", "1", "x", None, None).is_err());
        let _ = fs::remove_dir_all(&ws);
    }
}
