---
type: Entity
title: Components catalog (/components + app-components.ts)
description: "The `/components` route pair — a build-time inventory of every Svelte component in the app, next to [[library-page-library]] (which lists registered library com"
tags: [components, frontend, design-system, catalog]
timestamp: "2026-09-23T07:15:08.857Z"
---

# Components catalog (/components + app-components.ts)

The `/components` route pair — a build-time inventory of every Svelte component in the app, next to [library-page-library](./library-page-library.md) (which lists registered library components). `/components` lists all of `src/lib/components/**/*.svelte`; `/components/<slug>` is Visual-only (lrs.pippeloi.nl style): a live demo where the component runs standalone, an honest note otherwise — no source view.

## Details

- **Location**: `src/lib/app-components.ts` (inventory), `src/routes/components/+page.svelte` (catalog), `src/routes/components/[...slug]/+page.svelte` (detail)
- **Interface**: `parseComponent(path, source)` → `AppComponent {name, dir, path, lines, source}`; `loadAppComponents()` → dedupes by name with folder preference (`ds` > `charts/controls` > `charts` > root) — 58 active components, not 76 file entries
- **Discovery**: `import.meta.glob('/src/lib/components/**/*.svelte', {query: '?raw', eager: true})` at build time — new components appear automatically, no registration
- **Live demos**: `ComponentDemo.svelte` renders demos for 38 of 76 components — the 18 `ds/` self-showcase, the `charts/controls/` kit, root-kit components, and a real `BarChart` on dummy rows; app-wired components keep the honest note
- **Collision rule**: one active component per name — the loader dedupes by folder preference (`ds` > `charts/controls` > `charts` > root); the 18 root twins plus root `Toggle` are confirmed dead code (zero importers), legacy bare-name URLs still resolve to the active variant (see the learning in this wiki)
- **Hero**: classification → title → path/lines → Visual — the "Also at:" dup note and its sibling logic were removed (2026-09-23)

## Relationships

- [design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) — the token + component layer this catalog browses; `ds/` components are its self-showcasing half
- [library-registry-system](./library-registry-system.md) — /library is the registered-component counterpart; /components is the raw inventory

## Lifecycle

- 2026-09-23: catalog + showcase detail page shipped (`8112cc7`, `6b1a8f3`); same day slug collisions fixed to prefer the `ds/` showcase variant (`f19dd80`)
- 2026-09-23: Visual-only detail pages — source view removed, demos expanded to 33/76 via `ComponentDemo.svelte` (`d138bf1`)
- 2026-09-23: five root-only demoable components (Badge, Breadcrumb, LabsPlaceholder, PreviewPane, SearchAhead) got live demos — coverage 33→38 (`9799535`)
- 2026-09-23: loader dedupes by folder preference — one active component per name (58 entries), "Also at" dup note removed (`e8e329a`)
