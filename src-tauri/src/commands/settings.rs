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

fn workspace_dir(state: &State<'_, DuckDbState>) -> Option<std::path::PathBuf> {
    state.workspace_path.lock().clone().map(std::path::PathBuf::from)
}

/// Secrets never live in committable files (rule: secrets-never-live-in-workspace-content-files):
/// with a workspace open, `llmApiKey` moves to the workspace `.env` (LLM_API_KEY) and is
/// stripped from settings.json. Without a workspace the global app-data file is not
/// version-controlled — legacy behavior stays (reads still prefer env).
fn apply_secret_policy(settings: &mut Value, ws: Option<&std::path::Path>) -> Result<(), String> {
    let Some(ws) = ws else { return Ok(()) };
    if let Some(key) = settings.get("llmApiKey").and_then(|v| v.as_str()) {
        if !key.trim().is_empty() {
            crate::commands::connections::ws_env_set(ws, "LLM_API_KEY", key.trim())?;
        }
    }
    if let Some(obj) = settings.as_object_mut() {
        obj.remove("llmApiKey");
    }
    Ok(())
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

    apply_env_overrides(&mut settings, workspace_dir(&state).as_deref());
    Ok(settings)
}

// LLM_API_KEY / LLM_API_URL / LLM_MODEL from the real env or a .env file (cwd, then parent)
// override settings.json — .env is the source of truth in dev.
fn apply_env_overrides(settings: &mut Value, ws: Option<&std::path::Path>) {
    let obj = match settings.as_object_mut() {
        Some(o) => o,
        None => return,
    };

    for (env_name, setting_name) in
        [("LLM_API_KEY", "llmApiKey"), ("LLM_API_URL", "llmApiUrl"), ("LLM_MODEL", "llmModel")]
    {
        // workspace .env (gitignored, the secrets home) wins, then process env, then the
        // legacy cwd-walk .env (dev).
        let v = ws
            .and_then(|ws| crate::commands::connections::ws_env_value(ws, env_name))
            .or_else(|| env_value(env_name));
        if let Some(v) = v {
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
    let mut settings = settings;
    apply_secret_policy(&mut settings, workspace_dir(&state).as_deref())?;

    let content =
        serde_json::to_string_pretty(&settings).map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_ws(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("settings-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn secret_policy_moves_key_to_workspace_env_and_strips_it() {
        let ws = tmp_ws("move");
        let mut settings = serde_json::json!({
            "llmApiKey": "sk-test-123",
            "llmModel": "glm-4",
            "theme": "dark"
        });
        apply_secret_policy(&mut settings, Some(&ws)).unwrap();

        // key landed in the gitignored .env
        let env = fs::read_to_string(ws.join(".env")).unwrap();
        assert!(env.contains("LLM_API_KEY=sk-test-123"), "{env}");
        // and is gone from the (committable) settings doc; the rest survived
        assert!(settings.get("llmApiKey").is_none());
        assert_eq!(settings["llmModel"].as_str().unwrap(), "glm-4");
        assert_eq!(settings["theme"].as_str().unwrap(), "dark");

        // env resolution now finds it via the workspace .env
        assert_eq!(
            crate::commands::connections::ws_env_value(&ws, "LLM_API_KEY").as_deref(),
            Some("sk-test-123")
        );
        let _ = fs::remove_dir_all(&ws);
    }

    #[test]
    fn secret_policy_noop_without_workspace_and_empty_key() {
        let ws = tmp_ws("noop");
        let mut settings = serde_json::json!({ "llmApiKey": "sk-keep" });
        apply_secret_policy(&mut settings, None).unwrap(); // global file: legacy behavior
        assert_eq!(settings["llmApiKey"].as_str().unwrap(), "sk-keep");

        let mut settings = serde_json::json!({ "llmApiKey": "" });
        apply_secret_policy(&mut settings, Some(&ws)).unwrap();
        assert!(settings.get("llmApiKey").is_none());
        assert!(!ws.join(".env").exists(), "empty key must not create .env");
        let _ = fs::remove_dir_all(&ws);
    }
}
