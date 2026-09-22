---
type: Decision
title: Timeout and retry defend against hung IPC
description: Context
tags: [tauri, ipc, robustness, central-charts, error-handling]
status: accepted
timestamp: "2026-09-22T11:35:26.419Z"
---

# Timeout and retry defend against hung IPC

## Context

The /pages E2E session (reports/pages-e2e-feedback.md) surfaced a P1 "app hang": the known in-process DuckDB deadlock (see [[learnings/hard-reload-storms-deadlock-duckdb-in-process]]) leaves Tauri invokes hung **forever — they never reject**. Because hung invokes don't throw, every unprotected surface failed silently: ExprEditor validation spun "checking…" eternally; writes (`save_page`, `save_master_item`) never surfaced failure — silent 60s auto-save + hung invoke is how page blocks "vanished"; "Create page" aborted before its `goto` with no error.

## Choice

Frontend defense layer (2026-09-22):

- `withTimeout` helper — ExprEditor validation races 15s, then shows "validation timed out" instead of eternal "checking…".
- `writeInvoke` in `central-api.ts` — races every write command at 20s **and retries once after 300ms**, absorbing the observed first-invoke-after-load IPC failure that silently lost just-added blocks.
- "Create page" (`handleCreate`) surfaces save errors instead of aborting silently; its `goto` is protected by writeInvoke.

## Alternatives considered

- Fix the deadlock in Rust — the actual root cause, FIXED the same day via in-process recovery, see [[learnings/duckdb-app-hangs-poisoned-connection-windows]]. The frontend defense ships now because the deadlock may take its own investigation.
- Optimistic UI with rollback — heavier, and rollback on a hung (not failed) invoke is meaningless.
- Timeouts on every invoke — unnecessary; only writes and validation had user-visible hang symptoms.

## Consequences

- New write paths should route through `writeInvoke`, not raw `invoke` — see [[pages/entities/central-api-frontend-invoke-client]].
- Users get honest errors instead of eternal spinners; silent write loss is gone.
- The frontend **defends, doesn't cure**: the backend deadlock itself was the real bug; cured the same day by in-process connection recovery in `execute_query` (src-tauri/src/commands/queries.rs), see [[learnings/duckdb-app-hangs-poisoned-connection-windows]].
