---
type: Preference
title: CDP-verify the dev app via WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS (exact restart procedure)
description: How to get the dev app CDP-drivable (exact procedure)
tags: [cdp, tauri, webview2, dev-app, verification, e2e]
timestamp: "2026-09-17T13:42:38.440Z"
---

# CDP-verify the dev app via WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS (exact restart procedure)

# How to get the dev app CDP-drivable (exact procedure)

CDP UI verification needs the webview's debug port. It is set via env var, read at webview spawn, and silently ignored if a stale flag-less WebView2 process already holds the user-data-dir.

## Procedure (give the user this, in one PowerShell session, in order)

```powershell
# 1. close the dev app window, then kill leftover webview processes
Get-Process msedgewebview2 | Stop-Process -Force

# 2. set the flag in the SAME shell that launches the app (env vars don't cross shells)
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9223"

# 3. launch the usual dev command in that same session (user-owned — agent never starts it)
```

## Gotchas

- Env var name is `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` (NOT `WEBVIEW2_ADDITIONAL_ARGS`, NOT a CLI arg).
- Symptom of the stale-process trap: app runs on the vite URL (6123) but no 922x port listens. 19+ msedgewebview2 processes is normal — kill them all, relaunch with the env var set.
- Verify before driving: `curl http://localhost:9223/json/version` must return JSON.
- Node ≥21 has native WebSocket — CDP scripts need zero deps.
- `localhost` may resolve IPv6 vs vite's IPv4 — if the app can't reach the dev URL, vite needs `--host` (dual-stack).

## Source

- docs/wiki/learnings/webview2-cdp-gotchas-env-var-flag-stale.md
- Session 2026-09-17: user hit the stale-process trap after setting the flag correctly.
