# Dashboard Drag-and-Drop Grid Layout Libraries

Research Date: 2026-05-09
Focus: React ecosystem libraries and their Svelte/vanilla JS portability

---

## 1. react-grid-layout

| Field | Details |
|-------|---------|
| **npm** | `react-grid-layout` |
| **GitHub** | https://github.com/react-grid-layout/react-grid-layout |
| **Stars** | 22,200+ |
| **Latest** | v2.x (2025-2026, TypeScript rewrite) |
| **License** | MIT |
| **Status** | Actively maintained |

### Key Features
- Draggable AND resizable dashboard widgets
- Responsive breakpoints with per-breakpoint layouts
- CSS Transform or absolute positioning strategies
- Pluggable compaction algorithms (vertical, horizontal, custom)
- Drag from outside, bounded layouts, static elements
- **v2 modular architecture**:
  - `react-grid-layout` — React components + hooks
  - `react-grid-layout/core` — **Pure layout algorithms (framework-agnostic!)**
  - `react-grid-layout/legacy` — v1 compatibility
  - `react-grid-layout/extras` — optional components
- React 18+, full TypeScript
- Hooks: `useContainerWidth`, `useGridLayout`, `useResponsiveLayout`

### Svelte Port/Compatibility
- **No official Svelte port exists** (several community attempts have been abandoned/404)
- **CRITICAL for Svelte use**: v2 exposes `react-grid-layout/core` — pure layout engine functions (collide, compact, moveElement, validateLayout, etc.) with ZERO React dependency. This means the **algorithmic heart** of RGL can be consumed from any framework.
- A Svelte wrapper would need to:
  1. Import layout algorithms from `react-grid-layout/core`
  2. Wire mouse/touch event handling to `useGridLayout`-equivalent state management
  3. Provide Svelte component wrappers around the positioning logic

### Pros for Svelte Dashboard
- Battle-tested by Grafana, Metabase, Kibana, HubSpot, BitMEX
- Core algorithms are pure functions — directly importable
- 871 commits, very mature
- Full TypeScript types

### Cons for Svelte Dashboard
- No official/native Svelte bindings
- React component layer cannot be reused (would need re-implementation)
- Depends on `react-resizable` for resize handles (React-only)
- Significant effort to port UI layer, though algorithms are ready to go

---

## 2. gridstack.js

| Field | Details |
|-------|---------|
| **npm** | `gridstack` |
| **GitHub** | https://github.com/gridstack/gridstack.js |
| **Stars** | 8,900+ |
| **Latest** | v12.6.0 (Apr 2026) |
| **License** | MIT |
| **Status** | Very actively maintained |

### Key Features
- **Vanilla JS/TypeScript** — no framework dependency required
- Mobile-friendly, responsive dashboards
- Drag and drop, resize, serialize/deserialize layout
- Multi-column responsive (CSS variables in v12+)
- Nested grids, sub-grids
- Built-in bindings for Angular and React (and Vue demo)
- Pure TypeScript — no jQuery since v3
- 3,337 commits — very mature

### Svelte Port/Compatibility
- **Highly compatible**. Because GridStack is framework-agnostic vanilla JS, wrapping it in Svelte requires only:
  - A Svelte action or component that calls `GridStack.init()` on mount
  - Reactive binding to layout state via GridStack events
- No existing official Svelte wrapper, but significantly easier than RGL because there's no React layer to strip away
- React wrapper code exists at `gridstack.js/react/` for reference on patterns

### Pros for Svelte Dashboard
- Vanilla JS core — trivial to wrap in Svelte
- Very actively maintained (most recent: Apr 2026)
- Battle-tested (8.9k stars)
- Built-in serialization, responsive breakpoints, nested grids
- CSS variable-based layout (no extra CSS needed for custom columns in v12+)

### Cons for Svelte Dashboard
- No existing Svelte ecosystem (docs/examples/starter kit)
- Heavier than a native Svelte solution (includes its own DOM handling)
- API is imperative, not declarative (mismatches Svelte reactive paradigm somewhat)
- Resize handles are built-in but customizeable

---

## 3. Muuri

| Field | Details |
|-------|---------|
| **npm** | `muuri` |
| **GitHub** | https://github.com/haltu/muuri |
| **Stars** | 11,000+ |
| **Latest** | v0.9.5 (stable but slow release cycle) |
| **License** | MIT |
| **Status** | Maintained (522 commits) |

### Key Features
- **Vanilla JS** — zero dependencies, framework-agnostic
- Combines: Packery + Masonry + Isotope + Sortable
- Fully customizable layout engine (custom layout functions)
- Asynchronous layout calculations (Web Workers)
- Drag & drop (between grids), auto-scrolling, nested grids
- Filtering, sorting
- Web Animations API for animations
- Granular control: dragStartPredicate, dragSortPredicate, etc.

### Svelte Port/Compatibility
- **Good compatibility**. Vanilla JS with imperative API.
- Wrap via Svelte `action` or on `onMount`/`onDestroy` lifecycle
- More complex to wrap than GridStack (more configuration options, more imperative API surface)

### Pros for Svelte Dashboard
- Extremely flexible layout engine (custom layout functions)
- 11k stars, long history
- Web Worker support for layout calculations
- Filtering and sorting built-in
- Between-grid drag support

### Cons for Svelte Dashboard
- Last release is v0.9.5 (pre-1.0, slow development)
- No TypeScript (pure JS with JSDoc comments)
- More complex API surface than GridStack
- Web Animations API polyfill may be needed for older browsers
- No built-in responsive breakpoints — custom layout functions needed for that

---

## 4. @hello-pangea/dnd

| Field | Details |
|-------|---------|
| **npm** | `@hello-pangea/dnd` |
| **GitHub** | https://github.com/hello-pangea/dnd |
| **Stars** | 3,900+ |
| **Latest** | v18.0.1 (Feb 2025) |
| **License** | Apache 2.0? (forked from react-beautiful-dnd) |
| **Status** | Actively maintained |

### Key Features
- Beautiful, accessible drag and drop for React **lists**
- Vertical lists, horizontal lists, between-list movement
- Keyboard and screen reader accessibility built in
- Virtual list support (10,000 items @ 60fps)
- Multi-drag support
- Auto-scrolling

### Svelte Port/Compatibility
- **NOT suitable** for dashboard grids. The README explicitly states:
  > "One shortcoming is that grid layouts are not supported (yet). So @hello-pangea/dnd might not be for you depending on what your use case is."
- This is a **list-based DnD** library, not a grid layout library
- It handles 1D reordering (vertical/horizontal lists) well, NOT 2D grid snapping/resizing
- React-only — full rewrite needed for any other framework

### Pros for Svelte Dashboard
- Not applicable — wrong abstraction level

### Cons for Svelte Dashboard
- Explicitly does not support grid layouts
- React-only
- For list sorting only, not dashboard widgets with resizing

---

## 5. react-resizable (RGL dependency)

| Field | Details |
|-------|---------|
| **npm** | `react-resizable` |
| **GitHub** | https://github.com/react-grid-layout/react-resizable |
| **Stars** | 2,600+ |
| **Latest** | v3.1.0 (Dec 2025) |
| **Status** | Maintained (same org as RGL) |

### Key Features
- React component providing resize handles (n, s, e, w, ne, nw, se, sw)
- Grid snapping, aspect ratio locking, axis constraints
- Used as the resize layer in react-grid-layout

### Svelte Port/Compatibility
- React-only. For Svelte, alternative resize approaches:
  - Use native `ResizeObserver` + custom handle elements
  - Use CSS `resize` property (limited)
  - Build custom resize handle logic (not overly complex)

---

## 6. dnd-kit

| Field | Details |
|-------|---------|
| **npm** | `@dnd-kit/core` (+ `@dnd-kit/sortable`, `@dnd-kit/utilities`) |
| **GitHub** | https://github.com/clauderic/dnd-kit (not fetched but well-known) |
| **Stars** | 13,000+ |
| **Latest** | ~v6.x (2024) |
| **License** | MIT |

### Key Features
- Modern, lightweight, extensible DnD toolkit for React
- Sortable presets, collision detection algorithms
- Pointer and keyboard sensor support
- Tree-shakeable, modular packages

### Svelte Port/Compatibility
- **React-only**. No official Svelte port.
- Some community attempts exist (svelte-dnd-kit) but none mature
- Better suited for 1D list reordering; 2D grid dashboards would require significant custom code

---

## 7. Emerging / Alternative Libraries (2024-2025)

### Pragmatic drag and drop (Atlassian)
- **npm**: `@atlaskit/pragmatic-drag-and-drop`
- Vanilla JS core, framework-agnostic by design
- Released 2024 as successor to react-beautiful-dnd
- Potentially wrappable for Svelte but focused on list/board DnD, not grid layouts

### Interact.js
- **npm**: `interact.js`
- Vanilla JS drag/resize/gesture library
- 12k+ stars, mature
- Could serve as the drag/resize engine for a custom Svelte grid layout
- Framework-agnostic, good Svelte compatibility via actions

### Moveable
- **npm**: `moveable` (by Daybrush/Naver)
- Vanilla JS, handles drag, resize, rotate, warp, pinch
- 10k+ stars on GitHub
- Could pair with a custom grid layout engine

### Golden Layout
- **npm**: `golden-layout`
- Dock-based panel layout (IDE-style), not traditional grid dashboard
- Vanilla JS, has React and Angular wrappers
- v2 is a complete rewrite (2024)

---

## Summary & Recommendations

### For Svelte Dashboard Development

| Approach | Effort | Quality | Recommendation |
|----------|--------|---------|----------------|
| **GridStack.js + Svelte wrapper** | Low | High | **Best option.** Vanilla JS, mature, actively maintained. Wrap in Svelte action. |
| **react-grid-layout/core + custom Svelte UI** | High | Very High | Best algorithmic quality. Import pure layout functions from `core/`, build Svelte components on top. High effort but most flexible. |
| **Interact.js + custom grid engine** | Medium-High | High | Build exactly what you need. More work but no framework coupling. |
| **Muuri + Svelte wrapper** | Medium | Medium-High | Good if you need filtering/sorting too. Slower release cycle. |
| **Build from scratch** | Very High | Variable | Not recommended unless you have very specific requirements. |

### Key Insight

**react-grid-layout v2's `core/` module** is the most important finding. It exports pure TypeScript functions:

```
import { 
  verticalCompactor, horizontalCompactor, noCompactor, getCompactor,
  moveElement, collides, getFirstCollision, validateLayout,
  cloneLayout, cloneLayoutItem, getStatics, compactItemVertical,
  compactItemHorizontal, resolveCompactionCollision
} from 'react-grid-layout/core';
```

These functions handle ALL the algorithmic complexity of dashboard grid layouts (collision detection, compaction, constraint validation) with zero React dependency. A Svelte dashboard could:

1. Use these algorithms directly
2. Implement its own drag/resize UI layer using Svelte actions
3. Manage state reactively with Svelte stores

This approach gives you the battle-tested layout engine of the most popular dashboard library while keeping the UI layer idiomatic to Svelte.
