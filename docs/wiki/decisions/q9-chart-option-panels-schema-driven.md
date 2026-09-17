---
type: Decision
title: "Q9 locked: chart option panels are schema-driven with a custom-panel hatch"
description: Context
tags: [chart-spec, report-pages, options-schema, config-panel, two-surface, labs]
status: accepted
timestamp: "2026-09-15T08:42:03.695Z"
---

# Q9 locked: chart option panels are schema-driven with a custom-panel hatch

## Context

Continuation of the 2026-09-15 chart-spec/report-page design interview. Q8 ([[q8-one-canonical-query-engine-per-type-hooks]]) left **Q9 open**: how per-type chart options reach the no-code UI — hand-built panels per type (the current [[each-labs-chart-owns-its-config-panel]] rule), schema-driven panels auto-rendered from a registry options schema, or B + a custom-panel escape hatch.

The user answered **C**.

## Choice

**C locked — schema-driven options + custom-panel hatch.** Each chart type's options are declared once in its registry options schema (field name, kind, default, min/max); the option panel is auto-rendered from that schema, and code mode edits the same schema as JSON (unknown field → validation error). When the generated panel can't express a chart's needs, the chart supplies a custom panel — but the schema **still declares the field**, so both surfaces (code + no-code) stay consistent.

## Alternatives considered

- **A) Hand-built panels per type** — 32 bespoke panels, maximum freedom, guaranteed drift between code mode and UI mode.
- **B) Pure schema-driven, no hatch** — consistent and cheap, but the first chart needing bespoke UI forces a fork.

## Consequences

- The options schema becomes part of each chart type's contract alongside its role schema (from Q8) — one declaration drives rendering, the option panel, and code-mode JSON.
- Refines (does not supersede) [[each-labs-chart-owns-its-config-panel]]: ownership stays per-chart; panel construction is schema-generated, custom panel is the exception.
- Keeps [[two-surface-report-page-format]] parity by construction: schema is the single source both surfaces edit.

## Related open question

**Q10 (open, lean B)** — what a click/selection does on a published page: (A) local highlight only, (B) **cross-filter** — selection becomes a `WHERE` on the dimension, other charts re-query DuckDB (Qlik-style, the payoff of the Q8 query engine), (C) B + Evidence-style `sync` (linked tooltips/crosshair/zoom). Evidence separates `sync` (linked visuals) from `filters` (propagation) — useful distinction for when Q10 locks.

## Source

- Design interview turn, 2026-09-15 (user's Q9 answer "C"; Q10 posed with lean B).
