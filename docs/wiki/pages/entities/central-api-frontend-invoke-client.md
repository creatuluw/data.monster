---
type: Entity
title: central-api (frontend invoke client)
description: What is it?
tags: [central-charts, frontend, tauri, persistence, ipc-hardening]
timestamp: "2026-09-22T11:36:40.977Z"
---

# central-api (frontend invoke client)

## What is it?

The frontend client for the central-charts internal DB: a thin, typed set of Tauri `invoke` wrappers in `src/lib/central-api.ts` covering pages CRUD (`list_pages` / `get_page` / `save_page` / `delete_page`), master items (`list/save/delete_master_item`), and relationships (`list/save/delete_relationship`). `PageDoc` JSON is stringified/parsed at this boundary — callers only ever see typed objects.

## Why it matters

It is the single import surface every central-charts UI route uses to reach persistence — no route invokes these Tauri commands directly. One place to change when a command signature moves. Since 2026-09-22 it is also the choke point for **hung-invoke defense**: all writes go through `writeInvoke`.

## Details

- **Location**: `src/lib/central-api.ts`
- **Write hardening**: `writeInvoke` races every write command at 20s and retries once after 300ms — absorbing the first-invoke-after-load IPC failure and the hung-invoke/DuckDB-deadlock failure mode (see [Timeout and retry defend against hung IPC](../../decisions/timeout-and-retry-defend-against-hung-ipc.md)). New write paths should use it, not raw `invoke`.
- **Consumers**: `src/routes/pages/+page.svelte` (list/create/delete), `src/routes/pages/[slug]/+page.svelte` (editor load/save), `src/lib/components/charts/ItemEditor.svelte`, `src/lib/components/charts/RelationshipEditor.svelte`
- **Rust side**: `src-tauri/src/commands/{pages,items,relationships}.rs` — see [Pages & master-items storage (Rust)](./pages-master-items-storage-rust.md) for the DuckDB tables behind these commands
- **Types**: re-exports `PageDoc` from [Chart page spec (spec-types + validator)](./chart-page-spec-spec-types-validator.md), `MasterItem`, `Relationship` from `$lib/charts/{items,relationships}`

## Lifecycle

- First added: 2026-09-16 (central-charts FR-10/14/15 storage tasks) — replaced ad-hoc invokes scattered in routes
- 2026-09-22: `listMasterItems` now normalizes Rust's `tableName` field to the frontend `table` at this boundary — the single choke point that un-broke every ref-based master-item chart (see [tableName/table mismatch learning](../../learnings/ref-based-master-items-tablename-mismatch-broke.md)). Boundary contract: callers only ever see corrected, typed objects — raw Rust serialization quirks are mapped here, never in consumers.
- 2026-09-22: writes hardened with `writeInvoke` (20s timeout + one 300ms retry) — hung invokes used to fail silently (blocks "vanished" via silent auto-save); they now surface errors (see [Timeout and retry defend against hung IPC](../../decisions/timeout-and-retry-defend-against-hung-ipc.md)).
