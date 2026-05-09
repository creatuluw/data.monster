# Svelte Grid Layout Libraries for Dashboard Page Builder

Research date: 2026-05-09

---

## 1. svelte-grid-layout

| Field | Detail |
|-------|--------|
| **npm package** | `svelte-grid-layout` |
| **GitHub** | https://github.com/MarieanneWU20/svelte-grid-layout |
| **Latest version** | 1.0.3 |
| **Monthly downloads** | ~196 |
| **Stars** | 2 |
| **License** | MIT |
| **Last publish date** | Dec 2021 (npm), repo has 80 commits |

### Features
- Static grid layout: `<Grid>`, `<Row>`, `<Column>` components
- Responsive sizing (columns auto-calculate based on container)
- Width/height props for columns

### Missing Features
- **No drag & drop** (listed as "future development")
- **No resize** (listed as "future development")
- **No snap-to-grid**
- **No collision handling**
- **No layout serialization**

### Svelte 5 Compatibility
**No.** Peer dependency: `svelte: ^3.44.2`. Svelte 3 only. Uses Svelte 3 component API (no runes).

### Bundle size
~8 KB unpacked (small).

### Maintenance Status
**Unmaintained / abandoned.** Last npm publish Dec 2021. 2 stars, 1 fork, 8 open issues with no activity. Essentially a student project that was never completed.

### Pros
- Minimal footprint

### Cons
- Not a dashboard builder — static grid layout only
- No drag, no resize, no interactivity
- Svelte 3 only
- Dead project

**Verdict: NOT suitable for dashboard building.**

---

## 2. svelte-grid (vaheqelyan)

| Field | Detail |
|-------|--------|
| **npm package** | `svelte-grid` |
| **GitHub** | https://github.com/vaheqelyan/svelte-grid |
| **Latest version** | 5.1.2 |
| **Monthly downloads** | ~5,133 |
| **Stars** | ~1,000 |
| **License** | MIT |
| **Last npm publish** | Aug 2023 |

### Features
- **Drag & drop** — draggable widgets
- **Resize** — resizable widgets
- **Static widgets** support
- **Layout serialization/restoration**
- **Responsive breakpoints**
- **Min/max width/height** constraints
- **Custom dragging** handles
- **Grid gap** configuration
- **Soft autoscroll** during drag
- 100% Svelte, no 3rd party dependencies (vanilla JS drag)
- Sapper/SSR compatible

### Missing Features
- No explicit snap-to-grid (though implicit via grid positioning)
- No collision strategies (push/compress)

### Svelte 5 Compatibility
**No.** Peer dependency: `svelte: ^3.30.0` (devDep). Uses Svelte 3 API.

### Bundle size
~158 KB unpacked (moderate).

### Maintenance Status
**NOT MAINTAINED.** The repo README explicitly states "[NOT MAINTAINED]" in the title. Last npm publish Aug 2023. 419 commits, 40 open issues, 13 open PRs with no activity. The author (Vahe Araqelyan) has moved on.

### Pros
- Feature-rich for its time
- Good star count and community validation
- Responsive breakpoints
- Layout serialization

### Cons
- **NOT maintained** — explicitly abandoned by author
- Svelte 3 only (no Svelte 4 or 5 support)
- No collision strategies
- No active bug fixes or support
- Uses older rollup build tooling

**Verdict: NOT suitable for new Svelte 5 dashboard projects.**

---

## 3. svelte-grid-extended

| Field | Detail |
|-------|--------|
| **npm package** | `svelte-grid-extended` |
| **GitHub** | https://github.com/cuire/svelte-grid-extended |
| **Latest version** | 1.2.1 |
| **Monthly downloads** | ~7,566 |
| **Stars** | 115 |
| **License** | MIT |
| **Last release** | Jun 2024 |

### Features
- **Drag & drop** — draggable widgets
- **Resize** — resizable widgets
- **Static grid** mode
- **Responsive** — fluid or fixed column/row sizing
- **Layout serialization** via two-way binding
- **Collision strategies**: `none`, `push`, `compress`
- **Auto-compress** option
- **Custom move/resize handles** via slots
- **Min/max size** constraints
- **Grid gap**
- **Infinite grid** (cols=0, rows=0)
- **Grid Controller** API — `getFirstAvailablePosition()`, `compress()`
- **`change` event** for tracking updates
- **Read-only** mode
- **TypeScript** support
- Built with SvelteKit + Vite (modern tooling)

### Missing Features
- No explicit snap-to-grid toggle (grid-based positioning is implicit)
- No responsive breakpoints (unlike svelte-grid)

### Svelte 5 Compatibility
**No.** Peer dependency: `svelte: ^4.0.0`. This is the **best maintained Svelte-native option**, but it targets Svelte 4. Since Svelte 5 has breaking API changes (runes replace stores/reactivity), it would need a port. The project uses modern tooling (SvelteKit 2, Vite 5, Vitest, Playwright) so a Svelte 5 migration is technically feasible but not done.

### Bundle size
~58 KB unpacked (compact).

### Maintenance Status
**Low activity.** Last release Jun 2024 (v1.2.1). 125 commits. 18 open issues, 7 open PRs. The author (cuire) is active but updates are infrequent. Uses modern tooling (Vitest, Playwright for e2e tests), suggesting good code quality.

### Pros
- Best Svelte-native grid layout available
- Collision strategies (push/compress) — unique feature
- Grid Controller API for programmatic control
- TypeScript support
- Modern tooling (SvelteKit, Vite, Vitest, Playwright)
- Good documentation with REPL examples
- Active fork of abandoned svelte-grid

### Cons
- Svelte 4 only (not Svelte 5)
- Low maintenance frequency
- Missing responsive breakpoints from original svelte-grid
- Smaller community than alternatives
- Would need porting for Svelte 5 runes

**Verdict: Best Svelte-native option, excellent feature set for dashboards, but needs Svelte 5 port.**

---

## 4. gridstack.js

| Field | Detail |
|-------|--------|
| **npm package** | `gridstack` |
| **GitHub** | https://github.com/gridstack/gridstack.js |
| **Latest version** | 12.6.0 |
| **Monthly downloads** | ~1,734,000 |
| **Stars** | 8,900 |
| **License** | MIT |
| **Last release** | Apr 2026 |

### Features
- **Drag & drop** — native mouse + touch events, no external dependencies
- **Resize** — multi-handle resizing
- **Responsive** — column-based, mobile-friendly, breakpoints
- **Snap-to-grid** — grid-based positioning (columns + rows)
- **Layout serialization** — save/load layouts
- **Collision detection** — automatic
- **Nested grids** — sub-grid support
- **Static widgets** support
- **Min/max constraints**
- **CSS variables** for columns (v12+)
- **Touch support** — native touch events
- **Accessibility** — keyboard navigation, ARIA
- **TypeScript** — built with TS
- **Framework wrappers**: React, Angular (official in repo), Vue, Knockout, Ember, Aurelia
- **No external dependencies** (since v1)
- **3,337 commits** — very mature project

### Svelte 5 Compatibility
**Framework-agnostic.** gridstack.js is a vanilla TypeScript library that operates on DOM elements. It can be used with Svelte 5 by:

1. Import the CSS and JS directly
2. Create a Svelte 5 component wrapper using `$effect()` and DOM bindings
3. Use Svelte actions (`use:`) to initialize/destroy gridstack instances
4. Since it manipulates DOM directly (not through Svelte's reactivity), careful integration is needed for two-way data binding

No official Svelte wrapper exists, but building one is straightforward. The library's API is imperative (`.addWidget()`, `.load()`, `.save()`).

### Bundle size
~1.9 MB unpacked (large due to multiple builds). Production minified CSS+JS is much smaller (~60-80 KB gzipped). The `gridstack.min.css` + `gridstack-all.js` bundle is the standard approach.

### Maintenance Status
**ACTIVELY MAINTAINED.** Last release Apr 2026 (v12.6.0). 3 maintainers. Regular releases every few months. 56 open issues actively triaged. 8.9k stars, 1.4k forks. This is the most popular dashboard layout library on npm.

### Pros
- Mature, battle-tested (8+ years of development)
- Purpose-built for dashboards
- Huge community and ecosystem
- Excellent documentation and demos
- Framework-agnostic — works with any framework via DOM
- Responsive breakpoints out of the box
- Touch/mobile support
- Layout serialization built-in
- Active maintenance (Apr 2026)

### Cons
- No official Svelte wrapper (needs manual integration)
- Imperative API requires adapter layer for Svelte reactivity
- Larger bundle than Svelte-native solutions
- DOM manipulation may conflict with Svelte's approach
- Learning curve for integration

**Verdict: Best overall option for a dashboard builder. Massive community, actively maintained, production-proven. Needs a Svelte 5 wrapper component.**

---

## 5. @neodrag/svelte

| Field | Detail |
|-------|--------|
| **npm package** | `@neodrag/svelte` |
| **GitHub** | https://github.com/PuruVJ/neodrag |
| **Latest version** | 2.3.3 |
| **Monthly downloads** | ~42,000 |
| **Stars** | 2,400 |
| **License** | MIT |
| **Last release** | Jun 2025 |

### Features
- **Drag only** — no resize, no grid layout
- **Svelte action** API (`use:draggable`)
- Axis constraints (x, y, both, none)
- Bounds (parent, custom element, CSS selector)
- Touch support
- Cancel/disabled options
- Multi-framework (React, Vue, Solid, Svelte, vanilla)
- TypeScript
- Tiny bundle

### Missing Features
- **No resize** — this is drag-only
- **No grid layout** — no positioning system
- **No snap-to-grid**
- **No collision handling**
- **No layout serialization**

### Svelte 5 Compatibility
**Unknown / likely partial.** The package uses Svelte actions (compatible with Svelte 5 via `$effect()`), but published as Svelte 3/4 component. Latest release Jun 2025. Would need testing with Svelte 5.

### Bundle size
Very small (~2-3 KB gzipped).

### Maintenance Status
**Moderate activity.** 2,400 stars. Monorepo with 5 framework packages. Last release Jun 2025.

### Pros
- Best drag primitive for Svelte
- Tiny bundle
- Simple API via Svelte actions
- Multi-framework consistency

### Cons
- **Drag only** — no resize, no grid
- Would need significant additional code for dashboard layout
- Not a dashboard solution on its own

**Verdict: Useful as a building block for drag behavior, but NOT a grid layout solution alone.**

---

## 6. svelte-dnd-action

| Field | Detail |
|-------|--------|
| **npm package** | `svelte-dnd-action` |
| **GitHub** | https://github.com/isaacHagoel/svelte-dnd-action |
| **Latest version** | 0.9.69 |
| **Monthly downloads** | ~506,000 |
| **Stars** | 2,100 |
| **License** | MIT |
| **Last release** | Dec 2025 |

### Features
- **Drag & drop** reordering within and between containers
- **Svelte action** API (`use:dndzone`)
- Horizontal, vertical, any container shape
- **Nested dnd-zones** (Trello-like boards)
- **Touch support**
- **Keyboard accessible** (ARIA, screen reader)
- **Custom drag handles**
- **Copy on drag** patterns
- **Multi-select drag**
- **Scroll containers/window** during drag
- No external dependencies
- TypeScript support (Svelte 3/4)

### Missing Features
- **No resize** — D&D only, for list reordering
- **No grid layout** — no positioning system (HTML flow-based)
- **No snap-to-grid**
- **No layout serialization**

### Svelte 5 Compatibility
**Partial.** README has a Svelte 5 section noting using `onconsider` and `onfinalize` (instead of `on:consider`) for Svelte 5 syntax. The underlying approach (Svelte actions + event dispatching) is compatible with Svelte 5. Actively maintained with Dec 2025 release. Still pre-1.0 (v0.9.x).

### Bundle size
Small (no external deps).

### Maintenance Status
**Actively maintained.** ~500k monthly downloads. 2,100 stars. Latest release Dec 2025. The author is actively responding to issues and merging PRs. Svelte 5 support is being tracked.

### Pros
- Most popular Svelte D&D library
- Excellent features for list/board reordering
- Accessibility built-in
- Actively maintained
- Svelte 5 aware

### Cons
- **No resize** — libraries purpose is reordering, not resizing
- **No grid layout** — not a dashboard widget system
- Pre-1.0 API (may have breaking changes)
- Flow-based, not grid-positioned

**Verdict: Excellent for reorderable lists, but NOT a dashboard grid layout solution on its own.**

---

## 7. @thisux/sveltednd

| Field | Detail |
|-------|--------|
| **npm package** | `@thisux/sveltednd` |
| **GitHub** | (not found on GitHub — possibly private or new) |
| **Latest version** | 0.4.1 |
| **Monthly downloads** | ~62,000 |
| **Stars** | Unknown |
| **License** | MIT |
| **Last publish** | Apr 2026 |

### Features
- **Svelte 5 native** drag and drop
- Lightweight, flexible
- TypeScript
- SvelteKit 2 tooling

### Missing Features
- Very new (v0.4.1), limited features
- No resize
- No grid layout
- Documentation sparse (no GitHub repo found)

### Svelte 5 Compatibility
**Yes — native Svelte 5.** Peer dependency: `svelte: ^5.0.0`. Uses Svelte 5 runes. The only D&D library built specifically for Svelte 5.

### Bundle size
~84 KB unpacked.

### Maintenance Status
**Very new but active.** Published Apr 2026 (very recent).

### Pros
- Only D&D library built for Svelte 5 runes
- Active development (Apr 2026)
- TypeScript

### Cons
- Very immature (v0.4.1)
- Drag only — no resize, no grid
- No visible GitHub repo for issues/PRs
- Sparse documentation
- Not battle-tested

**Verdict: Promising Svelte 5 D&D primitive, but too immature for production dashboard use.**

---

## Honorable Mentions

### Data Grid Libraries (NOT dashboard layout tools)
- **`@svar-ui/svelte-grid`** (v2.6.2) — Data table/datagrid component, NOT a dashboard widget layout. 34k monthly downloads. Svelte 5 tagged but for data display.
- **`revolist/svelte-datagrid`** — Excel-like spreadsheet datagrid, NOT a dashboard layout. 138 stars.
- **`ag-grid-community`** — Enterprise data grid with Svelte support, NOT a dashboard widget layout.

### Framework-Agnostic Alternatives
- **`muuri`** — Responsive, sortable, filterable, draggable grid layouts. Vanilla JS. Could be wrapped for Svelte 5.
- **`packery`** — Bin-packing layout library by Metafizzy. Drag via draggabilly.
- **`interact.js`** — Drag, resize, multi-touch gestures. Very popular. Can be used as a foundation for building your own grid.

---

## Recommendations for Dashboard Page Builder with Svelte 5

### Option A: gridstack.js + Custom Svelte 5 Wrapper (RECOMMENDED)

**Best overall choice for production dashboard building.**

- Wrap gridstack.js in a Svelte 5 component using `$effect()` for init/destroy
- Use a Svelte action for DOM binding
- Bridge imperative gridstack API with Svelte's declarative reactivity
- Benefits: battle-tested, actively maintained, all dashboard features built-in
- Effort: 1-2 days to build a solid Svelte 5 wrapper

### Option B: Port svelte-grid-extended to Svelte 5

**Best choice for a pure Svelte-native solution.**

- Fork `cuire/svelte-grid-extended` and update to Svelte 5 runes
- Replace `$:` reactive statements with `$derived`/`$effect`
- Replace stores with `$state`
- Update event dispatching (on: -> on)
- Benefits: pure Svelte, smaller bundle, full control
- Effort: 1-2 weeks depending on complexity

### Option C: Build from scratch with @neodrag/svelte or svelte-dnd-action

**Maximum control, maximum effort.**

- Use `@neodrag/svelte` for drag, build resize yourself
- Use CSS grid for layout
- Build collision detection, snapping, serialization from scratch
- Benefits: zero external dependency overhead, fully custom
- Effort: 4-8 weeks

### Option D: Use @thisux/sveltednd + build grid layer

**Risky due to immaturity, but Svelte 5 native.**

- Same as Option C but using Svelte 5 native D&D
- Risk: immature library, may change API or be abandoned

---

## Summary Comparison Table

| Library | Drag | Resize | Collision | Serialize | Responsive | Svelte 5 | Maintained | Dashboard Ready |
|---------|------|--------|-----------|-----------|------------|----------|------------|-----------------|
| svelte-grid-layout | No | No | No | No | Yes | No | Dead | No |
| svelte-grid | Yes | Yes | No | Yes | Yes | No | Abandoned | Near |
| svelte-grid-extended | Yes | Yes | Yes | Yes | Fluid | No (Svelte 4) | Low | Yes |
| **gridstack.js** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Wrapper needed** | **Active** | **Yes** |
| @neodrag/svelte | Yes | No | No | No | N/A | Partial | Moderate | No (building block) |
| svelte-dnd-action | Yes | No | No | No | N/A | Partial | Active | No (building block) |
| @thisux/sveltednd | Yes | No | No | No | N/A | Yes (native) | New | No (building block) |

**Bottom line:** For a Svelte 5 dashboard page builder, `gridstack.js` with a custom Svelte 5 wrapper is the most pragmatic choice. For a pure-Svelte approach, port `svelte-grid-extended` to Svelte 5 runes.
