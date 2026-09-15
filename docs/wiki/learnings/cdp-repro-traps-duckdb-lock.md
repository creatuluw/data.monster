---
type: Learning
title: "CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface"
description: "Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hit while chasing the "
tags: [cdp, tauri, duckdb, debugging, repro]
timestamp: "2026-09-14T08:59:18.655Z"
---

# CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface

Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hit while chasing the 2026-09-14 Labs bar-chart hang:

1. **DuckDB workspace lock → second instance stuck at `/`.** `d8a_monster.duckdb` is single-writer; if an app instance is already running against the same workspace (e.g. the user's hung one), a freshly launched CDP instance fails `app.init` on the file lock and never leaves the workspace picker (`/`). A probe instance that "won't navigate away from `/`" almost always means **another instance holds the DB**, not that routing is broken. Kill existing instances (or point the probe at a different workspace) before reproducing.
2. **Plain-headless probes are contaminated.** Loading the app in a non-Tauri browser throws in the layout's Tauri `listen` wiring — that failure is noise, not the user's bug. To reproduce headless, mock the Tauri surface (what `app.init` invokes) first; otherwise every result is a false negative.

## Why it matters

Both traps burn hours by producing misleading symptoms (blocked navigation, phantom exceptions) that look like app bugs but are repro-harness artifacts. Check for a second instance and mock Tauri before trusting any probe result.
