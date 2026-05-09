# Vanilla/ Framework-Agnostic JavaScript Grid Layout Libraries for Dashboard Building

Research conducted May 2026. Focus: drag-and-drop dashboard builder, desktop app, Svelte 5 integration.

---

## 1. Gridstack.js — ★ Top Recommendation

| Field | Detail |
|---|---|
| **npm** | `gridstack` |
| **GitHub** | https://github.com/gridstack/gridstack.js |
| **Latest version** | v12.6.0 (April 8, 2026) |
| **Maintenance** | **Actively maintained** — 130 releases, 3,337 commits, multi-year continuous development by Alain Dumesny |
| **License** | MIT |
| **Stars** | 8.9k |
| **Weekly downloads** | ~445,000 |
| **Bundle size** | 83.9 kB min / 22.9 kB gzip (zero dependencies) |
| **TypeScript** | Yes, first-class TS support with built-in type declarations |

### Key Features
- **Drag & drop** — native mouse/touch events (no jQuery required since v6), between-grid drag in/out
- **Resize** — handles on all sides configurable (`e,se,s,sw,w`), min/max constraints
- **Responsive** — column breakpoints (`columnOpts`), auto column sizing, CSS variable-based layout (v12+)
- **Snap-to-grid** — column-based implicit grid, cell height configurable
- **Nested grids** — sub-grids supported, drag items between parent/child grids
- **Serialization** — `grid.save()` / `grid.load()` with full layout state
- **Static mode** — can disable drag/resize for display-only grids
- **Lazy loading** — added in v11
- **RTL support** — full RTL added in v12.6
- **Touch/mobile** — native touch support since v6
- **Side panel** — helper for dragging new widgets from a palette into the grid
- **Custom layout engine** — extend/replace the default engine (v5.1+)
- **`renderCB` / `addRemoveCB`** — framework-friendly callbacks for Angular/React/Vue/Svelte

### Framework Agnostic?
**Yes.** Zero dependencies, vanilla TypeScript. Ships built-in Angular component wrapper. Has official React wrapper (in repo), Vue demos (v2+v3), Knockout, Ember, Aurelia integrations available.

### Svelte 5 Integration
Excellent candidate. The `renderCB` and `addRemoveCB` callbacks (v11+) let you control DOM creation, which maps cleanly to Svelte's `mount()` for client-side rendering of widget content. No Svelte-specific wrapper exists but one is trivial to build — init GridStack on a `div`, use `renderCB` to mount Svelte components into each widget's content div. The imperative API (`grid.addWidget()`, `grid.removeWidget()`, `grid.update()`) integrates cleanly with Svelte 5 `$effect` runes.

### Pros for Dashboard Building
- **Purpose-built** for dashboards — this is the primary use case
- **Batch-tested** — 445k weekly downloads, battle-tested at scale
- **Excellent serialization** — save/load entire dashboard layouts as JSON
- **Column-based grid** — natural for dashboard panels (widgets span N columns)
- **Responsive breakpoints** — different column counts at different widths
- **No framework lock-in** — works with any framework or vanilla JS
- **Well-documented** — extensive API docs, 130+ version migration guides, many demos

### Cons
- **No free-form pixel placement** — items always snap to the column grid (by design)
- **No item grouping** — no built-in "group" or "tab" container widget
- **Single maintainer** — bus factor of 1 (Alain Dumesny)
- **CSS class churn** — API has changed significantly across major versions (though v12 seems stable)
- **v11 breaking change** — `addWidget` no longer accepts HTML strings directly, requires `renderCB`

### Demo
- Homepage: https://gridstackjs.com
- Demos: https://gridstackjs.com/demo/
- API docs: https://gridstack.github.io/gridstack.js/doc/html/

---

## 2. Muuri

| Field | Detail |
|---|---|
| **npm** | `muuri` |
| **GitHub** | https://github.com/haltu/muuri |
| **Latest version** | v0.9.5 (July 9, 2025) |
| **Maintenance** | **Low activity** — last release ~10 months ago, 522 commits total, sporadic releases. v0.9.4 (May 2025), v0.9.3 (Aug 2024), v0.9.0 (Jun 2024). Active issues/PRs but slow merge cadence. |
| **License** | MIT |
| **Stars** | 11k |
| **Bundle size** | 83.5 kB min / 23.1 kB gzip (zero dependencies) |
| **TypeScript** | Typings added in v0.9.0, but library itself is vanilla JS |

### Key Features
- **Drag & drop** — custom drag engine (Hammer.js deprecated in v0.8), between-grid drag supported
- **Sort** — programmatic sorting by any attribute
- **Filter** — show/hide items with animations
- **Custom layouts** — provide your own layout algorithm function, async via web workers
- **Nested grids** — supported
- **Auto-scroll** — during drag (v0.9+)
- **Drag placeholder** — visual placeholder during drag
- **Drag axis** — constrain to horizontal or vertical
- **Drag handle** — CSS selector for drag handle element
- **Drag sort heuristics** — fine-tuned sort behavior (interval, min distance, bounce-back angle)
- **Web Animations API** — uses native WAAPI for animations (polyfill available)

### Framework Agnostic?
**Yes.** Pure vanilla JS, no dependencies. Works with any DOM.

### Svelte 5 Integration
Feasible but more work than Gridstack. Muuri manages item positions via absolute positioning — you'd need to manage the item DOM yourself and call Muuri's `add()`/`remove()` methods. The custom layout function could be used to implement column-based dashboard layouts, but it's not built-in. No `renderCB` equivalent.

### Pros
- **Extremely flexible layout** — custom layout function means you can implement any positioning logic
- **Web worker layout** — async layout calculations for performance
- **Rich animation** — WAAPI-based, show/hide/layout/drag-release animations configurable
- **Sort & filter built-in** — good for dynamic dashboards
- **Drag between grids** — could support dragging widgets between dashboard tabs

### Cons
- **No resize handles** — Muuri has no built-in resize functionality. You'd need to implement resize handles yourself.
- **No column/snap-to-grid** — no built-in grid system; you must implement the layout algorithm
- **Not dashboard-focused** — designed for general sortable/filterable layouts, not dashboards
- **Lower maintenance** — slower release cadence, unclear future
- **Still pre-1.0** — API may change
- **No serialization** — no built-in save/load

### Demo
- https://muuri.dev/
- Codepen collection: https://codepen.io/collection/AWopag/

---

## 3. Packery

| Field | Detail |
|---|---|
| **npm** | `packery` |
| **GitHub** | https://github.com/metafizzy/packery |
| **Latest version** | v2.1.2 (June 29, 2018) |
| **Maintenance** | **Abandoned** — no updates in ~8 years |
| **License** | MIT |
| **Stars** | 4.3k |
| **Bundle size** | 30.9 kB min / 9.0 kB gzip (2 dependencies: outlayer, get-size) |
| **TypeScript** | No |

### Key Features
- Bin-packing gapless grid layout (fills gaps unlike Masonry)
- Draggable items (requires additional Draggabilly library)
- No resize handles
- No responsive breakpoints built-in
- jQuery or vanilla JS API

### Verdict
**Do not use for new projects.** Dead project. Muuri describes itself as a combination of Packery + Masonry + Isotope + Sortable, making all three Metafizzy libraries obsolete.

---

## 4. Masonry

| Field | Detail |
|---|---|
| **npm** | `masonry-layout` |
| **GitHub** | https://github.com/desandro/masonry |
| **Latest version** | v4.2.2 (July 4, 2018) |
| **Maintenance** | **Abandoned** — no updates in ~8 years |
| **License** | MIT |
| **Stars** | 16.7k |
| **Bundle size** | 22.2 kB min / 6.9 kB gzip (2 dependencies) |
| **TypeScript** | No |

### Key Features
- Cascading grid layout (Pinterest-style)
- No drag, no resize, no snap-to-grid

### Verdict
**Not suitable for dashboards.** Layout-only, no interactivity. Abandoned.

---

## 5. Isotope

| Field | Detail |
|---|---|
| **npm** | `isotope-layout` |
| **GitHub** | https://github.com/metafizzy/isotope |
| **Latest version** | v3.0.5 (April 6, 2018) |
| **Maintenance** | **Abandoned** — no updates in ~8 years |
| **License** | GPLv3 (commercial license available) |
| **Stars** | 11.1k |
| **TypeScript** | No |

### Key Features
- Filter and sort layouts
- Multiple layout modes (masonry, fitRows, vertical, etc.)
- No drag, no resize

### Verdict
**Not suitable for dashboards.** No drag/resize. Abandoned. GPL license problematic.

---

## 6. griddle.js

**Does not exist** — 404 on GitHub, npm blocked. Possibly a typo or abandoned project. No trace found.

---

## 7. Other Alternatives Worth Noting

### Golden Layout
- **npm**: `golden-layout`
- **GitHub**: https://github.com/golden-layout/golden-layout
- Multi-window layout manager (IDE-style tabbed panes, split panes)
- v2 is a complete rewrite (in progress), v1 is stable but older
- **Focus**: window management, not widget grids
- **Not recommended** for widget dashboard — overkill and wrong paradigm

### React-Grid-Layout / Vue-Grid-Layout
- Framework-specific wrappers around a React/Vue reactive grid
- **Not framework-agnostic** — React-Grid-Layout requires React
- Popular in their ecosystems but not suitable for a Svelte app

### Dashibase (insert)
- A Notion-like dashboard builder
- Not a library — a SaaS product

---

## Summary Comparison

| Library | Drag | Resize | Snap-to-Grid | Responsive | Maintained | Bundle (gzip) | License |
|---|---|---|---|---|---|---|---|
| **Gridstack.js** | Yes | Yes | Yes (column) | Yes | **Active (Apr 2026)** | 22.9 kB | MIT |
| **Muuri** | Yes | **No** | No (custom) | Manual | Low (Jul 2025) | 23.1 kB | MIT |
| **Packery** | External | No | No | No | Dead (2018) | 9.0 kB | MIT |
| **Masonry** | No | No | No | No | Dead (2018) | 6.9 kB | MIT |
| **Isotope** | No | No | No | No | Dead (2018) | ~8 kB | GPLv3 |

## Recommendation

**Gridstack.js is the clear winner** for a drag-and-drop dashboard builder:

1. It is the **only actively maintained** library with dashboard-specific features (drag + resize + column grid + responsive + serialization).
2. Zero dependencies, vanilla TypeScript — easy to wrap in a Svelte 5 component.
3. The `renderCB` + `addRemoveCB` + `updateCB` callbacks provide clean integration points for Svelte component mounting.
4. 445k weekly downloads and 130 releases indicate a mature, battle-tested library.
5. Muuri is the only credible alternative but lacks resize handles and is less frequently maintained. It could serve as a supplementary library for sorting/filtering individual panels but not as the core dashboard grid.

### Suggested Svelte 5 Wrapper Approach

```svelte
<script lang="ts">
  import { GridStack } from 'gridstack';
  import 'gridstack/dist/gridstack.min.css';
  import { onMount } from 'svelte';

  let gridEl: HTMLDivElement;
  let grid: GridStack;

  onMount(() => {
    grid = GridStack.init({
      column: 12,
      cellHeight: 80,
      // ... options
    }, gridEl);
  });
</script>

<div bind:this={gridEl} class="grid-stack"></div>
```

For Svelte 5 with runes, a class-based wrapper using `$state` and `$effect` would provide reactive layout management.
