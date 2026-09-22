---
type: Entity
title: RolePickerModal component
description: "The pick/create modal opened from the SkeletonSetup card buttons (new-page-modal pattern): a searchable list over ⭐ master items, source-table fields, and linke"
tags: [central-charts, page-editor, skeleton]
timestamp: "2026-09-17T17:31:28.140Z"
---

# RolePickerModal component

The pick/create modal opened from the SkeletonSetup card buttons (new-page-modal pattern): a searchable list over ⭐ master items, source-table fields, and linked-table fields, plus a + New form that creates a master item on the spot. Applying pushes the pick into the chart spec via the shared pick codec — the config drawer stays closed.

## Details

- **Location**: `src/lib/components/charts/RolePickerModal.svelte`; opened by `SkeletonSetup.svelte`, props wired in `src/routes/pages/[slug]/+page.svelte`.
- **Props**: `kind: 'dimension' | 'measure'`, `chart: ChartBlockSpec`, `schemas`, `items: MasterItem[]`, `relationships`, `onCreateMasterItem?(kind, table, label, expr) => Promise<string>`, `onExternalCreate?(kind, table)`, `onClose()`.
- **Logic**: entries built from `availableItems`/`linkedTables` (`$lib/charts/relationships`); picks decoded via `dimensionFromPick`/`measureFromPick`, labels via `roleLabels()` (`$lib/charts/pickers`); shows an in-chart summary of what the chart already carries (PR #13); form chrome from the shared controls kit (`TextInput`, `Btn`).

## Relationships

- Implements the pick/create half of [skeleton-pick-create-moved-from-inline-dropdowns](../../decisions/skeleton-pick-create-moved-from-inline-dropdowns.md); opened from [skeletonsetup-component](./skeletonsetup-component.md).
- Decodes through the one pick codec [pick-values-flow-through-one-codec-src-lib-charts](../../rules/pick-values-flow-through-one-codec-src-lib-charts.md); labels resolve via [pick-display-labels-resolve-through-rolelabels](../../rules/pick-display-labels-resolve-through-rolelabels.md).
- Form controls from [shared-controls-kit-charts-controls](./shared-controls-kit-charts-controls.md).
- The + New form's expression field is the [ExprEditor component](./expreditor-component.md); "Create in /data" opens the [create-in-data-round-trip](./create-in-data-round-trip.md).

## Lifecycle

- First added: 2026-09-17 (PR #12), replacing SkeletonSetup's inline dropdowns + ✚ Create… mini form; in-chart summary added in PR #13; rewired onto the shared controls kit in PR #18.
- 2026-09-21: create form's expression input → ExprEditor; "Create in /data" button added beside "New master item".
