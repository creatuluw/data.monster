---
type: Decision
title: Labs reorganized to one card per chart type; heatmap built on ported SveltePlot component
description: Labs goes per-chart-type; first chart (heatmap) built on SveltePlot
tags: [charts, labs, svelteplot, heatmap]
status: accepted
supersedes: "["decisions/consolidate-chart-engines-to-picasso-js"]"
timestamp: "2026-09-14T07:26:30.635Z"
---

# Labs reorganized to one card per chart type; heatmap built on ported SveltePlot component

# Labs goes per-chart-type; first chart (heatmap) built on SveltePlot

## Context

After [consolidating chart engines](consolidate-chart-engines-to-picasso-js.md), `/labs` still showed engine cards (picasso-charts, chart-lib, unovis-charts) plus a showcase link — an engine-comparison layout. The user pivoted: Labs should list **one card per chart type we intend to build**, starting with a heatmap. The heatmap reference lives in `E:\kees.pippeloi.nl\src\routes\work\high-level` and is SveltePlot-based.

## Decision

1. `/labs` index shows **one card per chart type** (no library cards, no showcase link). First card: Heatmap → `/labs/heatmap`.
2. The heatmap was **ported as-is from the kees.pippeloi.nl reference** using SveltePlot — not rebuilt in Picasso.js/LayerChart. SveltePlot 0.14.2 is still a dependency in `package.json` (the earlier consolidation removed lab routes, not the dependency), and version parity with the reference (same svelteplot 0.14.2, same Tailwind 4) made the port the cheapest correct move.
3. Per-type components live in `src/lib/components/` (reusable, e.g. `Heatmap.svelte`), with a playground route under `/labs/<type>` on synthetic data.

## Alternatives considered

- Keep the Picasso.js vs LayerChart engine comparison and build the heatmap twice — rejected: Labs' purpose changed from comparing engines to building a chart-type library.
- Rebuild the heatmap in Picasso.js/LayerChart — rejected for v1: the reference port is free; engine choice per type stays open.

## Consequences

- Old engine subroutes (`/labs/picasso-charts`, `/labs/chart-lib`, `/labs/unovis-charts`, `/labs/charts`) are **unlinked but still on disk** — delete when the per-type structure proves out.
- Future chart types each get a card + playground; port from the kees reference when a match exists (see [kees reference learning](../learnings/kees-reference-ports-cleanly.md)).
- The reference's task-breakdown drawer was NOT ported (kees-specific data model); revisit when the heatmap hooks into real query results.
