---
type: Decision
title: "Q15 locked: reference annotations are a declarative list; reference_line only in v1"
description: Context
tags: [charts, spec-design, annotations, q15]
status: accepted
timestamp: "2026-09-15T08:57:57.725Z"
---

# Q15 locked: reference annotations are a declarative list; reference_line only in v1

## Context

Question 15 in the chart/report spec design interview (series that locked Q3, Q5–Q13). Evidence.dev charts get annotations via `reference_line` / `reference_area` / `reference_point` children — the classic "one chart needs a target line" case. We had to decide whether annotations enter the declarative spec in v1, and if so how.

## Choice (B — reference_line only, now)

Chart spec gets an `annotations` list, v1 entry `reference_line`:

```json
"annotations": [{ "type": "line", "on": "y", "at": <measure expr or number>, "label": "Target", "color": null }]
```

- The **engine computes the value** — `at` can be a master-measure expression (e.g. *avg hours target*) or a literal number.
- Every chart type declares **how it draws a y-line via its registry mark** (~1 line for most cartesian charts; heatmap/pie opt out).
- Serializes trivially → code-mode/UI-mode parity by construction (consistent with [[two-surface-report-page-format]]).

## Alternatives considered

- **A — Defer**: not in v1; a chart needing a target line waits.
- **C — Full set now** (line + area band + point highlight): complete Evidence parity, but three render paths per chart type up front.

## Rationale

The mechanism is the real design: an annotation list + engine-evaluated value + per-type draw hook. `area` and `point` later are just new entries in the same list — no new architecture. B builds the mechanism without paying C's per-type render cost for features nobody has asked for yet.

## Consequences

- Spec carries a typed `annotations` array; the block/chart registry needs a "draw a y-line" hook per chart type.
- Non-cartesian types (heatmap, pie) must explicitly opt out.
- `area`/`point` annotations slot in later as new `type` values.

## Related

- Extends [[measures-dimensions-are-duckdb-expressions]] — `at` accepts a master-measure expression, evaluated by the engine.
- Extends [[q13-explicit-grid-rows-blocks-take-col-spans]] — annotations live inside a chart block's spec.
- Same serialization-first posture as [[two-surface-report-page-format]].
