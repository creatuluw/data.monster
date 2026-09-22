---
type: Entity
title: Workspace command module (Rust)
description: The Rust command module owning workspace identity — picking, persisting (workspace.json + open history), and resolving the active workspace folder.
tags: [workspace, rust, tauri-commands, portability]
timestamp: "2026-09-22T12:30:28.942Z"
---

# Workspace command module (Rust)

## What is it?

The Tauri command module that owns workspace **identity** — picking, persisting, and resolving the active workspace folder — plus the app-store switch flow that makes a workspace a portable bundle of data, definitions, settings, and content ([Workspaces are fully portable — switching reloads data, content, and settings](../../decisions/workspaces-are-fully-portable-switch-reloads.md) decision).

## Why it matters

Every DuckDB command and the settings loader resolve paths from the active workspace. This module is the only writer of the workspace pointer, and the switch flow it feeds is what makes switching load the right database and settings.

## Details

- **Location**: `src-tauri/src/commands/workspace.rs`; switch flow in `src/lib/stores/app.svelte.ts`; invoke wrappers in `src/lib/db-operations.ts`
- **Interface / commands**:
  - `choose_workspace_folder` — native folder picker; writes the pointer and returns the chosen path (or `None` on cancel)
  - `get_workspace_path` — resolves the pointer; returns `None` if no pointer or the folder no longer exists
  - `set_workspace_path` — writes the pointer directly
  - `list_workspaces` — returns the workspace history (`{ path, lastOpened }`) sorted newest first; silently drops folders that no longer exist
- **Configuration**: pointer file `<app-data>/workspace.json` = `{ "path": "..." }` — global by design (records *which* folder is open, never workspace content). Workspace-local state lives in the workspace folder itself: `d8a_monster.duckdb`, `data/main/`, and (since 2026-09-22) `settings.json`. History file `<app-data>/workspaces.json` = `[{ path, lastOpened }]` — upserted on every pick (dialog or [/workspaces page](./workspaces-page-workspaces.md)); a list *of* workspaces, so it can never live inside one.

## Relationships

- [database-command-module](./database-command-module.md) — owns the DuckDB lifecycle the switch flow drives: `shutdown_duckdb` → `initialize_duckdb(newPath)` (initialize no-ops while initialized)
- Settings module (`settings.rs`) — resolves `settings.json` inside the workspace folder when one is open, global app-data as fallback
- Glossary "Workspace" — the folder concept this module points at

## Lifecycle

- First added: with the Tauri migration, as the folder picker + pointer store.
- 2026-09-22 — switch reworked for full portability: shutdown-before-reinit, settings.json moved into the workspace, content reload + tab reset on switch; `selectWorkspace()` split into dialog + `selectWorkspaceByPath(path)` so the page and dialog share one switch path.
- 2026-09-22 — added workspace history (`workspaces.json`, `list_workspaces`) backing the new [/workspaces switcher page](./workspaces-page-workspaces.md).
