//! Saved PostgreSQL connections (workspace-file-first FR-5). The doc
//! `dm/connections.json` holds `{connections: [{name, urlEnv}]}` — NON-secret metadata
//! only; the connection-string URL (contains the password) lives in the workspace
//! `.env` under the referenced var (gitignored — rule:
//! secrets-never-live-in-workspace-content-files). Save is read-modify-write with the
//! clobber guard: a corrupt doc refuses saves (fix it first). Invoke surface is new.

use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

use crate::commands::dm_store;
use crate::commands::workspace;
use crate::utils::slugs;

fn ws_dir(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let ws = workspace::get_workspace_path(app.clone())?;
    ws.map(std::path::PathBuf::from)
        .ok_or_else(|| "No workspace open".to_string())
}

/// `DM_CONN_<SLUG>_URL` for a connection name.
pub fn env_var_for(name: &str) -> String {
    format!("DM_CONN_{}_URL", slugs::generate_slug(name).to_uppercase().replace('-', "_"))
}

fn load_in(ws: &Path) -> Result<Vec<serde_json::Value>, String> {
    let path = dm_store::connections_path(ws);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(&path).map_err(|e| format!("unreadable: {e}"))?;
    dm_store::check_json_object(&bytes, &["connections"])?;
    let v: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid JSON: {e}"))?;
    v["connections"]
        .as_array()
        .cloned()
        .ok_or_else(|| "connections must be an array".to_string())
}

fn store_in(ws: &Path, conns: &[serde_json::Value]) -> Result<(), String> {
    let doc = json!({ "connections": conns });
    dm_store::atomic_write(
        &dm_store::connections_path(ws),
        serde_json::to_string_pretty(&doc).unwrap().as_bytes(),
    )
}

/// Read `KEY=VALUE` from the workspace `.env` (gitignored), then process env.
fn ws_env_value(ws: &Path, key: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(ws.join(".env")) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if k.trim() == key {
                    let v = v.trim().trim_matches('"');
                    if !v.is_empty() {
                        return Some(v.to_string());
                    }
                }
            }
        }
        return None; // found the workspace .env; var not in it
    }
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}

/// Append or replace `KEY=VALUE` in the workspace `.env` (created if missing),
/// preserving every other line. Atomic write — never a partial `.env`.
fn ws_env_set(ws: &Path, key: &str, value: &str) -> Result<(), String> {
    let path = ws.join(".env");
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .map(|c| c.lines().map(String::from).collect())
        .unwrap_or_default();
    let new_line = format!("{key}={value}");
    match lines.iter().position(|l| l.trim_start().starts_with(&format!("{key}="))) {
        Some(i) => lines[i] = new_line,
        None => lines.push(new_line),
    }
    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    dm_store::atomic_write(&path, out.as_bytes())
}

pub fn list_connections_in(ws: &Path) -> Result<serde_json::Value, String> {
    Ok(json!({ "connections": load_in(ws)? }))
}

/// Upsert by name: writes the metadata entry + puts the URL into the workspace `.env`.
pub fn save_connection_in(ws: &Path, name: &str, url: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Connection name is required".to_string());
    }
    if url.trim().is_empty() {
        return Err("Connection URL is required".to_string());
    }
    let var = env_var_for(name);
    ws_env_set(ws, &var, url.trim())?;

    let mut conns = load_in(ws)?; // corrupt doc → Err, never clobber
    let entry = json!({ "name": name.trim(), "urlEnv": var });
    match conns.iter_mut().find(|c| c["name"] == entry["name"]) {
        Some(slot) => *slot = entry,
        None => conns.push(entry),
    }
    store_in(ws, &conns)
}

/// Idempotent. Removes the metadata entry; the `.env` var is left in place
/// (the .env is user-owned — we only append/replace our own vars, never delete lines).
pub fn delete_connection_in(ws: &Path, name: &str) -> Result<(), String> {
    let mut conns = load_in(ws)?;
    let before = conns.len();
    conns.retain(|c| c["name"] != json!(name));
    if conns.len() != before {
        store_in(ws, &conns)?;
    }
    Ok(())
}

/// Resolve a saved connection's URL from the workspace `.env`.
/// Missing var → error naming the var and the .env path.
pub fn resolve_connection_in(ws: &Path, name: &str) -> Result<String, String> {
    let conns = load_in(ws)?;
    let entry = conns
        .iter()
        .find(|c| c["name"] == json!(name))
        .ok_or_else(|| format!("Connection not found: {name}"))?;
    let var = entry["urlEnv"]
        .as_str()
        .ok_or_else(|| format!("Connection {name} has no urlEnv reference"))?;
    ws_env_value(ws, var).ok_or_else(|| {
        format!(
            "{var} is not set — add it to {} (gitignored)",
            ws.join(".env").display()
        )
    })
}

#[tauri::command]
pub fn list_connections(app: AppHandle) -> Result<serde_json::Value, String> {
    list_connections_in(&ws_dir(&app)?)
}

#[tauri::command]
pub fn save_connection(app: AppHandle, name: String, url: String) -> Result<(), String> {
    save_connection_in(&ws_dir(&app)?, &name, &url)
}

#[tauri::command]
pub fn delete_connection(app: AppHandle, name: String) -> Result<(), String> {
    delete_connection_in(&ws_dir(&app)?, &name)
}

#[tauri::command]
pub fn resolve_connection(app: AppHandle, name: String) -> Result<serde_json::Value, String> {
    Ok(json!({ "url": resolve_connection_in(&ws_dir(&app)?, &name)? }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("conn-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn env_var_name_is_uppercase_underscored() {
        assert_eq!(env_var_for("Prod DB"), "DM_CONN_PROD_DB_URL");
        assert_eq!(env_var_for("local"), "DM_CONN_LOCAL_URL");
    }

    #[test]
    fn save_writes_entry_and_env_then_round_trips() {
        let ws = tmp_ws("roundtrip");
        save_connection_in(&ws, "Prod DB", "postgresql://u:p@h:5432/db").unwrap();

        let conns = list_connections_in(&ws).unwrap()["connections"].as_array().unwrap().clone();
        assert_eq!(conns.len(), 1);
        assert_eq!(conns[0]["name"].as_str().unwrap(), "Prod DB");
        assert_eq!(conns[0]["urlEnv"].as_str().unwrap(), "DM_CONN_PROD_DB_URL");
        // the doc never contains the URL/password
        let doc = fs::read_to_string(dm_store::connections_path(&ws)).unwrap();
        assert!(!doc.contains("postgres"), "doc must be secret-free: {doc}");

        let url = resolve_connection_in(&ws, "Prod DB").unwrap();
        assert_eq!(url, "postgresql://u:p@h:5432/db");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn save_upserts_same_name_and_env_line_without_duplicates() {
        let ws = tmp_ws("upsert");
        save_connection_in(&ws, "Prod", "postgres://old").unwrap();
        save_connection_in(&ws, "Prod", "postgres://new").unwrap();

        let conns = list_connections_in(&ws).unwrap()["connections"].as_array().unwrap().clone();
        assert_eq!(conns.len(), 1);
        assert_eq!(resolve_connection_in(&ws, "Prod").unwrap(), "postgres://new");

        let env = fs::read_to_string(ws.join(".env")).unwrap();
        assert_eq!(env.matches("DM_CONN_PROD_URL=").count(), 1);
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn env_write_preserves_other_lines_and_comments() {
        let ws = tmp_ws("env-preserve");
        fs::write(ws.join(".env"), "# my secrets\nLLM_API_KEY=sk-123\n").unwrap();
        save_connection_in(&ws, "Prod", "postgres://u:p@h/db").unwrap();
        let env = fs::read_to_string(ws.join(".env")).unwrap();
        assert!(env.contains("# my secrets"));
        assert!(env.contains("LLM_API_KEY=sk-123"));
        assert!(env.contains("DM_CONN_PROD_URL=postgres://u:p@h/db"));
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn resolve_missing_var_names_var_and_path() {
        let ws = tmp_ws("missing-var");
        save_connection_in(&ws, "Prod", "postgres://x").unwrap();
        fs::remove_file(ws.join(".env")).unwrap();
        let err = resolve_connection_in(&ws, "Prod").unwrap_err();
        assert!(err.contains("DM_CONN_PROD_URL") && err.contains(".env"), "{err}");
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn delete_idempotent_and_validation_guards() {
        let ws = tmp_ws("delete");
        assert!(save_connection_in(&ws, "", "postgres://x").is_err());
        assert!(save_connection_in(&ws, "Prod", "  ").is_err());
        save_connection_in(&ws, "Prod", "postgres://x").unwrap();
        delete_connection_in(&ws, "Prod").unwrap();
        delete_connection_in(&ws, "Prod").unwrap(); // still Ok
        assert!(resolve_connection_in(&ws, "Prod").is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn corrupt_doc_errors_loudly_and_save_refuses_to_clobber() {
        let ws = tmp_ws("corrupt");
        fs::create_dir_all(dm_store::dm_dir(&ws)).unwrap();
        fs::write(dm_store::connections_path(&ws), "{oops").unwrap();
        assert!(list_connections_in(&ws).is_err());
        assert!(save_connection_in(&ws, "Prod", "postgres://x").is_err());
        assert_eq!(fs::read_to_string(dm_store::connections_path(&ws)).unwrap(), "{oops");
        let _ = fs::remove_dir_all(&ws);
    }
}
