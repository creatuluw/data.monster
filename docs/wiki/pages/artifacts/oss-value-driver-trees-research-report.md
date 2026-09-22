---
type: Artifact
title: OSS value driver trees research report
description: What it documents
tags: [research, value-driver-tree, oss, charts]
timestamp: "2026-09-17T13:07:44.566Z"
---

# OSS value driver trees research report

## What it documents

Fractal-research (te9-research) report answering: *which open source apps implement value driver trees (VDTs) with live data and/or analysis?* Produced 2026-09-17 for scoping a potential VDT feature in data.monster.

## Key findings

- **No mature OSS app does value driver trees with live data.** Closest match: **react-kpi-tree** (Apache-2.0, 3★, real polling-adapter data binding — but a learning project). Grafana's Interactive Tree Panel is the best live-data tree UX in mainstream BI; Superset/Metabase offer nothing real.
- The concept is split in two: **healthy compute engines without trees** (FinanceToolkit DuPont 5.4k★, revenue-model-builder, Modeleon, DoWhy) and **dead UIs without compute** (bambooBSC, hillfog).
- Power BI's decomposition tree has no OSS clone — ECharts' request was closed unimplemented.
- For data.monster: svelteplot 0.14.2 has no tree mark (verified in the installed package — see [svelteplot-has-no-tree-mark](../../learnings/svelteplot-has-no-tree-mark.md)), and no library computes roll-up math — an interactive, DuckDB-backed VDT would be the first mature OSS one.

## Details

- **Location**: `reports/2026-09-17-oss-value-driver-trees/report.html` (self-contained HTML; open directly in browser)
- **Audit trail**: `reports/2026-09-17-oss-value-driver-trees/agents/` + `research.log` + `metrics.json` (per-branch evidence, search trails, tensions)
- **Format**: fractal-research HTML report template (same lineage as the [LLM agent connection research report](llm-agent-connection-research-report.md))
- **Generated from**: depth-1 grounded fan-out, 5 leaves, via the te9-research skill

## Source

- `reports/2026-09-17-oss-value-driver-trees/` — report + audit trail
