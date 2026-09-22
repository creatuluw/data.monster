---
type: Preference
title: Agent may run the CDP restart chain (kill webviews + env flag + npm run dev) itself
description: Agent may run the CDP restart chain itself
tags: [cdp, tauri, dev-app, restart, verification]
timestamp: "2026-09-17T14:26:51.537Z"
---

# Agent may run the CDP restart chain (kill webviews + env flag + npm run dev) itself

# Agent may run the CDP restart chain itself

Amends [[never-start-npm-run-dev-tauri-dev]] for exactly one case: when CDP verification is needed and the app isn't running (or DuckDB/webview state is wedged), the agent MAY run this chained command itself, from the repo root:

```powershell
Get-Process msedgewebview2 -ErrorAction SilentlyContinue | Stop-Process -Force; $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9223"; npm run dev
```

- Granted by the user on 2026-09-17 after the CDP UI-pass sessions ("if for some reason you need it again you can run it yourself").
- Scope: this chain only. Starting the dev app any other way, or `npm run dev` without the flag, stays user-owned.
- Watch for: DuckDB writes failing with "resource deadlock would occur" after reload storms — that means the backend needs this full restart, not just a webview reload.
- Verify after launch: `curl http://localhost:9223/json/version` returns JSON.
