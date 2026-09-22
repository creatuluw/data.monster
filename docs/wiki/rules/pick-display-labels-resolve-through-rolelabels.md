---
type: Rule
title: Pick display labels resolve through roleLabels() — never hand-roll chip labels
description: Guideline
tags: [central-charts, page-editor, code-reuse]
timestamp: "2026-09-17T16:23:09.422Z"
---

# Pick display labels resolve through roleLabels() — never hand-roll chip labels

## Guideline

Any surface that shows **which dimensions/measures are already in a chart** (chips, "already in chart" summaries, inspector lists) must get display labels from the shared `roleLabels()` resolver in `src/lib/charts/pickers.ts` — never hand-roll label formatting in a component.

## When it applies

Every renderer of chart roles: the skeleton card's chips (dark pills = dimensions, light pills = measures), the pick/create modal's header summary, and any future surface (block inspector, /library demo editor, bulk edit).

## Rationale

One resolver resolves `ref:` / `col:` / `field:` picks into human labels (⭐ master labels, plain columns, expressions, linked tables flagged) so all surfaces stay consistent — same reason the [[pick-values-flow-through-one-codec-src-lib-charts|pick-value codec]] is centralized. Introduced PR #13 (visibility fix: users couldn't see what they'd already added while picking). Covered by `tests/pickers.test.ts` (2 label tests).
