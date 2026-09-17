---
type: Decision
title: "Q11 locked: tooltips are declarative fields + one template string, shared renderer per chart type"
description: Context
tags: [chart-spec, tooltip, interview, report-pages]
status: accepted
timestamp: "2026-09-15T08:53:44.610Z"
---

# Q11 locked: tooltips are declarative fields + one template string, shared renderer per chart type

## Context

Part of the ongoing Q&A locking the central reusable-chart / report-page spec design (Q3, Q5, Q6, Q7, Q8, Q9, Q10 already locked). Q11 asked how tooltips work. User answered **B** (2026-09-15).

## Decision

- **Tooltips are declarative**: the chart spec carries tooltip config as declarative fields (which values to show) plus **one optional template string** for custom formatting.
- **One shared tooltip renderer per chart type** — the chart-type component owns a single tooltip renderer that consumes those fields. No per-chart custom tooltip code in the spec layer.

## Alternatives / ruled out

- **Snippet hatch** (custom tooltip code/snippet per chart) — ruled out; would break serializability and the two-surface parity ([[two-surface-report-page-format]]). Custom rendering stays in shared per-type component code, not the spec.
- Note: the existing `/labs` chart components use a tooltip *snippet* pattern at the component level ([[chart-fundament]]) — that stays; this decision governs the report-page spec layer on top.

## Consequences

- Tooltip config serializes into the declarative chart spec — editable from both surfaces ([[two-surface-report-page-format]]).
- Consistent with [[q9-chart-option-panels-schema-driven]]: schema-driven config with custom logic living in shared component code, not in specs.
- Template string is the single escape hatch for formatting — keeps 99% of tooltips pure fields.

## Related open question

Q12 (chart colors: per-chart vs. page-consistent shared scale vs. master-dimension value colors) posed same day, not yet locked.
