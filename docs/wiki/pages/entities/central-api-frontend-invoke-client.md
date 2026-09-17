---
type: Entity
title: central-api (frontend invoke client)
description: What is it?
tags: [central-charts, frontend, tauri, persistence]
timestamp: "2026-09-16T17:13:20.930Z"
---

# central-api (frontend invoke client)

## What is it?

The frontend client for the central-charts internal DB: a thin, typed set of Tauri `invoke` wrappers in `src/lib/central-api.ts` covering pages CRUD (`list_pages` / `get_page` / `save_page` / `delete_page`), master items (`list/save/delete_master_item`), and relationships (`list/save/delete_relationship`). `PageDoc` JSON is stringified/parsed at this boundary — callers only ever see typed objects.

## Why it matters

It is the single import surface every central-charts UI route uses to reach persistence — no route invokes these Tauri commands directly. One place to change when a command signature moves.

## Details

- **Location**: `src/lib/central-api.ts`
- **Consumers**: `src/routes/pages/+page.svelte` (list/create/delete), `src/routes/pages/[slug]/+page.svelte` (editor load/save), `src/lib/components/charts/ItemEditor.svelte`, `src/lib/components/charts/RelationshipEditor.svelte`
- **Rust side**: `src-tauri/src/commands/{pages,items,relationships}.rs` — see [pages-master-items-storage-rust](./pages-master-items-storage-rust.md) for the DuckDB tables behind these commands
- **Types**: re-exports `PageDoc` from [chart-page-spec-spec-types-validator](./chart-page-spec-spec-types-validator.md), `MasterItem`, `Relationship` from `$lib/charts/{items,relationships}`

## Lifecycle

- First added: 2026-09-16 (central-charts FR-10/14/15 storage tasks) — replaced ad-hoc invokes scattered in routes
