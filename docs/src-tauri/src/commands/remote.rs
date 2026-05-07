use tauri::Emitter;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteFileProgress {
    pub status: String,
    pub progress_percent: f32,
    pub downloaded_mb: f32,
    pub total_mb: f32,
    pub message: String,
}

/// Fetch remote file from URL and save to folder
#[tauri::command]
pub async fn fetch_remote_file(app: tauri::AppHandle, url: String, folder_name: String) -> Result<String, String> {
    use std::fs;
    use std::io::Write;
    use futures_util::StreamExt;
    use crate::commands::files::get_data_dir;
    
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    
    if folder_name.is_empty() {
        return Err("Folder name cannot be empty".to_string());
    }
    
    if folder_name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err("Folder name contains invalid characters".to_string());
    }
    
    let data_dir = get_data_dir(&app)?;
    let remote_dir = data_dir.join("remote");
    let target_folder = remote_dir.join(&folder_name);
    
    fs::create_dir_all(&target_folder)
        .map_err(|e| format!("Failed to create folder: {}", e))?;
    
    let filename = url.split('/').last()
        .and_then(|s| s.split('?').next())
        .unwrap_or("downloaded_file")
        .to_string();
    
    let target_path = target_folder.join(&filename);
    
    // Emit starting event
    let _ = app.emit("remote-file-progress", RemoteFileProgress {
        status: "connecting".to_string(),
        progress_percent: 0.0,
        downloaded_mb: 0.0,
        total_mb: 0.0,
        message: "Connecting to server...".to_string(),
    });
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    let total_mb = total_size as f32 / 1_048_576.0;
    
    // Emit download started event
    let _ = app.emit("remote-file-progress", RemoteFileProgress {
        status: "downloading".to_string(),
        progress_percent: 0.0,
        downloaded_mb: 0.0,
        total_mb,
        message: format!("Downloading {} ({:.1} MB)...", filename, total_mb),
    });
    
    let mut file = std::fs::File::create(&target_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let mut last_progress_percent = 0;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Error while downloading: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        // Only emit progress every 5% to avoid flooding
        if total_size > 0 {
            let progress_percent = ((downloaded as f32 / total_size as f32) * 100.0) as i32;
            if progress_percent >= last_progress_percent + 5 || downloaded == total_size {
                last_progress_percent = progress_percent;
                let downloaded_mb = downloaded as f32 / 1_048_576.0;
                
                let _ = app.emit("remote-file-progress", RemoteFileProgress {
                    status: "downloading".to_string(),
                    progress_percent: progress_percent as f32,
                    downloaded_mb,
                    total_mb,
                    message: format!("Downloading... {:.1} / {:.1} MB ({:.0}%)", downloaded_mb, total_mb, progress_percent),
                });
            }
        }
    }
    
    file.flush().map_err(|e| format!("Failed to flush file: {}", e))?;
    
    // Emit completion event
    let _ = app.emit("remote-file-progress", RemoteFileProgress {
        status: "complete".to_string(),
        progress_percent: 100.0,
        downloaded_mb: total_mb,
        total_mb,
        message: format!("Download complete! ({:.1} MB)", total_mb),
    });
    
    Ok(target_path.to_str()
        .ok_or("Invalid target path")?
        .to_string())
}

