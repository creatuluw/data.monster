use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

fn app_data(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }
    Ok(dir)
}

/// History of opened workspaces: { "workspaces": [{ "path", "lastOpened" }] }.
/// lastOpened = unix millis; the /workspaces page orders by it descending.
fn workspaces_file(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    Ok(app_data(app)?.join("workspaces.json"))
}

fn read_workspaces(app: &tauri::AppHandle) -> Vec<serde_json::Value> {
    let file = match workspaces_file(app) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    fs::read_to_string(&file)
        .ok()
        .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
        .and_then(|v| v["workspaces"].as_array().cloned())
        .unwrap_or_default()
}

fn record_workspace(app: &tauri::AppHandle, path: &str) -> Result<(), String> {
    let mut list = read_workspaces(app);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    if let Some(entry) = list.iter_mut().find(|e| e["path"].as_str() == Some(path)) {
        entry["lastOpened"] = json!(now);
    } else {
        list.push(json!({ "path": path, "lastOpened": now }));
    }

    fs::write(
        workspaces_file(app)?,
        serde_json::to_string_pretty(&json!({ "workspaces": list })).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("Failed to save workspace history: {}", e))
}

#[tauri::command]
pub fn list_workspaces(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let mut list: Vec<(String, u64)> = read_workspaces(&app)
        .into_iter()
        .filter_map(|e| {
            let path = e["path"].as_str()?.to_string();
            // drop entries whose folder vanished
            if !Path::new(&path).exists() {
                return None;
            }
            Some((path, e["lastOpened"].as_u64().unwrap_or(0)))
        })
        .collect();
    list.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(json!({
        "workspaces": list
            .into_iter()
            .map(|(path, last_opened)| json!({ "path": path, "lastOpened": last_opened }))
            .collect::<Vec<_>>()
    }))
}

#[tauri::command]
pub fn choose_workspace_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder_path = app
        .dialog()
        .file()
        .blocking_pick_folder();

    match folder_path {
        Some(path) => {
            let path_str = path.to_string();
            let workspace_file = app_data(&app)?.join("workspace.json");
            fs::write(
                &workspace_file,
                serde_json::to_string_pretty(&json!({ "path": path_str }))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| format!("Failed to save workspace config: {}", e))?;
            record_workspace(&app, &path_str)?;

            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub fn get_workspace_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let workspace_file = app_data_dir.join("workspace.json");

    if !workspace_file.exists() {
        return Ok(None);
    }

    let content =
        fs::read_to_string(&workspace_file).map_err(|e| format!("Failed to read workspace config: {}", e))?;

    let config: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse workspace config: {}", e))?;

    let path = config["path"].as_str().map(String::from);

    if let Some(ref p) = path {
        if !Path::new(p).exists() {
            return Ok(None);
        }
    }

    Ok(path)
}

#[tauri::command]
pub fn set_workspace_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let workspace_file = app_data(&app)?.join("workspace.json");
    fs::write(
        &workspace_file,
        serde_json::to_string_pretty(&json!({ "path": path }))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("Failed to save workspace config: {}", e))?;
    record_workspace(&app, &path)
}
