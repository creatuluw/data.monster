---
type: Entity
title: data/incoming drop folder (incoming.rs)
description: data/incoming drop folder (incoming.rs)
tags: [rust, backend, workspace-file-first, ingest, duckdb]
timestamp: "2026-09-22T15:29:40.287Z"
---

# data/incoming drop folder (incoming.rs)

# data/incoming drop folder (incoming.rs)

`src-tauri/src/commands/incoming.rs` — the workspace `data/incoming/` drop zone (workspace-file-first FR-13, 2026-09-22): drop a CSV/Parquet/JSON file there and it is ingested into the workspace DuckDB automatically, with zero app interaction.

## Why it matters

Ingest previously ran only through the /connect page; the drop folder makes ingest ambient. A user drags a file into the workspace folder — or a coding agent copies one in — and the table exists. It is the file-first analog of Connect: the filesystem gesture replaces the UI flow.

## Details

- **Location**: `src-tauri/src/commands/incoming.rs`; entry point `ingest_incoming_file(conn, ws, path) -> table name`
- **Readers**: reuses the Connect pipeline's DuckDB readers — `read_csv_auto('…', ignore_errors=true)`, `read_parquet`, `read_json_auto`; other extensions error ("unsupported file type")
- **Table naming**: filename stem, lowercased, spaces/dashes → `_`; a table with that name already existing is an error that names the fix ("rename the file to ingest as a new table")
- **Success**: `CREATE TABLE … AS SELECT`, then the file is moved to `data/main/` (the canonical source-file home)
- **Failure**: unsupported type / bad name / SQL error / duplicate table → `dm:error` event, file left untouched in `data/incoming/`
- **Tests**: in-module `#[cfg(test)]` — csv round-trip + both error paths (per [rules/rust-backend-tests-live-in-module-via-cfg-test](../../rules/rust-backend-tests-live-in-module-via-cfg-test.md))

## Relationships

- [dm-watch-command-module](./dm-watch-command-module.md) — the watcher side that emits `dm:error` for failed ingests
- [dm-store-command-module](./dm-store-command-module.md) — name validation (`valid_name`) shared
- [workspace-file-first-spec-tasks](./workspace-file-first-spec-tasks.md) — FR-13 of the executable spec (the one "cuttable" piece — it shipped)

## Lifecycle

- First added: 2026-09-22, FR-13 of the workspace-file-first build
