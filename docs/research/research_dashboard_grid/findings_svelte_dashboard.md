# Svelte Dashboard Grid & Layout Research Findings

Date: 2026-05-09

---

## 1. LayerChart (Already in Project)

| Field | Detail |
|-------|--------|
| **npm** | `layerchart` |
| **Version** | `2.0.0-next.62` (latest stable: `1.0.13`) |
| **GitHub** | https://github.com/techniq/layerchart |
| **Stars** | 1.2k |
| **License** | MIT |
| **Maintainer** | Sean Lynch (techniq) — very active, daily commits |

**Layout/Grid capabilities:** LayerChart is a **pure charting library** — it has NO grid layout or dashboard layout features. It provides composable SVG-based chart components (Bar, Area, Scatter, Pie, Sankey, Treemap, Geo, etc.) plus interactions (tooltip, pan/zoom, highlights). It does not offer any widget container, dashboard grid, or layout management.

**Companion package — Svelte UX:** The same author maintains `svelte-ux` (npm: `svelte-ux`, v1.0.10, GitHub: https://github.com/techniq/svelte-ux, 1.1k stars, MIT). Svelte UX is a large component library (~100+ components) that includes a **`Grid.svelte`** component and **`AppLayout.svelte`**. The `Grid.svelte` component provides CSS grid-based layout for arranging items in columns/rows with responsive breakpoints. However, it is **NOT a drag-and-drop dashboard grid** — it's a static grid layout helper.

**Relevance to dashboard:** High for charts, zero for layout. LayerChart is excellent for rendering dashboard chart widgets. Svelte UX's `Grid` component could be used for static dashboard panels. For a dynamic/resizable/draggable dashboard grid, neither LayerChart nor Svelte UX provides this — a separate solution is needed.

**Pros:** Actively maintained, TypeScript-first, excellent chart variety, Svelte 5 runes, Canvas + SVG renderers.
**Cons:** No dashboard layout; pre-release v2 API may have breaking changes; Svelte UX has Tailwind dependency.

---

## 2. Pre-built Svelte Dashboard Frameworks / Templates

### 2a. Notus Svelte (Free Admin Dashboard)

| Field | Detail |
|-------|--------|
| **npm** | Not an npm package — download from Creative Tim |
| **GitHub** | https://github.com/creativetimofficial/notus-svelte |
| **Stars** | 872 |
| **License** | MIT |
| **Last Release** | v1.1.0 (March 2021) — **abandoned/unmaintained** |

**Layout capabilities:** Uses Tailwind CSS grid classes for dashboard layouts. Static layout, not a drag-and-drop grid. Provides pre-built dashboard/admin pages with sidebar, navbar, cards, tables. Uses Svelte 3 (not SvelteKit, not Svelte 5).

**Relevance to dashboard:** Low. The template is outdated (Svelte 3/Rollup era), unmaintained for 5+ years. UI patterns could be referenced but code is not reusable for modern Svelte 5 + SvelteKit apps.

**Pros:** Free, MIT, good design reference for dashboard UI patterns.
**Cons:** Abandoned, Svelte 3 only, no SvelteKit, no drag-and-drop grid, not installable via npm.

### 2b. SvelteKit Dashboard Demo

| Field | Detail |
|-------|--------|
| **GitHub** | https://github.com/theetherGit/sveltekit-dashboard-demo |
| **Stars** | 6 |
| **License** | No license specified |
| **Last Updated** | April 2024 |

Uses SvelteKit + TailwindCSS + ShadCN-Svelte + Mode Watcher. Responsive demo with sidebar navigation. Good reference for modern SvelteKit dashboard patterns but not a library. No grid layout components.

### 2c. take-mine (SvelteKit Dashboard)

| Field | Detail |
|-------|--------|
| **GitHub** | https://github.com/mohamadadithya/take-mine |
| **Stars** | 18 |
| **License** | Not specified |
| **Last Updated** | August 2023 |

Fully customizable dashboard for SvelteKit with TailwindCSS. Very small project, limited community. No drag-and-drop grid.

---

## 3. Svelte Component Libraries with Grid/Layout Components

### 3a. shadcn-svelte

| Field | Detail |
|-------|--------|
| **npm** | `shadcn-svelte` (CLI), components copied into your project |
| **Version** | 1.2.7 |
| **GitHub** | https://github.com/huntabyte/shadcn-svelte |
| **Stars** | 8.7k |
| **License** | MIT |
| **Status** | Very actively maintained, frequent releases |

**Grid layout capabilities:** shadcn-svelte does NOT have a dedicated dashboard grid or grid layout component. It provides individual UI primitives (Button, Card, Dialog, Table, etc.) built on top of Bits UI. It is a design system, not a layout framework. Cards could be used as dashboard widgets but layout must be handled by Tailwind CSS grid classes or a custom solution.

**Relevance to dashboard:** Medium. Excellent for styling dashboard widgets (Cards, Tables, Charts inside Card components). No grid layout or drag-and-drop features. Many SvelteKit dashboards use shadcn-svelte for the widget chrome/styling.

**Pros:** Very popular, well-maintained, Svelte 5 runes, accessible (Bits UI), beautiful components.
**Cons:** No grid layout component, no drag-and-drop, no dashboard-specific features.

### 3b. Skeleton UI (Svelte package)

| Field | Detail |
|-------|--------|
| **npm** | `@skeletonlabs/skeleton-svelte` |
| **Version** | 4.15.2 |
| **GitHub** | https://github.com/skeletonlabs/skeleton |
| **Stars** | 6k |
| **License** | MIT |
| **Status** | Very actively maintained, 585 releases, multi-framework |

**Grid layout capabilities:** Skeleton is a full design system built on Tailwind. It provides layout primitives but NO dedicated dashboard grid component. The `@skeletonlabs/skeleton-common` package provides shared utilities. Components are built with Zag.js for accessibility. It has AppShell, AppBar, drawers, tabs — all useful for dashboard chrome. Grid layouts are done via Tailwind utility classes.

**Relevance to dashboard:** Medium. Good for building dashboard UI chrome (sidebar, header, cards). No drag-and-drop grid. Works well as the styling foundation for a custom-built dashboard grid.

**Pros:** Very actively maintained (4.15.2 released Apr 2026), Svelte 5 ($state runes), accessible, rich component set, theme generator.
**Cons:** Steep Tailwind dependency, no grid layout/dashboard component, large bundle.

### 3c. Flowbite Svelte

| Field | Detail |
|-------|--------|
| **npm** | `flowbite-svelte` |
| **Version** | 1.33.1 |
| **GitHub** | https://github.com/themesberg/flowbite-svelte |
| **Stars** | 2.7k |
| **License** | MIT |
| **Status** | Actively maintained, 211 releases |

**Grid layout capabilities:** No dashboard grid component. Provides Gallery (Masonry) component for image grids. Layout for dashboards would be done via Tailwind grid classes. Has cards, tables, modals, navbars, sidebars — all useful dashboard building blocks.

**Relevance to dashboard:** Medium. Good component set for dashboard UI. Gallery/masonry component could be adapted for widget grids but lacks resize/drag capabilities.

**Pros:** Official Flowbite port, large component set (40+), Tailwind CSS, actively maintained, good docs.
**Cons:** No grid layout system, CSS-heavy (43% CSS), no drag-and-drop, Tailwind dependency.

---

## 4. Drag-and-Drop / Grid Layout Libraries

### 4a. @thisux/sveltednd (SvelteDnD)

| Field | Detail |
|-------|--------|
| **npm** | `@thisux/sveltednd` |
| **Version** | 0.4.1 |
| **GitHub** | https://github.com/thisuxhq/sveltednd |
| **Stars** | 569 |
| **License** | MIT |
| **Status** | Actively maintained, Svelte 5 native |

**Grid layout capabilities:** This is a **drag-and-drop** library (Kanban, sortable lists), NOT a dashboard grid layout. It supports `direction: 'grid'` mode but this is for reordering items in a CSS grid, not for resizable dashboard panels. Key features:
- Dual-mode drag (HTML5 + pointer events)
- Vertical, horizontal, **grid** direction modes
- Drop indicators, drag handles, nested containers
- Svelte 5 runes (`$state`), TypeScript, zero dependencies

**Relevance to dashboard:** Low-medium. Could potentially be used as the foundation for a custom dashboard grid (dragging widgets between containers), but it lacks:
- Resize handles
- Position persistence (localStorage)
- Grid snapping
- Panel resize/minimize/maximize
These would all need to be built on top.

**Pros:** Svelte 5 native, lightweight, zero deps, TypeScript, MIT, good docs.
**Cons:** Not a dashboard grid library, early stage (v0.4.1), no resize support, active community but small.

### 4b. svelte-dnd-action (legacy DnD for Svelte 3/4)

| Field | Detail |
|-------|--------|
| **npm** | `svelte-dnd-action` |
| **Version** | 0.9.69 |
| **GitHub** | https://github.com/isaacHagoel/svelte-dnd-action |
| **License** | MIT |
| **Status** | Legacy — designed for Svelte 3/4 (peer: `>=3.23.0 || ^5.0.0-next.0`). Last updated Dec 2025. |

**Grid layout capabilities:** Actions-based DnD for sortable lists, nested containers. Not a dashboard grid. Same limitations as sveltednd but uses the old Svelte 4 actions API rather than Svelte 5 runes.

**Relevance to dashboard:** Low. Prefer `@thisux/sveltednd` for Svelte 5 projects.

### 4c. Svelvet (Node-based flowcharts)

| Field | Detail |
|-------|--------|
| **npm** | `svelvet` |
| **Version** | 11.0.0 (latest release on npm, v8.0.4 on GitHub tags) |
| **GitHub** | https://github.com/open-source-labs/Svelvet |
| **Stars** | 2.8k |
| **License** | MIT |
| **Status** | Development appears paused (last GitHub release v8.0.4, Jun 2023; mentions v11 in README but inconsistent). |

**Grid layout capabilities:** Svelvet is a **node-based flowchart/diagram** library (similar to React Flow). It provides a canvas with draggable nodes, edge connections, minimap, controls. It is NOT suitable for dashboard grid layouts — it's designed for building flowcharts, state machines, and node editors.

**Relevance to dashboard:** Very low. Wrong use case entirely. Snap-to-grid is for node placement in diagrams, not widget layout.

**Pros:** Good for flowchart features, snap-grid, custom nodes, minimap.
**Cons:** Not a dashboard grid, maintenance uncertain, Svelte 4 codebase (migration mentioned but not fully Svelte 5).

---

## 5. Svelte Grid Libraries (Specialized)

### 5a. svar-widgets/grid (Svelte Datagrid)

| Field | Detail |
|-------|--------|
| **npm** | `svar-widgets-grid` (presumed) |
| **GitHub** | https://github.com/svar-widgets/grid |
| **Stars** | 214 |
| **License** | MIT |
| **Status** | Updated Apr 2026, Svelte 4/5 support |

This is a **data table/datagrid** component, not a dashboard layout grid. It's for displaying tabular data with sorting, filtering, editing. Not relevant to dashboard widget layout.

### 5b. revo-grid / svelte-datagrid

| Field | Detail |
|-------|--------|
| **GitHub** | https://github.com/revolist/svelte-datagrid |
| **Stars** | 138 |
| **License** | MIT |
| **Status** | Updated May 2026 |

Excel-like data grid/spreadsheet component. Not a dashboard layout grid.

---

## 6. React Equivalent (for reference) — react-grid-layout

The React ecosystem has `react-grid-layout` (20k+ stars, MIT) which is the gold standard for dashboard widget grids. It provides:
- Draggable and resizable widgets
- Responsive breakpoints
- Layout persistence (localStorage)
- Grid snapping
- Auto-layout / compact

**There is NO direct Svelte/Svelte 5 equivalent of `react-grid-layout`.** This is a significant gap in the Svelte ecosystem. A search for `svelte-grid-layout` on npm returns no results (the npm page returns 403 but there are no matching packages listed on npm trends or GitHub topics).

---

## 7. Recommendations for data.monster Dashboard

Given the ecosystem gap, here are the realistic options ranked by feasibility:

### Option A: Build Custom Grid with Tailwind CSS (Recommended)
- Use Tailwind CSS `grid` / `grid-cols-*` classes for responsive widget layout
- Use `@thisux/sveltednd` for drag-to-reorder widgets
- Store layout configuration in a Svelte 5 `$state` store (persisted to localStorage)
- Implement resize handles manually (or via CSS `resize: both`)
- Use **LayerChart** for chart widgets and **shadcn-svelte** or **Skeleton UI** for widget card chrome

**Effort:** Medium | **Flexibility:** Maximum | **Dependencies:** Minimal

### Option B: Use Svelte UX Grid + Custom Logic
- `svelte-ux`'s `Grid.svelte` component provides basic responsive grid columns
- Layer on `@thisux/sveltednd` for drag-to-reorder
- Still requires custom resize and persistence logic

**Effort:** Medium | **Flexibility:** High | **Dependencies:** svelte-ux + tailwind

### Option C: Use a Framework-Agnostic Solution
- Consider using a vanilla JS grid library and wrapping it in Svelte
- **Muuri** (11k stars, MIT) — responsive, sortable, filterable, draggable grid layout
- **GridStack.js** (2k+ stars, MIT) — dashboard-style grid with drag, resize, serialize
- **Packery** (discontinued but stable) — bin-packing layout
- Wrap with Svelte actions or `$effect` for reactivity

**Effort:** Medium-High | **Flexibility:** High | **Dependencies:** 1 external lib

### Option D: Wait for Svelte Ecosystem Maturity
- None of the existing Svelte component libraries plan a dashboard grid feature
- The ecosystem may develop one in 6-12 months but there's no clear signal

---

## 8. Summary Table

| Library | Type | Grid Layout? | DnD? | Resize? | Svelte 5? | Stars | Recommendation |
|---------|------|-------------|------|---------|-----------|-------|----------------|
| LayerChart | Charts | No | No | No | Yes (v2) | 1.2k | Use for charts |
| Svelte UX | Component library | `Grid.svelte` (static) | No | No | Yes | 1.1k | Use for static grids |
| shadcn-svelte | Design system | No | No | No | Yes | 8.7k | Use for card/UI chrome |
| Skeleton UI | Design system | No | No | No | Yes | 6k | Use for dashboard chrome |
| Flowbite Svelte | Component library | No (Gallery/Masonry only) | No | No | Yes | 2.7k | Use for card/UI components |
| @thisux/sveltednd | DnD library | Grid mode for reorder | Yes | No | Yes | 569 | Use for drag reorder |
| svelte-dnd-action | DnD (legacy) | No | Yes | No | Partial | N/A | Skip (legacy) |
| Svelvet | Node editor | Snap-grid (diagrams) | Yes (nodes) | Yes | No (Svelte 4) | 2.8k | Skip (wrong use case) |
| react-grid-layout | React grid | Yes | Yes | Yes | N/A | 20k+ | Reference only |

---

## 9. Key Finding

**There is no pre-built, Svelte 5-native dashboard grid layout library.** Every existing Svelte dashboard template uses static Tailwind CSS grid layouts. For a dynamic dashboard with draggable, resizable widgets, you must either:

1. Build a custom Svelte 5 solution using `@thisux/sveltednd` + Tailwind CSS grid
2. Wrap a framework-agnostic library like GridStack.js or Muuri
3. Consider moving to React for dashboard-heavy UIs (not recommended, given SvelteKit investment)

**Option 1 is the recommended path** — it keeps dependencies minimal and gives full control over the user experience.
