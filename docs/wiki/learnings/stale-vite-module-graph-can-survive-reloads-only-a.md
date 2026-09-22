---
type: Learning
title: Stale vite module graph can survive reloads — only a full app restart clears it
description: "Discovered 2026-09-17 while wiring the skeleton's "Add dimension" button in the page editor. Extends [[apparent-ui-bug-stale-hmr-webview]]: that learning's fix "
tags: [hmr, vite, webview, cdp, verification, dev-loop, gotcha]
timestamp: "2026-09-17T14:02:42.517Z"
---

# Stale vite module graph can survive reloads — only a full app restart clears it

Discovered 2026-09-17 while wiring the skeleton's "Add dimension" button in the page editor. Extends [[apparent-ui-bug-stale-hmr-webview]]: that learning's fix (Ctrl+R) is the *first* rung — this is the case where reload doesn't work.

**Symptom**: newly edited code provably on disk (and vite serving fresh transforms for some modules) but one component (PageGrid) keeps running old code — instrumented `console.log` in the new handler never fires. Hard reloads **and cache-busted reloads do not clear it**; only some modules (`+page.svelte`, `app.css`) hot-update. The webview is stuck on a stale module graph.

**Collateral damage**: the repeated reload attempts themselves degraded IPC — the webview entered a `vite connecting` + `AppState.shutdown` loop where every reload killed DuckDB IPC.

**Diagnosis traps that cost real time**:
- URL-probing vite transforms via curl with **unencoded `[` / `]`** returns curl error 3 and empty output — looks exactly like "stale transform served". Always URL-encode brackets before drawing conclusions.
- **Python/shell-written edits may not trip vite's watcher** (hit on a CRLF file): disk file correct, HMR never fired. A real edit-tool change provably fires HMR; if you must script the edit, use python with newline preservation to force the watcher.

**Fix**: stop probing and restart the app fully — close it, `Get-Process msedgewebview2 | Stop-Process -Force`, relaunch with the CDP env var (exact procedure: [[cdp-verify-the-dev-app-via-webview2-additional]]). And when verification is blocked by staleness you can't break, prefer restructuring the fix so it doesn't depend on cross-module wiring you can't verify live (this turn: dropped a host→PageGrid prop; the inspector now self-seeds on mount).

Related: [[labs-hang-vite-reload-loop]] (the other vite reload-loop failure mode), [[webview2-cdp-gotchas-env-var-flag-stale]].
