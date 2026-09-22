---
type: Rule
title: Config drawers are one scrolling column — settings sections, Danger zone last
description: Guideline
tags: [page-editor, drawers, ui-consistency, data-drawer-pattern]
timestamp: "2026-09-18T10:55:10.000Z"
---

# Config drawers are one scrolling column — settings sections, Danger zone last

## Guideline

Every config/settings drawer follows the /data (TableDrawer) pattern: a single
scrolling column of flat `Section` groups separated by dashed hairline dividers,
with the `DangerZone` section (dashed divider + outline danger button → inline
confirm) always last. No tabbed Settings/Danger chrome.

## When it applies

- Page-editor drawers: block inspector, row settings, column settings, page settings.
- Chart config drawers (`ChartConfigDrawer`): /labs charts, /library preview config.
- Any future drawer: compose it from the shared kit (`Section`, `Field`, controls) — never hand-roll body chrome.

## Rationale

The user directed (2026-09-18) that all drawers share the styling and design
patterns of the /data route drawer. DrawerTabs was deleted in the same pass;
keeping it would fork the pattern. Danger zones are always visible at the
bottom of the scroll, matching /data's Danger zone section.
