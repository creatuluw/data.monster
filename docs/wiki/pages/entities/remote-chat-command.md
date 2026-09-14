---
type: Entity
title: remote_chat command
description: "Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events. "
tags: [tauri, llm, analyst, streaming, rust]
timestamp: "2026-09-11T21:48:43.083Z"
---

# remote_chat command

Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events. It is the ONLY sanctioned path for remote LLM calls — webview `fetch` to these APIs fails CORS (see decision "Proxy remote LLM calls through Rust").

## Details

- **Location**: `src-tauri/src/commands/local_llm.rs` (registered in `src-tauri/src/main.rs`)
- **Interface**: invoked from the frontend via `remoteChat()` in `src/lib/db-operations.ts`; consumes `streamViaEvents()` in `src/lib/stores/analyst.svelte.ts`
- **Behavior**: POSTs via `reqwest` in a background thread, parses the SSE stream, emits `local-llm:token` / `local-llm:done` / `local-llm:error` — the same event contract as local llama.cpp generation
- **Cancellation**: shares the `generate_cancel` flag with local generation, so the Analyst Stop button covers both modes

## Relationships

- Local llama.cpp generation path (same local_llm.rs module) — both emit the same `local-llm:token` / `local-llm:done` / `local-llm:error` event contract
- Decision `proxy-remote-llm-calls-through-rust-not-webview-fetch` in docs/wiki/decisions/ — why this command exists

## Lifecycle

- First added: 2026-09 — replaced the broken fetch-based `sendRemote` after CORS blocked direct webview calls (~100 lines net deleted by unifying both paths on `streamViaEvents()`).
