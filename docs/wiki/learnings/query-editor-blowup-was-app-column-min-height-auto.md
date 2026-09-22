---
type: Learning
title: "Query editor blowup was .app-column min-height:auto — mock-Tauri browser repro technique"
description: "Symptom: on /query, clicking a Data-source table made the SQL editor pane "huge" (1689px in a 786px window) while the initial page looked fine."
tags: [css, flexbox, layout, query-page, debugging]
timestamp: "2026-09-17T11:35:05.562Z"
---

# Query editor blowup was .app-column min-height:auto — mock-Tauri browser repro technique

Symptom: on /query, clicking a Data-source table made the SQL editor pane "huge" (1689px in a 786px window) while the initial page looked fine.

Root cause: `.app-shell` (100vh flex column) → `.app-column { flex: 1 }` had NO `min-height: 0`. Flex items default to `min-height: auto` (min-content floor), so once the query page's content min-content (100-row result table + multi-line query) exceeded the viewport, `.app-column` grew past 100vh (4613px), every inner `flex: 1` / `min-height: 0` chain followed the inflated parents, and the editor pane (height: 38% of `.editor-results-split`) ballooned. The inner chain (`min-height: 0` + `overflow: hidden` on every level) is useless unless EVERY flex-item level from the 100vh root down also releases `min-height: auto` — `.app-column` was the one missing link. Fix: one line, `min-height: 0` on `.app-column` in `src/routes/+layout.svelte`.

Verified: vite build + `vite preview` on a spare port, Tauri mocked by injecting `window.__TAURI_INTERNALS__` (invoke returning fake workspace/tables/DESCRIBE/COUNT/SELECT results) in `src/app.html` guarded by `?mocktauri=1` — the full app boots in a plain browser and the click flow reproduces/verifies. GOTCHA: after rebuilding, RESTART `vite preview` — it resolves the asset manifest at startup and keeps serving the OLD hashed CSS to cache-busted navigations, which silently defeats re-verification.

Static DOM repro of just the page's own CSS is not enough for layout bugs — the bug lived in the layout chain above the page, and only the full app build showed it.
