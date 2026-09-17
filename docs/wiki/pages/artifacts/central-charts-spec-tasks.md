---
type: Artifact
title: Central charts spec & tasks
description: "The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/valida"
tags: [central-charts, spec, tasks, planning, charts]
timestamp: "2026-09-15T09:31:01.992Z"
---

# Central charts spec & tasks

The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/validation, query compiler, registry, color/tooltip), Renderers (ChartCard shell, bar, heatmap, table), Page runtime (grid + cross-filter bus), Persistence & routes (`d8a_monster_pages` + Rust `pages.rs`, `/pages` list + create, `/pages/[slug]` dual-mode editor), and Cleanup (labs migration, retire top-level `BarChart.svelte` + `pages - Copy`).

## What it documents

- Phase-1 v1 scope: bar + heatmap + table + text blocks only; master items, auto-JOIN, custom panels, annotations beyond `ruleY` explicitly out of scope (see [central-chart-component-design](./central-chart-component-design.md) and the proposed master-items amendment that reverses the deferral)
- Per-task methodology: TDD (RED-GREEN-REFACTOR) for all pure logic in `tests/` (never `src/`), Karpathy minimal-diff review before done, completion logged to `.specs/central-charts/tasks-log.json` per repo convention — instantiating the rule [spec-driven-features-tdd-karpathy-in-todos](../../rules/spec-driven-features-tdd-karpathy-in-todos.md)
- Key FR map: FR-1 spec types + pure validator (`{path,message}[]`, never throws), FR-2 SQL string-builder compiler with escaping + injection guard, FR-3 registry (`ChartTypeDefinition` roles/optionsSchema/hooks/annotation whitelist), FR-9 selection bus, FR-10 persistence, FR-12 Design ⇄ Code editor

## Details

- **Format**: Spec-driven markdown (`.specs/central-charts/spec.md`) + machine-readable tasks (`.specs/central-charts/tasks.json` with acceptanceCriteria per task)
- **Constraint carried in spec**: no new npm dependencies (hand-rolled validation, no zod); test files live in `tests/`, never `src/`

## Relationships

- [central-chart-component-design](./central-chart-component-design.md) — the design doc this spec implements
- [chart-fundament](../entities/chart-fundament.md) — L0 of the design; `buildBars` becomes the `topN` query hook
- [central-charts-v1-scope decision](../../decisions/central-charts-v1-scope-bar-heatmap-table.md) — defines the phase-1 scope the spec encodes
- [Master-items amendment (proposed)](../../decisions/master-items-amendment-semantic-layer-moves-early.md) — pending change that inserts the semantic layer early

## Source

- `.specs/central-charts/spec.md` — the 13 FRs
- `.specs/central-charts/tasks.json` — the 13 tasks with acceptance criteria
