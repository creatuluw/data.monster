# Local LLM Integration for Data Monster

## Feature Overview

Data Monster is a Tauri v2 desktop application (Rust backend + SvelteKit frontend) for local data analysis with an embedded DuckDB engine. The AI Analyst feature currently requires a remote LLM API endpoint — the user must provide a URL, API key, and model name via Settings. This excludes users who lack an LLM provider or who prefer fully offline operation.

This feature adds **on-device LLM inference** using GGUF-quantized models loaded through llama.cpp, embedded directly into the Rust backend via the `llama-cpp-2` crate. Users who have no remote endpoint (or choose not to use one) can download a small model, load it locally, and use the AI Analyst identically to the remote flow. The system auto-detects available RAM and recommends models that fit the user's hardware.

## Success Criteria

1. A user with no API key can open the AI Analyst, download a recommended model (~1.9–5 GB), and receive streamed SQL-generating responses entirely on-device.
2. Token streaming via Tauri events produces the same real-time UX as the existing remote SSE flow.
3. Auto-extracted SQL from model responses executes against DuckDB identically to remote mode.
4. On a casual laptop with 8 GB RAM and AVX2 CPU, a 3B Q4_K_M model runs at ≥10 tokens/second with 4K context.
5. Users can switch between remote and local modes from Settings without losing conversation state.
6. Downloaded models are stored in the Tauri app data directory and persist across sessions.
7. The model is unloaded when not actively generating to free RAM for DuckDB operations.

## Design Goals

### Primary (Must)
- Embed llama.cpp into the Rust backend via `llama-cpp-2` crate
- Provide a curated model catalog with tier recommendations based on detected RAM
- Stream tokens from Rust to frontend via Tauri events
- Reuse the existing system prompt, SQL extraction, and plan-parsing logic unchanged
- Support download-with-progress, load, unload, and delete lifecycle for models
- Store inference mode preference (`remote` | `local`) in the existing `settings.json`

### Secondary (Nice to Have)
- GPU acceleration via CUDA for users with dedicated GPUs
- Model download resume after interruption
- Automatic model recommendation based on detected CPU features (AVX2 vs AVX-512)
- Multiple concurrent model slots (swap between models without re-downloading)

## User Experience

When the user opens the AI Analyst for the first time and has no API key configured, they see a setup prompt: "Choose how to power your AI Analyst" with two options — **Connect to API** (existing remote flow) or **Run Locally** (new). Selecting "Run Locally" shows a model catalog listing models with name, size, RAM requirement, and a tier badge (Fast / Balanced). The system auto-highlights models that fit the detected RAM. The user picks one, sees a download progress bar, and once complete the model auto-loads. From that point, the Analyst chat works identically to the remote mode — streaming tokens, auto-executing SQL, plan selection. In Settings, a new "Local AI" section lets users manage downloaded models, switch between local and remote mode, and see disk usage.

## Design Rationale

**Why llama.cpp embedded rather than Ollama?** Ollama is an external service (~800 MB) requiring separate installation and daemon management. Embedding llama.cpp keeps Data Monster self-contained — no external dependencies, no service lifecycle, no port conflicts. The `llama-cpp-2` crate compiles llama.cpp as a static library linked into the Tauri binary. Inference performance is identical (same underlying GGUF kernels).

**Why GGUF over ONNX/WebGPU?** GGUF is the standard quantized format supported by llama.cpp with highly optimized CPU kernels (AVX2, AVX-512, ARM NEON). ONNX via Transformers.js requires WebGPU which is experimental in Tauri's WebView2 and inconsistent across platforms. GGUF + llama.cpp provides proven cross-platform CPU inference.

**Why offer multiple models instead of one default?** Hardware varies widely. An 8 GB laptop cannot run a 7B model comfortably, while a 32 GB workstation can handle it easily. Offering a curated catalog with tier recommendations lets users make informed tradeoffs between quality and performance.

## Constraints and Assumptions

- **Windows-only for now** — Data Monster ships Windows NSIS/MSI installers. CMake must be available at build time for llama.cpp compilation.
- **CPU inference first** — GPU (CUDA) support is a future enhancement. The initial implementation targets AVX2 CPU inference.
- **Text-only models** — Data Monster's AI Analyst is text-to-SQL. Multimodal models (Gemma 4 with vision/audio) work but their vision/audio encoders are unused overhead. Text-only code models (Qwen 2.5 Coder, Phi-4 Mini) are preferred for efficiency.
- **No fine-tuning** — Models are used as-is with the existing system prompt. No local fine-tuning or LoRA adapters.
- **Model files are large** — Downloads range from ~1 GB to ~5 GB. The UI must show progress and allow cancellation.
- **RAM is shared with DuckDB** — DuckDB uses memory for query execution. The LLM model must be unloadable when not generating to avoid RAM contention.
- **Existing API unchanged** — The remote inference path (fetch + SSE) is not modified. Local inference is an additive parallel path.
- **`llama-cpp-2` crate** — This is the most active Rust binding (559+ stars, 146+ releases) maintained by utilityai. It wraps llama.cpp's C API via `llama-cpp-sys-2`.

## Functional Requirements

### FR-1: System Capability Detection
The app detects total system RAM and reports it to the frontend. This is used to recommend appropriate model tiers and warn when a model exceeds available resources.

**Acceptance Criteria:**
- `detect_system_ram()` Tauri command returns total physical RAM in bytes
- Frontend receives this value and uses it to filter/highlight model catalog entries
- On 8 GB systems, models requiring >6 GB RAM show a warning
- On 16 GB+ systems, all models are shown without warnings

### FR-2: Model Catalog
A hardcoded catalog of recommended GGUF models with metadata (name, params, quantization, download URL, RAM requirement, tier). The catalog is returned via a Tauri command.

**Acceptance Criteria:**
- `list_available_models()` returns a JSON array of model entries
- Each entry includes: id, name, description, sizeBytes, ramRequiredBytes, tier ("fast"|"balanced"), downloadUrl, filename
- Catalog includes at minimum: Qwen 2.5 Coder 3B, Qwen 2.5 3B Instruct, Gemma 4 E2B, Phi-4 Mini, Llama 3.2 3B, Qwen 2.5 Coder 7B, Gemma 4 E4B, Mistral 7B v0.3
- Fast tier models require ≤4 GB RAM; Balanced tier models require ≤7 GB RAM

### FR-3: Model Download with Progress
Download a GGUF model file from a HuggingFace URL to the app data directory with real-time progress reporting to the frontend.

**Acceptance Criteria:**
- `download_model(model_id)` downloads the GGUF file to `{app_data_dir}/models/`
- Progress is emitted as a Tauri event `local-llm:download-progress` with `{modelId, bytesDownloaded, bytesTotal, percent}`
- Download can be cancelled via `cancel_download(model_id)`
- If the file already exists and matches expected size, skip download
- On disk full or network error, return a descriptive error message
- Download directory is created if it doesn't exist

### FR-4: Model Load and Unload
Load a downloaded GGUF model into memory for inference. Unload to free RAM.

**Acceptance Criteria:**
- `load_model(model_id)` loads the GGUF file via llama.cpp and keeps it in memory
- Returns error if the file doesn't exist or is corrupt
- Returns error if insufficient RAM (estimated: model size × 1.3 for KV cache)
- `unload_model()` releases all model resources
- Only one model can be loaded at a time; loading a new one unloads the previous
- `is_model_loaded()` returns `{loaded: bool, modelId: string | null}`
- Model auto-unloads after 5 minutes of inactivity (configurable)

### FR-5: Streaming Chat Generation
Generate tokens from the loaded model in response to a chat messages array, streaming each token to the frontend via Tauri events.

**Acceptance Criteria:**
- `generate_chat(messages, system_prompt)` accepts the same message format as the existing remote API
- Applies the model's chat template (Gemma, ChatML, Llama, etc.) automatically via llama.cpp
- Streams tokens via Tauri event `local-llm:token` with `{token: string}`
- Emits `local-llm:done` when generation completes
- Emits `local-llm:error` on failure with `{error: string}`
- Supports `stop_generation()` to cancel mid-stream
- Uses sensible defaults: temperature 0.6, top_p 0.95, max_tokens 2048, context_size 4096
- Generation fails cleanly if no model is loaded (returns error, no panic)

### FR-6: Settings Integration
Inference mode (remote/local) and the selected local model ID are persisted in settings.json alongside existing LLM settings.

**Acceptance Criteria:**
- Settings JSON gains two new optional keys: `inferenceMode` ("remote"|"local") and `localModelId` (string|null)
- Settings UI shows a toggle between "Remote API" and "Local AI" modes
- When switching to local mode, the model catalog is shown
- When switching to remote mode, the existing URL/key/model fields are shown
- Settings are persisted and restored across app restarts
- Default mode is "remote" for existing users; "local" if no API key is configured

### FR-7: Analyst Store Integration
The existing `analyst.svelte.ts` routes generation requests to either the remote fetch path or the local Tauri invoke path based on the inference mode setting.

**Acceptance Criteria:**
- `AnalystState.sendMessage()` branches on `inferenceMode`
- Remote path: unchanged existing fetch + SSE logic
- Local path: calls `invoke('generate_chat', ...)` and listens for `local-llm:token` events
- Both paths produce identical side effects: streamingContent updates, SQL extraction, plan parsing
- `analyst.stop()` cancels local generation via `invoke('stop_generation')`
- Mode can be switched mid-session; existing messages are preserved

### FR-8: Model Manager UI
A new section in the Settings page for managing local models: view catalog, download, delete, load status.

**Acceptance Criteria:**
- Shows model catalog grouped by tier (Fast / Balanced)
- Each model card shows: name, size, RAM needed, download status, load status
- Download button with progress bar during download
- Delete button for downloaded models (with confirmation)
- "Loaded" indicator for the active model
- Disk usage summary at the bottom
- Warning badges for models that exceed detected RAM

## Edge Cases

1. **No network during download**: Show clear error, allow retry. Partial downloads are deleted.
2. **Disk full during download**: Detect and show "Not enough disk space" error before/during download.
3. **Corrupt GGUF file**: If model loading fails with a parse error, suggest re-downloading. Delete the corrupt file.
4. **RAM pressure**: If loading a model causes the OS to page heavily (DuckDB + LLM exceed physical RAM), the model may be extremely slow. The UI should warn about RAM usage.
5. **App shutdown during download**: Download is cancelled. Partial file is cleaned up on next app start.
6. **App shutdown during generation**: Generation stops. No data corruption (streaming content is ephemeral).
7. **Model file deleted externally**: `load_model` returns file-not-found error. UI shows "model missing, re-download".
8. **Rapid mode switching**: Switching from local→remote→local mid-conversation should not lose messages. The model stays loaded if already loaded.
9. **First-time user with no settings**: Default to showing the setup prompt (not a blank Analyst page).
10. **Multiple model downloads simultaneously**: Only one download at a time. Queue or reject concurrent downloads.
