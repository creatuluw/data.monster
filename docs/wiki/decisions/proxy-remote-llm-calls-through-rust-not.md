---
type: Decision
title: Proxy remote LLM calls through Rust, not webview fetch
description: Context
tags: [tauri, llm, analyst, cors, streaming]
status: accepted
timestamp: "2026-09-11T21:48:43.079Z"
---

# Proxy remote LLM calls through Rust, not webview fetch

## Context

The Analyst page (`/analyst`) talked to remote LLM providers (e.g. z.ai) by calling `fetch` directly from the SvelteKit webview. Browsers enforce CORS, the provider endpoints send no `Access-Control-Allow-Origin`, so every remote request failed with `net::ERR_FAILED` — remote chat was completely broken. The local llama.cpp path already worked via Rust commands emitting `local-llm:*` events.

## Choice

Proxy ALL remote LLM calls through Rust:

- New `remote_chat` Tauri command in `src-tauri/src/commands/local_llm.rs` — POSTs via `reqwest` (no CORS in native code) in a background thread, parses the SSE stream, and emits the same `local-llm:token` / `local-llm:done` / `local-llm:error` events the local path uses.
- Frontend: deleted the fetch-based `sendRemote`; extracted one shared `streamViaEvents()` in `src/lib/stores/analyst.svelte.ts` used by both local and remote paths (~100 lines net deleted).
- Shares the existing `generate_cancel` flag, so the Stop button works for both modes.

## Alternatives considered

- Keep webview fetch and configure a CORS proxy / ask providers for CORS headers — external dependency, fragile, per-provider.
- Separate event channel for remote — rejected; one streaming code path for both local and remote is less code and uniform behavior (Stop, partial-message retention).

## Consequences

- Any future remote LLM provider must be added on the Rust side, never via webview fetch.
- One streaming contract (`local-llm:*` events) covers all LLM modes in the frontend.
- Rust changes require a dev-server restart to take effect.
