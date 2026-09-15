---
type: Decision
title: "Q12 locked: page-level consistent colors via shared scale, optional manual overrides in spec"
description: Context
tags: [chart-spec, colors, interview, report-pages]
status: accepted
timestamp: "2026-09-15T08:55:45.302Z"
---

# Q12 locked: page-level consistent colors via shared scale, optional manual overrides in spec

## Context

Part of the ongoing Q&A locking the central reusable-chart / report-page spec design (Q3, Q5–Q11 already locked). Q12 asked how chart colors work: per-chart palettes vs. page-consistent shared scale vs. master-dimension value colors. User answered **B** (2026-09-15).

## Decision

- **Colors are consistent at page level via a shared scale**: charts on a report page that encode the same dimension share one color scale, so the same category is the same color across every chart on the page.
- The spec carries **optional manual color overrides** — defaults come from the shared scale, and a chart (or item) can pin an explicit color when the user wants manual control.

## Alternatives / ruled out

- **Per-chart colors** (each chart picks its own palette) — ruled out; breaks cross-chart readability, the point of putting several charts on one page.
- **Master-dimension value colors** (value→color mapping stored on master items, Qlik-style) — deferred/ruled out for now; heavier metadata commitment than the current master-item model ([[master-item-library-table-binding-q6]]) warrants.

## Consequences

- The shared scale lives at the report-page layer, not per-chart — chart-type components consume resolved colors rather than each owning a palette.
- Overrides stay declarative in the spec, preserving two-surface parity ([[two-surface-report-page-format]]) and serializability.
- Complements the declarative-config pattern of [[q11-locked-tooltips-are-declarative-fields-one-template-stri]] and [[q9-locked-chart-option-panels-are-schema-driven-with-a-custo]]: defaults are automatic, manual control is fields in the spec.

## Related open question

Q13 posed same day, not yet locked: page layout model — flow with `width` per block (Evidence model, assistant's lean) vs. explicit 12-col grid vs. free canvas.
