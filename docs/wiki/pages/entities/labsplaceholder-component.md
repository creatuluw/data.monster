---
type: Entity
title: LabsPlaceholder component
description: The shared Svelte 5 placeholder shell that renders a section page with "Placeholder — coming soon." Every not-yet-built chart type in `/labs` shows it, and other not-yet-built sections (e.g. `/library`) reuse it via the `section` prop.
tags: [labs, charts, svelte, frontend]
timestamp: "2026-09-17T06:00:00.000Z"
---

# LabsPlaceholder component

A small Svelte 5 component that renders the standard section page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` shows until its real implementation lands — and, since 2026-09-17, the shared shell for not-yet-built sections outside Labs too.

## Details

- **Location**: `src/lib/components/LabsPlaceholder.svelte`
- **Interface**: `title: string` (page `<h1>`) plus optional `section: string` (defaults to `'Labs'`) — the `<svelte:head>` title is `[title, section, 'Data Monster'].filter(Boolean).join(' — ')`, so an empty `section` drops the middle segment (e.g. `Library — Data Monster`)
- **Style**: same `section-header` shell as other Labs pages (display-font title only; the LABS eyebrow badge was removed 2026-09-15 from all chart pages — the `/labs` catalog index keeps its own header)

## Relationships

- Instantiates the placeholder side of the decision [labs-per-chart-type](../../decisions/labs-per-chart-type.md) / [labs-catalog-placeholder-first](../../decisions/labs-catalog-placeholder-first.md)
- Used by the 30 unbuilt chart routes under `src/routes/labs/` (32 total; the built ones are [heatmap-component](./heatmap-component.md) and [barchart-component](./barchart-component.md))
- Used by `src/routes/library/+page.svelte` — first consumer outside `/labs` (`section=""`)
- When a chart is implemented, this component is replaced by one built on the shared reusable-chart fundament ([labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md))

## Lifecycle

- First added: 2026-09-14, when the Labs catalog was scaffolded to mirror theunspokenpitch.com's 30 chart types
- 2026-09-17: gained the optional `section` prop when `/library` became the first non-Labs consumer
- Each placeholder route is a 3–5-line file: import, render `<LabsPlaceholder title="..." />`

## Source

- `src/lib/components/LabsPlaceholder.svelte`
- `src/routes/labs/*/+page.svelte` — consumers
- `src/routes/library/+page.svelte` — first non-Labs consumer
