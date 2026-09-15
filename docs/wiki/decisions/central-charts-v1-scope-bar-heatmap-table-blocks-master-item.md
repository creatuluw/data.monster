---
type: Decision
title: "Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred"
description: Context
tags: [central-charts, report-pages, scope, v1]
status: accepted
timestamp: "2026-09-15T09:18:44.117Z"
---

# Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred

## Context

The central reusable-chart build (report pages: charts + blocks on a 12-col grid, dual-mode editor) starts from `.specs/central-charts/spec.md` — 13 FRs in 5 phases. The interview had already locked Q3–Q16 (spec format, expressions-as-measures, master items, auto-JOIN, query engine, option panels, cross-filter, tooltips, shared colors, grid, annotations, routes). This decision records the **v1 scope calls** made while writing the spec (reported to the user for veto, none vetoed at time of writing).

## Choice — v1 ships bar chart, heatmap, and table as the first page objects

Plus a text block. Six scope calls:

1. **Master items + relationship auto-JOIN deferred** to the next phase. v1 uses inline expressions only; a `ref` field is reserved in the spec types so master items slot in later without a breaking change.
2. **Annotations v1 = `ruleY` on bar only** — one mark from the Q15 basic-marks whitelist; heatmap opts out; other types additive later.
3. **Text block included** — locked in Q14-B and nearly free.
4. **Old draft `/pages` is replaced**; top-level `BarChart.svelte` and the `pages - Copy` route retired (FR-13 cleanup).
5. **No new dependencies** — hand-rolled spec validator, no zod.
6. **Table block = plain query rows** — no aggregation, limit 50, respects cross-filter selection.

## Alternatives considered

- Building master items + auto-JOIN now (rejected: biggest risk surface, delays first renderable page).
- Richer annotation set from day one (rejected: per-type whitelisting makes it additive; ruleY proves the pipeline).
- zod for spec validation (rejected: adds a dependency for a schema we own end-to-end).

## Consequences

- First user-visible page can be composed from bar + heatmap + table + text on the grid.
- The `ref` reservation means master-item adoption later is non-breaking.
- Phase 5 explicitly deletes the draft `/pages` and the copied route — no legacy page format survives.
