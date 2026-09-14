# Line / Area Chart — Re-usable Chart Spec

> Picasso.js implementation · `PicassoLineAreaChartCanvas` · `src/lib/charts/PicassoLineAreaChartCanvas.svelte`

---

## When to Use

Use the line/area chart when you need to:

- Show **trends and continuity** over an ordered dimension (time, sequence, rank)
- Visualize how a **single aggregated measure changes** across an ordered X axis
- Emphasize **magnitude/volume** under a trend (area mode) vs. pure direction (line mode)
- Render **smooth or linear curves** through data points with optional dot markers
- Display up to **500 data points** in a single series

Do NOT use for:

- Comparing discrete, unordered categories (use a bar chart instead)
- Multi-series overlays (not yet supported — single metric only)
- More than one dimension (this chart is single-dimension only)
- Part-to-whole proportions (use a pie/donut chart)

---

## Dimensions & Measures

### Dimensions (X-axis ordering)

| Constraint | Value |
|------------|-------|
| **Min** | `1` dimension (required) |
| **Max** | `1` dimension |

Best suited for ordered/sequential columns — dates, timestamps, integers representing rank or order. Also works with categorical columns if they have a natural sort order.

Accepted column types:

```
VARCHAR, TEXT, STRING, CHAR, BPCHAR, NAME, UUID, ENUM, BOOLEAN, BOOL
(and numeric types used as ordered dimensions)
```

> **Note:** The dimension is used for `GROUP BY` and `ORDER BY`. The sort default is `ASC` (chronological) rather than `DESC` (unlike the bar chart).

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

`LineAreaChartConfig` (`src/lib/charts/types.ts`)

```ts
interface LineAreaChartConfig {
  id: string;                               // Unique chart instance identifier
  table: string;                            // DuckDB source table name
  dimension: {
    field: string;                          // Column name for X-axis grouping
    label?: string;                         // Display label (defaults to field)
  };
  metric: {
    field: string;                          // Column name for aggregation
    label?: string;                         // Display label (defaults to field)
    aggregate?: 'COUNT' | 'SUM' | 'AVG' | 'MIN' | 'MAX';  // Default: 'SUM'
  };
  chartType?: 'line' | 'area';             // Default: 'line'
  filters?: string[];                       // Additional SQL WHERE clauses
  sortDirection?: 'ASC' | 'DESC';          // Default: 'ASC' (chronological)
  limit?: number;                           // Max data points (default: 500)
  curve?: 'linear' | 'monotone';           // Default: 'monotone' (smooth)
  showDots?: boolean;                       // Data point markers (default: true)
  showValues?: boolean;                     // Value labels on points (default: false)
  valueFormat?: 'number' | 'currency' | 'percent' | 'compact';  // Default: 'number'
  colors?: string[];                        // Override line/area color (uses colors[0])
}
```

### Component Props

`PicassoLineAreaChartCanvas` accepts:

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `LineAreaChartConfig` | yes | Full chart configuration |
| `onPointClick` | `(detail: { category, value, row }) => void` | no | Callback on data point click |

---

## Chart Requirements

### Runtime Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `picasso.js` | `^2.11.3` | Chart rendering engine |

### Internal Dependencies

| Module | Exports Used |
|--------|-------------|
| `$lib/charts/types` | `LineAreaChartConfig`, `BarChartData` |
| `$lib/charts/engine/DataModelConnector` | `executeLineAreaChartQuery` |
| `$lib/charts/chart-helpers` | `PALETTE` (10-color oklch palette) |
| `$lib/db-operations` | `extractErrorMessage` |

### Data Requirements

- A connected DuckDB instance with the target table available
- `executeQuery()` from `$lib/db-operations` must be functional (Tauri IPC)
- The target table must have at least one column for the dimension and one numeric column for the metric

### Generated SQL Pattern

```sql
SELECT "{dimension}", {AGGREGATE}("{metric}") AS value
FROM "{table}"
[WHERE {filters}]
GROUP BY "{dimension}"
ORDER BY "{dimension}" {ASC|DESC}
LIMIT {n}
```

> **Key difference from bar chart:** `ORDER BY` uses the **dimension column** (not `value`) to preserve chronological order.

### CSS Variables Required

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
| **Active** | Data loaded, chart mounted | Interactive Picasso SVG line/area chart |

---

## Chart Anatomy — ASCII Node Tree

```
PicassoLineAreaChartCanvas
├── chart-shell                              ← border-radius: lg, flex column, fills height
│   ├── chart-header                         ← flex-shrink: 0, padded
│   │   └── chart-title-text                 ← "{Line|Area}: {AGG} of {metric} by {dimension}"
│   │
│   └── chart-body                           ← flex: 1, min-height: 0, overflow: hidden
│       │                                    ← Picasso mounts <svg> here
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
│               ├── <g> — y-axis             ← value scale, docked left
│               │   └── <text> × N           ← tick labels
│               ├── <g> — x-axis             ← dimension scale, docked bottom
│               │   └── <text> × N           ← category/time labels
│               ├── <g> — line               ← Picasso 'line' component
│               │   ├── <path>               ← stroke: PALETTE[1] or colors[0], strokeWidth: 2
│               │   │                        ← curve: monotone (bezier) or linear
│               │   └── [conditional: chartType === 'area']
│               │       └── <path> fill      ← gradient: color at 0.35 opacity → 0.03 opacity
│               ├── [conditional: showDots === true]
│               │   └── <circle> × N         ← stroke: lineColor, fill: #fff, r: 3.5
│               │
│               └── [conditional: showValues === true]
│                   └── <text> × N           ← value-label: injected post-render via DOM
│                                                position: above point (y - 6)
│                                                font: Lekton, 10px, fill: #333
```

### Page-Level Anatomy (consumer at `labs/picasso-charts/line`)

```
+page.svelte
└── chart-detail                             ← flex row, height: 100%
    ├── chart-panel                          ← flex: 1, border-right
    │   ├── chart-panel-header               ← badge bar
    │   │   └── section-number               ← "PICASSO-002"
    │   │
    │   ├── chart-panel-body                 ← flex: 1, relative positioning
    │   │   ├── [unconfigured]
    │   │   │   └── chart-placeholder        ← centered icon + heading + description
    │   │   │       ├── LineChart (icon)
    │   │   │       ├── h3                   ← "Configure your chart"
    │   │   │       └── p                    ← instruction text
    │   │   │
    │   │   ├── PicassoLineAreaChartCanvas   ← (see anatomy above)
    │   │   │
    │   │   └── [conditional: point clicked]
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
    │           ├── config-section: Chart Type
    │           │   └── orient-btn × 2       ← Line · Area
    │           │
    │           ├── config-section: Options
    │           │   ├── orient-btn × 2       ← Smooth · Linear (curve)
    │           │   ├── select#chart-sort     ← ASC / DESC
    │           │   ├── input#chart-limit     ← max points (1–10000)
    │           │   ├── input#chart-fqid      ← chart instance ID
    │           │   ├── toggle: showDots      ← on/off switch
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
LineAreaChartConfig
    │
    ▼
compileLineAreaChartQuery(config, filterState?)  ──→  SQL string
    │
    ▼
executeLineAreaChartQuery(config, filterState?)  ──→  { rows: BarChartData[], columns: string[] }
    │                                                  via Tauri IPC → DuckDB
    ▼
PicassoLineAreaChartCanvas
    │
    ├─ matrixData()   ──→  { type: 'matrix', data: [[header], ...rows] }
    │
    ├─ buildSettings()  ──→  Picasso scales + components
    │   ├── scales: { y: linear (inverted), t: band/linear (dimension) }
    │   └── components: [ yaxis, xaxis, lines ]
    │       └── lines settings:
    │           ├── curve: 'monotone' | 'linear'
    │           ├── stroke: colors[0] || PALETTE[1]
    │           ├── fill: gradient (area) | transparent (line)
    │           └── dots: optional circle markers
    │
    └─ picasso.chart({ element, data, settings })
        └─ Renders <svg> into chart-body container
```

### Reactive Triggers

Data is re-fetched when any of these config fields change:

`table` · `dimension.field` · `metric.field` · `metric.aggregate` · `sortDirection` · `limit` · `chartType`

The chart is re-mounted when data rows or visual settings change:

`curve` · `showDots` · `showValues` · `valueFormat` · `colors[0]` · `chartType`

---

## Line vs Area Mode

| Aspect | Line Mode | Area Mode |
|--------|-----------|-----------|
| `chartType` | `'line'` | `'area'` |
| Fill | `transparent` | Vertical gradient: 0.35 → 0.03 opacity |
| Stroke | PALETTE[1], width 2 | Same |
| Best for | Direction, trend, comparison | Volume, magnitude, accumulation |
| Visual weight | Light | Medium-heavy |

---

## Re-use Checklist

To embed this chart in any page:

1. Import `PicassoLineAreaChartCanvas` from `$lib/charts`
2. Import `LineAreaChartConfig` from `$lib/charts/types`
3. Build a `LineAreaChartConfig` object with at minimum: `id`, `table`, `dimension`, `metric`
4. Bind `config` to the component: `<PicassoLineAreaChartCanvas {config} onPointClick={handler} />`
5. Ensure the container has a defined height (the chart fills 100% height via flex)
6. Ensure DuckDB is connected and the target table is queryable
7. Optionally handle `onPointClick` for external state updates

Minimal example (line):

```svelte
<script>
  import { PicassoLineAreaChartCanvas } from '$lib/charts';
  import type { LineAreaChartConfig } from '$lib/charts/types';

  const config: LineAreaChartConfig = {
    id: 'revenue-over-time',
    table: 'orders',
    dimension: { field: 'order_date' },
    metric: { field: 'amount', aggregate: 'SUM' },
    chartType: 'line',
    curve: 'monotone',
    showDots: true,
  };
</script>

<div style="height: 400px;">
  <PicassoLineAreaChartCanvas {config} />
</div>
```

Minimal example (area):

```svelte
<script>
  import { PicassoLineAreaChartCanvas } from '$lib/charts';
  import type { LineAreaChartConfig } from '$lib/charts/types';

  const config: LineAreaChartConfig = {
    id: 'traffic-volume',
    table: 'page_views',
    dimension: { field: 'date' },
    metric: { field: 'views', aggregate: 'SUM' },
    chartType: 'area',
    curve: 'monotone',
    showDots: false,
    limit: 200,
  };
</script>

<div style="height: 400px;">
  <PicassoLineAreaChartCanvas {config} />
</div>
```
