---
type: Decision
title: Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot
description: Context
tags: [charts, labs, picasso, layerchart]
status: accepted
timestamp: "2026-09-12T11:06:03.726Z"
---

# Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot

## Context

Labs (`/labs`) originally hosted five chart-engine experiments: ECharts, Observable (Plot), SveltePlot, Picasso.js, and chart-lib (LayerChart). Each only had a bar chart, so comparisons were shallow and maintenance was spread across five stacks (~2,900 lines of lab code for three of them alone).

## Decision

Consolidate down to **two engines**:

1. **Picasso.js** (`/labs/picasso-charts`)
2. **LayerChart / chart-lib** (`/labs/chart-lib`)

Deleted entirely: `echarts-charts/`, `observable-charts/`, `svelteplot-charts/` routes and `ObservableBarChartCanvas.svelte`. Both surviving engines now cover **four chart types each** — bar, line/area, pie/donut, scatter — via paired canvas components (`PicassoBarChartCanvas`, `PicassoLineAreaChartCanvas`, `PicassoPieDonutCanvas`, `PicassoScatterCanvas` and the non-prefixed LayerChart equivalents) exported from `src/lib/charts/index.ts`.

## Alternatives considered

- Keep all five engines with one chart type each — rejected: shallow comparisons, 5× maintenance.
- Pick one engine now — rejected: the Labs comparison between Picasso.js (canvas control, custom interaction) and LayerChart (Svelte-native composition) is still the point of the experiment.

## Consequences

- ~3,400 lines of lab/chart code removed; new chart types added to both survivors.
- `docs/src/lib/charts/svelteplot/` reference components also deleted.
- Future chart-type work goes into both engines in parallel so the comparison stays fair.
