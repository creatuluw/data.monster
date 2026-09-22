---
type: Rule
title: Drawer form controls come from the shared controls kit — never hand-roll input chrome
description: Guideline
tags: [svelte, ui, drawers, design-system, controls-kit]
timestamp: "2026-09-17T17:25:31.321Z"
---

# Drawer form controls come from the shared controls kit — never hand-roll input chrome

## Guideline

When building or editing any drawer, inspector, or modal form in the app, its controls come from the shared kit in `src/lib/components/charts/controls/` — `Field`, `TextInput`, `NumberInput`, `Select`, `Toggle`, `Section`, `DangerZone`, `RemoveBtn`, `Btn` — styled by the single `controls.css`.

Never hand-roll input chrome, add per-consumer styles for inputs, or drop back to raw zinc / ad-hoc Tailwind classes. If a control need isn't covered, **extend the kit** (new component or prop), don't fork a local copy.

## When it applies

- Existing surfaces: page-editor drawers (block inspector, row/column, page settings), RolePickerModal.
- Planned reuse: `/library` pages, `/connect`, and the query editor (named as next steps in PR #18) — reuse the kit as-is.

## Rationale

Before PR #18 (2026-09-17) five surfaces had drifted apart: three duplicated hand-rolled danger panels, fake-toggle checkboxes, mixed input heights and focus states, raw zinc everywhere. One kit on the app's semantic tokens (30px height, tabular numerals, ledger-green focus rings) keeps every surface identical and future drawer work cheap. Related: config drawers must be one scrolling column (see [[config-drawers-one-scrolling-column]]) and reuse the `drawerResize` action.
