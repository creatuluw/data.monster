---
type: Entity
title: ExprEditor component
description: "Smart DuckDB expression editor for master items (Qlik-Sense-style): autocomplete over bound-table fields, master items and a curated DuckDB function catalog, SQL syntax highlighting, per-kind starter templates, and live validation + result preview against the bound table."
tags: [central-charts, master-items, duckdb, editor, svelte]
timestamp: "2026-09-22T07:24:11.009Z"
---

# ExprEditor component

Smart DuckDB expression editor for master items (Qlik-Sense-style): autocomplete as you type over bound-table fields, ⭐ master items, and a curated DuckDB function catalog; SQL syntax highlighting under the caret; per-kind starter templates; live validation + result preview run against the bound table. Replaced the plain mono `TextInput` in every master-item expression field.

## Details

- **Location**: `src/lib/components/charts/ExprEditor.svelte`; function catalog in `src/lib/charts/duckdb-functions.ts` (both 2026-09-21).
- **Props**: `value` (bindable), `kind: 'measure' | 'dimension'`, `table`, `columns` (string or `{ name, type }` — typed columns power smarter suggestions), `masterItems`, `placeholder`, `preview` (default true).
- **Autocomplete**: fuzzy-ranked token matches (fields ×10, master items ×9, functions ×8), grouped listbox — ↑/↓ + Enter/Tab insert, Esc dismiss, Ctrl+Space lists all. Master-item matches insert the item's full expression.
- **Function catalog**: `DUCKDB_FUNCTIONS` — ~80 curated entries in 7 categories (`agg` incl. `FILTER`, `win`, `date`, `str`, `math`, `cond`, `cast`); snippets mark the caret landing spot with `§` (stripped on insert).
- **Templates**: per-kind starter chips (measure: share-of-total, conditional agg, filtered count, vs-last-period…; dimension: month period, year-month, bucket mapping…), appended to the current expression with the caret placed.
- **Live preview**: debounced 500ms, runs `select (<expr>) as v from "<table>" limit 5` (1 row for measures) via `executeQuery`; shows ✓ result chips or the truncated DuckDB error. A serialized promise queue guarantees **never two preview queries in flight** — overlapping invokes while typing are what tripped the IPC deadlock in the Tauri backend (see [hard-reload-storms-deadlock-duckdb-in-process](../../learnings/hard-reload-storms-deadlock-duckdb-in-process.md)).
- **Hosts**: [ItemEditor](./create-in-data-round-trip.md)'s create form (`/data` measures + dimensions tabs), BlockInspector's in-form creation (focused config drawer), and [RolePickerModal](./rolepickermodal-component.md)'s + New form.

## Relationships

- Feeds master-item creation in the [central-charts component system](./central-charts-component-system.md) semantic layer.
- Its serialized preview queue exists because of [hard-reload-storms-deadlock-duckdb-in-process](../../learnings/hard-reload-storms-deadlock-duckdb-in-process.md) — the execute_query `spawn_blocking` fix is the backend half of the same fix.

## Lifecycle

- First added: 2026-09-21 — replaced the mono TextInput expression fields in ItemEditor, BlockInspector, and RolePickerModal in the same pass.
