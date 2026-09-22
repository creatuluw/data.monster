---
type: Learning
title: Hard-reload storms deadlock DuckDB in-process — writes fail with "resource deadlock would occur" until full restart
description: Discovered 2026-09-17 while CDP-testing the master-items create flow in the page editor.
tags: [cdp, e2e, duckdb, tauri, debugging]
timestamp: "2026-09-17T14:10:03.156Z"
---

# Hard-reload storms deadlock DuckDB in-process — writes fail with "resource deadlock would occur" until full restart

Discovered 2026-09-17 while CDP-testing the master-items create flow in the page editor.

## Symptom

After a session of repeated hard reloads (the reload storm during earlier e2e attempts), **every backend write fails** with DuckDB `resource deadlock would occur`. The UI keeps working — forms fill, probes pass, no visible error — but saves silently no-op. Probing the frontend forever finds nothing because the frontend is fine; the Rust-side connection is wedged.

## Root cause

Each hard webview reload races `shutdownDuckdb` against in-flight commands in the same instance. Once the race is lost, the DuckDB connection is deadlocked in-process and never recovers — distinct from [[cdp-repro-traps-duckdb-lock]], where a *second instance* is pinned by the workspace file lock.

## Fix / prevention

- **Fix**: full app restart (close app → `Get-Process msedgewebview2 | Stop-Process -Force` → relaunch with the CDP env var per [[cdp-verify-the-dev-app-via-webview2-additional]]). No amount of UI probing fixes a wedged connection.
- **Prevention**: run an e2e pass as **exactly one clean pass, zero hard reloads**. If a pass goes sideways mid-flight, stop, restart, and rerun the whole pass — don't reload-and-continue.

Related: [[couldn-t-find-callback-id-tauri-warning]] (benign reload artifact — but reloads are not free), [[stale-vite-module-graph-can-survive-reloads-only-a]] (the other reason restarts beat reloads).

## Update 2026-09-21 — second root cause found and fixed in the backend

The same "resource deadlock would occur" error has a **second, distinct cause**: `execute_query` used to run synchronously on the main/UI thread, so a long query blocked the thread while webview IPC re-entered it — Windows failed the call with EDEADLK-class errors. Overlapping frontend invokes (e.g. [expreditor-component](../pages/entities/expreditor-component.md) firing preview queries while typing) trigger exactly this.

**Fix shipped** (`src-tauri/src/commands/queries.rs` + `state.rs`): `execute_query` is now an `async` command that clones the connection (`DuckDbState.conn` is now `Arc<Mutex<Option<Connection>>>`) into `spawn_blocking` — query work no longer touches the main/UI thread. Frontend-side, ExprEditor additionally serializes its preview queries so two invokes can't overlap.
