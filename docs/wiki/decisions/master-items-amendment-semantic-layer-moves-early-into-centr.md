---
type: Decision
title: "Master-items amendment: semantic layer moves early into central-charts v1"
description: Context
tags: [central-charts, master-items, semantic-layer, planning]
status: proposed
timestamp: "2026-09-15T09:31:01.992Z"
---

# Master-items amendment: semantic layer moves early into central-charts v1

## Context

The central-charts v1-scope decision ([central-charts-v1-scope](../decisions/central-charts-v1-scope-bar-heatmap-table-blocks-master-item.md)) explicitly **deferred** the master-item library and relationship-graph auto-JOIN out of phase 1, and the written spec (`.specs/central-charts/spec.md`, FR-1..13) reflects that deferral.

At the end of the design session (2026-09-15) the user amended this: master items + auto-JOIN + editors should be built **as part of the central-charts build, inserted early** — the semantic layer feeds the query engine, so it belongs before it.

## The amendment (agreed, not yet written into the spec)

- **~4 new FRs/todos** inserted early in the task list: master-item library, relationship graph + auto-JOIN, and the three `/data` editors — a **relationship editor** plus **Measures | Dimensions tabs** under `/data`.
- Internal DB tables: `d8a_monster_pages`, `d8a_monster_items`, `d8a_monster_relationships` (note: the design doc says `d8a_monster_master_items`; the amendment proposes `d8a_monster_items` — name pending confirmation).
- Cleanup confirmed as part of the plan: old draft `/pages` replaced, top-level `src/lib/components/BarChart.svelte` and the `pages - Copy` route retired (already FR-13).

## Pending confirmations (asked, not yet answered)

1. Amendment tweaks — scope of the ~4 new FRs.
2. Cleanup OK — retire `BarChart.svelte` (top-level) + `pages - Copy`.
3. Table names — the three `d8a_monster_*` names above.

## Consequences

- Supersedes **only the deferral clause** of the v1-scope decision; bar + heatmap + table + text blocks remain the chart/block citizens.
- Once confirmed, `.specs/central-charts/spec.md` + `tasks.json` get the new FRs/todos inserted before the query-engine FR (semantic layer first).
- [[q6]] master-item model (stable ids, table-bound) and [[q7]] relationship-graph auto-JOIN become v1 requirements instead of follow-ups.

## Source

- Session summary turn, 2026-09-15 (assistant's three pre-build confirmation questions).
