---
type: Rule
title: Card spacing comes from the grid gap, never per-card margins
description: In any grid of chart/component cards (page editor canvas, labs), inter-card
tags: [design-system, layout, charts, page-editor]
timestamp: "2026-09-15T13:14:38.562Z"
---

# Card spacing comes from the grid gap, never per-card margins

In any grid of chart/component cards (page editor canvas, labs), inter-card
spacing comes from the **grid container's gap alone** — cards must not carry
their own margins. Overflow room (tooltips clipping past the last row) goes on
the container's bottom padding, never on the cards.

## When it applies
- Every card renderer wrapped by a grid: ChartCard, table/text blocks in
  PageGrid, labs chart canvases.

## Rationale
Before 2026-09-15, `ChartCard` had a per-card `mb-16` while PageGrid rows used
`gap-4` — the two spacing systems stacked on top of each other, so gaps varied
between side-by-side cards and stacked rows. Removing the per-card margin makes
all gaps uniformly 16px (`space-y-4` between rows, `gap-4` within a row), and
the tooltip-overflow allowance moved to the grid's `pb-16`.

## Evidence
- `src/lib/components/charts/ChartCard.svelte` — explicit comment: "No vertical margin: spacing comes from the page grid gap; tooltip overflow room lives on the grid container (PageGrid pb-16 / labs page padding)."
- `src/lib/components/charts/PageGrid.svelte` — `space-y-4 pb-16` container, `grid grid-cols-12 gap-4` rows; verified 16px in both directions via CDP (PR #4).
