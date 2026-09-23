---
type: Entity
title: Components page (/components)
description: "Companion page to [[pages/entities/library-page-library]]: a card grid over **every component the app itself uses** — one card per `.svelte` file under `src/lib"
tags: [frontend, components, routes, developer-experience]
timestamp: "2026-09-23T06:37:31.047Z"
---

# Components page (/components)

Companion page to [pages/entities/library-page-library](./library-page-library.md): a card grid over **every component the app itself uses** — one card per `.svelte` file under `src/lib/components/` (including `charts/`, `charts/controls/`, `charts/renderers/`, `ds/`). 76 components at launch. Whereas [pages/entities/library-registry-system](./library-registry-system.md) is a curated registration point for reusable chart components, `/components` is the zero-maintenance inventory: nothing is registered, the list is generated from the filesystem.

## Why it matters

- Gives devs and agents a one-click index of the app's real UI building blocks, with source view + Copy for reuse in drawers, pages, and agents authored content.
- Self-maintaining: a component added or deleted appears/disappears on its own — no registry edits.

## Details

- **Location**: `src/routes/components/+page.svelte`; inventory module `src/lib/app-components.ts`; tests `tests/app-components.test.ts`.
- **Inventory mechanism**: `import.meta.glob('/src/lib/components/**/*.svelte', { query: '?raw', import: 'default', eager: true })` at build time — raw sources bundled so the page can show line counts and full source without fetches.
- **Card**: icon chip (Box / BarChart3 for `charts/`, Shapes for `ds/`), component name, mono meta line `133 lines · components` (folder label shown for subfolders). Folder-then-name sorted via `parseComponent(path, source)` → `{ name, dir, path, lines, source }`.
- **Detail drawer**: 50% drawer (house pattern — `drawerResize` action, overlay/Escape close) with path + line count in the header, full source in a scrollable mono `<pre>`, Copy button.
- **Homepage anchor row**: `/labs → · /library → · /components →` (`src/routes/+page.svelte`).

## Relationships

- [pages/entities/library-page-library](./library-page-library.md) — sibling page; `/components` reuses its card-grid design language.
- [pages/entities/library-registry-system](./library-registry-system.md) — counterpart approach: curated registry vs. filesystem glob.
- [pages/entities/design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) — shares the app-wide styling layer.

## Lifecycle

- First added: 2026-09-23 — user asked for "a similar page with cards, each card a component we use in the app found in src/lib/components" (commit `8112cc7`).
