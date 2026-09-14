---
type: Learning
title: get_settings merges env/.env over settings.json — env is source of truth
description: Discovered while wiring `.env` into the app (2026-09-11).
tags: [settings, env, tauri, llm, configuration]
timestamp: "2026-09-11T22:03:44.298Z"
---

# get_settings merges env/.env over settings.json — env is source of truth

Discovered while wiring `.env` into the app (2026-09-11).

## Behavior

`get_settings` in `src-tauri/src/commands/settings.rs` reads `LLM_API_KEY`, `LLM_API_URL`, `LLM_MODEL` from the real environment first, then from a `.env` file, and **merges them over `settings.json`** — env wins. If the Settings UI "ignores" your saved values, check whether `.env` at the repo root defines them; env is the source of truth.

## Gotchas

- **Tauri's cwd is `src-tauri`**, not the repo root — the `.env` lookup searches upward from cwd to find the project-root `.env`.
- `LLM_API_URL` is accepted with or without the `/chat/completions` suffix (base URL is normalized automatically).
- No dotenvy crate and no reload logic — `get_settings` re-reads env/`.env` on every call, so a restart is the only "reload" needed.

## Related

- Settings UI and Analyst both consume `get_settings`, so no frontend change was required for env config.
- See [[decisions/proxy-remote-llm-calls-through-rust-not-webview-fetch]] for the related rule that all LLM calls go through Rust.
