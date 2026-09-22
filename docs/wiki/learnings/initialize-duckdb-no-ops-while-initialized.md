---
type: Learning
title: initialize_duckdb no-ops while initialized — workspace switch must shutdown first
description: Fact
tags: [duckdb, workspace, lifecycle, gotcha]
timestamp: "2026-09-22T12:30:28.942Z"
---

# initialize_duckdb no-ops while initialized — workspace switch must shutdown first

## Fact

`initialize_duckdb` (`src-tauri/src/commands/database.rs`) early-returns `"DuckDB already initialized"` whenever `DuckDbState` already holds a connection. It never re-points the connection at a new workspace path.

## Consequence

A workspace switch that just calls `initialize_duckdb` again **silently keeps the old workspace's database** — no error, but every surface (tables, pages, master items, saved queries, labels, field functions) still shows the previous workspace's data.

## Pattern

The switch flow in `src/lib/stores/app.svelte.ts` closes first, with a code comment saying exactly this:

```ts
// Close the old workspace's DB first — initialize_duckdb no-ops while initialized
await shutdownDuckdb();
await initializeDuckdb(newPath);
```

Same shutdown → initialize sequence already used for the in-process hang-recovery path ([[duckdb-app-hangs-poisoned-connection-windows]]).

## Rule of thumb

Any code path that changes the workspace must run `shutdown_duckdb` before `initialize_duckdb` — never rely on `initialize_duckdb` to re-point at a new path.
