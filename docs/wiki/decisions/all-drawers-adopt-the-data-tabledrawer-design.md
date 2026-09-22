---
type: Decision
title: All drawers adopt the /data (TableDrawer) design pattern — DrawerTabs removed
description: Context
tags: [drawers, design-system, data-drawer-pattern, controls-kit]
status: accepted
supersedes: drawer-chrome-restyle-reverted-control-kit-stands
timestamp: "2026-09-18T10:55:35.018Z"
---

# All drawers adopt the /data (TableDrawer) design pattern — DrawerTabs removed

## Context

All drawers must share the styling and design patterns of the /data route drawer (TableDrawer) — direct user instruction, 2026-09-18. The /data pattern: flat section groups separated by dashed hairline dividers, 9px uppercase micro-labels/section-titles, mono for data values, outline danger button that flips into an inline confirm panel (Cancel / Delete permanently), single scrolling column — no tabbed Settings/Danger chrome.

This supersedes the restraint in [[drawer-chrome-restyle-reverted-control-kit-stands]]: that revert rejected motifs synthesized from the lms/kees reference projects. This change is different in kind — it copies an in-app surface (/data) at the user's explicit direction, verified visually against the live app.

## Decision

- `Section` (controls kit) renders flat — no card border/background; consecutive sections get a dashed top divider via `.section + .section`; title 9px/700/0.1em uppercase tertiary.
- `Field` labels are 9px/600/0.06em uppercase tertiary (TableDrawer's drawer-label); kit inputs use radius-xs to match `.input`.
- `DangerZone` renders the /data danger flow: "Danger zone" micro-title, outline danger trigger (confirmLabel), inline confirm panel with description + Cancel/"Delete permanently". Internal confirm state; API unchanged.
- `DrawerTabs` deleted; block/row drawers in /pages show settings sections then DangerZone (danger always visible at the bottom).
- /library config snippet and /labs bar-chart + heatmap config snippets were converted from hand-rolled zinc controls to the kit.
- BlockInspector's redundant in-body title removed (drawer header owns the title).

## Consequences

- The "config drawers use DrawerTabs" rule is replaced: one scrolling column, Danger zone last.
- The /labs + /library pages now import the kit (Section/Field/Select/Toggle/TextInput/NumberInput) inside their config snippets.
- Verification was done via the mocktauri harness (documented repro technique) + CDP screenshots against the dev server.
