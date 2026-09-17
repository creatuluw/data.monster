---
type: Decision
title: "Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)"
description: "Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)"
tags: [chart-spec, layout, grid, interview, report-pages]
status: accepted
timestamp: "2026-09-15T08:56:45.848Z"
---

# Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)

# Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)

## Context

Part of the ongoing Q&A locking the central reusable-chart / report-page spec design (Q3, Q5–Q12 already locked). Q13 asked the page layout model — flow with `width` per block (Evidence.dev model, assistant's lean) vs. explicit 12-col grid vs. free canvas. User answered **B** (2026-09-15).

## Decision

- The report page is an **explicit grid**: the spec declares **rows**, and every block carries a **column span on a 12-column grid** (e.g. `rows: [ { blocks: [ { span: 6, ...blockSpec } ] } ]`).
- Layout is fully declared in the spec — no auto-placement inference and no free-form positioning.

## Alternatives / ruled out

- **Flow layout with `width` per block** (Evidence.dev model) — ruled out; implicit row breaking is harder to reason about and round-trip in spec text than explicit rows.
- **Free canvas** — ruled out; absolute positioning bloats the spec and fights two-surface parity ([[two-surface-report-page-format]]).

## Consequences

- `span` is a declarative field on the block, so both surfaces (code editor + no-code UI) produce identical layout from the same spec.
- Sits on top of [[q12-page-level-consistent-colors-shared-scale]]: the page layer now owns both grid structure and the shared color scale.
- Sets up Q14: what the blocks are (charts only vs. a general block envelope with a registry).

## Related open question

Q14 posed same turn, not yet locked: is the page a list of charts (v1) or a general block system from day one — every cell `{ type, span, props }` resolved via a **block registry** (`chart` wrapping a chart-type spec + `text` today; `table`, `kpi`, `image` later) — or the full set built now. Assistant leans the block envelope (B).
