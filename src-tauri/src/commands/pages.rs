//! Pages: file-backed storage under `dm/pages/<slug>.json` (workspace-file-first FR-2).
//!
//! The file IS the document: one PageDoc per file, slug = filename. `spec` is written
//! verbatim (even mid-edit invalid JSON — the watcher surfaces it); `title` lives inside
//! the doc, extracted best-effort for the list. Command signatures keep their invoke
//! names/params so the frontend (`central-api.ts`) is unchanged.

use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use tauri::AppHandle;

use crate::commands::dm_store;
use crate::commands::workspace;

/// Resolve the active workspace directory (commands only; cores take `ws` directly).
fn ws_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let ws = workspace::get_workspace_path(app.clone())?;
    ws.map(std::path::PathBuf::from)
        .ok_or_else(|| "No workspace open".to_string())
}

fn mtime_iso(path: &Path) -> Option<String> {
    let t: SystemTime = fs::metadata(path).ok()?.modified().ok()?;
    Some(chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
}

/// Best-effort `title` from a page file (never fails the listing).
fn title_from(bytes: &[u8]) -> String {
    serde_json::from_slice::<serde_json::Value>(bytes)
        .ok()
        .and_then(|v| v.get("title").and_then(|t| t.as_str()).map(String::from))
        .unwrap_or_default()
}

pub fn list_pages_in(ws: &Path) -> Result<serde_json::Value, String> {
    let dir = dm_store::pages_dir(ws);
    if !dir.exists() {
        return Ok(json!({ "pages": [], "errors": [] }));
    }

    let mut pages = Vec::new();
    let mut errors = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| format!("Failed to read {}: {e}", dir.display()))? {
        let path = match entry.map_err(|e| e.to_string()).and_then(|e| {
            e.file_type()
                .map(|t| t.is_file())
                .map_err(|e| e.to_string())
                .map(|is_file| (e.path(), is_file))
        }) {
            Ok((path, true)) => path,
            Ok((_, false)) => continue,
            Err(e) => {
                errors.push(json!({ "slug": null, "error": e }));
                continue;
            }
        };
        let Some(slug) = dm_store::page_slug_from_path(&path) else {
            continue; // non-.json or unreadable name — not ours
        };
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                errors.push(json!({ "slug": slug, "error": format!("unreadable: {e}") }));
                continue;
            }
        };
        if let Err(e) = dm_store::check_json_object(&bytes, &["rows"]) {
            errors.push(json!({ "slug": slug, "error": e }));
            continue;
        }
        pages.push(json!({
            "slug": slug,
            "title": title_from(&bytes),
            "createdAt": null,
            "updatedAt": mtime_iso(&path),
        }));
    }

    pages.sort_by(|a, b| {
        let key = |v: &serde_json::Value| {
            v.get("updatedAt")
                .and_then(|u| u.as_str())
                .unwrap_or("")
                .to_string()
        };
        key(b).cmp(&key(a)) // mtime desc — most recent first
    });

    Ok(json!({ "pages": pages, "errors": errors }))
}

pub fn get_page_in(ws: &Path, slug: &str) -> Result<serde_json::Value, String> {
    let path = dm_store::page_path(ws, slug)?;
    let bytes = fs::read(&path).map_err(|_| "Page not found".to_string())?;
    Ok(json!({
        "slug": slug,
        "title": title_from(&bytes),
        "spec": String::from_utf8_lossy(&bytes),
        "createdAt": null,
        "updatedAt": mtime_iso(&path),
    }))
}

pub fn save_page_in(ws: &Path, slug: &str, spec: &str) -> Result<(), String> {
    let path = dm_store::page_path(ws, slug)?;
    dm_store::atomic_write(&path, spec.as_bytes())
}

pub fn delete_page_in(ws: &Path, slug: &str) -> Result<(), String> {
    let path = dm_store::page_path(ws, slug)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete page: {e}"))?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_pages(app: AppHandle) -> Result<serde_json::Value, String> {
    list_pages_in(&ws_dir(&app)?)
}

#[tauri::command]
pub fn get_page(app: AppHandle, slug: String) -> Result<serde_json::Value, String> {
    get_page_in(&ws_dir(&app)?, &slug)
}

#[tauri::command]
pub fn save_page(app: AppHandle, slug: String, title: String, spec: String) -> Result<(), String> {
    // `title` kept for invoke compatibility; the doc carries its own title.
    let _ = title;
    save_page_in(&ws_dir(&app)?, &slug, &spec)
}

#[tauri::command]
pub fn delete_page(app: AppHandle, slug: String) -> Result<(), String> {
    delete_page_in(&ws_dir(&app)?, &slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("pages-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const DOC_A: &str = r#"{"slug":"a","title":"Page A","rows":[]}"#;

    #[test]
    fn save_then_get_round_trips_spec_verbatim() {
        let ws = tmp_ws("roundtrip");
        save_page_in(&ws, "a", DOC_A).unwrap();

        let got = get_page_in(&ws, "a").unwrap();
        assert_eq!(got["spec"].as_str().unwrap(), DOC_A);
        assert_eq!(got["title"].as_str().unwrap(), "Page A");
        assert_eq!(got["slug"].as_str().unwrap(), "a");
        assert!(got["updatedAt"].as_str().is_some()); // mtime, for later conflict detection
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn save_writes_exact_file_and_is_idempotent_overwrite() {
        let ws = tmp_ws("file-exact");
        save_page_in(&ws, "a", DOC_A).unwrap();
        let path = dm_store::page_path(&ws, "a").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), DOC_A);
        save_page_in(&ws, "a", "{\"title\":\"B\",\"rows\":[]}").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "{\"title\":\"B\",\"rows\":[]}");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn save_writes_invalid_spec_verbatim_file_mirrors_editor() {
        let ws = tmp_ws("invalid-spec");
        save_page_in(&ws, "broken", "{\"title\": \"x\", \"rows\": [").unwrap();
        let path = dm_store::page_path(&ws, "broken").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "{\"title\": \"x\", \"rows\": [");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn list_skips_invalid_files_into_errors_and_sorts_by_mtime() {
        let ws = tmp_ws("list");
        save_page_in(&ws, "a", DOC_A).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30)); // distinct mtimes
        save_page_in(&ws, "b", "{\"title\":\"B\",\"rows\":[]}").unwrap();
        // invalid file (hand-written, not via save — mirrors agent writing garbage)
        let bad = dm_store::page_path(&ws, "bad").unwrap();
        fs::write(&bad, "{oops").unwrap();
        // non-json ignored
        fs::write(dm_store::pages_dir(&ws).join("notes.md"), "hi").unwrap();

        let out = list_pages_in(&ws).unwrap();
        let pages = out["pages"].as_array().unwrap();
        assert_eq!(pages.len(), 2, "invalid file must not be listed");
        assert_eq!(pages[0]["slug"].as_str().unwrap(), "b", "most recent mtime first");
        assert_eq!(pages[0]["title"].as_str().unwrap(), "B");
        assert_eq!(pages[1]["slug"].as_str().unwrap(), "a");

        let errors = out["errors"].as_array().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0]["slug"].as_str().unwrap(), "bad");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn list_on_missing_dir_is_empty_not_error() {
        let ws = tmp_ws("empty");
        let out = list_pages_in(&ws).unwrap();
        assert_eq!(out["pages"].as_array().unwrap().len(), 0);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn get_missing_page_is_not_found() {
        let ws = tmp_ws("missing");
        assert_eq!(get_page_in(&ws, "nope").unwrap_err(), "Page not found");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn delete_removes_file_and_is_idempotent() {
        let ws = tmp_ws("delete");
        save_page_in(&ws, "a", DOC_A).unwrap();
        delete_page_in(&ws, "a").unwrap();
        assert!(!dm_store::page_path(&ws, "a").unwrap().exists());
        delete_page_in(&ws, "a").unwrap(); // second delete: still Ok
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn slug_guard_rejects_traversal() {
        let ws = tmp_ws("guard");
        assert!(save_page_in(&ws, "../evil", DOC_A).is_err());
        let _ = fs::remove_dir_all(&ws);
    }
}
