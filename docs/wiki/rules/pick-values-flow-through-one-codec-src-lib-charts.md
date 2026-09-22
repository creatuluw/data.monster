---
type: Rule
title: Pick values flow through one codec — src/lib/charts/pickers.ts
description: Guideline
tags: [central-charts, page-editor, code-reuse]
timestamp: "2026-09-17T15:32:10.396Z"
---

# Pick values flow through one codec — src/lib/charts/pickers.ts

## Guideline

Any surface that turns a dimension/measure picker selection into a chart-spec entry (or back) must use the pure codec in `src/lib/charts/pickers.ts` — `dimensionFromPick` / `dimensionPickValue` / `measureFromPick` / `measurePickValue`. The wire format is `ref:<id>` (master item) | `col:<table>:<col>` | `field:<table>:<col>`, and the linked-table measure fallback (`sum("table"."col")` + `table`) lives there too.

## When it applies

Adding any third surface that edits chart roles (e.g. a /library demo editor, a bulk-edit panel). Never re-implement the string format in a component.

## Rationale

The block inspector (drawer) and the in-chart [[skeletonsetup-component]] both encode/decode the same pick strings; one module (8 unit tests, `tests/pickers.test.ts`) is the only way to keep them from drifting. Introduced PR #10.
