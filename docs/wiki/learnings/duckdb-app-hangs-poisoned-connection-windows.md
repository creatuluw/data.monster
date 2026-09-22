---
type: Learning
title: "DuckDB app hangs = poisoned connection on Windows (duckdb-rs #209); in-process recovery fix"
description: Diagnosed 2026-09-22 while investigating the data.monster app hangs (reports/pages-e2e-feedback.md).
tags: [duckdb, deadlock, windows, backend, bug, tauri]
timestamp: "2026-09-22T11:46:19.999Z"
---

# DuckDB app hangs = poisoned connection on Windows (duckdb-rs #209); in-process recovery fix

Diagnosed 2026-09-22 while investigating the data.monster app hangs (reports/pages-e2e-feedback.md).

**Root cause of the app wedges**: a poisoned DuckDB connection on Windows (matches [duckdb-rs issue #209](https://github.com/duckdb/duckdb-rs/issues/209) exactly — same error string, Windows label). After certain failed/interrupted statements, EVERY later operation on that Connection fails with `resource deadlock would occur` (Windows file-lock EDEADLK surfaced through DuckDB) until process restart. Trigger in-app: reload storms — webview unload invokes `shutdownDuckdb` while statements are in flight; also failed statements (binder errors) right before the wedge.

Static-analysis map that found it: `DuckDbState.conn: Arc<parking_lot::Mutex<Option<Connection>>>`; `run_query` holds the lock for the whole statement; `shutdown_duckdb` runs CHECKPOINT+close under the same lock; `open_with_retry` already handled WAL/lock errors at open; `spawn_blocking` was a prior fix for the same EDEADLK class (comment in queries.rs).

**Fix** (queries.rs `execute_query`): detect `resource deadlock` in the error → `shutdown_duckdb` + `initialize_duckdb` (workspace path from state, schema re-init) → retry the query once. In-process recovery replaces app restarts. Frontend unchanged (AppHandle is Tauri-injected, not an IPC arg).

Ceiling: only `execute_query` auto-recovers; other conn-using commands surface the poisoning as an error instead of hanging. Also note: the Save button's 'Saved' flip is transient (2s) — poll timing can miss it, which mimicked a save failure during debugging.
