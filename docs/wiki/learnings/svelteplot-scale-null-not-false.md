---
type: Learning
title: "SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale"
description: Symptom
tags: [svelteplot, charts, labs, gotcha]
timestamp: "2026-09-15T06:49:08.558Z"
---

# SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale

## Symptom

In `/labs/bar-chart`, per-bar selection colors rendered wrong: selecting the top bar made it blue, any other bar orange — instead of the intended literal colors (`#888888` deselected, DS green selected).

## Root cause

The BarChart's `BarX` fill channel was declared as `fill={{ scale: false, value: fillOf }}`. SveltePlot 0.14.2's `getUsedScales` checks `scale !== null` — so `false` does **not** bypass the scale. The literal color strings returned by `fillOf` leaked into the default categorical color scale and got remapped to palette colors (hence blue/orange).

## Fix

Only `scale: null` bypasses svelteplot's scales:

```svelte
fill={{ scale: null, value: (d: any) => fillOf(d as T) }}
```

The [[heatmap-component]] already used `scale: null` for its stroke and in-cell Text fill channels — that's why it never showed the bug.

## Applies to

Any literal per-datum color in a svelteplot mark channel (`fill`, `stroke`, …) that must NOT be scaled: declare `scale: null`, never `scale: false`.

- Fixed 2026-09-14 in `src/lib/components/charts/BarChart.svelte` (also switched deselected color to `#888888`, selected to DS green `oklch(0.44 0.1 158)`).
- Related engine gotchas: [[svelteplot-datum-identity-empty-guard]], [[svelteplot-ordinal-domain-sorts-alphabetically]]
