---
type: Entity
title: Library registry system (src/lib/library + /library routes)
description: "The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the "
tags: [library, registry, charts, central-charts]
timestamp: "2026-09-17T07:18:39.178Z"
---

# Library registry system (src/lib/library + /library routes)

The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the central chart registry, so an extension-style component package becomes usable app-wide (library page, page-editor block picker, schema-driven config panel) by construction.

## Details

- **Location**: `src/lib/library/` — `types.ts` (`LibraryEntry`, `LibraryDemo`), `registry.ts` (`registerLibraryComponent` → also calls `registerChartType`, plus `getLibraryComponents`/`getLibraryComponent`), and `components/<type>/` packages
- **Routes**: `/library` (card grid) + `/library/[id]` (detail view, tabs: Preview / Schema / Code / Docs — Preview default)
- **Registration**: one `registerLibraryComponent(entry)` line per component in `src/lib/charts/registry-setup.svelte.ts` (currently bar-chart + heatmap)
- **Extension contract** — one folder per component:
  - `def.ts` — `ChartTypeDefinition` (roles, optionsSchema, hooks, annotations)
  - `demo.ts` — dummy rows + aliases (no DuckDB needed)
  - `docs.md` — usage notes (Docs tab)
  - `index.ts` — wires def + REAL renderer + demo + `?raw` source (Code tab)
- **Demos render the real renderer** with bundled dummy query-shaped data — no demo-only clones, no workspace required
- `app.d.ts` references `vite/client` so `?raw` imports type-check
- Table/text page-editor blocks stay built-ins, not library entries

## Relationships

- Implements [library-registry-drives-editor-and-library](../../decisions/library-registry-drives-editor-and-library.md) (see [the decision](../../decisions/library-registry-drives-editor-and-library.md)) — supersedes display-only v1
- Built on [chart-page-spec-spec-types-validator](./chart-page-spec-spec-types-validator.md) — `ChartTypeDefinition` is the same type driving `PageGrid` renderers and `BlockInspector` panels
- The page editor's add-block picker iterates the registry with role-aware defaults (e.g. heatmap pre-gets 2 dimensions); config panels are registry-driven via `getChartType`
- Related decisions: [library-extension-style-components](../../decisions/library-extension-style-components.md), [library-demos-reuse-real-components](../../decisions/library-demos-reuse-real-components.md), [library-q4-dedicated-tabbed-views](../../decisions/library-q4-dedicated-tabbed-views.md)

## Lifecycle

- First added: 2026-09-17 — full scope in one pass: registry + `/library` routes + editor picker wiring; 85 tests pass, build green
