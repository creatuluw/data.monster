# Reporting & Dashboard Pages

## Feature Overview

Reporting & Dashboard Pages is the content creation and insight-sharing layer of Data Monster. It enables data analysts and business users to build rich, interactive dashboard pages by assembling reusable components — charts, tables, filters, search, and AI blocks — onto a configurable grid. Pages can be exported as professionally designed PDFs and bookmarked to capture specific data views. This initiative transforms Data Monster from a query tool for engineers into a complete analytics platform where non-technical users can create, present, and distribute actionable insights from their data.

The feature builds on the existing DuckDB-powered data engine, SvelteKit frontend, and the LayerChart charting library. It introduces a pluggable component architecture so new visualization and control blocks can be added by anyone following a standard contract. Page definitions are stored on disk (for portability, version control, and cross-workspace sharing) with internal DB metadata for fast lookups.

## Problem Statement

Data Monster currently provides powerful data ingestion and SQL querying, but these features target engineers and developers. There is no way for a data analyst or business user to turn query results into shareable reports, dashboards, or presentation-ready insights. The existing Pages feature (`/pages`) is fully automatic — it renders bar charts from the first table with zero configuration — offering no user control over what data is shown, which visualizations are used, or how content is arranged. Users who need to create reports must export data to external tools like Excel or separate BI platforms. This capability gap means Data Monster has limited value as an analytics tool beyond raw data exploration.

## Success Criteria

- A user can create a new page and begin adding components from the library within 1 minute of opening the app
- Adding a component and wiring it to data requires selecting fields from a guided picker with column provenance and sample preview values — no SQL knowledge needed
- A user can build a multi-component dashboard (charts + table + filter) with data bound to all components in under 5 minutes
- PDF export produces a document with professional typography, sensible page breaks, and clear visual hierarchy
- A bookmark captures and restores the exact page state (filters, component selections) across sessions
- Users can share page definition files between workspaces

## Design Goals

### Primary (Must)

- Guided, low-friction component placement and data binding — the user should never feel lost
- Configurable grid layout with snapping, drag-to-rearrange, and resize
- Reusable component library with a standard contract that external contributors can target
- Page definitions stored as files on disk — self-contained, portable, version-controllable
- All components work against the existing DuckDB engine via Tauri commands

### Secondary (Nice to Have)

- Runtime hardware profiling to guide users on workspace sizing
- Component-level query caching to reduce duplicate work across widgets
- Dark mode render for PDF export

## User Personas & Experience

### Personas

- **Data Analyst**: Technical enough to understand data schemas and column types, but prefers a guided UI over writing SQL. Needs to produce reports for stakeholders. Primary user.
- **Business User**: Non-technical. Needs to view data, apply filters, and export clean PDFs. May be a consumer of pages built by analysts rather than a builder themselves.
- **Decision Maker**: Views dashboards and PDF reports in review sessions. Cares about presentation quality and clarity. Does not build pages.

### User Experience

**Before this feature:** A user wanting a report opens Data Monster, queries data in the SQL editor, copies results to Excel, and builds charts manually. Or they use a completely separate BI tool.

**After this feature:**

1. User opens Data Monster with a workspace loaded
2. Navigates to Pages, creates a new page
3. Opens the component library panel, browses available blocks (bar chart, table, filter, etc.)
4. Drags a bar chart component onto the grid — it snaps into place
5. A split-screen editor opens showing field picker. The user selects a dimension and a metric. Each field shows its source table and sample values to guide selection
6. The chart renders immediately from the bound data
7. User adds a table component, selects different fields
8. User adds a filter component — it automatically filters all components on the page (opt-out per component)
9. User rearranges and resizes components on the grid
10. User exports a PDF that renders the page as a professionally formatted document
11. User bookmarks a specific filter state to return to later or share context with a colleague

## Scope & Boundaries

### In Scope

- Page CRUD (create, rename, delete, duplicate)
- GridStack.js-powered grid layout with drag, resize, snap-to-grid
- Component library panel with the following initial blocks: bar chart, treemap, line chart, scatter chart, table
- Field picker with table provenance and sample value previews
- Page-level filter component with per-component opt-out
- Page-scoped search component with ranked hitlist
- PDF export with professional print layout and typography
- Global bookmarks with deep-link state restoration
- Component contract/interface standard (pluggable architecture)
- Page definition files stored on disk (JSON), internal DB metadata for indexing
- Hardware profiling guidance

### Out of Scope

- User authentication, roles, or permissions
- Scheduled or emailed reports
- Embedding dashboards in external applications
- Real-time data streaming or live-updating dashboards
- Dashboard templates (save-as-template, copy-from-template)
- Multi-user collaboration or concurrent editing
- Version history or undo/redo across sessions

## High-Level Capabilities

### Page Builder Core

The page builder is the canvas where users compose dashboards. It provides a configurable grid powered by GridStack.js, supporting drag-to-place, drag-to-rearrange, and resize of components. The grid is column-based with configurable spacing and snap-to-grid behavior. Users create, rename, delete, and switch between pages. Each page's layout — component positions, sizes, and configurations — is serialized to a JSON definition file on disk within a workspace directory, enabling portability, import into other workspaces, and future version control. An internal DuckDB metadata table indexes pages for fast listing and lookup.

### Component Contract & Registry

Every component in the library follows a standard contract: it receives a typed configuration object, declares which fields it needs from the data layer, and emits state changes through a defined event interface. The registry auto-discovers components from a filesystem directory, making it trivial for contributors to add new block types by creating a Svelte component that satisfies the contract. The component library panel — shown when a user is adding blocks to a page — lists all registered components with names, descriptions, and preview thumbnails.

### Chart & Table Components

The initial component library includes five data-bound blocks: bar chart, treemap, line chart, scatter chart, and table. All are built on LayerChart for consistency. When a user places a component on the page and opens the split-screen editor, they see a field picker that lists all available columns across all tables. Each field entry shows the source table name and sample values (first 5 distinct values) to guide selection. The user picks dimension and metric fields; the component generates and executes the appropriate DuckDB query via Tauri commands. Charts support click-to-filter interactions. The table component supports column selection, sorting, and pagination within its allocated grid space.

### Page Filter Component

A standalone filter block that applies cross-component filtering. When placed on a page, it automatically subscribes all other components to its filter state. Each component can individually opt out of filter subscription. The filter component emits its state (selected values per field), and subscribed components re-query with the combined filter conditions. Filter chips are displayed in a bar showing active filters, with individual and bulk clear actions.

### Page Search Component

A search block scoped to the current page. It indexes the content of all components on the page — data rows, chart labels, metadata — and provides a ranked hitlist of matches as the user types. Results show the matching text and which component it came from, with the ability to navigate to the match. The search scope includes component names, data records, filter values, and any text content visible on the page.

### PDF Export

Export the current page as a PDF. The rendering is WYSIWYG — what the user sees on the page is what appears in the PDF — but with professional print optimizations: appropriate page size and orientation, intelligent page breaks that avoid splitting components, consistent typography, section grouping, and clean margins. The export respects the component layout from the grid.

### Bookmarks

A global bookmark system that captures a snapshot of a page's exact state — component positions, filter values, chart selections, scroll position. Bookmarks live outside any single page; a user can open a bookmark and be navigated directly to the page with all state restored to what was captured. Bookmarks are stored in the internal metadata DB and can be named/renamed.

### Hardware Profiling

On workspace load or page creation, the app evaluates available system resources (CPU cores, RAM, disk speed) and provides guidance on reasonable workspace sizes and page complexity. This helps users understand whether their hardware can support the dashboards they want to build before they encounter performance issues.

## Spec Candidates

The following capabilities should each become their own spec document. A spec-writer should create one spec per item below.

### Spec: Page Builder Core

- **Description**: GridStack.js integration into a Svelte 5 page builder. Page CRUD operations. Grid configuration (columns, spacing, snap behavior). Component placement via drag from library panel, rearrange, resize. Page layout serialization to disk as JSON definition files. Internal DB metadata table for page indexing. Page switching and navigation.
- **Dependencies**: None (foundational)
- **Priority**: Must-have

### Spec: Component Contract & Registry

- **Description**: Define the standard component interface (config shape, data binding, event contract). Implement auto-discovery of components from a filesystem directory. Build the component library panel/browser UI that lists registered components for users to select and place. Include documentation of the contract so external contributors can add new blocks.
- **Dependencies**: Page Builder Core (components need a page to live on)
- **Priority**: Must-have

### Spec: Chart & Table Components

- **Description**: Build the five initial data-bound components: bar chart, treemap, line chart, scatter chart, and table. Each implements the component contract. Each includes a split-screen field picker editor that shows all available columns with source table name and sample preview values. Components generate and execute DuckDB queries via Tauri commands. Charts use LayerChart; table uses a paginated data view. Chart click-to-filter interaction. Shared query execution and caching layer.
- **Dependencies**: Component Contract, existing Tauri query commands
- **Priority**: Must-have

### Spec: Page Filter Component

- **Description**: A filter block component that broadcasts its filter state to all subscribed components on the page. Per-component opt-out mechanism. Filter state composition (multiple fields, AND/OR logic). Filter chip bar UI showing active filters with individual/bulk clear. Components re-query DuckDB when filter state changes.
- **Dependencies**: Component Contract, Chart & Table Components
- **Priority**: Should-have

### Spec: Page Search Component

- **Description**: A search block scoped to the current page. Indexes visible content across all page components (data rows, labels, metadata). Ranked hitlist UI as user types. Results show matching text and source component. Navigation to match location.
- **Dependencies**: Component Contract, Page Builder Core
- **Priority**: Should-have

### Spec: PDF Export

- **Description**: WYSIWYG PDF rendering of the current page. Professional print layout: page size/orientation selection, intelligent page breaks (avoid splitting components), consistent typography and margins, section grouping. Leverages browser print APIs or a PDF generation library.
- **Dependencies**: Page Builder Core, Chart & Table Components
- **Priority**: Should-have

### Spec: Bookmarks

- **Description**: Global bookmark system. Capture full page state snapshot (component positions, filter values, chart selections, scroll position). Named bookmarks with create/rename/delete. Bookmark list UI. Opening a bookmark navigates to the page and restores all captured state. Storage in internal metadata DB.
- **Dependencies**: Page Builder Core, Page Filter Component
- **Priority**: Nice-to-have

### Spec: Hardware Profiling

- **Description**: Runtime detection of system resources (CPU cores, RAM, disk speed) via Tauri system APIs. Heuristic guidance on recommended workspace size and page complexity limits. Displayed as an advisory notice when loading large workspaces or creating pages.
- **Dependencies**: None (standalone)
- **Priority**: Nice-to-have

## Technical Approach

### Architecture

The feature extends the existing Pages route (`/pages`) with a full page builder. The builder uses **GridStack.js** (v12.6.0, vanilla TypeScript, zero dependencies) wrapped in a Svelte 5 action for grid layout management. GridStack provides drag, resize, snap-to-grid, responsive breakpoints, and layout serialization — all features confirmed in the research evaluation.

Each dashboard component is a Svelte 5 component following a standard contract (defined as a TypeScript interface). Components are discovered from a dedicated directory on disk. The component library panel scans this directory and presents available blocks to the user.

Data binding is handled through a shared data context. Each component declares its data requirements (table, fields, aggregates) and a query compiler generates the appropriate SQL. Queries execute against the local DuckDB engine via existing Tauri commands (`runPagedQuery`). Results are cached per component to avoid redundant execution.

### Tech Stack Integration

| Concern | Technology |
|---------|-----------|
| Grid layout | GridStack.js v12 + Svelte 5 action wrapper |
| Charts | LayerChart v2 (already in project) |
| Styling | Tailwind CSS 4 |
| Data queries | Existing Tauri `runPagedQuery` command against DuckDB |
| Page storage | JSON definition files on disk + `d8a_monster_pages` internal DB table |
| PDF export | Browser print API or `jspdf` + `html2canvas` |
| Icons | Lucide (already in project) |

### Data Flow

```
User picks fields → Component generates query config
  → Query compiler builds SQL
    → Tauri invoke runPagedQuery (DuckDB)
      → Results streamed back
        → Component renders (LayerChart for charts)
          → User interacts (click, filter, resize)
            → Component emits state change
              → Subscribed components re-query
```

### Component Contract

Each component must satisfy:

```typescript
interface PageComponent {
  id: string;
  type: string;
  config: Record<string, unknown>;
  position: { x: number; y: number; w: number; h: number };
  filterOptOut: boolean;
}

interface ComponentContract {
  // Metadata for registry
  metadata: { name: string; description: string; icon: string; version: string };
  // Configuration editor component (split-screen editor)
  editor: SvelteComponent;
  // Main render component
  render: SvelteComponent;
  // Required data fields schema
  dataSchema: DataFieldSchema;
  // Query compiler
  compileQuery(config: ComponentConfig, filters: FilterState): string;
}
```

### Page Storage

Pages are stored in two locations:

1. **Disk** — `workspace/pages/<page-id>.page.json` containing the full definition (layout, components, metadata). Portable, version-controllable, importable.
2. **Internal DB** — `d8a_monster_pages` table with columns: `id`, `name`, `created_at`, `updated_at`, `thumbnail` (base64). Used for fast listing and search.

## Constraints & Assumptions

- **Desktop-only**: The app is a Tauri desktop application. All rendering, layout, and PDF logic targets a single window environment. No mobile responsiveness required beyond what GridStack provides for different window sizes.
- **Single-user**: No concurrency concerns for page editing. No locking or conflict resolution needed.
- **DuckDB performance**: Assume DuckDB can handle multiple concurrent queries from a single page. Heavy dashboards may need query result caching or lazy component loading.
- **GridStack.js licensing**: MIT license, compatible with the project.
- **LayerChart v2 is pre-release**: The project already uses `2.0.0-next.62`. API stability risk acknowledged but accepted.
- **Svelte 5 runes throughout**: All new code uses `$state`, `$derived`, `$effect`, `$props` — no legacy stores.
- **No backend changes required**: All data already lives in DuckDB. Existing Tauri commands are sufficient. New Tauri commands only needed for hardware profiling.

## Risks & Open Questions

### Risks

- **GridStack.js + Svelte reactivity conflict**: GridStack manipulates the DOM directly. The Svelte 5 wrapper must carefully reconcile imperative GridStack operations with Svelte's declarative reactivity. Mitigation: use a Svelte action pattern with `$effect` for lifecycle management, and treat GridStack state as the source of truth for layout, syncing into Svelte `$state` via GridStack events.
- **Component contract adoption**: If the component contract is too rigid or too loose, it will impede component creation. Mitigation: define the contract iteratively with the first 3 components, then stabilize.
- **Query performance at scale**: A page with 6+ charts each running aggregate queries could create noticeable latency. Mitigation: implement query result caching, lazy execution for off-screen components, and user-facing loading states.
- **PDF rendering fidelity**: HTML-to-PDF conversion (especially for canvas-rendered LayerChart components) may lose quality. Mitigation: evaluate `html2canvas` + `jspdf` vs. native `window.print()` with `@media print` CSS. LayerChart may need a static image snapshot mode for export.
- **LayerChart API stability**: Using a pre-release v2 version. Breaking changes could occur. Mitigation: lock version and wrap LayerChart usage behind adapter components to isolate API changes.

### Open Questions

- Which PDF generation approach to use — browser print API with print stylesheet, or `jspdf` + `html2canvas` for programmatic control?
- Should the component library panel support folders/categories, or is a flat list sufficient initially?
- What is the minimum window size for the page builder to be usable? GridStack handles responsive columns but extreme narrow widths need testing.
- How should bookmarks handle pages that have been deleted or components that have been removed since the bookmark was captured?
- Should the field picker show all tables or let users filter by table/schema first? Large workspaces could have hundreds of columns.

## Phasing & Rollout

### Phase 1: MVP — Core Page Builder + Chart Components

- Page Builder Core (GridStack integration, page CRUD, layout persistence)
- Component Contract & Registry
- Chart & Table Components (bar, treemap, line, scatter, table with field picker)

This phase delivers the minimum value: a user can create a page, add charts and tables, wire them to data, and arrange them on a grid.

### Phase 2: Interactivity & Insight

- Page Filter Component (cross-component filtering with opt-out)
- Page Search Component (scoped search with ranked hitlist)
- PDF Export (WYSIWYG with professional print layout)

This phase adds the interactive and sharing capabilities that make the pages usable for analysis sessions and stakeholder presentations.

### Phase 3: Polish & Ecosystem

- Bookmarks (global snapshots with deep-link restoration)
- Hardware Profiling (runtime guidance for workspace sizing)

This phase adds quality-of-life features and prepares the platform for broader component contributions.

### Rollout Plan

- No feature flags needed — the existing `/pages` route can be replaced since the current auto-generated page has minimal user investment.
- No external documentation or training dependencies (desktop app, self-contained).
- The component contract should be documented within the app's developer-facing settings or a `CONTRIBUTING.md` in the component directory, enabling community contributions from day one.
