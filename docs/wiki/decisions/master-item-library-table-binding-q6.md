---
type: Decision
title: "Q6 locked: workspace-level master-item library (stable ids) with explicit table binding; Q7 open on binding depth"
description: Context
tags: [chart-spec, report-pages, master-items, measures, dimensions, qlik]
status: accepted
timestamp: "2026-09-15T08:32:28.659Z"
---

# Q6 locked: workspace-level master-item library (stable ids) with explicit table binding; Q7 open on binding depth

## Context

Continuation of the 2026-09-15 chart-spec/report-page design interview. Q5 ([[measures-dimensions-are-duckdb-expressions]]) settled measures/dimensions as arbitrary DuckDB expression strings and left **Q6 open**: where the master-item library (reusable dimensions/measures) lives.

The user answered **A** to Q6 and added a requirement: *each measure/dimension is associated with the table/data model it was created for, and this needs smart logic.*

## Choice

**A locked — workspace-level master library, plus explicit table binding:**

- Master items live workspace-level in the internal DB (à la `d8a_monster_master_items`), **referenced by stable id** from chart specs — renames propagate; deletion surfaces a "missing master item" state instead of silently breaking.
- Every master item carries a **binding to the table/data model it was created for** (e.g. `table: "bookings"`). Binding is explicit metadata on the item, not inferred.

## Q7 open — how the binding drives chart item availability ("smart logic")

- **A) Strict table matching** — a chart on table X sees exactly the items bound to X; save-time validation parses the expression's columns against that table. Simple, siloed.
- **B) Table matching + relationship graph** — chart on X offers items from X **and tables related via the table-relationships model** (existing PRD); picking an item from a related table makes the chart's query auto-`JOIN` along the declared relationship (`bookings ⋈ clients`). Qlik-association feel.
- **C) Inference-first** — no explicit binding; parse expressions and match against all schemas (ambiguities → user resolves). Most "smart", least predictable — renames or shared column names shift associations silently.

**Assistant lean: B** — it reuses the table-relationships PRD as the source of truth for "smart" instead of inventing inference magic, and keeps the explicit binding greppable in the spec.

## Consequences

- The master library schema needs at minimum: `{ id, label, expr, fmt, table }`.
- Table-relationships (`.prds/table-relationships`) becomes a load-bearing dependency of chart item availability if B is picked.
- Chart SQL generation gains a JOIN-planning responsibility for cross-table items under B.

## Source

- Design interview turn, 2026-09-15 (user's Q6 answer "A + table binding with smart logic"; Q7 posed with lean B).
