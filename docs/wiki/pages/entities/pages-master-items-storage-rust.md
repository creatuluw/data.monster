---
type: Entity
title: Pages & master-items storage (Rust)
description: "The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD"
tags: [central-charts, rust, duckdb, storage]
timestamp: "2026-09-15T10:55:52.612Z"
---

# Pages & master-items storage (Rust)

The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageDoc`s, workspace-level master items (measures/dimensions), and the table relationship graph.

Backs the `/pages` → `/page/<slug>` dual-mode editor and the `/data` Relationships + Measures/Dimensions tabs. Master items implement [master-item-library-table-binding-q6](../../decisions/master-item-library-table-binding-q6.md); the relationship graph implements [q7-locked-relationship-graph-drives-chart-item-availability-](../../decisions/q7-locked-relationship-graph-drives-chart-item-availability-.md) (BFS auto-JOIN lives frontend-side in the query compiler).

## Details

- **Tables**: `d8a_monster_pages` (page docs by slug), `d8a_monster_items` (master items, stable ids, created in `commands/database.rs`), `d8a_monster_relationships` (relationship graph)
- **Commands**: `commands/pages.rs` — `list_pages`, `get_page(slug)`, `save_page`, `delete_page`; `commands/items.rs` — `list_master_items` + CRUD; `commands/relationships.rs` — graph CRUD
- **Tests**: 8 cargo tests (e.g. upsert preserves `created_at`)
- Persists documents described by [chart-page-spec-spec-types-validator](./chart-page-spec-spec-types-validator.md); first shipped 2026-09-15 on branch `feature/central-charts` (all 17 FRs of the central-charts build).

## Lifecycle

- First added: 2026-09-15, central-charts build (commits `ae48ef2` + `35966c6`), CDP-smoke-verified end to end (create → save → persist → list).
