---
type: Entity
title: BarChart component
description: What is it?
tags: [charts, svelteplot, labs]
timestamp: "2026-09-15T07:33:47.551Z"
---

# BarChart component

## What is it?

Reusable SveltePlot-based bar chart component (`BarChart.svelte`), generic over the caller's row type. The second `/labs` chart-type built (after [heatmap-component](./heatmap-component.md)) — its card now leads the reordered Labs grid (Barchart → Line → Area → Scatter → Heatmap → …). Built on the [chart-fundament](./chart-fundament.md).

## Why it matters?

Second instance of the reusable-chart contract — proves the pattern generalizes past heatmaps. Aggregation runs through `buildBars` (top-N + Other bucket), so the component is a thin renderer.

## Details

- **Location**: `src/lib/components/charts/BarChart.svelte`; playground route `src/routes/labs/bar-chart/`; card in `src/routes/labs/+page.svelte`
- **Orientation**: horizontal — `BarX` mark, measure on the x number axis (grid + `tickFormat` via the renamed `formatValue` prop), dimension on the y band axis (`padding: 0.3`). The y domain is built explicitly from data order with `reverse: true`, so the caller's sort wins over svelteplot's alphabetical ordinal default (see [svelteplot-ordinal-domain-sorts-alphabetically](../../learnings/svelteplot-ordinal-domain-sorts-alphabetically.md)).
- **Interface**: accessor props (`category`, `value` accessors, generic `<T>`), bindable selection (matched via `sameDatum`, not identity), tooltip snippet (anchored x=value, y=category), mono data labels, empty-data guard
- **Config panel**: optional `config` snippet prop — when passed, a `<Bolt />` button at the top-right of the chart card toggles the chart's own [chartconfigdrawer-component](./chartconfigdrawer-component.md) (title `<title> configuration`); the page renders its fields via `{#snippet config()}` (see [each-labs-chart-owns-its-config-panel](../../rules/each-labs-chart-owns-its-config-panel.md))
- **Selection label**: inline right of the card title, vertically centered, `·` divider — `Booked hours per month · May 25 · 122h` (no floating pill). Card click-to-deselect ignores `button` clicks so toggling config doesn't clear the selection
- **Styling**: bars `#888888` deselected (`color` prop); selected bar fills DS green `oklch(0.44 0.1 158)` via `fillOf`. The fill channel passes `scale: null` so the literal color strings bypass svelteplot's color scale — `scale: false` does NOT bypass it (see [svelteplot-scale-bypass-needs-scale-null](../../learnings/svelteplot-scale-null-not-false.md))
- **Height**: `plotHeight = innerHeight * heightVh` — default `heightVh = 0.3` (30vh), overridable per call (e.g. `<BarChart heightVh={0.5}>`); same rule as the heatmap
- **Configuration**: optional `topN`/`otherLabel` aggregation opts passed to `buildBars`
- Type gotchas solved during build: Svelte 5 generic needs a `type` alias (not `interface`) for the implicit index signature; svelteplot `tickFormat` wants a `TickFormatFunction` (RawValue param) — wrap typed props

## Relationships

- [chart-fundament](./chart-fundament.md) — supplies `buildBars` + `sameDatum`
- [heatmap-component](./heatmap-component.md) — sibling, first chart on the same contract
- [chartconfigdrawer-component](./chartconfigdrawer-component.md) — hosted per-chart via the `config` snippet + Bolt toggle
- [labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md) — the rule both follow

## Lifecycle

- First added: 2026-09-14 via PR creatuluw/data.monster#1 (commit `62a9162`), TDD-first
- 2026-09-14: playground page rebuilt as a structural mirror of the heatmap page (everything inline, only the component imported — no `$lib/charts` import from the page) to fix the vite reload-loop hang (PR creatuluw/data.monster#3, commit `9f18749`; see [labs-hang-vite-reload-loop](../../learnings/labs-hang-vite-reload-loop.md))
- 2026-09-14: flipped from vertical BarY to horizontal BarX (see [svelteplot-barx-bar-y-orientation](../../learnings/svelteplot-barx-bar-y-orientation.md)); playground now charts real synthetic data — 12 months of booked hours, one dimension + one measure, sorted desc so the longest bar lands topmost
- 2026-09-14: selection colors fixed — deselected `#888888`, selected DS green; `scale: false` had leaked the fill strings into svelteplot's default categorical scale (rendered blue/orange). the heatmap adopted this same height rule
- 2026-09-15: height switched from the width-derived 2:1 aspect (never below 25vh) to a fixed viewport fraction — `heightVh` prop, default 0.3 (30vh), in both charts
- 2026-09-15: config moved into the chart — optional `config` snippet + `<Bolt />` toggle top-right of the card, page-level Configure button/drawer removed; selection label moved inline next to the title with a `·` divider; click-deselect now ignores `button` clicks
