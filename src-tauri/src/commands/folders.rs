use std::fs;
use std::path::Path;

#[tauri::command]
pub fn initialize_data_folders(
    workspace_path: String,
) -> Result<(), String> {
    let data_dir = Path::new(&workspace_path).join("data");
    let main_dir = data_dir.join("main");

    fs::create_dir_all(&main_dir)
        .map_err(|e| format!("Failed to create data directories: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn list_folders(
    workspace_path: String,
) -> Result<Vec<String>, String> {
    let data_dir = Path::new(&workspace_path).join("data");

    if !data_dir.exists() {
        return Ok(vec!["main".to_string()]);
    }

    let mut folders = vec!["main".to_string()];

    let entries = fs::read_dir(&data_dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name != "main" && !folders.contains(&name) {
                folders.push(name);
            }
        }
    }

    folders.sort();
    Ok(folders)
}

#[tauri::command]
pub fn create_folder(
    workspace_path: String,
    folder_name: String,
) -> Result<(), String> {
    let folder_path = Path::new(&workspace_path)
        .join("data")
        .join(&folder_name);

    if folder_path.exists() {
        return Err(format!("Folder '{}' already exists", folder_name));
    }

    fs::create_dir_all(&folder_path)
        .map_err(|e| format!("Failed to create folder: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_initialize_data_folders() {
        let dir = TempDir::new().unwrap();
        let ws = dir.path().to_str().unwrap().to_string();
        initialize_data_folders(ws.clone()).unwrap();
        assert!(Path::new(&ws).join("data").join("main").exists());
    }

    #[test]
    fn test_initialize_idempotent() {
        let dir = TempDir::new().unwrap();
        let ws = dir.path().to_str().unwrap().to_string();
        initialize_data_folders(ws.clone()).unwrap();
        initialize_data_folders(ws.clone()).unwrap();
        assert!(Path::new(&ws).join("data").join("main").exists());
    }

    #[test]
    fn test_list_folders_default() {
        let dir = TempDir::new().unwrap();
        let ws = dir.path().to_str().unwrap().to_string();
        let folders = list_folders(ws).unwrap();
        assert_eq!(folders, vec!["main"]);
    }

    #[test]
    fn test_create_and_list() {
        let dir = TempDir::new().unwrap();
        let ws = dir.path().to_str().unwrap().to_string();
        initialize_data_folders(ws.clone()).unwrap();

        create_folder(ws.clone(), "archive".to_string()).unwrap();
        create_folder(ws.clone(), "staging".to_string()).unwrap();

        let folders = list_folders(ws).unwrap();
        assert!(folders.contains(&"archive".to_string()));
        assert!(folders.contains(&"main".to_string()));
        assert!(folders.contains(&"staging".to_string()));
    }

    #[test]
    fn test_create_duplicate_fails() {
        let dir = TempDir::new().unwrap();
        let ws = dir.path().to_str().unwrap().to_string();
        initialize_data_folders(ws.clone()).unwrap();
        create_folder(ws.clone(), "test".to_string()).unwrap();
        let result = create_folder(ws, "test".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }
}
