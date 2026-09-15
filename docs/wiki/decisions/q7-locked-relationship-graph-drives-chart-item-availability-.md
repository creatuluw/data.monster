---
type: Decision
title: "Q7 locked: relationship graph drives chart item availability and auto-JOIN"
description: Context
tags: [chart-spec, report-pages, master-items, table-relationships, auto-join, qlik]
status: accepted
timestamp: "2026-09-15T08:33:48.687Z"
---

# Q7 locked: relationship graph drives chart item availability and auto-JOIN

## Context

Continuation of the 2026-09-15 chart-spec/report-page design interview. Q6 ([[master-item-library-table-binding-q6]]) locked the workspace-level master-item library with explicit table binding and left **Q7 open**: how the binding drives chart item availability.

The user answered **B** to Q7.

## Choice

**B locked — table matching + relationship graph.** The relationship graph drives both sides of the data path:

- A chart bound to table X offers master items from X **and tables related via the table-relationships model** (`.prds/table-relationships` PRD).
- Picking an item from a related table makes the chart's query auto-`JOIN` along the declared relationship (`bookings ⋈ clients`) — Qlik-association feel.

This closes the data path: stable-id master items + explicit table binding + relationship-driven availability/auto-JOIN.

## Alternatives considered

- **A) Strict table matching** — simple and siloed; rejected because it can't deliver the requested "smart logic" across related tables.
- **C) Inference-first** (parse expressions, match against all schemas) — most "smart", least predictable; renames or shared column names shift associations silently.

## Consequences

- Table-relationships (`.prds/table-relationships`) becomes a **load-bearing dependency** of chart item availability — not just a documentation PRD.
- Chart SQL generation gains a **JOIN-planning responsibility** for cross-table items.
- This reopened Q5's follow-up as **Q8 (open)**: who generates the SQL — one shared query engine for all 32 chart types (chart type = pure renderer + role schema), per-type query templates, or shared engine + per-type hooks (date grain transform, top-N + Other, funnel ordering). Assistant lean: shared engine + type hooks (C) — one canonical query covers ~28 types, the few needing reshaping (heatmap grid pivot, funnel) get a hook instead of a forked engine.

## Source

- Design interview turn, 2026-09-15 (user's Q7 answer "B"; Q8 posed with lean C).
