---
type: Decision
title: "Q10 locked: chart selections cross-filter other charts via re-query (Qlik-style, transient)"
description: Context
tags: [chart-spec, selection, cross-filter, interview, report-pages]
status: accepted
timestamp: "2026-09-15T08:42:11.113Z"
---

# Q10 locked: chart selections cross-filter other charts via re-query (Qlik-style, transient)

## Context

Part of the ongoing Q&A locking the central reusable-chart / report-page spec design (Q3, Q5, Q6, Q7, Q8 already locked). Q10 asked how chart selections behave. User answered **B** (2026-09-15).

## Decision

- **Selection on a chart cross-filters the OTHER charts** on the page by **re-querying** the data with the selection applied as a filter — Qlik-style.
- **The chart that owns the selection excludes its own selection from its own filter** (so it keeps showing the full distribution; other charts narrow).
- **Selections are transient exploration state, never persisted** in the chart/report spec. Saved selections were explicitly deferred ("flag me if you want 'saved selections' someday").

## Alternatives / ruled out

- Selection that only highlights within its own chart (no cross-chart filtering) — rejected.
- Persisted/saved selections as part of the spec — rejected for now; keeps the spec purely about structure, not exploration state.

## Consequences

- Cross-filtering runs through [[q8-locked-one-canonical-query-engine-with-per-type-hooks]] — every chart re-queries on selection change; no client-side filtering layer to invent.
- With [[q7-locked-relationship-graph-drives-chart-item-availability-]] auto-JOIN, a selection on one table's field filters charts joined from related tables via the relationship graph.
- Selection state lives outside the serializable spec (page-level runtime state), keeping the two-surface code/no-code parity ([[two-surface-report-page-format]]) clean.

## Related open question

Q11 (tooltips: declarative fields + optional template string vs. snippet hatch) posed same day, not yet locked.
