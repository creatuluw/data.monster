//! Relationships: file-backed storage as one doc, `dm/relationships.json`
//! (workspace-file-first FR-3). Save is read-modify-write, so a corrupt existing file
//! REFUSES to save (never clobber an agent's hand-edited doc — fix it first).
//! No timestamps in the file (git-safe). Invoke names/params unchanged.

use serde_json::json;
use std::fs;
use std::path::Path;
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

/// Load the graph doc; missing file = empty graph, invalid file = loud error.
fn load_in(ws: &Path) -> Result<Vec<serde_json::Value>, String> {
    let path = dm_store::relationships_path(ws);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(&path).map_err(|e| format!("unreadable: {e}"))?;
    dm_store::check_json_object(&bytes, &["relationships"])?;
    let v: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid JSON: {e}"))?;
    v["relationships"]
        .as_array()
        .cloned()
        .ok_or_else(|| "relationships must be an array".to_string())
}

pub(crate) fn store_in(ws: &Path, rels: &[serde_json::Value]) -> Result<(), String> {
    let doc = json!({ "relationships": rels });
    let path = dm_store::relationships_path(ws);
    dm_store::atomic_write(&path, serde_json::to_string_pretty(&doc).unwrap().as_bytes())
}

pub fn list_relationships_in(ws: &Path) -> Result<serde_json::Value, String> {
    Ok(json!({ "relationships": load_in(ws)? }))
}

/// Upsert by id; empty/missing id → generated slug `rel-<from>-<to>`.
pub fn save_relationship_in(
    ws: &Path,
    from_table: &str,
    from_column: &str,
    to_table: &str,
    to_column: &str,
    id: Option<&str>,
) -> Result<(), String> {
    let id = match id {
        Some(id) if !id.trim().is_empty() => id.trim().to_string(),
        _ => slugs::generate_slug(&format!("rel {from_table} {to_table}")),
    };

    let mut rels = load_in(ws)?; // corrupt file → Err, never clobber
    let entry = json!({
        "id": id,
        "fromTable": from_table,
        "fromColumn": from_column,
        "toTable": to_table,
        "toColumn": to_column,
    });
    match rels.iter_mut().find(|r| r["id"] == entry["id"]) {
        Some(slot) => *slot = entry,
        None => rels.push(entry),
    }
    store_in(ws, &rels)
}

/// Idempotent.
pub fn delete_relationship_in(ws: &Path, id: &str) -> Result<(), String> {
    let mut rels = load_in(ws)?;
    let before = rels.len();
    rels.retain(|r| r["id"] != json!(id));
    if rels.len() != before {
        store_in(ws, &rels)?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_relationships(app: AppHandle) -> Result<serde_json::Value, String> {
    list_relationships_in(&ws_dir(&app)?)
}

#[tauri::command]
pub fn save_relationship(
    app: AppHandle,
    from_table: String,
    from_column: String,
    to_table: String,
    to_column: String,
    id: Option<String>,
) -> Result<(), String> {
    save_relationship_in(
        &ws_dir(&app)?,
        &from_table,
        &from_column,
        &to_table,
        &to_column,
        id.as_deref(),
    )
}

#[tauri::command]
pub fn delete_relationship(app: AppHandle, id: String) -> Result<(), String> {
    delete_relationship_in(&ws_dir(&app)?, &id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rel-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn save_orders(ws: &Path, id: Option<&str>) {
        save_relationship_in(ws, "orders", "customer_id", "customers", "id", id).unwrap();
    }

    #[test]
    fn save_then_list_round_trips_and_generates_ids() {
        let ws = tmp_ws("roundtrip");
        save_orders(&ws, None);
        let out = list_relationships_in(&ws).unwrap();
        let rels = out["relationships"].as_array().unwrap();
        assert_eq!(rels.len(), 1);
        assert_eq!(rels[0]["id"].as_str().unwrap(), "rel-orders-customers");
        assert_eq!(rels[0]["fromTable"].as_str().unwrap(), "orders");
        assert_eq!(rels[0]["toColumn"].as_str().unwrap(), "id");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn upsert_by_id_replaces_not_duplicates() {
        let ws = tmp_ws("upsert");
        save_orders(&ws, Some("rel-orders-customers"));
        save_relationship_in(&ws, "orders", "cust_id", "customers", "id", Some("rel-orders-customers")).unwrap();
        let rels = list_relationships_in(&ws).unwrap()["relationships"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(rels.len(), 1);
        assert_eq!(rels[0]["fromColumn"].as_str().unwrap(), "cust_id");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn delete_removes_and_is_idempotent() {
        let ws = tmp_ws("delete");
        save_orders(&ws, None);
        delete_relationship_in(&ws, "rel-orders-customers").unwrap();
        assert_eq!(list_relationships_in(&ws).unwrap()["relationships"].as_array().unwrap().len(), 0);
        delete_relationship_in(&ws, "rel-orders-customers").unwrap(); // still Ok
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn missing_file_is_empty_graph() {
        let ws = tmp_ws("empty");
        assert_eq!(list_relationships_in(&ws).unwrap()["relationships"].as_array().unwrap().len(), 0);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn corrupt_doc_errors_loudly_and_save_refuses_to_clobber() {
        let ws = tmp_ws("corrupt");
        fs::create_dir_all(dm_store::dm_dir(&ws)).unwrap();
        fs::write(dm_store::relationships_path(&ws), "{oops").unwrap();
        assert!(list_relationships_in(&ws).is_err());
        assert!(
            save_relationship_in(&ws, "orders", "customer_id", "customers", "id", None).is_err(),
            "save must not clobber a hand-edited broken doc"
        );
        // the corrupt bytes are untouched after the refused save
        assert_eq!(fs::read_to_string(dm_store::relationships_path(&ws)).unwrap(), "{oops");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn file_has_no_timestamps() {
        let ws = tmp_ws("git-safe");
        save_orders(&ws, None);
        let content = fs::read_to_string(dm_store::relationships_path(&ws)).unwrap();
        assert!(!content.contains("createdAt"));
        let _ = fs::remove_dir_all(&ws);
    }
}
