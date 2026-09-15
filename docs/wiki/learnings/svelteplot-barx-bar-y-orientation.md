---
type: Learning
title: "SveltePlot BarX vs BarY: BarX is the horizontal bar mark"
description: "Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Bars"):"
tags: [svelteplot, charts, labs]
timestamp: "2026-09-14T10:11:55.083Z"
---

# SveltePlot BarX vs BarY: BarX is the horizontal bar mark

Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Bars"):

- **`BarX` renders horizontal bars** — the measure runs along x, the dimension/band axis is y. `BarY` is the vertical counterpart. The suffix names the measure axis, not the category axis, so "X" ≠ "category on x".
- When flipping orientation, the axis-scoped bits move with it: number-axis `tickFormat` and grid move to `AxisX`, the tooltip anchor swaps sides, and ascending row order puts the largest bar on top for horizontal bars.
- Component had no consumers at flip time, so a direct `BarY`→`BarX` edit was safe — no orientation prop needed (YAGNI until a second consumer disagrees).

Related engine gotchas: [[svelteplot-datum-identity-empty-guard]]. Component: [[barchart-component]].
