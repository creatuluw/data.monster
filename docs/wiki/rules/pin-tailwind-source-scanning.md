---
type: Rule
title: Pin Tailwind @source scanning to src/ and app.html in app.css
description: Pin Tailwind @source scanning to src/ and app.html in app.css
tags: [tailwind, vite, conventions, frontend]
timestamp: "2026-09-14T09:43:37.653Z"
---

# Pin Tailwind @source scanning to src/ and app.html in app.css

# Pin Tailwind @source scanning to src/ and app.html in app.css

## Guideline

`src/app.css` must keep its Tailwind 4 `@source` directives pinned:

```css
@source './**/*.{svelte,ts,html}';
@source '../app.html';
```

Do not widen them back to scanning the repo root, and do not remove them "to pick up classes from new folders" without pinning the new folder explicitly.

## When it applies

Any edit to `src/app.css` or any time Tailwind seems to "miss" classes — the lazy fix is adding an explicit `@source` for the new folder, not unpinning.

## Rationale

Without the pin, the Tailwind dev watcher scans the whole project tree. *Any* file write — vite's own log files, wiki writes (`docs/wiki/`), agent session files — makes Tailwind re-emit `app.css` → SvelteKit performs a full reload → the writing continues → infinite reload loop (the `/labs` hang amplifier, 2026-09-14). The repo generates constant non-source file churn during agent-assisted development, so this loop is not hypothetical here. Full story: [[labs-hang-vite-reload-loop]].

## Source

- `src/app.css` lines 6–7 — the pin
- PR creatuluw/data.monster#3, commit `9f18749`
