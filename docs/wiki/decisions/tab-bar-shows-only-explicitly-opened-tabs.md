---
type: Decision
title: Tab bar shows only explicitly opened tabs — navigation never creates tabs
description: Context
tags: [navigation, tabs, ux]
status: accepted
timestamp: "2026-09-17T17:01:26.482Z"
---

# Tab bar shows only explicitly opened tabs — navigation never creates tabs

## Context

PR #16 shipped the virtual tab system with a bottom tab bar. As first built, every navigation kept a tab in play (`ensureActive` on every nav) and closing your last tab spawned a fresh one at `/` — so the bar filled with every route you visited, not just tabs you asked for. The user asked: "in the tabs bar show only tabs I opened with open in new tab."

## Choice (PR #17, +7/−11, one file: `src/lib/tabs.svelte.ts`)

- The bar **starts empty**. Only **Open in new tab** (right-click context menu) creates a tab chip.
- Navigation **re-targets a tab only while you are *in* an opened tab**. Plain navigation with no active tab creates nothing.
- Closing your **last** tab empties the bar and **leaves you on the current page** (no auto-respawn at `/`).

## Alternatives considered

- Auto-create a tab per visited route (browser-history style, as the first build effectively did) — rejected: the bar became clutter, defeating its purpose.
- Respawn a home tab on last close — rejected: surprising navigation the user didn't ask for.

## Consequences

- Tabs are now purely user-created; any future nav code must not auto-spawn tabs.
- Amends (does not supersede) [[app-gets-virtual-multi-tab-navigation-bottom-bar]] — the tab system itself is unchanged, only its creation/retention semantics.
