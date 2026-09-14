---
type: Entity
title: Chart fundament
description: What is it?
tags: [charts, svelteplot, testing]
timestamp: "2026-09-14T08:29:20.418Z"
---

# Chart fundament

## What is it?

The shared, pure-TypeScript core under every `/labs` chart component. Data prep (aggregation, sorting, selection matching) lives here as tested pure functions; chart components stay thin renderers. Enforces the conventions: accessor-based props API (generic over the caller's row type) and selection matched by grid/category position, never object identity (svelteplot re-copies records on each transform pass).

## Why it matters?

It is the concrete implementation of the [labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md) rule — every chart type card (heatmap, bar chart, future ones) is a production-chart candidate by construction. Also carries the project's first test infra: vitest (devDep + `npm test` script).

## Details

- **Location**: `src/lib/charts/fundament.ts`, tests in `src/lib/charts/fundament.test.ts` (9 tests, TDD red→green)
- **Interface**: `sameDatum<T>(keys) => (a, b) => boolean` — positional datum matcher for selection binding; `buildBars<T>(rows, category, value, opts?) => BarDatum[]` — sum/sort aggregation with optional `topN` cap + `Other` bucket
- **Configuration**: none — pure functions

## Relationships

- [bar-chart-component](./barchart-component.md) — first consumer of `buildBars`
- [heatmap-component](./heatmap-component.md) — first component following the contract
- [labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md) — the rule this implements

## Lifecycle

- First added: 2026-09-14, built TDD-first alongside the bar chart lab (PR creatuluw/data.monster#1)
