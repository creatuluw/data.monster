---
type: Entity
title: dm_store command module
description: "`src-tauri/src/commands/dm_store.rs` — the Rust foundation module for the workspace-file-first architecture: dm/ path conventions, name validation, atomic write"
tags: [rust, backend, workspace-file-first, storage, dm-store]
timestamp: "2026-09-22T13:57:08.173Z"
---

# dm_store command module

`src-tauri/src/commands/dm_store.rs` — the Rust foundation module for the workspace-file-first architecture: dm/ path conventions, name validation, atomic writes, and parse-level shape checks. Not an invoke-command module itself; it exposes plain functions that the content command modules (pages, master-items, saved-queries, connections, drafts) build on. Shipped 2026-09-22 as task `files-001` (commit `4b722d9` on `feature/workspace-file-first`), 11 in-module cargo tests green.

## Why it matters

It is the single choke point where "path is identity" becomes real: every read/write of agent-authored workspace content routes through these helpers, so agents editing files and the frontend writing via commands land on identical paths and never observe a partial file.

## Details

- **Location**: `src-tauri/src/commands/dm_store.rs` (declared in `src-tauri/src/commands/mod.rs`)
- **Interface**:
  - Path mapping — `dm_dir`, `page_path` (`dm/pages/<slug>.json`), `item_path` (`dm/master-items/{measure,dimension}/<id>.json`), `relationships_path`, `saved_query_path` (`dm/saved-queries/<slug>.sql`), `connections_path`, `drafts_dir`
  - Path→name inverses consumed by the dm_watch watcher (`files-007`) — `page_slug_from_path`, `item_from_path`, `saved_query_slug_from_path`
  - `valid_name` — rejects traversal, separators, leading dots, padded whitespace; allows interior spaces (`Q1 Sales`)
  - `atomic_write` — temp file + rename, temp cleanup on failure, parents auto-created
  - `check_json_object` — parse-level shape check the dm_watch watcher uses to classify `dm:changed` vs `dm:error`
- **Configuration**: none — paths derive from the active workspace root
- **Documented ceiling**: crash *between* temp-write and rename is not unit-testable without fault-injection seams; the atomicity guarantee is stated in the module doc-comment instead of theater-tested.

## Relationships

- [Workspace content tree decision](../../decisions/workspace-content-tree-dm-with-path-is-identity-native-forma.md) — implements its "path is identity, native formats" contract
- [Workspace-file-first spec & tasks](../artifacts/workspace-file-first-spec-tasks.md) — FR-1 of the executable spec; `files-002` (remapping pages commands) is its first consumer
- [pages-master-items-storage-rust](./pages-master-items-storage-rust.md) — the DuckDB-backed storage it will progressively replace for pages/master-items
- [database-command-module](./database-command-module.md) — sibling DuckDB lifecycle module in the same commands registry
- [dm_watch command module](./dm-watch-command-module.md) — consumes the path inverses and shape checks; `atomic_write` marks self-writes into its echo registry

## Lifecycle

- First added: 2026-09-22, task `files-001` — TDD (RED run caught two real bugs: `.sql` files fell through the JSON-only stem helper; `valid_name` self-contradicted on interior spaces)
- 2026-09-22: `files-002`–`006` shipped — pages, master items, saved queries, connections, and the one-time migration are all file-backed; `files-007` (dm_watch) consumes the path→name inverses, and `atomic_write` now feeds the watcher’s echo registry
- Next: `files-008` — frontend `dm:changed`/`dm:error` listeners
