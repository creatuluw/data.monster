---
type: Decision
title: "Q8 locked: one canonical query engine with per-type hooks"
description: Context
tags: [chart-spec, report-pages, query-engine, labs, registry-hooks, svelteplot]
status: accepted
timestamp: "2026-09-15T08:37:38.178Z"
---

# Q8 locked: one canonical query engine with per-type hooks

## Context

Continuation of the 2026-09-15 chart-spec/report-page design interview. Q7 ([[q7-relationship-graph-drives-item-availability]]) left **Q8 open**: who generates the SQL — one shared query engine for all 32 chart types, per-type query templates, or shared engine + per-type hooks.

The user answered **C**.

## Choice

**C locked — one canonical query engine + per-type hooks.** One shared query engine turns filled data roles into SQL for every chart type (chart type = pure renderer + role schema). The few shapes that need reshaping register hooks on the engine instead of forking it: **grain** (date transforms), **top-N + "Other"** bucketing, **grid pivot** (heatmap), **funnel ordering**. One canonical query covers ~28 types; hooks cover the rest.

## Alternatives considered

- **A) One engine for all, no hooks** — can't express grain/pivot/funnel reshaping.
- **B) Per-type query templates** — 32 forks of query logic guarantee drift and untested corners.

## Consequences

- Query generation stays in one tested place; special shapes are declarative per-type hooks. Pairs with [[chart-fundament]] (buildBars aggregation) as the shared core every /labs chart builds on.
- Same turn proposed the **prop taxonomy** (not yet locked): a shared base every chart gets — roles (from Q8), titles (`title`/`subtitle`), per-role `fmt` (one format library), legend, axis groups (title/ticks/gridlines/min-max/fit/labels/rotate), size (`heightVh`), states (empty, error, loading, *missing master item*), selection (`sameDatum`) — while the registry declares per-type mark options (scatter `size`, line `missing_values`/curve, heatmap `scheme`/`threshold`, bar `topN`/Other/orientation/stacked). Consistent with [[evidence-chart-architecture]].
- **Q9 (open)**: how per-type options reach the no-code UI — (A) hand-built panels per type (current rule, [[each-labs-chart-owns-its-config-panel]]), (B) schema-driven panels auto-rendered from a registry options schema (field name, kind, default, min/max; JSON in code mode; unknown field → validation error), (C) **B + custom panel snippet escape hatch** (schema still declares the field so both surfaces stay consistent). Assistant lean: **C**.

## Source

- Design interview turn, 2026-09-15 (user's Q8 answer "C"; prop taxonomy proposed, Q9 posed with lean C).
