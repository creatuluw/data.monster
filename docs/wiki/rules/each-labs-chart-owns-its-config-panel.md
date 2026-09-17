---
type: Rule
title: Each /labs chart owns its config panel
description: Guideline
tags: [labs, library, charts, config]
timestamp: "2026-09-15T07:33:25.479Z"
---

# Each /labs chart owns its config panel

## Guideline

In `/labs`, chart configuration UI lives **inside the chart component**, never on the page:

- The chart component accepts an optional `config` snippet prop.
- When the snippet is passed, the component renders a `<Bolt />` icon button at the **top-right of the chart card**; clicking it toggles the chart's own [[chartconfigdrawer-component]] (drawer title = `<title> configuration`).
- The page renders its fields into `{#snippet config()}` and passes them to the chart. Page-level "Configure" buttons and page-hosted drawers are removed.

## When it applies

Every chart-type component built on the [[chart-fundament]], on **every surface that shows a chart** — not just `/labs`. Confirmed surfaces: `/labs` pages, `/library` detail-page previews (2026-09-17: the demo passes a `config` snippet to the real renderer, fields rendered schema-driven with the same markup as `BlockInspector`), and the `/pages` focused config mode. User directive 2026-09-17: "this is an integral part of every chart" — the Bolt toggle + drawer ships with the chart, never bolted on per-page.

## Rationale

A labs page can host many chart cards — a page-level drawer can only configure one of them. Per-chart panels scale to the catalog. Requested explicitly 2026-09-15 ("each chart has its own panel for config"); the `/labs/bar-chart` page was migrated the same day.

## Gotchas

- Card click-to-deselect must ignore `button` clicks (and the drawer carries `data-drawer`) so toggling config doesn't clear the bar selection.
- Only **config props** get UI controls — data accessors/state stay out of drawers (see [[chartconfigdrawer-component]]).
