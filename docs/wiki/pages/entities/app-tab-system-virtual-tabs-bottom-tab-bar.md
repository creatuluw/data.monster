---
type: Entity
title: App tab system (virtual tabs + bottom tab bar)
description: "The app's browser-like tab system: right-click an internal link → "Open in new tab"; the bottom bar lists the open tabs. Tabs are **virtual** — plain routes tra"
tags: [navigation, tabs, layout]
timestamp: "2026-09-17T17:01:49.911Z"
---

# App tab system (virtual tabs + bottom tab bar)

The app's browser-like tab system: right-click an internal link → "Open in new tab"; the bottom bar lists the open tabs. Tabs are **virtual** — plain routes tracked in a rune store, rendered in one webview (see the [tab-system decision](../../decisions/app-gets-virtual-multi-tab-navigation-bottom-bar.md)). Since PR #17 the bar holds **only tabs the user explicitly opened** — regular navigation never creates one.

## Details

- **Location**: `src/lib/tabs.svelte.ts` (store + actions), `src/routes/+layout.svelte` (nav-sync effect, context menu, tab-bar rendering — `.status-bar.tab-bar`)
- **Interface / Schema**:
  - `type Tab = { id: number; path: string; label: string }`
  - `tabs` — exported `$state` store exposing `list` / `activeId`; the bar renders empty when no tabs are open
  - `openInNewTab(path, label?)` — the **only** tab-creating action (push tab + `goto`)
  - Navigation re-targets the active tab **only while `activeId` is set**; plain nav with no open tab creates nothing
  - `activate(id)` / `closeTab(id)` — switch / remove; closing the **last** tab empties the bar and stays on the current route
  - `pathLabel(path)` — label fallback from the last path segment
- **Configuration**: none — pure client state, not persisted across restarts
- [Tab bar shows only explicitly opened tabs](../../decisions/tab-bar-shows-only-explicitly-opened-tabs.md) — PR #17 semantics amendment
- [design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) — the bottom bar / tab chips use the shared design system

## Lifecycle

- First added: 2026-09-17 (PR #16, 2 files +195) — replaced the empty status bar with the tab bar.
- Significant changes: 2026-09-17 (PR #17, one file +7/−11) — explicit-only tabs: bar starts empty, nav never creates a tab, last-close leaves the page instead of respawning at `/`.
