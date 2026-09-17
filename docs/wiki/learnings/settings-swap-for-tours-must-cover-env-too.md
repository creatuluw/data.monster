---
type: Learning
title: Settings-swap for tours must cover .env too, and the app webview must never navigate off-origin
description: Discovered 2026-09-16 building the settings-tour + analyst-tour (docs/tours/RUNBOOK.md CRITICAL section).
tags: [tours, settings, api-key, cdp, playwright, security]
timestamp: "2026-09-16T14:40:01.120Z"
---

# Settings-swap for tours must cover .env too, and the app webview must never navigate off-origin

Discovered 2026-09-16 building the settings-tour + analyst-tour (docs/tours/RUNBOOK.md CRITICAL section).

**The runbook's settings swap alone leaks the real key.** The app's `get_settings` (src-tauri/src/commands/settings.rs) merges `LLM_API_KEY`/`LLM_API_URL`/`LLM_MODEL` from the real env or a **`.env` in cwd (E:/data.monster) over settings.json** — .env wins. The real key lived in `.env` (LLM_API_KEY=830526a2...), not in settings.json. Swap procedure that is actually safe:

1. `cp .env /tmp/env.bak` and `cp $APPDATA/com.data-monster.app/settings.json /tmp/settings.bak`
2. Write dummies to BOTH — and in the app's **flat key shape** (`llmApiKey`/`llmApiUrl`/`llmModel`), NOT the runbook's nested `{llm:{provider,apiKey,model}}` shape which the app does not read.
3. After capture restore BOTH from backups and diff-verify byte-identical; confirm live via `invoke('get_settings')`.

**Recovery trap that kills the app page:** navigating the webview page to `about:blank` over CDP permanently drops the Tauri init script — after navigating back, `window.__TAURI_INTERNALS__` is undefined and the app shows its "requires the Tauri desktop runtime" fallback. Only fix: relaunch `npm run dev` (with `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222`) per the runbook's dead-CDP recovery. Never navigate the app webview off-origin.

**Playwright 1.63 chokes on the webview's shared_worker target** (`assert(targetInfo.browserContextId)` — shared workers have none). `chromatrix.connectOverCDP` dies with an uncaught assert. Fix without touching the shared capture machinery: one guard at the top of the DRAAIBOEK-adjacent header:

```js
process.on('uncaughtException', (e) => { if (!/shared_worker/.test(String(e?.message ?? e))) { console.error(e); process.exit(1); } });
```

The worker stays unattached (harmless — captures only drive the page target).
