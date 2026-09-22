---
type: Entity
title: SkeletonSetup component
description: "The in-chart setup card rendered inside a `ChartCard` when `needsSetup(chart)` is true: a card button per unmet role opens the RolePickerModal (searchable picks over ⭐ master items | source-table fields | linked-table fields, + New) — the common configuration path never opens the config drawer."
tags: [central-charts, page-editor, skeleton]
timestamp: "2026-09-17T15:32:10.396Z"
---

# SkeletonSetup component

The in-chart setup card rendered inside a `ChartCard` when `needsSetup(chart)` is true: a card button per unmet role (e.g. **Add dimension** / **Add measure**) plus summary chips of what the chart already carries; each button opens the [RolePickerModal](./rolepickermodal-component.md) — searchable picks over ⭐ master items, source-table fields, and linked-table fields, with a + New form — so the common configuration path never opens the config drawer.

## Details

- **Location**: `src/lib/components/charts/SkeletonSetup.svelte`; hosted by `src/lib/components/charts/PageGrid.svelte`, props + `onCreateMasterItem` callback wired in `src/routes/pages/[slug]/+page.svelte` (PR #10).
- **Props**: `chart: ChartBlockSpec`, `schemas`, `items`, `relationships`, `onCreateMasterItem?(kind, table, label, expr) => Promise<id>`, `onExternalCreate?(kind, table)` — the host persists the item and refreshes the library; the component mutates the spec directly, same as the drawer. `onExternalCreate` opens the [create-in-/data round-trip](./create-in-data-round-trip.md) instead of creating in-chart.
- **Logic**: derives unmet roles from the registry's role minimums; item availability via `availableItems`/`linkedTables` from `$lib/charts/relationships`; pick decoding via the shared codec.

## Relationships

- Implements the inline half of [chart-blocks-start-empty-needssetup-gate](../../decisions/chart-blocks-start-empty-needssetup-gate.md) (see [skeleton-card-inline-role-assignment](../../decisions/skeleton-card-is-the-inline-role-assignment.md)).
- Uses [pick-values-flow-through-one-codec-pickers-ts](../../rules/pick-values-flow-through-one-codec-src-lib-charts.md) (`src/lib/charts/pickers.ts`), shared with [chartconfigdrawer-component](./chartconfigdrawer-component.md).
- Rendered by [pagegrid-component](./pagegrid-component.md) inside the block card shell.
- Opens [rolepickermodal-component](./rolepickermodal-component.md) for pick/create (PR #12).

## Lifecycle

- First added: 2026-09-17 (PR #10), replacing the drawer-opening buttons on the skeleton. CDP e2e 5/5, vitest 117/117.
- 2026-09-17 (PR #12): inline dropdowns + `✚ Create…` mini form replaced by card buttons opening the RolePickerModal; in-chart summary chips added in PR #13.
- 2026-09-21: `onExternalCreate` pass-through — the modal's "Create in /data" button reaches the host via the skeleton ([create-in-data-round-trip](./create-in-data-round-trip.md)).
