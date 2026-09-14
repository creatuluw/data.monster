---
type: Rule
title: Route external API calls through Rust commands, never webview fetch
description: Guideline
tags: [tauri, cors, http, webview]
timestamp: "2026-09-11T21:48:43.081Z"
---

# Route external API calls through Rust commands, never webview fetch

## Guideline

Never call external HTTP APIs directly from the Tauri webview with `fetch` — the webview is a browser context and CORS applies. Route the call through a Tauri command using `reqwest` on the Rust side, which has no CORS restrictions.

## When it applies

Any time the frontend needs to reach a third-party API (LLM providers, URLs ingested from Connect, webhooks, etc.).

## Rationale

The Analyst page's remote LLM chat was completely broken because `fetch` to `https://api.z.ai` failed CORS preflight. The fix (2026-09) moved the call into the `remote_chat` Rust command streaming back over `local-llm:*` events. Webview-origin network restrictions will bite every direct external fetch the same way.

## Note

Fetching user-supplied data URLs during Connect/ingest is already handled on the Rust side for the same reason.
