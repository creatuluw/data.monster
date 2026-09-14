---
type: Entity
title: BarChart component
description: What is it?
tags: [charts, svelteplot, labs]
timestamp: "2026-09-14T08:29:28.935Z"
---

# BarChart component

## What is it?

Reusable SveltePlot-based bar chart component (`BarChart.svelte`), generic over the caller's row type. The second `/labs` chart-type card (after [heatmap-component](./heatmap-component.md)), built on the [chart-fundament](./chart-fundament.md).

## Why it matters?

Second instance of the reusable-chart contract — proves the pattern generalizes past heatmaps. Aggregation runs through `buildBars` (top-N + Other bucket), so the component is a thin renderer.

## Details

- **Location**: `src/lib/components/charts/BarChart.svelte`; playground route `src/routes/labs/bar-chart/`; card in `src/routes/labs/+page.svelte`
- **Interface**: accessor props (`category`, `value` accessors, generic `<T>`), bindable selection (matched via `sameDatum`, not identity), tooltip snippet, mono data labels, empty-data guard
- **Configuration**: optional `topN`/`otherLabel` aggregation opts passed to `buildBars`
- Type gotchas solved during build: Svelte 5 generic needs a `type` alias (not `interface`) for the implicit index signature; svelteplot `tickFormat` wants a `TickFormatFunction` (RawValue param) — wrap typed props

## Relationships

- [chart-fundament](./chart-fundament.md) — supplies `buildBars` + `sameDatum`
- [heatmap-component](./heatmap-component.md) — sibling, first chart on the same contract
- [labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md) — the rule both follow

## Lifecycle

- First added: 2026-09-14 via PR creatuluw/data.monster#1 (commit `62a9162`), TDD-first
