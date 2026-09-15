---
type: Learning
title: SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse
description: "Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordinal scales **sort th"
tags: [svelteplot, charts, labs, ordinal-scale]
timestamp: "2026-09-14T10:15:13.077Z"
---

# SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse

Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordinal scales **sort the domain alphabetically by default** (svelteplot source, `Plot.scale` domain building, ~line 187). Data order is irrelevant unless you take control of the domain.

## What to do

- **Order the categories explicitly**: `y={{ domain: [...yourOrder] }}` — data order is otherwise discarded.
- Or flip the range: `reverse: true` (supported; flips the range so domain[0] renders topmost on the y-axis).
- Bar gaps: default band **padding is 0.15**; bump to ~0.3 for clearly separated bars.

## Gotcha

If sorting "doesn't work" on an svelteplot bar/axis, suspect silent alphabetical domain sorting first — check that an explicit `domain` or `reverse` is set.

Applies to [[barchart-component]] (`src/lib/components/charts/BarChart.svelte`) and any [[labs-per-chart-type]] chart using ordinal scales. Related: [[svelteplot-barx-bar-y-orientation]], [[svelteplot-datum-identity-empty-guard]].
