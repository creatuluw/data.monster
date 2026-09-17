---
type: Artifact
title: Central chart component design
description: What it documents
tags: [charts, design, report-pages, architecture]
timestamp: "2026-09-15T09:08:33.566Z"
---

# Central chart component design

## What it documents

- [chart-fundament](../entities/chart-fundament.md) — the design formalizes and extends it into an 8-layer architecture
- The report-page system (`/pages` list → `/pages/<slug>` dual-mode editor)
- Master-item library + relationship-graph smart association

## Summary

Complete design for one central chart component, distilled from a 17-question interview (2026-09-15) informed by Evidence.dev's prop taxonomy and SveltePlot's mark system. Core model: every chart = a declarative spec block resolved through shared pipeline (semantic layer → canonical query engine with per-type hooks → chart-type registry → shell) into a SveltePlot render. Two editor surfaces (no-code Design / code mode) edit the same spec document in `d8a_monster_pages` — parity by construction. Dimensions/measures are DuckDB expressions with a workspace-level master-item library (stable ids, table-bound, relationship-graph auto-JOIN). Selection cross-filters via re-query; page-consistent colors; tooltips declarative fields + template; annotations in SveltePlot basic-mark vocabulary whitelisted per type; explicit 12-col grid; general block envelope with registry.

## Source

- `docs/design/central-chart-component-design.md` — the full design
- Evidence.dev docs (scatter_chart + core-concepts/components) — prop-taxonomy reference
- svelteplot.dev marks — annotation vocabulary
