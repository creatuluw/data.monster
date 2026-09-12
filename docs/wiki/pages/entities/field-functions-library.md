---
type: Entity
title: Field Functions library
description: A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the 
tags: [field-functions, sql, tables, settings]
timestamp: "2026-09-12T11:06:03.726Z"
---

# Field Functions library

A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the `d8a_monster_field_functions` internal DuckDB table.

## What it is

The Field Functions subsystem lets users define reusable SQL templates that transform a column, then apply them when browsing tables. Functions are managed in Settings (`/settings/field-functions`) and applied via the column drawer (`ColumnFunctionDrawer.svelte`) on the Data/Table views. Spec: `.specs/field-function-library`.

## Why it matters

It extends the table browsing experience beyond read-only viewing — users can derive new columns without writing full SQL, which feeds the Analyst and Pages workflows.

## Details

- **Rust backend**: `src-tauri/src/commands/field_functions.rs` — CRUD Tauri commands over `d8a_monster_field_functions` (id, label, description, sql_template, applies_to, output_type), registered in `commands/mod.rs` / `main.rs`
- **Frontend library**: `src/lib/field-functions/library.ts`
- **UI**: `src/lib/components/ColumnFunctionDrawer.svelte` (apply to a column), `src/routes/settings/field-functions/` (manage library)
- **Storage**: internal DuckDB table `d8a_monster_field_functions`, browsable via Settings → Internal DB browser

## Relationships

- Implements a spec-driven feature: `.specs/field-function-library`
- Depends on the DuckDB Rust backend state/locking pattern used by all command modules
- Complements saved queries and tags as internal-DB-backed user metadata

## Lifecycle

- First added: 2026-09 — uncommitted working tree; spec under `.specs/field-function-library`
