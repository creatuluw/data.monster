use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::Manager;

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
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data dir: {}", e))?;

            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir)
                    .map_err(|e| format!("Failed to create app data dir: {}", e))?;
            }

            let workspace_file = app_data_dir.join("workspace.json");
            fs::write(
                &workspace_file,
                serde_json::to_string_pretty(&json!({ "path": path_str }))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| format!("Failed to save workspace config: {}", e))?;

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
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }

    let workspace_file = app_data_dir.join("workspace.json");
    fs::write(
        &workspace_file,
        serde_json::to_string_pretty(&json!({ "path": path }))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("Failed to save workspace config: {}", e))?;

    Ok(())
}
