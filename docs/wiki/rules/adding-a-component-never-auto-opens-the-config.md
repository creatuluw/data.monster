---
type: Rule
title: Adding a component never auto-opens the config drawer — skeleton is the start state; drawer-open seeds picker rows
description: When a component is added to a page-editor row (`addComponent` in the page editor), the **config drawer must NOT auto-open**. The newly added block stays on the
tags: [central-charts, page-editor, ux, config-drawer]
timestamp: "2026-09-17T14:33:20.445Z"
---

# Adding a component never auto-opens the config drawer — skeleton is the start state; drawer-open seeds picker rows

When a component is added to a page-editor row (`addComponent` in the page editor), the **config drawer must NOT auto-open**. The newly added block stays on the canvas as its skeleton (per [[chart-blocks-start-empty-needssetup-gate]]) until the user explicitly opens the drawer — via the cog icon or the skeleton's Add dimension / Add measure buttons. Auto-opening was a bug (PR #8, 2026-09-17): it hid the skeleton state the needsSetup design is built around.

The complementary half: when the drawer **does** open on an empty chart, it seeds one picker row per unfilled role (once per mount). The user starts from a guided row, not a blank list — but deleting the seeded row still works and no data renders until role minimums are met.

Applies to: any code path that adds a block to a page (`+ Component`, future drag-drop, agent-driven APIs). Never open the config drawer from an add path; open it only from a user click.
