---
type: Rule
title: Each /labs chart owns its config panel
description: Guideline
tags: [labs, charts, config]
timestamp: "2026-09-15T07:33:25.479Z"
---

# Each /labs chart owns its config panel

## Guideline

In `/labs`, chart configuration UI lives **inside the chart component**, never on the page:

- The chart component accepts an optional `config` snippet prop.
- When the snippet is passed, the component renders a `<Bolt />` icon button at the **top-right of the chart card**; clicking it toggles the chart's own [[chartconfigdrawer-component]] (drawer title = `<title> configuration`).
- The page renders its fields into `{#snippet config()}` and passes them to the chart. Page-level "Configure" buttons and page-hosted drawers are removed.

## When it applies

Every chart-type component built on the [[chart-fundament]] (bar chart done; heatmap gets it with one prop pass when its page needs a panel; applies to the 30 charts still to come).

## Rationale

A labs page can host many chart cards — a page-level drawer can only configure one of them. Per-chart panels scale to the catalog. Requested explicitly 2026-09-15 ("each chart has its own panel for config"); the `/labs/bar-chart` page was migrated the same day.

## Gotchas

- Card click-to-deselect must ignore `button` clicks (and the drawer carries `data-drawer`) so toggling config doesn't clear the bar selection.
- Only **config props** get UI controls — data accessors/state stay out of drawers (see [[chartconfigdrawer-component]]).
