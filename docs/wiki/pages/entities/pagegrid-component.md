---
type: Entity
title: PageGrid component
description: "The canvas renderer + editing surface of the central-charts page editor: lays out a `PageDoc` as rows of 12-col CSS grids — each row an optional-height shell of"
tags: [central-charts, page-editor, svelte]
timestamp: "2026-09-17T10:44:47.272Z"
---

# PageGrid component

The canvas renderer + editing surface of the central-charts page editor: lays out a `PageDoc` as rows of 12-col CSS grids — each row an optional-height shell of explicit `PageColumn`s (span 1–12, optional pixel height) — dispatches each block to its renderer (chart/table/text), and hosts the grid-editing affordances (drag-resize grips, + Component / + Row buttons, per-block/row/column config entry points). PageGrid stays dumb: callbacks hand edits to the host editor (`/pages/[slug]`), which mutates the doc and autosaves.

## Details

- **Location**: `src/lib/components/charts/PageGrid.svelte`
- **Interface**: props `{ doc: PageDoc, runtime: PageRuntime, configureId?: string | null, onConfigure?: (blockId) => void, onConfigureRow?: (ri) => void, onConfigureColumn?: (ri, ci) => void, onAdd?: (ri, ci) => void, onAddRow?: () => void }`
- **Layout**: row shells stack (`space-y-4` + tooltip overflow room); each row renders `rowColumns(row)` (from spec-types) as `grid grid-cols-12 gap-4` — the **column** owns the span per [q13-explicit-grid-rows-blocks-take-col-spans](../../decisions/q13-explicit-grid-rows-blocks-take-col-spans.md) (amended 2026-09-17: span moved from block to column); legacy flat-`blocks` rows keep rendering through the same helper. Rows are the only page-level primitive — components are added inside columns.
- **Block dispatch**: `chart` → registry renderer when data is ready, else `ChartCard` fallback (missing/loading/error/empty states); `table` → TableRenderer; `text` → plain card.
- **Editing affordances** (2026-09-17):
  - Row bottom grip — pointer-capture drag sets `row.height` (min 80px).
  - Grip between columns — takes/gives `span` from the next sibling, clamped 1–12.
  - `+` per column — `onAdd(ri, ci)`; the host opens the library-registry-driven component picker (picking into an occupied column makes the host spawn a fresh column).
  - `+ Row` — `onAddRow()`; host appends `{ columns: [{ span: 12, blocks: [] }] }` and autosaves (empty canvas shows the button top-left).
  - Row/column ⚙ `Settings2` buttons → `onConfigureRow` / `onConfigureColumn` (host-side drawers); per-block ⚡ Bolt button → focused two-panel config per [page-editor-block-config-focused-two-panel](../../decisions/page-editor-block-config-focused-two-panel.md).
- **Focused mode**: when `configureId` is set, renders only that block full-width; the editor supplies the 50vw drawer.
- **Spacing**: implements [card-spacing-from-grid-gap-not-margins](../../rules/card-spacing-from-grid-gap-not-margins.md) — gap-only spacing, uniform 16px.

## Relationships

- Renders [chart-page-spec-spec-types-validator](./chart-page-spec-spec-types-validator.md) `PageDoc`s; runtime block ids are `r{ri}-c{ci}-b{bi}`.
- Config surfaces: [chartconfigdrawer-component](./chartconfigdrawer-component.md) (blocks) plus host row/column drawers in `/pages/[slug]`.
- The `+ Component` picker iterates [library-registry-system](./library-registry-system.md) — registered components are the menu, by construction.
- [page-editor-tri-mode-design-code-settings](../../decisions/page-editor-tri-mode-design-code-settings.md) — canvas shows only visualizations & data; settings live in drawers.

## Lifecycle

- First added: 2026-09-15 with the central-charts page editor; canvas made full-width same day (tri-mode decision).
- 2026-09-17: explicit-columns rework — row/column drag-resize grips, `+ Component` / `+ Row` affordances, row/column config callbacks, host picker wiring; spans moved from block to column.
