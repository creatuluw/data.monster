---
type: Decision
title: Workspaces are fully portable — switching reloads data, content, and settings
description: Context
tags: [workspace, settings, duckdb, portability, switch-flow]
status: accepted
timestamp: "2026-09-22T12:30:28.941Z"
---

# Workspaces are fully portable — switching reloads data, content, and settings

## Context

User definition (2026-09-22): *a workspace is a portable version of the app's data, definitions, settings, and content as it has been added/created.* Switching workspaces via the top-right folder picker must load ALL of it. Two things broke that contract:

1. `initialize_duckdb` early-returns "DuckDB already initialized" while a connection exists — so a switch silently kept the old workspace's DuckDB (old tables, pages, master items, saved queries, labels, field functions).
2. `settings.json` lived in the global app-data dir, so LLM config etc. never traveled with the workspace.

Content already lived in the workspace's DuckDB — it just never got reloaded.

## Choice

- **Switch = close, then open**: the frontend switch flow (`src/lib/stores/app.svelte.ts`) calls `shutdown_duckdb` BEFORE `initialize_duckdb(newPath)`, then reloads content, resets virtual tabs (`resetTabs()`) and navigates home so no old-workspace detail route survives. Same-path picks are a no-op.
- **Settings are workspace-scoped**: `settings.json` lives inside the workspace folder when a workspace is open (`src-tauri/src/commands/settings.rs`). The global app-data `settings.json` remains only as pre-workspace fallback and pre-migration read (its values carry over until first save). The `.env` override layer is unchanged and still wins for LLM_API_KEY/URL/MODEL in dev.

## Alternatives considered

- Keep settings global — rejected: the workspace then isn't portable, violating the definition.
- Remove the `initialize_duckdb` idempotency guard — rejected: shutdown-then-init at the switch site is explicit and keeps the guard protecting other double-init paths.

## Consequences

- The workspace folder is the unit of backup/move/copy: `d8a_monster.duckdb` + `data/main/` + `settings.json` is the whole app state.
- `workspace.json` (the pointer to the active workspace) deliberately stays global in app-data — it records WHICH folder is open, not workspace content.
- Any new code path that changes the workspace must run `shutdown_duckdb` first; see [initialize_duckdb no-ops while initialized — switch must shutdown first](../learnings/initialize-duckdb-no-ops-while-initialized.md).
- Known ceiling: sitting on `/settings` during a switch shows stale values until the next remount (the settings page reads on mount; analyst re-reads per message).
