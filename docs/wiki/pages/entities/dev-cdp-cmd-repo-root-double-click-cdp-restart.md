---
type: Entity
title: dev-cdp.cmd (repo-root double-click CDP restart)
description: `dev-cdp.cmd` is a double-clickable Windows command script at the repo root that restarts the dev app in a CDP-drivable state — the packaged version of the manu
tags: [cdp, tooling, dev-experience, tauri]
timestamp: "2026-09-17T16:12:55.453Z"
---

# dev-cdp.cmd (repo-root double-click CDP restart)

`dev-cdp.cmd` is a double-clickable Windows command script at the repo root that restarts the dev app in a CDP-drivable state — the packaged version of the manual restart procedure in [preferences/cdp-verify-the-dev-app-via-webview2-additional](../../preferences/cdp-verify-the-dev-app-via-webview2-additional.md).

## What it does

One double-click instead of remembering the 3-step PowerShell chain:

1. Kills leftover `msedgewebview2` processes (the stale-process trap that silently drops the debug flag).
2. Sets `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`.
3. Launches the dev app (`npm run dev`) with the env var in scope.

## Details

- **Location**: `dev-cdp.cmd` (repo root)
- **Why it exists**: the env var must be set in the same shell that launches the app, and stale flag-less WebView2 processes silently ignore it — easy to get wrong by hand, so it's scripted.
- Related: [drive-data-monster-s-real-ui-over-cdp](../../learnings/drive-data-monster-s-real-ui-over-cdp.md), [webview2-cdp-gotchas-env-var-flag-stale](../../learnings/webview2-cdp-gotchas-env-var-flag-stale.md)

## Lifecycle

- First added: 2026-09-17, shipped in PR #12 alongside the skeleton pick/create modal flow.
