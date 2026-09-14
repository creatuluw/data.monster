---
type: Rule
title: Inter for UI text, Geist Mono only for data detail
description: Guideline
tags: [design, typography, frontend]
timestamp: "2026-09-14T07:47:03.229Z"
---

# Inter for UI text, Geist Mono only for data detail

## Guideline
Every new UI surface follows the two-font rule set in `src/app.css`:

- **Inter** (`--font-display`, `--font-body`) for all UI text — headings, labels, buttons, prose, nav, modals.
- **Geist Mono** (`--font-mono`) **only for data detail**: table cells/headers (`.data-table th/td`), chart axis ticks, in-cell chart labels (Picasso canvases, svelteplot `Heatmap.svelte`), and code/IDs.

## When it applies
Any new component, route, or chart. If the text renders a data value or helps read one off a chart/table → mono. Everything else → Inter.

## Rationale
User decision (2026-09-14, see [[typography-inter-mono-for-data]]): mono visually marks "machine data" vs UI chrome, and `tabular-nums` alignment in mono keeps dense tables legible. Never hardcode a font-family — use the CSS vars so token changes propagate.
