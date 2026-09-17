---
type: Entity
title: PageGrid component
description: "The canvas renderer for the central-charts page editor: lays out a"
tags: [central-charts, page-editor, svelte]
timestamp: "2026-09-15T13:14:38.562Z"
---

# PageGrid component

The canvas renderer for the central-charts page editor: lays out a
[chart-page-spec-spec-types-validator](./chart-page-spec-spec-types-validator.md) `PageDoc` as rows of 12-col CSS grid
rows, one block per cell, and dispatches each block to its renderer (chart,
table, or text). Also implements focused config mode.

## Details

- **Location**: `src/lib/components/charts/PageGrid.svelte`
- **Interface / Schema**: props `{ doc: PageDoc, runtime: PageRuntime, configureId?: string | null, onConfigure?: (id) => void }`
- **Layout**: outer `div.space-y-4.pb-16` (16px between rows + tooltip overflow room); each row `grid grid-cols-12 gap-4`; blocks take `grid-column: span <block.span>` per [q13-explicit-grid-rows-blocks-take-col-spans](../../decisions/q13-explicit-grid-rows-blocks-take-col-spans.md).
- **Block dispatch**: `chart` → registry renderer when data is ready, else `ChartCard` fallback (missing/loading/error/empty states); `table` → TableRenderer; `text` → plain card.
- **Config entry**: per-block ⚡ Bolt button (z-index 30 above svelteplot overlay svgs) → focused two-panel config per [page-editor-block-config-focused-two-panel](../../decisions/page-editor-block-config-focused-two-panel.md) via [chartconfigdrawer-component](./chartconfigdrawer-component.md).
- **Focused mode**: when `configureId` is set, renders only that block full-width; the editor supplies the 50vw drawer.
- **Spacing**: implements [card-spacing-from-grid-gap-not-margins](../../rules/card-spacing-from-grid-gap-not-margins.md) — gap-only spacing, uniform 16px.

## Lifecycle

- First added: 2026-09-15 with the central-charts page editor; sidebar removed and canvas made full-width by [page-editor-tri-mode-design-code-settings](../../decisions/page-editor-tri-mode-design-code-settings.md) (same day).

## Source

- `src/lib/components/charts/PageGrid.svelte`
