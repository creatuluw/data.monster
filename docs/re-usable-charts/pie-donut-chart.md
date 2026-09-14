# Pie / Donut Chart — Re-usable Chart Spec

> Picasso.js implementation · `PicassoPieDonutCanvas` · `src/lib/charts/PicassoPieDonutCanvas.svelte`

---

## When to Use

Use the pie/donut chart when you need to:

- Show **part-to-whole proportions** for a small number of categories (3–12 slices ideal)
- Compare **relative magnitudes** of a single aggregated measure across discrete groups
- Present data where **percentage share matters more than absolute values**
- Render a **donut** variant to show a total value in the center hole
- Display with an **interactive color legend** docked to the right

Do NOT use for:

- More than ~12 slices (labels overlap, slices become unreadable)
- Time-series or sequential data (use a line/area chart)
- Comparing absolute values across categories (use a bar chart)
- Correlation between two measures (use a scatter plot)
- More than one dimension (this chart is single-dimension only)

---

## Dimensions & Measures

### Dimensions (slice categories)

| Constraint | Value |
|------------|-------|
| **Min** | `1` dimension (required) |
| **Max** | `1` dimension |

Accepted column types (categorical):

```
VARCHAR, TEXT, STRING, CHAR, BPCHAR, NAME, UUID, ENUM, BOOLEAN, BOOL
```

### Measures (slice size)

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

`PieDonutChartConfig` (`src/lib/charts/types.ts`)

```ts
interface PieDonutChartConfig {
  id: string;                               // Unique chart instance identifier
  table: string;                            // DuckDB source table name
  dimension: {
    field: string;                          // Column name for slice categories
    label?: string;                         // Display label (defaults to field)
  };
  metric: {
    field: string;                          // Column name for aggregation
    label?: string;                         // Display label (defaults to field)
    aggregate?: 'COUNT' | 'SUM' | 'AVG' | 'MIN' | 'MAX';  // Default: 'SUM'
  };
  chartType?: 'pie' | 'donut';             // Default: 'pie'
  filters?: string[];                       // Additional SQL WHERE clauses
  sortDirection?: 'ASC' | 'DESC';          // Default: 'DESC' (largest first)
  limit?: number;                           // Max slices (default: 20)
  showValues?: boolean;                     // Value labels on slices (default: false)
  showLegend?: boolean;                     // Color legend docked right (default: true)
  valueFormat?: 'number' | 'currency' | 'percent' | 'compact';  // Default: 'number'
  colors?: string[];                        // Override default PALETTE for slice colors
}
```

### Component Props

`PicassoPieDonutCanvas` accepts:

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `PieDonutChartConfig` | yes | Full chart configuration |
| `onSliceClick` | `(detail: { category, value, row }) => void` | no | Callback on slice click |

---

## Chart Requirements

### Runtime Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `picasso.js` | `^2.11.3` | Chart rendering engine (pie component) |

### Internal Dependencies

| Module | Exports Used |
|--------|-------------|
| `$lib/charts/types` | `PieDonutChartConfig`, `BarChartData` |
| `$lib/charts/engine/DataModelConnector` | `executePieDonutChartQuery` |
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
ORDER BY value {DESC|ASC}
LIMIT {n}
```

> **Note:** Default sort is `DESC` (largest slices first) with a limit of 20 to keep the chart readable.

### CSS Variables Required

```
--color-surface, --color-border, --color-accent, --color-accent-muted,
--color-accent-dark, --color-text, --color-text-secondary, --color-text-tertiary,
--color-text-on-accent, --color-danger, --color-surface-sunken,
--radius-lg, --radius-xs, --radius-full,
--space-1..space-6, --space-24,
--text-xs, --text-sm, --text-md, --text-lg,
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
| **Active** | Data loaded, chart mounted | Picasso SVG pie/donut with optional legend |

---

## Chart Anatomy — ASCII Node Tree

```
PicassoPieDonutCanvas
├── chart-shell                              ← border-radius: lg, flex column, relative positioning
│   ├── chart-header                         ← flex-shrink: 0, padded
│   │   └── chart-title-text                 ← "{Pie|Donut}: {AGG} of {metric} by {dimension}"
│   │
│   ├── [conditional: chartType === 'donut' && total > 0]
│   │   └── donut-center-hint                ← absolute center overlay, pointer-events: none
│   │       ├── donut-total                  ← formatted total value (mono, large, bold)
│   │       └── donut-total-label            ← "total" label (mono, 9px, uppercase)
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
│               │
│               ├── [conditional: showLegend === true]
│               │   └── <g> — legend-cat     ← categorical color legend, docked right
│               │       ├── <rect> × N       ← color swatches
│               │       └── <text> × N       ← category labels
│               │
│               └── <g> — pie                ← Picasso 'pie' component
│                   ├── <path> × N           ← arc slices
│                   │                        ← fill: color scale from PALETTE or config.colors
│                   │                        ← stroke: rgba(255,255,255,0.5), strokeWidth: 1
│                   │                        ← outerRadius: 0.9
│                   │                        ← cornerRadius: 4 (donut) | 1 (pie)
│                   │
│                   ├── [pie mode]
│                   │   └── innerRadius: 0   ← full pie, no hole
│                   │   └── padAngle: 0
│                   │
│                   └── [donut mode]
│                       └── innerRadius: 0.5 ← center hole
│                       └── padAngle: 0.01   ← small gap between slices
│
│               └── [conditional: showValues === true]
│                   └── <text> × N           ← value-label: injected post-render via DOM
│                                                text: "{value} ({pct}%)"
│                                                font: Lekton, 10px, fill: #333
│                                                [donut] positioned at slice center
│                                                [pie] positioned at 65% radius from center
```

### Page-Level Anatomy (consumer at `labs/picasso-charts/pie`)

```
+page.svelte
└── chart-detail                             ← flex row, height: 100%
    ├── chart-panel                          ← flex: 1, border-right
    │   ├── chart-panel-header               ← badge bar
    │   │   └── section-number               ← "PICASSO-003"
    │   │
    │   ├── chart-panel-body                 ← flex: 1, relative positioning
    │   │   ├── [unconfigured]
    │   │   │   └── chart-placeholder        ← centered icon + heading + description
    │   │   │       ├── PieChart (icon)
    │   │   │       ├── h3                   ← "Configure your chart"
    │   │   │       └── p                    ← instruction text
    │   │   │
    │   │   ├── PicassoPieDonutCanvas        ← (see anatomy above)
    │   │   │
    │   │   └── [conditional: slice clicked]
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
    │           │   └── orient-btn × 2       ← Pie · Donut
    │           │
    │           ├── config-section: Options
    │           │   ├── select#chart-sort     ← DESC / ASC
    │           │   ├── input#chart-limit     ← max slices (2–50)
    │           │   ├── input#chart-fqid      ← chart instance ID
    │           │   ├── toggle: showLegend    ← on/off switch
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
PieDonutChartConfig
    │
    ▼
compilePieDonutChartQuery(config, filterState?)  ──→  SQL string
    │
    ▼
executePieDonutChartQuery(config, filterState?)  ──→  { rows: BarChartData[], columns: string[] }
    │                                                  via Tauri IPC → DuckDB
    ▼
PicassoPieDonutCanvas
    │
    ├─ matrixData()   ──→  { type: 'matrix', data: [[header], ...rows] }
    │
    ├─ buildSettings()  ──→  Picasso scales + components
    │   ├── scales: { c: color scale mapped to PALETTE or config.colors }
    │   └── components:
    │       ├── [legend-cat]              ← docked right, color scale
    │       └── [pie]                     ← arc slices
    │           ├── arc.ref: 'num' (value field)
    │           ├── fill: color scale 'c'
    │           ├── outerRadius: 0.9
    │           ├── innerRadius: 0 (pie) | 0.5 (donut)
    │           ├── cornerRadius: 1 (pie) | 4 (donut)
    │           └── padAngle: 0 (pie) | 0.01 (donut)
    │
    └─ picasso.chart({ element, data, settings })
        └─ Renders <svg> into chart-body container
```

### Reactive Triggers

Data is re-fetched when any of these config fields change:

`table` · `dimension.field` · `metric.field` · `metric.aggregate` · `sortDirection` · `limit` · `chartType`

The chart is re-mounted when data rows or visual settings change:

`chartType` · `showValues` · `valueFormat` · `showLegend` · `colors`

---

## Pie vs Donut Mode

| Aspect | Pie Mode | Donut Mode |
|--------|----------|------------|
| `chartType` | `'pie'` | `'donut'` |
| `innerRadius` | `0` (no hole) | `0.5` (50% radius hole) |
| `padAngle` | `0` (flush) | `0.01` (small gap) |
| `cornerRadius` | `1` | `4` |
| Center display | None | Total value + "total" label |
| Best for | Few categories, emphasis on proportion | Many categories, modern look, KPI in center |
| Visual weight | Heavy | Medium |

---

## Interaction: Slice Click

```
User clicks slice
    │
    ▼
chart.shapesAt({x, y, w:1, h:1})  ──→  hit-test → find category
    │
    ▼
onSliceClick({ category, value, row })  ──→  consumer callback
    │
    ▼
Toast notification with category + value (2s timeout)
```

> Unlike the bar chart, the pie/donut does not maintain a toggle-selection filter set. Click is a simple event callback.

---

## Re-use Checklist

To embed this chart in any page:

1. Import `PicassoPieDonutCanvas` from `$lib/charts`
2. Import `PieDonutChartConfig` from `$lib/charts/types`
3. Build a `PieDonutChartConfig` object with at minimum: `id`, `table`, `dimension`, `metric`
4. Bind `config` to the component: `<PicassoPieDonutCanvas {config} onSliceClick={handler} />`
5. Ensure the container has a defined height (the chart fills 100% height via flex)
6. Ensure DuckDB is connected and the target table is queryable
7. Optionally handle `onSliceClick` for external state updates

Minimal example (pie):

```svelte
<script>
  import { PicassoPieDonutCanvas } from '$lib/charts';
  import type { PieDonutChartConfig } from '$lib/charts/types';

  const config: PieDonutChartConfig = {
    id: 'revenue-by-category',
    table: 'orders',
    dimension: { field: 'category' },
    metric: { field: 'revenue', aggregate: 'SUM' },
    chartType: 'pie',
    limit: 8,
  };
</script>

<div style="height: 400px;">
  <PicassoPieDonutCanvas {config} />
</div>
```

Minimal example (donut):

```svelte
<script>
  import { PicassoPieDonutCanvas } from '$lib/charts';
  import type { PieDonutChartConfig } from '$lib/charts/types';

  const config: PieDonutChartConfig = {
    id: 'traffic-sources',
    table: 'analytics',
    dimension: { field: 'source' },
    metric: { field: 'sessions', aggregate: 'SUM' },
    chartType: 'donut',
    showValues: true,
    valueFormat: 'percent',
    limit: 6,
  };
</script>

<div style="height: 400px;">
  <PicassoPieDonutCanvas {config} />
</div>
```
