use encoding_rs::UTF_8;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaChatMessage, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager};

pub(crate) struct LoadedModel {
    model: LlamaModel,
    id: String,
}

pub struct LlmState {
    pub backend: LlamaBackend,
    pub loaded_model: Mutex<Option<LoadedModel>>,
    pub download_cancel: Mutex<Option<Arc<AtomicBool>>>,
    pub generate_cancel: Arc<AtomicBool>,
}

unsafe impl Send for LlmState {}
unsafe impl Sync for LlmState {}

impl LlmState {
    pub fn new() -> Result<Self, String> {
        let backend =
            LlamaBackend::init().map_err(|e| format!("Failed to init llama backend: {}", e))?;
        Ok(Self {
            backend,
            loaded_model: Mutex::new(None),
            download_cancel: Mutex::new(None),
            generate_cancel: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub params: String,
    pub size_bytes: u64,
    pub ram_required_bytes: u64,
    pub tier: String,
    pub download_url: String,
    pub filename: String,
}

fn model_catalog() -> Vec<ModelEntry> {
    vec![
        ModelEntry {
            id: "qwen25-coder-3b-q4km".into(),
            name: "Qwen 2.5 Coder 3B".into(),
            description: "Best code/SQL generation at small size. Text-only, efficient.".into(),
            params: "3B".into(),
            size_bytes: 1_900_000_000,
            ram_required_bytes: 3_000_000_000,
            tier: "fast".into(),
            download_url: "https://huggingface.co/bartowski/Qwen2.5-Coder-3B-Instruct-GGUF/resolve/main/Qwen2.5-Coder-3B-Instruct-Q4_K_M.gguf".into(),
            filename: "Qwen2.5-Coder-3B-Instruct-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "qwen25-3b-q4km".into(),
            name: "Qwen 2.5 3B Instruct".into(),
            description: "Strong structured data understanding, 29 languages.".into(),
            params: "3B".into(),
            size_bytes: 1_900_000_000,
            ram_required_bytes: 3_000_000_000,
            tier: "fast".into(),
            download_url: "https://huggingface.co/bartowski/Qwen2.5-3B-Instruct-GGUF/resolve/main/Qwen2.5-3B-Instruct-Q4_K_M.gguf".into(),
            filename: "Qwen2.5-3B-Instruct-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "phi4-mini-q4km".into(),
            name: "Phi-4 Mini".into(),
            description: "Excellent reasoning for SQL, function calling support.".into(),
            params: "3.8B".into(),
            size_bytes: 2_500_000_000,
            ram_required_bytes: 3_500_000_000,
            tier: "fast".into(),
            download_url: "https://huggingface.co/bartowski/Phi-4-mini-instruct-GGUF/resolve/main/Phi-4-mini-instruct-Q4_K_M.gguf".into(),
            filename: "Phi-4-mini-instruct-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "gemma4-e2b-q4km".into(),
            name: "Gemma 4 E2B".into(),
            description: "Google's efficient on-device model. Multimodal (text-only used).".into(),
            params: "2.3B eff".into(),
            size_bytes: 3_100_000_000,
            ram_required_bytes: 4_000_000_000,
            tier: "fast".into(),
            download_url: "https://huggingface.co/bartowski/gemma-4-E2B-it-GGUF/resolve/main/gemma-4-E2B-it-Q4_K_M.gguf".into(),
            filename: "gemma-4-E2B-it-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "llama32-3b-q4km".into(),
            name: "Llama 3.2 3B".into(),
            description: "Good instruction following, tool use capabilities.".into(),
            params: "3B".into(),
            size_bytes: 2_000_000_000,
            ram_required_bytes: 3_000_000_000,
            tier: "fast".into(),
            download_url: "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf".into(),
            filename: "Llama-3.2-3B-Instruct-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "qwen25-coder-7b-q4km".into(),
            name: "Qwen 2.5 Coder 7B".into(),
            description: "Excellent code/SQL generation. Best quality at this size.".into(),
            params: "7B".into(),
            size_bytes: 4_700_000_000,
            ram_required_bytes: 6_000_000_000,
            tier: "balanced".into(),
            download_url: "https://huggingface.co/bartowski/Qwen2.5-Coder-7B-Instruct-GGUF/resolve/main/Qwen2.5-Coder-7B-Instruct-Q4_K_M.gguf".into(),
            filename: "Qwen2.5-Coder-7B-Instruct-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "gemma4-e4b-q4km".into(),
            name: "Gemma 4 E4B".into(),
            description: "Google's efficient on-device model. Strong coding benchmarks.".into(),
            params: "4.5B eff".into(),
            size_bytes: 5_000_000_000,
            ram_required_bytes: 6_500_000_000,
            tier: "balanced".into(),
            download_url: "https://huggingface.co/bartowski/gemma-4-E4B-it-GGUF/resolve/main/gemma-4-E4B-it-Q4_K_M.gguf".into(),
            filename: "gemma-4-E4B-it-Q4_K_M.gguf".into(),
        },
        ModelEntry {
            id: "mistral-7b-q4km".into(),
            name: "Mistral 7B v0.3".into(),
            description: "Strong general-purpose model. Good all-round performance.".into(),
            params: "7B".into(),
            size_bytes: 4_400_000_000,
            ram_required_bytes: 6_000_000_000,
            tier: "balanced".into(),
            download_url: "https://huggingface.co/bartowski/Mistral-7B-Instruct-v0.3-GGUF/resolve/main/Mistral-7B-Instruct-v0.3-Q4_K_M.gguf".into(),
            filename: "Mistral-7B-Instruct-v0.3-Q4_K_M.gguf".into(),
        },
    ]
}

fn models_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("models");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create models dir: {}", e))?;
    }
    Ok(dir)
}

fn find_catalog_entry(model_id: &str) -> Option<ModelEntry> {
    model_catalog().into_iter().find(|m| m.id == model_id)
}

#[tauri::command]
pub fn detect_system_ram() -> Result<u64, String> {
    Ok(sysinfo::System::new_all().total_memory())
}

#[tauri::command]
pub fn list_available_models() -> Vec<ModelEntry> {
    model_catalog()
}

#[derive(Debug, Serialize, Clone)]
pub struct DownloadProgress {
    pub model_id: String,
    pub bytes_downloaded: u64,
    pub bytes_total: u64,
    pub percent: f64,
}

#[tauri::command]
pub fn download_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, LlmState>,
    model_id: String,
) -> Result<(), String> {
    let entry = find_catalog_entry(&model_id).ok_or("Model not found in catalog")?;
    let dir = models_dir(&app)?;
    let dest = dir.join(&entry.filename);

    if dest.exists() {
        if let Ok(meta) = fs::metadata(&dest) {
            if meta.len() > 0 {
                return Ok(());
            }
        }
    }

    let cancel_flag = Arc::new(AtomicBool::new(false));
    {
        let mut guard = state.download_cancel.lock();
        if guard.is_some() {
            return Err("A download is already in progress".into());
        }
        *guard = Some(cancel_flag.clone());
    }

    let result = download_model_inner(&app, &entry, &dest, &cancel_flag);

    {
        let mut guard = state.download_cancel.lock();
        *guard = None;
    }

    result
}

fn download_model_inner(
    app: &tauri::AppHandle,
    entry: &ModelEntry,
    dest: &PathBuf,
    cancel_flag: &Arc<AtomicBool>,
) -> Result<(), String> {
    let response = reqwest::blocking::Client::new()
        .get(&entry.download_url)
        .send()
        .map_err(|e| {
            let _ = fs::remove_file(dest);
            format!("Download request failed: {}", e)
        })?;

    let total = response.content_length().unwrap_or(entry.size_bytes);
    let mut downloaded: u64 = 0;

    let mut file = std::io::BufWriter::new(
        fs::File::create(dest).map_err(|e| {
            let _ = fs::remove_file(dest);
            format!("Failed to create file: {}", e)
        })?,
    );

    let mut reader = std::io::BufReader::new(response);
    let mut buffer = [0u8; 8192];

    loop {
        if cancel_flag.load(Ordering::Relaxed) {
            drop(file);
            let _ = fs::remove_file(dest);
            return Err("Download cancelled".into());
        }

        let bytes_read = reader.read(&mut buffer).map_err(|e| {
            let _ = fs::remove_file(dest);
            format!("Download read error: {}", e)
        })?;

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read]).map_err(|e| {
            let _ = fs::remove_file(dest);
            format!("Write error: {}", e)
        })?;

        downloaded += bytes_read as u64;
        let percent = if total > 0 {
            (downloaded as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let _ = app.emit(
            "local-llm:download-progress",
            DownloadProgress {
                model_id: entry.id.clone(),
                bytes_downloaded: downloaded,
                bytes_total: total,
                percent,
            },
        );
    }

    Ok(())
}

#[tauri::command]
pub fn cancel_download(state: tauri::State<'_, LlmState>) -> Result<(), String> {
    let guard = state.download_cancel.lock();
    if let Some(flag) = guard.as_ref() {
        flag.store(true, Ordering::Relaxed);
        Ok(())
    } else {
        Err("No download in progress".into())
    }
}

#[derive(Debug, Serialize)]
pub struct DownloadedModel {
    pub model_id: String,
    pub name: String,
    pub filename: String,
    pub size_bytes: u64,
}

#[tauri::command]
pub fn list_downloaded_models(app: tauri::AppHandle) -> Result<Vec<DownloadedModel>, String> {
    let dir = models_dir(&app)?;
    let catalog = model_catalog();
    let mut result = Vec::new();

    let entries = fs::read_dir(&dir).map_err(|e| format!("Failed to read models dir: {}", e))?;
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".gguf") {
                if let Some(cat_entry) = catalog.iter().find(|c| c.filename == name) {
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    result.push(DownloadedModel {
                        model_id: cat_entry.id.clone(),
                        name: cat_entry.name.clone(),
                        filename: cat_entry.filename.clone(),
                        size_bytes: size,
                    });
                }
            }
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn delete_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, LlmState>,
    model_id: String,
) -> Result<(), String> {
    let entry = find_catalog_entry(&model_id).ok_or("Model not found in catalog")?;
    let dir = models_dir(&app)?;
    let path = dir.join(&entry.filename);

    if !path.exists() {
        return Err("Model file not found on disk".into());
    }

    {
        let guard = state.loaded_model.lock();
        if let Some(ref loaded) = *guard {
            if loaded.id == model_id {
                drop(guard);
                let mut guard = state.loaded_model.lock();
                *guard = None;
            }
        }
    }

    fs::remove_file(&path).map_err(|e| format!("Failed to delete model: {}", e))
}

#[tauri::command]
pub fn load_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, LlmState>,
    model_id: String,
) -> Result<(), String> {
    let entry = find_catalog_entry(&model_id).ok_or("Model not found in catalog")?;
    let dir = models_dir(&app)?;
    let path = dir.join(&entry.filename);

    if !path.exists() {
        return Err("Model file not found. Download it first.".into());
    }

    {
        let guard = state.loaded_model.lock();
        if let Some(ref loaded) = *guard {
            if loaded.id == model_id {
                return Ok(());
            }
        }
    }

    {
        let mut guard = state.loaded_model.lock();
        *guard = None;
    }

    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&state.backend, &path, &model_params)
        .map_err(|e| format!("Failed to load model: {}", e))?;

    let mut guard = state.loaded_model.lock();
    *guard = Some(LoadedModel { model, id: model_id });

    Ok(())
}

#[tauri::command]
pub fn unload_model(state: tauri::State<'_, LlmState>) -> Result<(), String> {
    let mut guard = state.loaded_model.lock();
    *guard = None;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct ModelLoadStatus {
    pub loaded: bool,
    pub model_id: Option<String>,
}

#[tauri::command]
pub fn is_model_loaded(state: tauri::State<'_, LlmState>) -> ModelLoadStatus {
    let guard = state.loaded_model.lock();
    match guard.as_ref() {
        Some(m) => ModelLoadStatus {
            loaded: true,
            model_id: Some(m.id.clone()),
        },
        None => ModelLoadStatus {
            loaded: false,
            model_id: None,
        },
    }
}

#[tauri::command]
pub fn generate_chat(
    app: tauri::AppHandle,
    messages: Vec<serde_json::Value>,
    system_prompt: String,
) -> Result<(), String> {
    let state = app.state::<LlmState>();

    {
        let guard = state.loaded_model.lock();
        if guard.is_none() {
            return Err("No model loaded. Load a model first.".into());
        }
    }

    state.generate_cancel.store(false, Ordering::Relaxed);

    let cancel = state.generate_cancel.clone();

    std::thread::spawn(move || {
        let state = app.state::<LlmState>();
        let _ = generate_loop_inner(&app, &state, &messages, &system_prompt, &cancel);
    });

    Ok(())
}

fn generate_loop_inner(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, LlmState>,
    messages: &[serde_json::Value],
    system_prompt: &str,
    cancel: &Arc<AtomicBool>,
) -> Result<(), String> {
    let guard = state.loaded_model.lock();
    let loaded = guard
        .as_ref()
        .ok_or("Model unloaded during generation")?;

    let tmpl = loaded
        .model
        .chat_template(None)
        .map_err(|e| format!("Chat template error: {}", e))?;

    let mut chat_messages: Vec<LlamaChatMessage> = Vec::new();
    chat_messages.push(
        LlamaChatMessage::new("system".to_string(), system_prompt.to_string())
            .unwrap_or_else(|_| LlamaChatMessage::new("system".to_string(), String::new()).unwrap()),
    );

    for msg in messages {
        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("user");
        let content = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");
        if let Ok(m) = LlamaChatMessage::new(role.to_string(), content.to_string()) {
            chat_messages.push(m);
        }
    }

    let prompt_text = loaded
        .model
        .apply_chat_template(&tmpl, &chat_messages, true)
        .map_err(|e| format!("Template error: {}", e))?;

    let tokens = loaded
        .model
        .str_to_token(&prompt_text, AddBos::Always)
        .map_err(|e| format!("Tokenize error: {}", e))?;

    let ctx_params = LlamaContextParams::default()
        .with_n_ctx(Some(NonZeroU32::new(4096).unwrap()))
        .with_n_threads(
            std::thread::available_parallelism()
                .map(|n| n.get() as i32)
                .unwrap_or(4),
        )
        .with_n_threads_batch(
            std::thread::available_parallelism()
                .map(|n| n.get() as i32)
                .unwrap_or(4),
        );

    let mut ctx = loaded
        .model
        .new_context(&state.backend, ctx_params)
        .map_err(|e| format!("Context error: {}", e))?;

    let mut batch = LlamaBatch::new(512, 1);
    let last_idx = (tokens.len() - 1) as i32;
    for (i, &token) in tokens.iter().enumerate() {
        let is_last = i as i32 == last_idx;
        batch
            .add(token, i as i32, &[0], is_last)
            .map_err(|e| format!("Batch error: {}", e))?;
    }

    ctx.decode(&mut batch)
        .map_err(|e| format!("Decode error: {}", e))?;

    let mut n_cur = batch.n_tokens();
    let mut sampler = LlamaSampler::chain_simple([LlamaSampler::dist(1234), LlamaSampler::greedy()]);

    let max_tokens = 2048;
    let mut decoder = UTF_8.new_decoder();

    while n_cur < max_tokens {
        if cancel.load(Ordering::Relaxed) {
            break;
        }

        let token = sampler.sample(&ctx, batch.n_tokens() - 1);
        sampler.accept(token);

        if loaded.model.is_eog_token(token) {
            break;
        }

        match loaded.model.token_to_piece(token, &mut decoder, true, None) {
            Ok(text) => {
                let _ = app.emit("local-llm:token", serde_json::json!({"token": text}));
            }
            Err(_) => {}
        }

        batch.clear();
        if batch.add(token, n_cur, &[0], true).is_err() {
            break;
        }
        n_cur += 1;

        if ctx.decode(&mut batch).is_err() {
            break;
        }
    }

    let _ = app.emit("local-llm:done", serde_json::json!({}));
    Ok(())
}

#[tauri::command]
pub fn stop_generation(state: tauri::State<'_, LlmState>) -> Result<(), String> {
    state.generate_cancel.store(true, Ordering::Relaxed);
    Ok(())
}
