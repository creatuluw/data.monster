use std::path::PathBuf;
use std::fs;
use chrono::Local;

/// Create a timestamped screenshots folder
#[tauri::command]
pub fn create_screenshots_folder(_app: tauri::AppHandle) -> Result<String, String> {
    use std::env;
    
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let project_root = if cfg!(debug_assertions) {
        current_dir.parent()
            .ok_or("Failed to get parent directory")?
            .to_path_buf()
    } else {
        current_dir
    };
    
    let screenshots_base = project_root.join("screenshots");
    
    if !screenshots_base.exists() {
        fs::create_dir_all(&screenshots_base)
            .map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }
    
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let screenshots_folder = screenshots_base.join(&timestamp);
    
    fs::create_dir_all(&screenshots_folder)
        .map_err(|e| format!("Failed to create timestamped folder: {}", e))?;
    
    Ok(screenshots_folder.to_str()
        .ok_or("Invalid path")?
        .to_string())
}

/// Save a screenshot file to the screenshots folder
#[tauri::command]
pub fn save_screenshot_file(source_path: String, dest_folder: String, filename: String) -> Result<String, String> {
    let source = PathBuf::from(&source_path);
    let dest = PathBuf::from(&dest_folder).join(&filename);
    
    if !source.exists() {
        return Err(format!("Source file not found: {}", source_path));
    }
    
    fs::copy(&source, &dest)
        .map_err(|e| format!("Failed to copy screenshot: {}", e))?;
    
    Ok(dest.to_str()
        .ok_or("Invalid destination path")?
        .to_string())
}

/// Open a folder in the system file explorer
#[tauri::command]
pub fn open_folder_in_explorer(folder_path: String) -> Result<(), String> {
    let path = PathBuf::from(&folder_path);
    
    if !path.exists() {
        return Err(format!("Folder not found: {}", folder_path));
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    Ok(())
}

