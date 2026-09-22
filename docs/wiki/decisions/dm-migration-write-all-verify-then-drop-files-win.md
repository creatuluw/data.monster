---
type: Decision
title: "dm/ migration: write all, verify, then drop — files win"
description: Context
tags: [workspace-file-first, migration, data-safety, rust]
status: accepted
timestamp: "2026-09-22T14:41:49.628Z"
---

# dm/ migration: write all, verify, then drop — files win

## Context

Phase A of the workspace-file-first feature (`files-006`, branch `feature/workspace-file-first`) needed to move the four content DuckDB tables (pages, master items, relationships, saved queries — plus connections) out of the internal DB and into `dm/` files, without any data-loss window.

## Choice

One-shot migration in `src-tauri/src/commands/migration.rs`, hooked into `initialize_duckdb`:

- **Export reuses the store functions themselves** (`dm_store` / per-domain store_in) — format consistency by construction, no parallel serializer to drift.
- **Crash-safe ordering**: write ALL files → verify each exists → only then drop the tables. A crash mid-run leaves tables intact; retry is idempotent with **files-win** semantics.
- **Failure is a warning, not an error** — tables intact means next launch finishes the job.
- Migration writes the **exact DB slug**, which may differ from `generate_slug(name)` after renames.

## Alternatives considered

- Per-table write+drop loop — smaller code but opens a data-loss window if the process dies mid-run.
- A separate export serializer — risks format drift from what the live stores write.

## Consequences

- After migration, the content tables in `d8a_monster.duckdb` are transitional/export-only — the `dm/` files are canonical (see [[workspace-files-are-canonical-agents-author-content-by-editi]]).
- Single-doc stores must materialize their file **even when empty**, or verify fails on a fresh workspace (see the single-doc-stores rule).
- Until the user restarts the dev app, the migration hasn't run — empty lists after a rebuild are the not-yet-migrated state, not data loss.
