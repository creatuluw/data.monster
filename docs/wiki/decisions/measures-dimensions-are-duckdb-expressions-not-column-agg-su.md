---
type: Decision
title: Measures/dimensions are DuckDB expressions, not column+agg sugar (Q5, settled)
description: Context
tags: [chart-spec, report-pages, measures, dimensions, qlik]
status: accepted
timestamp: "2026-09-15T08:28:20.004Z"
---

# Measures/dimensions are DuckDB expressions, not column+agg sugar (Q5, settled)

## Context

During the 2026-09-15 report-page/chart-spec design interview, the user asked for Qlik Sense-style analysis: expressions like `sum(if(type = 'class', hours))` instead of picking a column + aggregation, plus reusable **master dimensions/measures** they define once and customize, then add to any chart.

Related: [[two-surface-report-page-format]] (the spec document this feeds into) and [[chart-authoring-two-surfaces-serializable-spec]].

## Choice

**Measures and dimensions are arbitrary DuckDB expression strings, not `{col, agg}` sugar.**

- A measure = `{ label: "Class hours", expr: "sum(if(type = 'class', hours))", fmt: "..." }` — any valid DuckDB expression.
- The UI's column/agg dropdowns are **autocomplete sugar that writes expressions**; the stored chart spec always holds the expression string.
- Chart templates slot expressions into generated SQL: `SELECT {{dimensions}}, {{measures}} ... GROUP BY {{dimensions}}`.

## Alternatives considered

- Structured `{column, aggregation}` model — rejected: cannot express conditionals, arithmetic, or multi-column measures like `sum(if(...))`; the user explicitly needs them.
- Expression-only UI (no dropdowns) — rejected as UX: dropdowns stay as sugar, but the persisted model is the expression.

## Consequences

- The serializable chart spec stores expression strings; validity is whatever DuckDB accepts.
- Master items (reusable dimensions/measures) become named `{label, expr, fmt}` records — same shape, referenced by charts.
- **Q6 open**: where the master-item library lives — (A) workspace-level in internal DB (`d8a_monster_master_items`, referenced by stable id), (B) page-level inside the page spec, or (C) both with a promotion flow (page-local → workspace), resolving page-local first. Pair A/C with **stable id** references so renames propagate and deletion surfaces a "missing master item" state instead of breaking silently.

## Source

- Design interview turn, 2026-09-15 (user request for Qlik-style measures/dimensions and master items).
