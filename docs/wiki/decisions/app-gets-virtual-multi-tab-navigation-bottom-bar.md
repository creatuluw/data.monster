---
type: Decision
title: "App gets virtual multi-tab navigation: bottom bar is the tab bar"
description: Context
tags: [navigation, layout, tab-bar, frontend]
status: accepted
timestamp: "2026-09-17T16:58:38.989Z"
---

# App gets virtual multi-tab navigation: bottom bar is the tab bar

## Context

The user wanted browser-like tabbed navigation inside the desktop app: right-click any link to another route (or a navigating button) → "Open in new tab", with the tabs shown in the bottom bar — the bottom bar **is** the tab bar.

## Choice

**Virtual tabs in the single webview** (PR #16, 2026-09-17), not OS-level windows:

- A `$state`-backed store in `src/lib/tabs.svelte.ts` owns `{ list, activeId }`; a tab is just `{ id, path, label }` — no per-tab component state.
- `ensureActive(path, label)` runs on every navigation (`$page` subscribe in the root layout): normal nav re-targets the current tab; `openInNewTab` pushes a tab + `goto`s.
- A custom context menu (`<svelte:window oncontextmenu>` in `+layout.svelte`) offers **Open in new tab** on internal `<a href>` links; the bottom status bar renders the tab chips (click = activate, × = close; closing the last tab lands on `/` with a fresh tab).
- Tab labels reuse breadcrumb route labels.

## Alternatives considered

- **Tauri multi-window / multi-webview** (real OS tabs) — heavier lifecycle, loses shared DuckDB-state ergonomics of one webview, way more code for the same UX.
- **Browser-native context menu** — webview menu can't be extended with app actions; a custom menu is required.

## Known limitation

Right-click only works on real `<a href>` links. Buttons that navigate via `goto()` expose no destination, so they keep re-targeting the current tab; specific buttons can be tagged opt-in later if wanted.
