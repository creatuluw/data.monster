---
type: Decision
title: Linked-table raw fields are transient with auto-JOIN — master-item creation stays optional
description: Context
tags: [central-charts, master-items, data-binding]
status: accepted
timestamp: "2026-09-17T13:06:00.758Z"
---

# Linked-table raw fields are transient with auto-JOIN — master-item creation stays optional

## Context

User asked for table-first data binding on all chart components: pick a source table, then pick dimensions/measures from that table's fields or fields of relationship-linked tables — either by selecting an existing master item from the library or creating one on the spot (saved to the lib).

Existing locked decisions: [[master-item-library-table-binding-q6]] (workspace-level master items, stable ids, explicit table binding) and [[q7-relationship-graph-drives-item-availability]] (relationship graph drives availability + auto-JOIN).

Open fork: when a user picks a **raw field from a linked table** (not the source table), does it become a master item or stay transient?

- **A) Transient auto-JOIN** — chart stores field + table directly, query auto-JOINs; no library entry.
- **B) Must become a master item** — picking a linked field forces the create-form.
- **C) Hybrid** — source-table fields transient, linked-table fields forced into the lib.

## Decision

**A — raw fields (source or linked) stay transient.** The inspector picker pool is **master items + raw fields from the selected table + raw fields from linked tables**, symmetric for dimensions and measures. Creating a master item is an *optional* path (`✚ Create…` inline form → `saveMasterItem` → chart refs the stable id), never forced.

## Consequences (shipped, TDD red→green, vitest 104/104)

- `spec-types.ts` — `DimensionSpec`/`MeasureSpec` gain an optional `table` field (linked-table binding on raw entries; master-item refs unchanged).
- `items.ts` — raw entries with `table ≠ source` feed `involvedTables` → existing `buildJoins` BFS auto-JOINs them.
- `compile.ts` — linked dimensions compile qualified (`"clients"."region"`), validated against their own table.
- `relationships.ts` — new `linkedTables()` helper (BFS from source, excludes source) drives the picker groups.
- `BlockInspector.svelte` — table-first: switching source table prunes dims/refs no longer reachable; grouped pickers (⭐ master items | source fields | `⤳ linked` fields | `✚ Create…`); measures default `sum(field)` with an agg dropdown; host reloads the library via `onItemsChanged`.

## Alternatives rejected

- **B** — forcing every linked field into the lib pollutes the master-item library with one-offs and adds friction; contradicts "creation is optional".
- **C** — asymmetric rules are harder to explain than either extreme; the auto-JOIN machinery made transient linked fields nearly free.

## Rationale

The auto-JOIN machinery already handled master items from linked tables; extending `involvedTables` to raw entries was a smaller diff than inspector-enforced creation, and keeps the library curated rather than a dump of every picked field.
