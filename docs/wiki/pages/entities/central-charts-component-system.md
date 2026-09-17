---
type: Entity
title: Central-charts component system
description: "The shipped v1 implementation of the central-charts system: the reusable component set under `src/lib/components/charts/` that renders report pages composed of "
tags: [central-charts, charts, report-pages, svelte]
timestamp: "2026-09-16T17:03:44.333Z"
---

# Central-charts component system

The shipped v1 implementation of the central-charts system: the reusable component set under `src/lib/components/charts/` that renders report pages composed of chart/block objects, plus the `/pages` list and `/pages/[slug]` dual-mode editor routes. Built on the `feature/central-charts` branch (commit `ae48ef2`, 20 commits; master held at a restore point — see [central-charts-work-lives-on-feature-branch](../../learnings/central-charts-work-lives-on-feature-branch.md)).

Implements the locked Q6–Q16 decision set; the plan itself is documented in [central-charts-spec-amp-task-list](../artifacts/central-charts-spec-amp-task-list.md) and [central-chart-component-design](../artifacts/central-chart-component-design.md).

## Details

- **Location** (on `feature/central-charts`): `src/lib/components/charts/`
  - **Page runtime**: `PageGrid` (12-col grid host), `ChartCard` (block shell)
  - **Editor**: `BlockInspector`, `ItemEditor`, `RelationshipEditor` — the Design-side panels of the dual-mode editor
  - **Renderers**: `BarChartRenderer`, `HeatmapRenderer`, `TableRenderer` — one per v1 block type
  - Plus: registry, page specs, and semantic layer (v1 core)
- **Routes**: `/pages` — responsive card grid + create modal; `/pages/[slug]` — the dual-mode Design ⇄ Code editor (old `chart/[id]` route deleted)
- **v1 scope**: bar + heatmap + table blocks; master items and auto-JOIN deferred (see [central-charts-v1-scope decision](../../decisions/central-charts-v1-scope-bar-heatmap-table.md))

## Relationships

- [chart-fundament](./chart-fundament.md) — shared pure-TS core the labs charts sit on; central renderers are the sibling system for composed report pages
- [barchart-component](./barchart-component.md) and [heatmap-component](./heatmap-component.md) — labs single-chart components that predate and inform the renderers
- [two-surface-report-page-format](../../decisions/two-surface-report-page-format.md) — both editor surfaces edit one declarative spec; this system is that decision realized

## Lifecycle

- First added: 2026-09-15/16 on `feature/central-charts` (`ae48ef2`); newest `/pages` editor tweaks were in `stash@{0}` pending recovery — see [central-charts-work-lives-on-feature-branch](../../learnings/central-charts-work-lives-on-feature-branch.md)
