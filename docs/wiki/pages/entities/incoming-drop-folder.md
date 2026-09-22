---
type: Entity
title: Incoming drop folder
description: "The `data/incoming/` drop folder in the workspace (workspace-file-first FR-13, the "one cuttable piece" that shipped): drop a CSV, Parquet, or JSON file in and "
tags: [ingest, watcher, dm-tree, workspace, backend]
timestamp: "2026-09-22T15:30:37.206Z"
---

# Incoming drop folder

The `data/incoming/` drop folder in the workspace (workspace-file-first FR-13, the "one cuttable piece" that shipped): drop a CSV, Parquet, or JSON file in and it is auto-ingested — no connect step, no UI. Core function `ingest_incoming_file(conn, ws, path)` picks the reader by extension (`read_csv_auto` / `read_parquet` / `read_json_auto`), creates a table named after the filename stem, and moves the file to `data/main/` on success.

Behavior contract:

- **Success** → table exists, file relocated to `data/main/`, `dm:changed` fires so table lists hot-reload
- **Failure** (unsupported extension, duplicate table name, parse error) → file left **untouched**, named error returned
- Watched by a second notify watcher inside [pages/entities/dm-watch-command-module](./dm-watch-command-module.md); because `load_file` is State-coupled, the watcher thread reaches the DB via `app.state::<DuckDbState>()`

## Details

- **Location**: `src-tauri/src/commands/incoming.rs` (132 lines) · watcher wiring in `src-tauri/src/commands/dm_watch.rs`
- **Interface**: `fn ingest_incoming_file(conn: &Connection, ws: &Path, path: &Path) -> Result<String, String>` (returns the created table name)
- **Configuration**: none — folder path is fixed at `<workspace>/data/incoming/`
- **Tests**: `#[cfg(test)]` in-module — ingest+move, unsupported-type and duplicate-table error without touching the file

## Relationships

- [pages/entities/dm-watch-command-module](./dm-watch-command-module.md) — hosts the incoming watcher + ingest flusher
- [decisions/workspace-files-are-canonical-agents-author-content-by-editi](./decisions-workspace-files-are-canonical-agents-author-conten.md) — agents add data by placing files, no commands needed
- [agent-docs-system](./agent-docs-system.md) / [agent-prompts-page](./agent-prompts-page.md) — the ingest prompt points agents here as the zero-code path

## Lifecycle

- First added: files-013, 2026-09-22 — phase D of [pages/artifacts/workspace-file-first-spec-tasks](./workspace-file-first-spec-tasks.md); completed all 13 tasks of the branch
