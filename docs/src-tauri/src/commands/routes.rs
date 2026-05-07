use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use walkdir::WalkDir;

/// Get all routes from the src/routes directory
#[tauri::command]
pub fn get_all_routes(app: AppHandle) -> Result<Vec<String>, String> {
    let resource_path = app.path().resource_dir()
        .map_err(|e| format!("Failed to get resource dir: {}", e))?;
    
    // In development mode, use actual src/routes
    // In production, return empty (handle client-side)
    let routes_path = if cfg!(debug_assertions) {
        let mut path = resource_path.clone();
        path.pop();
        path.pop();
        path.push("src");
        path.push("routes");
        path
    } else {
        return Ok(vec![]);
    };
    
    if !routes_path.exists() {
        return Err(format!("Routes directory not found: {:?}", routes_path));
    }
    
    let mut routes = Vec::new();
    
    for entry in WalkDir::new(&routes_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.file_name() == Some(std::ffi::OsStr::new("+page.svelte")) {
            let relative = path.strip_prefix(&routes_path)
                .map_err(|e| format!("Failed to get relative path: {}", e))?;
            
            let route = if relative.parent().is_none() || relative.parent() == Some(PathBuf::from("").as_path()) {
                "/".to_string()
            } else {
                let route_path = relative.parent().unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                format!("/{}", route_path)
            };
            
            routes.push(route);
        }
    }
    
    routes.sort();
    
    Ok(routes)
}

