# Bar Chart — Re-usable Chart Spec

> Picasso.js implementation · `PicassoBarChartCanvas` · `src/lib/charts/PicassoBarChartCanvas.svelte`

---

## When to Use

Use the bar chart when you need to:

- Compare **quantitative values across discrete categories** (e.g. revenue by region, count by status)
- Show a **single aggregated measure** grouped by a single dimension
- Provide **interactive filtering** — users click bars to highlight/select categories
- Render in a **dashboard panel** where the chart fills its container and re-renders on config change

Do NOT use for:

- Time-series data (use a line chart instead)
- Multi-series or stacked comparisons (not yet supported)
- More than one dimension (this chart is single-dimension only)
- Continuous data distributions (use a histogram)

---

## Dimensions & Measures

### Dimensions (X-axis grouping)

| Constraint | Value |
|------------|-------|
| **Min** | `1` dimension (required) |
| **Max** | `1` dimension |

Accepted column types (categorical):

```
VARCHAR, TEXT, STRING, CHAR, BPCHAR, NAME, UUID, ENUM, BOOLEAN, BOOL
```

### Measures (Y-axis aggregation)

| Constraint | Value |
|------------|-------|
| **Min** | `1` metric (required) |
| **Max** | `1` metric |

Accepted column types (numeric):

```
INTEGER, BIGINT, SMALLINT, TINYINT, INT, INT2, INT4, INT8,
DOUBLE, FLOAT, FLOAT4, FLOAT8, REAL, DECIMAL, NUMERIC,
HUGEINT, UINTEGER, UBIGINT, USMALLINT, UTINYINT
```

Supported aggregations: `SUM` · `AVG` · `COUNT` · `MIN` · `MAX`

---

## Config Options

`BarChartConfig` (`src/lib/charts/types.ts:34`)

```ts
interface BarChartConfig {
  id: string;                               // Unique chart instance identifier
  table: string;                            // DuckDB source table name
  dimension: {
    field: string;                          // Column name for grouping
    label?: string;                         // Display label (defaults to field)
  };
  metric: {
    field: string;                          // Column name for aggregation
    label?: string;                         // Display label (defaults to field)
    aggregate?: 'COUNT' | 'SUM' | 'AVG' | 'MIN' | 'MAX';  // Default: 'SUM'
  };
  orientation?: 'vertical' | 'horizontal'; // Default: 'vertical'
  sortDirection?: 'ASC' | 'DESC';          // Default: 'DESC'
  limit?: number;                          // Max categories (default: 30)
  clickToFilter?: boolean;                 // Enable click-to-filter (default: true)
  filters?: string[];                      // Additional SQL WHERE clauses
  colors?: string[];                       // Override default PALETTE
  showValues?: boolean;                    // Value labels on bars (default: false)
  valueFormat?: 'number' | 'currency' | 'percent' | 'compact';  // Default: 'number'
}
```

### Component Props

`PicassoBarChartCanvas` accepts:

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `BarChartConfig` | yes | Full chart configuration |
| `onBarClick` | `(detail: { category, value, row }) => void` | no | Callback on bar click |

---

## Chart Requirements

### Runtime Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `picasso.js` | `^2.11.3` | Chart rendering engine |

### Internal Dependencies

| Module | Exports Used |
|--------|-------------|
| `$lib/charts/types` | `BarChartConfig`, `BarChartData` |
| `$lib/charts/engine/DataModelConnector` | `executeBarChartQuery` |
| `$lib/charts/chart-helpers` | `PALETTE` (10-color oklch palette) |
| `$lib/db-operations` | `extractErrorMessage` |

### Data Requirements

- A connected DuckDB instance with the target table available
- `executeQuery()` from `$lib/db-operations` must be functional (Tauri IPC)
- The target table must have at least one categorical column (dimension) and one numeric column (metric)

### Generated SQL Pattern

```sql
SELECT "{dimension}", {AGGREGATE}("{metric}") AS value
FROM "{table}"
[WHERE {filters}]
GROUP BY "{dimension}"
ORDER BY value {ASC|DESC}
LIMIT {n}
```

### CSS Variables Required

The chart shell uses these design tokens:

```
--color-surface, --color-border, --color-accent, --color-accent-muted,
--color-accent-dark, --color-text, --color-text-secondary, --color-text-tertiary,
--color-text-on-accent, --color-danger, --color-surface-sunken,
--radius-lg, --radius-xs, --radius-full,
--space-1..space-6, --space-24,
--text-xs, --text-sm, --text-md,
--font-display, --font-mono, --font-body,
--duration-fast, --ease-out-expo
```

### Chart States

| State | Trigger | Display |
|-------|---------|---------|
| **Loading** | `fetchData()` in progress | Spinner + "Loading chart data..." |
| **Error** | Query failure | "Query error: {message}" in danger color |
| **Empty** | Zero rows returned | "No data available" |
| **Unconfigured** | No `config` passed | Placeholder with icon + instructions |
| **Active** | Data loaded, chart mounted | Interactive Picasso SVG chart |

---

## Chart Anatomy — ASCII Node Tree

```
PicassoBarChartCanvas
├── chart-shell                              ← border-radius: lg, flex column, fills height
│   ├── chart-header                         ← flex-shrink: 0, padded
│   │   └── chart-title-text                 ← "{AGG} of {metric} by {dimension}"
│   │
│   └── chart-body                           ← flex: 1, min-height: 0, overflow: hidden
│       │                                    ← Picasso mounts <svg> here
│       │
│       ├── [conditional: click-to-filter active + selections exist]
│       │   └── filter-bar                   ← flex wrap row, gap, border-bottom
│       │       ├── filter-chip × N          ← category label + × close button
│       │       │   └── chip-x               ← "×" remove indicator
│       │       └── filter-clear             ← "Clear all" dashed button
│       │
│       ├── [state: loading]
│       │   └── chart-state
│       │       ├── spinner (<svg>)          ← rotating arc animation
│       │       └── span                     ← "Loading chart data..."
│       │
│       ├── [state: error]
│       │   └── chart-state.chart-error
│       │       └── span                     ← "Query error: {message}"
│       │
│       ├── [state: empty]
│       │   └── chart-state
│       │       └── span                     ← "No data available"
│       │
│       └── [state: active — Picasso rendered SVG]
│           └── <svg>                        ← Picasso.js generated
│               ├── <g> — y-axis             ← value labels (vertical) / category labels (horizontal)
│               │   └── <text> × N           ← tick labels
│               ├── <g> — x-axis             ← category labels (vertical) / value labels (horizontal)
│               │   └── <text> × N           ← tick labels
│               ├── <g> — bars               ← <rect> elements for each category
│               │   └── <rect> × N           ← fill: PALETTE[idx % 10], opacity dimmed if filtered
│               │
│               └── [conditional: showValues === true]
│                   └── <text> × N           ← value-label: injected post-render via DOM
│                                                position: above bar (vertical)
│                                                position: right of bar (horizontal)
│                                                font: Lekton, 11px, fill: #333
```

### Page-Level Anatomy (consumer at `labs/picasso-charts/bar`)

```
+page.svelte
└── chart-detail                             ← flex row, height: 100%
    ├── chart-panel                          ← flex: 1, border-right
    │   ├── chart-panel-header               ← badge bar
    │   │   └── section-number               ← "PICASSO-001"
    │   │
    │   ├── chart-panel-body                 ← flex: 1, relative positioning
    │   │   ├── [unconfigured]
    │   │   │   └── chart-placeholder        ← centered icon + heading + description
    │   │   │       ├── BarChart2 (icon)
    │   │   │       ├── h3                   ← "Configure your chart"
    │   │   │       └── p                    ← instruction text
    │   │   │
    │   │   ├── PicassoBarChartCanvas        ← (see anatomy above)
    │   │   │
    │   │   └── [conditional: bar clicked]
    │   │       └── bar-toast                ← fixed bottom-center popup, 2s timeout
    │   │           ├── bar-toast-cat         ← category name (bold)
    │   │           └── bar-toast-val         ← formatted value (dimmed)
    │   │
    │   └── aside.drawer                     ← width: 360px, flex column
    │       ├── drawer-header                ← icon + "Chart config"
    │       │   └── Settings (icon)
    │       │
    │       └── drawer-body                  ← scrollable config form
    │           ├── config-section: Data
    │           │   ├── select#chart-table    ← table selector
    │           │   ├── select#chart-dim      ← dimension column selector
    │           │   └── select#chart-metric   ← metric column selector
    │           │
    │           ├── config-section: Aggregation
    │           │   └── agg-btn × 5          ← SUM · AVG · COUNT · MIN · MAX
    │           │
    │           ├── config-section: Layout
    │           │   └── orient-btn × 2       ← Vertical · Horizontal
    │           │
    │           ├── config-section: Options
    │           │   ├── select#chart-sort     ← DESC / ASC
    │           │   ├── input#chart-limit     ← max categories (1–1000)
    │           │   ├── input#chart-fqid      ← chart instance ID
    │           │   ├── toggle: clickToFilter ← on/off switch
    │           │   ├── toggle: showValues    ← on/off switch
    │           │   └── [conditional: showValues]
    │           │       └── format-btn × 4   ← number · currency · percent · compact
    │           │
    │           └── config-section: Generated SQL
    │               └── sql-preview          ← <pre><code> read-only SQL
```

---

## Data Flow

```
BarChartConfig
    │
    ▼
compileBarChartQuery(config, filterState?)  ──→  SQL string
    │
    ▼
executeBarChartQuery(config, filterState?)  ──→  { rows: BarChartData[], columns: string[] }
    │                                            via Tauri IPC → DuckDB
    ▼
PicassoBarChartCanvas
    │
    ├─ matrixData()   ──→  { type: 'matrix', data: [[header], ...rows] }
    │
    ├─ buildSettings(selectedSet)  ──→  Picasso scales + components
    │   ├── scales: { cat: band, val: linear }
    │   └── components: [ yaxis, xaxis, bars ]
    │
    └─ picasso.chart({ element, data, settings })
        └─ Renders <svg> into chart-body container
```

### Reactive Triggers

Data is re-fetched when any of these config fields change:

`table` · `dimension.field` · `metric.field` · `metric.aggregate` · `sortDirection` · `limit` · `orientation`

The chart is re-mounted when data rows or label settings change.

---

## Interaction: Click-to-Filter

```
User clicks bar
    │
    ▼
chart.shapesAt({x, y, w:1, h:1})  ──→  hit-test → find category
    │
    ▼
Toggle category in selectedGroups Set
    │
    ▼
applySelection()
    ├── chart.update({ settings: buildSettings(selectedGroups) })
    │   └── Unselected bars → opacity: 0.35
    │       Selected bars → opacity: 1
    └── renderValueLabels()  (if showValues)
    │
    ▼
filter-bar renders chips for selected categories
    └── chip click → removeGroup()
        └── "Clear all" → clearAll()
```

---

## Re-use Checklist

To embed this chart in any page:

1. Import `PicassoBarChartCanvas` from `$lib/charts`
2. Import `BarChartConfig` from `$lib/charts/types`
3. Build a `BarChartConfig` object with at minimum: `id`, `table`, `dimension`, `metric`
4. Bind `config` to the component: `<PicassoBarChartCanvas {config} onBarClick={handler} />`
5. Ensure the container has a defined height (the chart fills 100% height via flex)
6. Ensure DuckDB is connected and the target table is queryable
7. Optionally handle `onBarClick` for external state updates

Minimal example:

```svelte
<script>
  import { PicassoBarChartCanvas } from '$lib/charts';
  import type { BarChartConfig } from '$lib/charts/types';

  const config: BarChartConfig = {
    id: 'sales-by-region',
    table: 'orders',
    dimension: { field: 'region' },
    metric: { field: 'revenue', aggregate: 'SUM' },
  };
</script>

<div style="height: 400px;">
  <PicassoBarChartCanvas {config} />
</div>
```
