---
type: Entity
title: ChartConfigDrawer component
description: "A reusable drawer shell for chart configuration panels, hosted **inside each chart component** in `/labs`: a chart accepts an optional `config` snippet and togg"
tags: [labs, charts, component, svelte, page-editor]
timestamp: "2026-09-15T11:55:15.360Z"
---

# ChartConfigDrawer component

A reusable drawer shell for chart configuration panels, hosted **inside each chart component** in `/labs`: a chart accepts an optional `config` snippet and toggles its own drawer via a `<Bolt />` button at the top-right of the chart card. Also reused by the `/page/<slug>` focused config mode at 50vw. First used by the BarChart component at `/labs/bar-chart`.

## Details

- **Location**: `src/lib/components/charts/ChartConfigDrawer.svelte`
- **Interface / Schema**: props are `open` (bindable), `title`, `width` (default `30vw`), `overlay` (default `true`, dim backdrop), and a `children` snippet for the fields. Ships shared `.field` / `.field-label` / `.field-hint` / `.input` styles for whatever the chart renders inside. Charts open it with the title `<title> configuration`. No `onclose` prop — close is via the bindable `open`.
- **Hosting pattern** (2026-09-15): the chart component renders the drawer when the caller passes a `config` snippet — the Bolt toggle only appears if the snippet is passed (opt-in; heatmap doesn't have it yet). Before this, the *page* hosted the drawer via a Configure button + Settings2 icon — superseded.
- **Two widths in use**: labs bolt-drawer stays 30vw; the page editor's focused config mode opens it at **50vw** with the focused chart alone on the left ([page-editor-block-config-focused-two-panel-mode-via-cog-icon](../../decisions/page-editor-block-config-focused-two-panel-mode-via-cog-icon.md)).
- **Pattern source**: the drawer shell (overlay + slide-in panel + header/close) reuses the established `ColumnFunctionDrawer.svelte` pattern rather than a new abstraction — pages stay thin, no generic field renderer needed.
- **Marked `data-drawer`** and card click-to-deselect ignores `button` clicks, so configuring never clears a selection.

## Config-vs-accessor convention

When exposing "reusable config variables" for a chart, only **config props** get UI controls — not data accessors or state:

- `title`, `subtitle` (strings) → text input
- `color` (hex) → native color picker
- `heightVh` (viewport fraction) → range slider with live readout

Props like `category`, `value`, `tooltip`, `labelFor`, `selected` are data accessors/state and stay out of config drawers.

## Relationships

- Hosted inside [barchart-component](./barchart-component.md) via its `config` snippet + Bolt toggle
- Shells the page editor's BlockInspector in focused config mode — [page-editor-block-config-focused-two-panel-mode-via-cog-icon](../../decisions/page-editor-block-config-focused-two-panel-mode-via-cog-icon.md)
- [each-labs-chart-owns-its-config-panel](../../rules/each-labs-chart-owns-its-config-panel.md) — the hosting rule
- Supports all charts built on the [chart-fundament](./chart-fundament.md) per [labs-charts-reusable-fundament](../../rules/labs-charts-reusable-fundament.md)
- Sibling shell pattern: ColumnFunctionDrawer (`src/lib/components/ColumnFunctionDrawer.svelte`)

## Lifecycle

- First added: 2026-09-15 — bar-chart config drawer request; four live-bound fields (title, subtitle, color, heightVh), edits apply immediately. Originally page-hosted (Configure button, Settings2 icon).
- 2026-09-15: hosting moved into the chart component — optional `config` snippet prop + `<Bolt />` toggle top-right of the chart card; page-level button and drawer removed; drawer title now `<title> configuration`.
- 2026-09-15: grew `width` + `overlay` props; reused by the `/page/<slug>` focused config mode at 50vw (labs behavior unchanged).
