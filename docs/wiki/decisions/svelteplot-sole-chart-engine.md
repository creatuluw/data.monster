---
type: Decision
title: SveltePlot is the sole chart engine — all legacy chart libraries removed
description: Context
tags: [charts, svelteplot, dependencies]
status: accepted
supersedes: decisions/consolidate-chart-engines-to-picasso-js
timestamp: "2026-09-14T08:29:20.417Z"
---

# SveltePlot is the sole chart engine — all legacy chart libraries removed

## Context

The 2026-09-12 decision (consolidate to Picasso.js + LayerChart) left several engines installed: unovis, picasso.js, layerchart, observable-plot, echarts remnants, plus svelteplot for Labs. During the bar-chart lab build (2026-09-14), `npm install` was broken by a peer conflict (`@unovis/svelte` vs Svelte 5). The user then directed: only svelteplot is needed, now and in the future.

## Decision

**svelteplot is the sole chart engine.** All legacy chart libraries are removed: unovis, picasso.js, layerchart, observable-plot, and echarts. The old `src/lib/charts/` Picasso-era lib (`{group, value}` model) is deleted; charts now build on the [[chart-fundament]] accessor pattern instead.

## What was checked before cutting

- `/pages` (product route) imports only the pure-SVG `src/lib/components/BarChart.svelte` — unaffected.
- All other old-lib consumers were doomed experiment routes — deleted with the libs.
- Build green after removal; the `@unovis/svelte` peer conflict that blocked `npm install` is gone.

## Consequences

- One engine, one component contract: every new chart is a SveltePlot component on the [[chart-fundament]] (accessor props, bindable selection, tooltip snippet, mono labels, empty guard).
- Deps shrink; installs work again.
- Supersedes the Picasso+LayerChart consolidation — that stack is no longer in the codebase.

Source: PR creatuluw/data.monster#1 (commits incl. `62a9162`).
