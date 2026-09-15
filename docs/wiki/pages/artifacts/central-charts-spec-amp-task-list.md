---
type: Artifact
title: Central-charts spec &amp; task list
description: "The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) edit"
tags: [central-charts, spec, report-pages]
timestamp: "2026-09-15T09:18:53.742Z"
---

# Central-charts spec &amp; task list

The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) editor over one declarative spec.

## What it documents

- The **v1 scope**: bar chart, heatmap, and table as the first page objects, plus a text block — see [central-charts-v1-scope-bar-heatmap-table-blocks-master-item](../../decisions/central-charts-v1-scope-bar-heatmap-table-blocks-master-item.md).
- 13 FRs across 5 phases:

| Phase | FRs | What lands |
|---|---|---|
| 1 Core (pure TS, all TDD) | 1–4 | spec types + validation, query compiler, registry, color scale + tooltip resolver |
| 2 Renderers | 5–8 | ChartCard shell, bar + heatmap registry entries, table block |
| 3 Page runtime | 9 | 12-col grid + cross-filter selection bus |
| 4 Persistence & routes | 10–11 | `d8a_monster_pages` (Rust), `/pages` list + create |
| 5 Editor & cleanup | 12–13 | `/page/[slug]` Design ⇄ Code editor, labs migration |

- `tasks.json` mirrors the FRs; todos live in `.pi/todos` per [spec-driven-features-tdd-karpathy-skills-referenced-in-every](../../rules/spec-driven-features-tdd-karpathy-skills-referenced-in-every.md).

## Details

- **Location**: `.specs/central-charts/spec.md`, `.specs/central-charts/tasks.json`
- **Format**: markdown spec (FRs/phases) + JSON task list
- Renderers build on the existing labs assets: [chart-fundament](../entities/chart-fundament.md), [barchart-component](../entities/barchart-component.md), [heatmap-component](../entities/heatmap-component.md).

## Source

- `.specs/central-charts/spec.md` — the spec itself
