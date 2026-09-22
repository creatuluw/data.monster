---
type: Entity
title: Shared controls kit (charts/controls)
description: "The shared form-controls kit for every drawer, inspector, and modal surface in the app: nine small Svelte 5 components plus one CSS file, all built on the app's"
tags: [svelte, design-system, drawers, controls, ui]
timestamp: "2026-09-17T17:25:31.321Z"
---

# Shared controls kit (charts/controls)

The shared form-controls kit for every drawer, inspector, and modal surface in the app: nine small Svelte 5 components plus one CSS file, all built on the app's semantic design tokens. Added 2026-09-17 (PR #18) to replace per-drawer hand-rolled input chrome.

## What it is

`src/lib/components/charts/controls/` — `Field`, `TextInput`, `NumberInput`, `Select`, `Toggle`, `Section`, `DangerZone`, `RemoveBtn`, `Btn`, plus `controls.css`. Every drawer speaks this one token language; no raw zinc or ad-hoc Tailwind input classes.

## Details

- **Location**: `src/lib/components/charts/controls/`
- **Interface**: Svelte 5 idioms throughout — `$bindable` values, snippet children (`{@render children()}`), callback props instead of event forwarding. `Toggle` is a real switch (not a checkbox), `DangerZone` is the single danger panel (three former duplicates collapsed into it), `RemoveBtn` grew a `disabled` prop during the rewire.
- **Configuration / chrome source**: `controls.css` is the single source of input chrome — 30px input height, 2–4px radii, tabular numerals, ledger-green focus rings, chevron selects, segmented tabs. It is loaded via a plain script import (component `<style>` blocks are scoped, so shared chrome must not live there).
- **Verified surfaces**: BlockInspector, page-editor row/column drawers, page settings, RolePickerModal, DrawerTabs — rewired in PR #18 with logic untouched; svelte-check clean, 119/119 tests, CDP interaction pass.

## Relationships

- Implements the [design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) token layer (semantic tokens, annual-report light + ledger green).
- Used by [chartconfigdrawer-component](./chartconfigdrawer-component.md), BlockInspector, RolePickerModal, and the `/pages/[slug]` row/column/page-settings drawers. (DrawerTabs was a consumer until it was deleted in the 2026-09-18 one-scrolling-column restyle.)
- Governed by the rule [drawer-form-controls-come-from-the-shared-controls-kit](../../rules/drawer-form-controls-come-from-the-shared-controls.md) — extend the kit, never fork it.

## Lifecycle

- First added: 2026-09-17, PR #18 (15 files, +995/−292) — kit built and all existing drawers rewired in one pass.
- 2026-09-18: kit restyled to the `/data` drawer pattern (Section flat + dashed dividers, Field labels, DangerZone per `/data`, tightened input radius); adoption extended to the `/labs` config snippets and the `/library/[id]` detail config snippet; `/pages` editor drawers dropped DrawerTabs for the flat flow with danger last (see decision all-drawers-adopt-the-data-pattern).
- Natural growth path remaining: `/connect` and the query editor reuse the kit as-is.
