use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

use crate::state::DuckDbState;
use tauri::State;

/// Settings are workspace-scoped (portable with the workspace's data and
/// content): the file lives inside the workspace folder when a workspace is
/// open. Falls back to the global app-data settings before a workspace exists
/// and when the workspace has no settings yet (pre-migration read).
fn settings_path(app: &tauri::AppHandle, state: &State<'_, DuckDbState>) -> Result<PathBuf, String> {
    if let Some(ws) = state.workspace_path.lock().clone() {
        return Ok(PathBuf::from(ws).join("settings.json"));
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }

    Ok(app_data_dir.join("settings.json"))
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle, state: State<'_, DuckDbState>) -> Result<Value, String> {
    let path = settings_path(&app, &state)?;

    // Workspace has no settings file yet → fall back to the global one so
    // pre-workspace-settings (LLM keys etc.) carry over until first save.
    let read_path = if path.exists() {
        path
    } else {
        app.path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?
            .join("settings.json")
    };

    let mut settings = if read_path.exists() {
        let content = fs::read_to_string(&read_path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))?
    } else {
        serde_json::json!({})
    };

    apply_env_overrides(&mut settings);
    Ok(settings)
}

// LLM_API_KEY / LLM_API_URL / LLM_MODEL from the real env or a .env file (cwd, then parent)
// override settings.json — .env is the source of truth in dev.
fn apply_env_overrides(settings: &mut Value) {
    let obj = match settings.as_object_mut() {
        Some(o) => o,
        None => return,
    };

    for (env_name, setting_name) in
        [("LLM_API_KEY", "llmApiKey"), ("LLM_API_URL", "llmApiUrl"), ("LLM_MODEL", "llmModel")]
    {
        if let Some(v) = env_value(env_name) {
            obj.insert(setting_name.to_string(), Value::String(v));
        }
    }

    // Accept both the base URL (.../v4) and the full chat-completions URL.
    if let Some(url) = obj.get("llmApiUrl").and_then(|v| v.as_str()).map(str::to_string) {
        if !url.contains("chat/completions") {
            let url = format!("{}/chat/completions", url.trim_end_matches('/'));
            obj.insert("llmApiUrl".to_string(), Value::String(url));
        }
    }
}

fn env_value(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.trim().is_empty() {
            return Some(v.trim().to_string());
        }
    }

    let mut dir = std::env::current_dir().ok()?;
    loop {
        let file = dir.join(".env");
        if let Ok(content) = fs::read_to_string(&file) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let Some((k, v)) = line.split_once('=') else { continue };
                if k.trim() == name {
                    let v = v.trim().trim_matches('"');
                    if !v.is_empty() {
                        return Some(v.to_string());
                    }
                }
            }
            return None; // found a .env, var not in it
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[tauri::command]
pub fn save_settings(
    app: tauri::AppHandle,
    state: State<'_, DuckDbState>,
    settings: Value,
) -> Result<(), String> {
    let path = settings_path(&app, &state)?;

    let content =
        serde_json::to_string_pretty(&settings).map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}
