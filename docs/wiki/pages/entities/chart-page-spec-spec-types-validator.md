---
type: Entity
title: Chart page spec (spec-types + validator)
description: "Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src"
tags: [central-charts, spec, types, validation, charts]
timestamp: "2026-09-15T09:36:11.957Z"
---

# Chart page spec (spec-types + validator)

Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src/lib/charts/spec-types.ts`) plus the pure structural validator (`src/lib/charts/validate.ts`). Everything is plain serializable data; this is the single source of truth both editing surfaces (Design inspector ⇄ Code JSON) read and write.

## What is it?

The data contract of the central chart system. A `PageDoc` is `{ slug, title, rows[] }`; each row holds blocks (`chart` | `table` | `text`) with optional 12-column `span`. Measures are DuckDB expressions (`{ expr, label?, fmt? }`), dimensions are columns with optional `grain` — or master-item `{ ref }` references per the master-items amendment. Annotations use whitelisted svelteplot marks (`arrow dot line ruleX ruleY text rect`).

## Why it matters

Every later FR builds on it: the query compiler (FR-2) compiles these specs to SQL, the registry (FR-3) defines per-type roles, the dual-mode editor round-trips them. `validatePageDoc(doc)` returns `{path, message}[]` and **never throws** — used by Code mode, save, and render-time guards. Hand-rolled, no zod (spec constraint: no new npm deps).

## Details

- **Location**: `src/lib/charts/spec-types.ts`, `src/lib/charts/validate.ts`; tests in `tests/validate.test.ts` (22 tests, suite 31/31)
- **Interface**: `validatePageDoc(doc: unknown): ValidationError[]`; type exports `PageDoc`, `ChartBlock/TableBlock/TextBlock`, `ChartBlockSpec`, `MeasureSpec`, `DimensionSpec`, `FilterSpec`, `SortSpec`, `AnnotationSpec`, `TooltipSpec`, `AxisOptions`
- **Configuration**: block/chart-type/grain/op whitelists are hardcoded in `validate.ts` with a `ponytail:` note — FR-3 swaps them for registry lookups
- **Span rule**: integer 1–12, matching the Q13 explicit-grid decision
- [chart-authoring-two-surfaces-serializable-spec](../../learnings/chart-authoring-two-surfaces-serializable-spec.md) — the user-stated requirement this module implements
- [central-charts-spec-tasks](../artifacts/central-charts-spec-tasks.md) — the spec/task plan (FR-1 = this module)
- [measures-dimensions-are-duckdb-expressions](../../decisions/measures-dimensions-are-duckdb-expressions.md) — why MeasureSpec is `expr`, not column+agg sugar
- [q15-annotation-vocabulary-svelteplot-basic-marks](../../decisions/q15-annotation-vocabulary-svelteplot-basic-marks.md) — the AnnotationMark whitelist
- [chart-fundament](./chart-fundament.md) — L0 /labs sibling; `buildBars` becomes the `topN` query hook

## Lifecycle

- First added: 2026-09-15, TDD red→green (FR-1 of 17). Build started from git tag `restore-point/central-charts-start` (commit `e936b7f`, pushed to GitHub) — the master restore point before the central-charts build.
- Planned: FR-3 moves the validator's hardcoded whitelists into the registry.
