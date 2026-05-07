use tauri::AppHandle;
use std::path::PathBuf;
use std::fs;
use crate::commands::files::get_data_dir;

/// Initialize data folder structure
#[tauri::command]
pub fn initialize_data_folders(app: AppHandle) -> Result<String, String> {
    let data_dir = get_data_dir(&app)?;
    
    let main_folder = data_dir.join("main");
    if !main_folder.exists() {
        fs::create_dir_all(&main_folder)
            .map_err(|e| format!("Failed to create main folder: {}", e))?;
    }
    
    Ok(format!("Data folders initialized at: {}", data_dir.display()))
}

/// Recursively list all folders in the data directory
fn list_folders_recursive(base_path: &PathBuf, relative_path: &str) -> Vec<String> {
    let mut folders = Vec::new();
    let current_path = if relative_path.is_empty() {
        base_path.clone()
    } else {
        base_path.join(relative_path)
    };
    
    if let Ok(entries) = fs::read_dir(&current_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        let folder_path = if relative_path.is_empty() {
                            name.to_string()
                        } else {
                            format!("{}/{}", relative_path, name)
                        };
                        folders.push(folder_path.clone());
                        folders.extend(list_folders_recursive(base_path, &folder_path));
                    }
                }
            }
        }
    }
    
    folders
}

/// List all folders in the data directory
#[tauri::command]
pub fn list_folders(app: AppHandle) -> Result<Vec<String>, String> {
    let data_dir = get_data_dir(&app)?;
    let mut folders = list_folders_recursive(&data_dir, "");
    
    if !folders.contains(&"main".to_string()) {
        folders.insert(0, "main".to_string());
    }
    
    folders.sort();
    
    Ok(folders)
}

/// Create a new folder
#[tauri::command]
pub fn create_folder(app: AppHandle, folder_name: String) -> Result<String, String> {
    if folder_name.is_empty() {
        return Err("Folder name cannot be empty".to_string());
    }
    
    if folder_name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err("Folder name contains invalid characters".to_string());
    }
    
    if folder_name == "." || folder_name == ".." {
        return Err("Invalid folder name".to_string());
    }
    
    let data_dir = get_data_dir(&app)?;
    let new_folder = data_dir.join(&folder_name);
    
    if new_folder.exists() {
        return Err(format!("Folder '{}' already exists", folder_name));
    }
    
    fs::create_dir_all(&new_folder)
        .map_err(|e| format!("Failed to create folder: {}", e))?;
    
    Ok(format!("Folder '{}' created successfully", folder_name))
}

/// Copy file to a specific folder
#[tauri::command]
pub fn save_file_to_folder(app: AppHandle, source_path: String, folder_name: String) -> Result<String, String> {
    let data_dir = get_data_dir(&app)?;
    let target_folder = data_dir.join(&folder_name);
    
    if !target_folder.exists() {
        fs::create_dir_all(&target_folder)
            .map_err(|e| format!("Failed to create folder: {}", e))?;
    }
    
    let source = PathBuf::from(&source_path);
    let filename = source.file_name()
        .ok_or("Invalid source file path")?
        .to_str()
        .ok_or("Invalid filename")?;
    
    let target_path = target_folder.join(filename);
    
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;
    
    Ok(target_path.to_str()
        .ok_or("Invalid target path")?
        .to_string())
}

/// Get the full path to a file in a specific folder
#[tauri::command]
pub fn get_file_path_in_folder(app: AppHandle, folder_name: String, filename: String) -> Result<String, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join(&folder_name).join(&filename);
    
    Ok(file_path.to_str()
        .ok_or("Invalid file path")?
        .to_string())
}

/// List all files in a specific folder
#[tauri::command]
pub fn list_files_in_folder(app: AppHandle, folder_name: String) -> Result<Vec<String>, String> {
    let data_dir = get_data_dir(&app)?;
    let folder_path = data_dir.join(&folder_name);
    
    if !folder_path.exists() {
        return Ok(Vec::new());
    }
    
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&folder_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(name.to_string());
                    }
                }
            }
        }
    }
    
    files.sort();
    
    Ok(files)
}

