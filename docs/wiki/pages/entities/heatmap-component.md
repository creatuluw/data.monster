---
type: Entity
title: Heatmap component
description: Reusable SveltePlot-based heatmap component (generic `<T>`, cell grid with threshold colors), ported 2026-09-14 from the kees.pippeloi.nl reference. First compo
tags: [charts, labs, svelteplot]
timestamp: "2026-09-14T07:26:30.635Z"
---

# Heatmap component

Reusable SveltePlot-based heatmap component (generic `<T>`, cell grid with threshold colors), ported 2026-09-14 from the kees.pippeloi.nl reference. First component of the per-chart-type Labs initiative — see [labs-per-chart-type](../../decisions/labs-per-chart-type.md).

## Details

- **Location**: `src/lib/components/Heatmap.svelte`; playground at `src/routes/labs/heatmap/+page.svelte`
- **Interface**: props `data, x, y, value, threshold, scheme (orrd default), isEmpty, tooltip, labelFor, label, selected ($bindable), xTicks, formatX…`; built on svelteplot `Plot, Cell, HTMLTooltip, Text, ColorLegend`
- **Features**: threshold color bands, in-cell labels, HTML tooltip, click-to-select pill, no-data cells, narrow-width axis flip
- **Deviation from reference**: header font uses the app's `--font-display`; kees's task-breakdown drawer not ported (kees-specific data model)

## Relationships

- SveltePlot 0.14.2 — rendering engine ([labs-per-chart-type](../../decisions/labs-per-chart-type.md))
- `src/routes/labs/heatmap/+page.svelte` — demo playground (weekly-utilization + activity-by-category synthetic grids)

## Source

- Ported from `E:\kees.pippeloi.nl\src\routes\work\high-level`
