# Re-usable Bar Chart Component — LayerChart

## Feature Overview

Data Monster needs a single, re-usable bar chart component that can be dropped onto any page to visualise data from the DuckDB data model. The component replaces the tightly-coupled draft chart currently in `data.monster/src/routes/+page.svelte` (which uses `@carbon/charts-svelte`) with a clean, configurable bar chart built on **LayerChart** (`layerchart@next`).

The chart must support clicking on bars to filter the underlying data model, display tooltips on hover, and accept configuration for which table/fields to visualise. It must use the app's existing Tauri + DuckDB data pipeline (`$lib/db-operations`) to execute SQL queries.

This is the first chart in what will become a chart library. The architecture must be general enough to extend to line, scatter, pie, etc. later, but this spec focuses exclusively on getting one bar chart working end-to-end.

## Success Criteria

- A `<BarChartCanvas>` Svelte 5 component renders a vertical bar chart from any DuckDB table with a single dimension and a single metric
- Clicking a bar toggles it in/out of an active filter set; unselected bars are visually dimmed
- Active filters are shown as removable chips above the chart with a "Clear all" button
- The chart auto-generates its title from the configured dimension and metric (e.g. "Sum of net_amount by region")
- Tooltips appear on hover showing the category and value
- The component works inside a Tauri WebView2 window with no rendering issues
- All chart elements (axes, colors, padding, height) have sensible defaults but can be overridden via props
- The component is imported and used on the `/analyst` page (or a new `/charts` test route) with sample data

## Design Goals

### Primary (Must)

- Use **LayerChart** (`layerchart@next`) as the chart rendering library — native Svelte 5 components, SVG-based, click events via `onBarClick`
- Component uses Svelte 5 runes (`$props()`, `$state()`, `$derived()`, `$effect()`)
- Data comes from DuckDB via Tauri `invoke('execute_query', { sql })` — no hardcoded JSON files
- Click-to-filter is opt-in (prop: `clickToFilter: boolean = true`)
- Config is a plain object (`BarChartConfig`) that can be serialised to JSON for persistence later

### Secondary (Nice to Have)

- Animated bar transitions on data change (LayerChart supports tween/spring out of the box)
- Responsive width (fills parent container)
- Export chart as PNG/SVG (LayerChart has `download` utility — not in scope for this spec but component should not block it)

## User Experience

The user navigates to a page that contains the bar chart. The chart is already configured with a base table, a dimension (x-axis), and a metric (y-axis). On load, the chart queries DuckDB, aggregates the data, and renders vertical bars sorted by value descending.

When the user clicks a bar, that category is added to the active filter set. The clicked bar stays fully coloured while all other bars are dimmed. A chip appears above the chart showing the selected category. Clicking the same bar again (or the chip's × button) removes it from the filter. Multiple bars can be selected. Clicking "Clear all" resets everything.

A tooltip appears on hover, showing the category name and formatted value.

## Design Rationale

**Why LayerChart over Carbon Charts**: The research in `docs/research/chart-library-candidates.md` evaluated 6 libraries. LayerChart was the top pick: native Svelte 5, built-in `onBarClick` callback, Tailwind-compatible SVG, tree-shakeable, actively maintained. Carbon Charts is heavy (~300KB), has IBM telemetry, and its Svelte 5 migration is incomplete.

**Why not SveltePlot for this first chart**: SveltePlot is grammar-of-graphics and excellent for analytical compositions, but LayerChart's `<BarChart>` simplified component gives us a faster path to a working bar chart with built-in tooltips, click handling, and legend support. SveltePlot can be added as a second renderer later.

**Data flow**: The component compiles a `BarChartConfig` into a SQL query, executes it via the existing Tauri + DuckDB pipeline, and feeds the result rows directly into LayerChart's `data` prop. This keeps the chart decoupled from any specific table or schema.

## Constraints/Assumptions

- **Stack**: Svelte 5 (runes), SvelteKit, Tauri v2, DuckDB (native via Rust backend), Tailwind CSS v4
- **Package**: `layerchart@next` (the `@next` tag is the Svelte 5 compatible version)
- **No SSR**: This is a Tauri desktop app — all chart rendering is client-side
- **Existing data pipeline**: `$lib/db-operations` already provides `executeQuery(sql)` which calls Tauri's `invoke('execute_query', { sql })`. The chart component must use this.
- **No design system dependency**: The chart uses LayerChart's default theming with CSS variables. It does not depend on Carbon, Skeleton, or any other UI framework
- **Single table initially**: Cross-table joins are out of scope for this first chart. The component queries one table at a time
- **Windows primary target**: Must render correctly in WebView2 (Chromium-based)

## Functional Requirements

### FR-1: Install and configure LayerChart

Install `layerchart@next` as a dependency. Add the LayerChart CSS variables to the app's global stylesheet so chart theming inherits from the app's design tokens (accent color, surface colors, text colors).

- **Acceptance**: `pnpm add layerchart@next` succeeds, CSS variables are set in `app.css` or equivalent, and a minimal `<BarChart>` renders without errors

### FR-2: Define BarChartConfig type

Create `src/lib/charts/types.ts` with the TypeScript interfaces for bar chart configuration:
- `BarChartConfig` with fields: `id`, `table`, `dimension` (field name + label), `metric` (field name + label + aggregate function), `filters[]`, `sortDirection`, `limit`, `clickToFilter`, `colors`
- `BarChartData` — the shape of each data row returned from DuckDB
- `FilterState` — `Map<string, Set<string>>` for active click-to-filter selections

- **Acceptance**: Types compile, are exported from `src/lib/charts/index.ts`, and are used by the chart component

### FR-3: Create DataModelConnector for SQL compilation

Create `src/lib/charts/engine/DataModelConnector.ts` with a `compileBarChartQuery(config: BarChartConfig, filterState?: FilterState)` function that generates a DuckDB SQL query:
- `SELECT {dimension}, {aggregate}({metric}) as value FROM {table}`
- Optional `WHERE` clauses from config filters + active filter state
- `GROUP BY {dimension}`
- `ORDER BY value {sortDirection}`
- `LIMIT {limit}`

Also provide `executeBarChartQuery(config, filterState)` which compiles + executes via `executeQuery()` from `$lib/db-operations`.

- **Acceptance**: Given a config `{ table: 'sales', dimension: 'region', metric: 'net_amount', aggregate: 'SUM' }`, produces `SELECT region, SUM(net_amount) as value FROM sales GROUP BY region ORDER BY value DESC`. Executes against DuckDB and returns `{ rows: BarChartData[] }`.

### FR-4: Create BarChartCanvas component

Create `src/lib/charts/BarChartCanvas.svelte` — the main re-usable component:
- Props: `config: BarChartConfig` (required), `height: number = 400`, `width: number | undefined` (responsive)
- Uses `$state()` for: `data`, `filterState`, `isLoading`, `error`
- Uses `$derived()` for: `colorMap` (selected bars get palette colors, unselected get dimmed color)
- On mount: calls `executeBarChartQuery(config)` and stores result in `data`
- Renders LayerChart `<BarChart>` with: `data={data}`, `x={config.dimension.field}`, `y="value"`, `onBarClick={handleBarClick}`, `tooltipContext={{ mode: 'band' }}`
- `handleBarClick` toggles the clicked category in `filterState`
- When `filterState` changes, re-queries with filter applied and updates `data`
- Renders filter chips above chart when `filterState` is non-empty

- **Acceptance**: Component renders a bar chart. Clicking a bar toggles it in/out of filter. Filter chips appear and are removable. Data refreshes on filter change.

### FR-5: Implement click-to-filter with visual dimming

Inside `BarChartCanvas.svelte`:
- Maintain a `selectedCategories: Set<string>` state
- On bar click: toggle the category in the set
- Derive a color map: selected categories get colors from a palette array; unselected categories get a dimmed color (`oklch(0.85 0.01 250 / 0.4)`)
- Pass the color map to LayerChart's color scale or per-bar fill override
- When filter state changes, re-execute query with `WHERE {dimension} IN (...)` clause (for include mode) and update chart data
- Show a filter chip bar: each selected category as a chip with × button, plus "Clear all" button

- **Acceptance**: Click a bar → it stays coloured, all others dim. Click again → it dims (deselected). Chips appear/disappear. Clear all resets everything. Re-query updates bar heights to reflect filtered data.

### FR-6: Add tooltips

Enable LayerChart's built-in tooltip system:
- Pass `tooltipContext={{ mode: 'band' }}` to the BarChart (band mode is optimal for bar charts per LayerChart docs)
- Tooltip shows: category name as header, formatted value as item
- Value formatting: numbers > 999 use locale string with `$` prefix if metric suggests currency, or just locale number

- **Acceptance**: Hovering over a bar shows a tooltip with category and formatted value. Tooltip follows pointer. No console errors.

### FR-7: Wire up on a test route

Create a test route (e.g. `src/routes/charts/+page.svelte`) or add the chart to an existing page:
- Import `BarChartCanvas` from `$lib/charts`
- Define a `BarChartConfig` pointing to an existing DuckDB table
- Render the component
- Verify it works end-to-end: data loads, bars render, click-to-filter works, tooltips work

- **Acceptance**: Navigating to `/charts` (or the chosen route) shows a working bar chart with all interactive features.

## Edge Cases

- **Table has 0 rows**: Show "No data available" message instead of empty chart
- **Table has 1 row**: Single bar renders correctly
- **Table has >50 distinct categories**: Bar chart still renders; consider default `limit: 30` in config to prevent overcrowding
- **All bars clicked then unclicked**: Returns to full unfiltered state cleanly
- **Query error (table doesn't exist, invalid column)**: Show error state in the component, don't crash the page
- **Null/undefined values in data**: Filter out or handle gracefully in aggregation
- **Very large values**: Number formatting should abbreviate (e.g. 1.2M, 3.4K) or use locale formatting
