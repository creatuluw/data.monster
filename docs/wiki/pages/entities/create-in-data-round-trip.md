---
type: Entity
title: Create-in-/data round-trip
description: "Deep-link flow from a /pages chart's pick surfaces to the full master-item editor in /data and back: chart → /data?tab=<kind>s&add=1&table=…&return=<slug>&block=<id> → ItemEditor preset form → save → /pages/<slug>?configure=<block>&attach=<itemId> → item attached to the chart + focused drawer reopened."
tags: [central-charts, master-items, navigation, data-route]
timestamp: "2026-09-22T07:24:11.009Z"
---

# Create-in-/data round-trip

Deep-link flow from a `/pages` chart's pick surfaces to the **full master-item editor in `/data`** and back. The in-chart RolePickerModal's + New form is compact; this is the escape hatch when the user wants the full-panel editor — "Create in /data" buttons in [RolePickerModal](./rolepickermodal-component.md) and the focused-drawer BlockInspector, wired through [SkeletonSetup](./skeletonsetup-component.md) and [PageGrid](./pagegrid-component.md).

## Details

- **Outbound URL contract**: `/data?tab=<measures|dimensions>&add=1&table=<boundTable>&return=<pageSlug>&block=<blockId>`.
- **Participants** (`src/lib/components/charts/` + both routes):
  - `onExternalCreate(kind, table)` prop — added to RolePickerModal, SkeletonSetup (pass-through), BlockInspector; PageGrid passes blockId and hosts call `openInData()` in `/pages/[slug]/+page.svelte`, which **silently saves the page first** (`handleSave(true)`) so in-place edits survive navigation.
  - `TableOverview.svelte` — parses the params at init (whitelisted `KNOWN_TABS`), builds an `ItemEditorPreset`, then `replaceState('/data', {})` so **params are consumed once** — reloading doesn't re-trigger the form.
  - `ItemEditor.svelte` — new `preset` + `metas` (typed columns for [ExprEditor](./expreditor-component.md) suggestions) props; an `$effect` waits for the table list, opens the creation form, preselects the bound table.
  - Return leg: after save with `returnTo`+`block` set, `goto(/pages/<slug>?configure=<block>&attach=<itemId>)`; the page editor's `$effect` finds the item, pushes `{ ref: id }` into the chart's dimensions/measures (skipping if already present), silently saves, reopens the focused config drawer with the new item, and strips the params with `replaceState`.
- **Edge handling**: `handleSave(silent)` grew a silent mode (no saving spinner / Saved flash) — also used by the 60s auto-save.

## Relationships

- Escape hatch beside the inline path: [skeleton-card-is-the-inline-role-assignment](../../decisions/skeleton-card-is-the-inline-role-assignment.md) (in-chart stays the default; /data is the full-panel option).
- Expression entry on both legs via [expreditor-component](./expreditor-component.md).

## Lifecycle

- First added: 2026-09-21 — "Create in /data" buttons + preset deep-link + attach-on-return, one pass.
