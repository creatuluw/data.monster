---
type: Entity
title: LabsPlaceholder component
description: A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` s
tags: [labs, charts, svelte, frontend]
timestamp: "2026-09-15T07:14:56.316Z"
---

# LabsPlaceholder component

A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` shows until its real implementation lands.

## Details

- **Location**: `src/lib/components/LabsPlaceholder.svelte`
- **Interface**: single prop `title: string` — rendered as the page `<h1>` and `<svelte:head>` title (`{title} — Labs — Data Monster`)
- **Style**: same `section-header` shell as other Labs pages (display-font title only; the LABS eyebrow badge was removed 2026-09-15 from all chart pages — the `/labs` catalog index keeps its own header)

## Relationships

- Instantiates the placeholder side of the decision [labs-per-chart-type](../../decisions/labs-per-chart-type.md) / [labs-catalog-placeholder-first](../../decisions/labs-catalog-placeholder-first.md)
- Used by the 30 unbuilt chart routes under `src/routes/labs/` (32 total; the built ones are [heatmap-component](./heatmap-component.md) and [barchart-component](./barchart-component.md))
- When a chart is implemented, this component is replaced by one built on the shared reusable-chart fundament ([labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md))

## Lifecycle

- First added: 2026-09-14, when the Labs catalog was scaffolded to mirror theunspokenpitch.com's 30 chart types
- Each placeholder route is a 5-line file: import, render `<LabsPlaceholder title="..." />`

## Source

- `src/lib/components/LabsPlaceholder.svelte`
- `src/routes/labs/*/+page.svelte` — consumers
