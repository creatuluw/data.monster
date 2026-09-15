---
type: Learning
title: "Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy"
description: Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/components/scatter_chart and
tags: [charts, svelteplot, evidence, design, labs]
timestamp: "2026-09-15T08:03:14.305Z"
---

# Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy

Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/components/scatter_chart and /core-concepts/components first).

## Key architectural insight

Evidence does **not** have one god chart component. It has one typed component **per chart type** (`scatter_chart`, `bar_chart`, …), each built over shared machinery (query builder + ECharts + shared prop vocabulary). Consistency lives in a **standardized attribute taxonomy**, not in a single `<Chart type=…>` component. This is the reference model for our central chart design: shared shell + shared prop types + one thin component per chart type (proposed default (b) in the interview; god components rot with 32 chart types).

## Evidence's prop taxonomy (distilled)

- **Data binding**: `data` (table), `x`/`y`/`y2` columns, `series`, `size` (scatter), `sort`, `limit`, filters — aggregation is part of the binding (`x="avg(avg_transaction_value)"`, in-query)
- **Title block**: `title`, `subtitle`, `info`/`info_link` — every component has it
- **Value formatting**: `x_fmt`, `y_fmt`, per-label `fmt` — one shared format vocabulary
- **Axis options**: typed option groups (`title, ticks, baseline, labels, gridlines, min, max, fit_to_data, interval, label_rotate`) — same shape for x/y/y2
- **Legend**: `legend`, `legend_position`, `series_order`
- **Styling**: `chart_options` (palette, series colors, zoom), `opacity`, and `echarts_options` — a raw escape hatch deep-merged over the config
- **Data labels**: position/fmt/size/rotate/show_overlap
- **Missing values**: `connect | gaps | zero` (time-series semantics)
- **Overlays**: `reference_line` / `reference_area` / `reference_point` as composable children
- **Cross-chart linking**: shared `link` id syncs tooltips/zoom across charts

## Where we already stand

[[chart-fundament]] (buildBars + sameDatum), generic accessor props `<T>`, bindable positional selection, tooltip snippet, empty guard, `heightVh`, and the per-chart config drawer rule ([[each-labs-chart-owns-its-config-panel]], [[labs-charts-reusable-fundament]]) already cover roughly 60% of this taxonomy — it just isn't written down as one central design yet. The pending interview decides: shared `ChartShell` + shared `AxisOptions`/`fmt`/`seriesColors` vocabulary vs per-chart props; centralized `aggregate()` fundament API (opt-in for scatter/line); single-select as standard.

Related: [[barchart-component]], [[heatmap-component]].
