---
type: Entity
title: Components page (/components)
description: "Card grid over every `.svelte` component the app itself uses (/components) plus a showcase-style detail page at /components/[...slug] — visual-only detail pages — live demos via ComponentDemo, source sections removed"
tags: [frontend, components, routes, developer-experience]
timestamp: "2026-09-23T06:37:31.047Z"
---

# Components page (/components)

Companion page to [pages/entities/library-page-library](./library-page-library.md): a card grid over **every component the app itself uses** — one card per `.svelte` file under `src/lib/components/` (including `charts/`, `charts/controls/`, `charts/renderers/`, `ds/`). 76 components at launch. Whereas [pages/entities/library-registry-system](./library-registry-system.md) is a curated registration point for reusable chart components, `/components` is the zero-maintenance inventory: nothing is registered, the list is generated from the filesystem.

## Why it matters

- Gives devs and agents a one-click index of the app's real UI building blocks, with a live visual demo where the component can run standalone (see [[componentdemo]]).
- Self-maintaining: a component added or deleted appears/disappears on its own — no registry edits.

## Details

- **Location**: `src/routes/components/+page.svelte`; inventory module `src/lib/app-components.ts`; tests `tests/app-components.test.ts`.
- **Inventory mechanism**: `import.meta.glob('/src/lib/components/**/*.svelte', { query: '?raw', import: 'default', eager: true })` at build time — raw sources bundled so the page can show line counts without fetches (sources are no longer displayed).
- **Card**: icon chip (Box / BarChart3 for `charts/`, Shapes for `ds/`), component name, mono meta line `133 lines · components` (folder label shown for subfolders). Folder-then-name sorted via `parseComponent(path, source)` → `{ name, dir, path, lines, source }`.
- **Detail page** (`src/routes/components/[...slug]/+page.svelte`; rest-param route so multi-segment paths like `charts/controls/Btn` resolve): showcase style after the lrs.pippeloi.nl reference — page hero (classification line, display-font title, mono `path · lines` meta, duplicate-path note for same-name components in different folders), numbered section headers `01 Visual` / `02 Source` with dashed dividers; components render directly on the page.
- **Detail page is Visual-only**: the Visual section renders [[componentdemo]] — live demos for the 33 showcaseable components (18 `ds/` self-showcase, controls kit, root kit, a real BarChart); genuinely app-wired components show a one-line honest note. The former Source section and highlight machinery were removed (2026-09-23). Unknown slugs render “Component not found” with a back link.
- **Homepage anchor row**: `/labs → · /library → · /components →` (`src/routes/+page.svelte`).

## Relationships

- [pages/entities/library-page-library](./library-page-library.md) — sibling page; `/components` reuses its card-grid design language.
- [pages/entities/library-registry-system](./library-registry-system.md) — counterpart approach: curated registry vs. filesystem glob.
- [pages/entities/design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) — shares the app-wide styling layer.

## Lifecycle

- First added: 2026-09-23 — user asked for "a similar page with cards, each card a component we use in the app found in src/lib/components" (commit `8112cc7`).
- 2026-09-23 — the 50% detail drawer was replaced by the dedicated showcase detail page at `/components/[...slug]` (commit `6b1a8f3`); card click now navigates to `/components/<dir>/<name>`.
- 2026-09-23 — Visual-only: Source sections + highlight machinery removed; live demos via `ComponentDemo.svelte` cover 33/76 components (commit `d138bf1`).
