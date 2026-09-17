---
type: Rule
title: Drawers reuse the shared drawerResize action
description: Guideline
tags: [ui, drawers, svelte]
timestamp: "2026-09-17T10:45:15.052Z"
---

# Drawers reuse the shared drawerResize action

## Guideline

Any right-anchored overlay drawer in the app attaches the shared action, instead of hand-rolled resize logic:

```svelte
<div class="drawer" class:drawer-open={open} use:drawerResize>
```

`src/lib/components/drawer-resize.ts` injects a left-edge drag handle. Because the drawer is anchored right, dragging the edge **left** widens it: `width = startW + (startX − clientX)`, clamped to `[240px, 90vw]`. Pointer-capture based; removes the handle on destroy.

## When it applies

Every new right-anchored drawer. Current adopters: `TableDrawer.svelte`, `ColumnFunctionDrawer.svelte`, `ChartConfigDrawer.svelte`.

## Rationale

One consistent resize interaction across the app — same clamp, same cursor, same cleanup. Introduced 2026-09-17 and adopted by all three existing drawers in the same pass; per-drawer re-implementations would drift on clamps and pointer handling.
