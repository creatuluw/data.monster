---
type: Entity
title: dm_watch command module
description: `src-tauri/src/commands/dm_watch.rs` — the Rust file watcher for the workspace `dm/` tree (task `files-007`, commit `72bb92e`, 2026-09-22, branch `feature/works
tags: [rust, backend, workspace-file-first, file-watcher, live-reload, dm-watch]
timestamp: "2026-09-22T14:52:22.837Z"
---

# dm_watch command module

`src-tauri/src/commands/dm_watch.rs` — the Rust file watcher for the workspace `dm/` tree (task `files-007`, commit `72bb92e`, 2026-09-22, branch `feature/workspace-file-first`). Turns agent file edits into Tauri events the frontend hot-reloads on — the "watch the agent build the page" half of the live-reload design. Shipped backend-side; the frontend listeners land in `files-008`.

## Why it matters

Agents author app content by editing workspace files; without the watcher the app only sees changes on the next command invocation. The watcher + echo suppression is what makes live co-editing safe — including no reload loops from the app's own write-throughs.

## Details

- **Location**: `src-tauri/src/commands/dm_watch.rs` (declared in `commands/mod.rs`; `notify = "8"` added to Cargo.toml)
- **Pipeline**: notify recursive watch on `<workspace>/dm/` → mpsc → debounced flusher thread (300ms window; a removal beats a modify of the same path in-window) → echo check → `classify` → emit `dm:changed {kind, name, removed}` / `dm:error {path, reason}`
- **Echo suppression lives with the writer**: `dm_store::atomic_write` calls `dm_watch::mark_self_write(dest)` after a successful rename; registry window 600ms ≥ write debounce 400ms — the app's own writes never loop back as events
- **Classification matrix** (`classify` is a pure fn, unit-tested without Tauri): `dm/pages/*.json` → requires `{"rows"}`; `dm/master-items/{measures,dimensions}/*.json` → `{"tableName","expr"}`; `dm/relationships.json` → `{"relationships"}`; `dm/saved-queries/*.sql` → header check via `saved_queries::check_query_file`; `dm/connections.json` → `{"connections"}`; drafts, docs, README, `.env`, settings, unknown files → ignored (agent scratch space is safe)
- **Lifecycle**: starts on `initialize_duckdb`; a workspace switch replaces it (dropping the watcher stops its thread)

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — supplies the path→name inverses (`page_slug_from_path`, `item_from_path`, `saved_query_slug_from_path`) and `check_json_object` shape checks that `classify` builds on; `atomic_write` feeds the echo registry
- [database-command-module](./database-command-module.md) — starts the watcher during DuckDB init (`database.rs`)
- [saved-queries-file-store-saved-queries-rs](./saved-queries-file-store-saved-queries-rs.md) — owns `check_query_file`, the saved-query header check classify reuses
- [Live-reload mechanics decision](../../decisions/live-reload-mechanics-notify-watcher-validate-dm-changed-dm-.md) — the design this implements (approach A: full watcher)
- [Workspace-file-first spec & tasks](../artifacts/workspace-file-first-spec-tasks.md) — FR-7 / `files-007` of the executable spec

## Lifecycle

- First added: 2026-09-22, `files-007` — 5 classification/echo tests; full suite 115/115
- Next: `files-008` — `dm:changed`/`dm:error` listeners, store invalidation, toast/inline problems surfacing
