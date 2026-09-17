---
type: Entity
title: database command module
description: "The Rust command module owning DuckDB **lifecycle** in the Tauri backend: initialize, graceful shutdown, and full reset. It is the code that turns a user-select"
tags: [rust, tauri, duckdb, backend]
timestamp: "2026-09-16T13:04:13.100Z"
---

# database command module

The Rust command module owning DuckDB **lifecycle** in the Tauri backend: initialize, graceful shutdown, and full reset. It is the code that turns a user-selected workspace folder into a live `d8a_monster.duckdb` session.

## Details

- **Location**: `src-tauri/src/commands/database.rs` (registered in `src-tauri/src/commands/mod.rs` + invoked from `src-tauri/src/main.rs`)
- **Commands**:
  - `initialize_duckdb(workspace_path)` — creates workspace + `data/main/` dirs, opens/creates `d8a_monster.duckdb`, recovers a previous session from the WAL (`d8a_monster.duckdb.wal`), runs orphaned-metadata cleanup, emits `db-init-progress` events to the frontend
  - `shutdown_duckdb()` — closes the connection cleanly (checkpoint/WAL flush)
  - `reset_all_data()` — wipes workspace data back to fresh state
- New module not yet in the overview's command-module list (2026-09-15)

## Relationships

- Sibling per-domain command module alongside [remote-chat-command](./remote-chat-command.md) (local_llm), files, queries, tables, etc.
- Its `initialize_duckdb` is the entry point for everything the [llm-agent-connection-research-report](../artifacts/llm-agent-connection-research-report.md) report would wrap as MCP tools

## Lifecycle

- First added: 2026-09-15 (extracted DB lifecycle out of app startup during central-charts work, commit "Restore point: pre central-charts build" on master)
