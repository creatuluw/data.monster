---
type: Decision
title: "Two-surface report pages: code mode edits a declarative spec, not Svelte source"
description: "Q3 LOCKED (A): the report page is one declarative spec document edited by both surfaces — parity by construction. C (registry escape hatch) stays a future growth path."
tags: [charts, pages, architecture, interview, code-no-code]
status: accepted
timestamp: "2026-09-15T08:16:22.375Z"
---

# Two-surface report pages: code mode edits a declarative spec, not Svelte source

## Context

Interview (2026-09-15), Q3. User requirement: charts must be addable to a report page from **two interchangeable surfaces** — code mode and no-code mode — and either surface can **create/edit the same page** with identical chart appearance, design, and logic. This is the Evidence model itself: one document that both the built-in editor and VS Code edit. It extends the chart-level requirement in [[chart-authoring-two-surfaces-serializable-spec]] to the whole report page.

This forces the central decision of the entire two-surface design: **what the "code" in code mode actually is**, because that defines the parity contract.

## The options presented (Q3)

1. **A — Constrained declarative spec**: the page is a document (JSON / JSON5 / small DSL) holding a list of chart blocks (`{ type, data: { table, x, y, series }, title, options }`). Code mode edits the text; no-code mode edits the same document via UI. **Perfect parity by construction** — nothing expressible in code lacks a UI representation (worst case the UI falls back to a raw JSON field, Evidence's `echarts_options` approach).
2. **B — Real Svelte markup**: code mode writes actual `<Chart>` components with snippets/functions/full JS. Maximum power, but no-code cannot round-trip arbitrary code — the page becomes partially UI-editable; unrepresentable parts render read-only or get clobbered.
3. **C — A + registry escape hatch**: declarative spec as the page format, plus custom blocks referencing registered components by name (`{ type: "custom:myRadar" }`). Custom *visuals* stay code-authored in app source; any *instance* stays UI-configurable via its declared props schema. Evidence has no equivalent — this is the "32-chart catalog as the registry" play (see [[labs-catalog-placeholder-first]]).

## Recommendation (assistant lean — user answer pending)

**A for the page/document format, C as the growth path** — C is A plus registration discipline, which the catalog already implies.

## Rationale

- Parity is a hard user requirement; only A guarantees it by construction rather than by ongoing enforcement.
- B permanently breaks the parity contract the user just stated.
- C preserves A's guarantees for built-in types while giving power users a code escape hatch that instances of still round-trip.

## Consequences

- Page persistence format must be a serializable document, not Svelte source (aligns with [[evidence-chart-architecture]] and [[chart-fundament]]).
- If C is adopted later, chart components need declared props schemas so the no-code surface can configure custom-block instances.

## Data binding (Q4 — answered 2026-09-15)

**User chose B, refined: each chart carries its own template query whose dimension/measure slots are filled dynamically** by what the chart declares — the query always aggregates exactly what the chart needs (a mini semantic layer per chart):

```sql
SELECT {{dimensions}}, {{measures}}
FROM bookings
GROUP BY {{dimensions}}
ORDER BY {{measures_desc}}
```

with the chart declaring `dimensions: ["month"]`, `measures: [{ col: "hours", agg: "sum" }]` → `sum(hours) AS hours`.

**Open follow-up (Q5)** — template *ownership*, which decides how much SQL users ever see:

- **A** per chart instance — each chart block stores its own template text + bindings (max SQL freedom)
- **B** per chart type — the 32-type registry ships each type's standard aggregation template; an instance declares only `table`, `dimensions`, `measures`, `filters` (no per-chart SQL exists)
- **C** layered — registry default per type + optional per-instance override (A's escape hatch, still a serializable string + slots)

Sub-point pending confirmation: aggregation is declared **on the measure** (`{ col, agg }`), never written as free text.

## Status

**Accepted — user locked A (2026-09-15).** The page is a declarative spec document; both surfaces edit it; parity is guaranteed by construction. C (registered custom-block escape hatch) was not chosen now but remains the natural growth path if power users need it.

Q4 locked (2026-09-15): chart blocks bind data via per-chart template queries with dynamic dimension/measure slots — see "Data binding (Q4)" above. Q5 (template ownership: instance / registry / layered) is the open question.