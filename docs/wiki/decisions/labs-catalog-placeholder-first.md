---
type: Decision
title: Labs chart catalog mirrors theunspokenpitch.com — scaffolded placeholder-first
description: Context
tags: [labs, charts, frontend]
status: accepted
timestamp: "2026-09-14T10:02:32.926Z"
---

# Labs chart catalog mirrors theunspokenpitch.com — scaffolded placeholder-first

## Context

Labs (`/labs`) hosts chart-engine experiments. Earlier decision [[labs-per-chart-type]] organized Labs as one card per chart type, with the heatmap built first on SveltePlot. The user then asked to add "the missing charts found in https://www.theunspokenpitch.com/charts" — a site listing 30 chart types.

## Decision

Adopt that site's 30-type list as the definitive Labs catalog and **scaffold every route and card immediately, placeholder-first**:

- Each type gets a route under `src/routes/labs/<name>/` and a card in the Labs grid (site order), each with a distinct lucide icon.
- Types without a real implementation render the shared [[labs-placeholder]] component ("Placeholder — coming soon.").
- 5 types already existed (line, area, scatter, bar, sankey — themselves placeholders); 25 were added (pie, venn, concentric, circular, bubble, bubble race, sunburst, fan, windrose, tape, gantt, tree map, grid, periodic table, arc, chord, radar, polar grid, spiral, timeline, flow chart, binary tree, mind map, decision tree, block scheme), plus parallel-coordinates — 32 routes total, all cards ↔ routes match.
- Real charts are then built one at a time on the shared reusable-chart fundament ([[labs-charts-reusable-fundament]]), replacing the placeholder. Card descriptions are written when the chart is built.

## Alternatives considered

- Only add a route when its chart is actually implemented — rejected: the catalog acts as a visible roadmap and keeps cards ↔ routes in sync from day one.

## Consequences

30 of 32 routes are placeholders awaiting real implementations on [[svelteplot-sole-chart-engine]] (heatmap and horizontal bar chart are built). When building a chart, swap `LabsPlaceholder` for the real component and add the card description.
