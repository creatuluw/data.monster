# Scatter Plot — Re-usable Chart Spec

> Picasso.js implementation · `PicassoScatterCanvas` · `src/lib/charts/PicassoScatterCanvas.svelte`

---

## When to Use

Use the scatter plot when you need to:

- Reveal **correlation between two numeric measures** (e.g. price vs. quantity, revenue vs. margin)
- Plot **raw data points** without aggregation — each row is a dot
- Optionally **color-code by a categorical group** to spot cluster patterns
- Show **distribution and outliers** across two dimensions simultaneously
- Render up to **10,000 data points** in a single chart

Do NOT use for:

- Time-series trends (use a line/area chart)
- Part-to-whole proportions (use a pie/donut chart)
- Comparing discrete categories by a single measure (use a bar chart)
- Data that requires aggregation before plotting (this chart plots raw rows)

---

## Axes & Data Model

The scatter plot is fundamentally different from the bar/line/pie charts. It does **not** use `GROUP BY` aggregation. Instead, it selects raw column values and plots each row as a point.

### X axis (horizontal, numeric)

| Constraint | Value |
|------------|-------|
| **Min** | `1` numeric field (required) |
| **Max** | `1` numeric field |

Accepted column types:

```
INTEGER, BIGINT, SMALLINT, TINYINT, INT, INT2, INT4, INT8,
DOUBLE, FLOAT, FLOAT4, FLOAT8, REAL, DECIMAL, NUMERIC,
HUGEINT, UINTEGER, UBIGINT, USMALLINT, UTINYINT
```

### Y axis (vertical, numeric)

| Constraint | Value |
|------------|-------|
| **Min** | `1` numeric field (required) |
| **Max** | `1` numeric field |

Same accepted types as X axis.

### Color grouping (optional, categorical)

| Constraint | Value |
|------------|-------|
| **Min** | `0` (ungrouped — all points same color) |
| **Max** | `1` categorical field |

Accepted column types:

```
VARCHAR, TEXT, STRING, CHAR, BPCHAR, NAME, UUID, ENUM, BOOLEAN, BOOL
```

When a group field is provided, points are colored by category using the PALETTE color scale, and a legend is rendered on the right.

---

## Config Options

`ScatterChartConfig` (`src/lib/charts/types.ts`)

```ts
interface ScatterChartConfig {
  id: string;                               // Unique chart instance identifier
  table: string;                            // DuckDB source table name
  xField: {
    field: string;                          // Column name for X axis
    label?: string;                         // Display label (defaults to field)
  };
  yField: {
    field: string;                          // Column name for Y axis
    label?: string;                         // Display label (defaults to field)
  };
  groupBy?: {                               // Optional color grouping
    field: string;                          // Categorical column for color groups
    label?: string;                         // Display label
  };
  filters?: string[];                       // Additional SQL WHERE clauses
  limit?: number;                           // Max rows (default: 500)
  pointSize?: number;                       // Point radius in px (default: 8)
  pointOpacity?: number;                    // Point opacity 0–1 (default: 0.75)
  showLegend?: boolean;                     // Color legend (default: true, only if groupBy)
  showValues?: boolean;                     // Value labels on points (default: false)
  valueFormat?: 'number' | 'currency' | 'percent' | 'compact';  // Default: 'number'
  colors?: string[];                        // Override default PALETTE
}
```

### Component Props

`PicassoScatterCanvas` accepts:

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `ScatterChartConfig` | yes | Full chart configuration |
| `onPointClick` | `(detail: { x, y, group, row }) => void` | no | Callback on point click |
| `onSelectionChange` | `(selected: BarChartData[]) => void` | no | Callback when selection changes (click or lasso) |

---

## Chart Requirements

### Runtime Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `picasso.js` | `^2.11.3` | Chart rendering engine (point component) |

### Internal Dependencies

| Module | Exports Used |
|--------|-------------|
| `$lib/charts/types` | `ScatterChartConfig`, `BarChartData` |
| `$lib/charts/engine/DataModelConnector` | `executeScatterChartQuery` |
| `$lib/charts/chart-helpers` | `PALETTE` (10-color oklch palette) |
| `$lib/db-operations` | `extractErrorMessage` |

### Data Requirements

- A connected DuckDB instance with the target table available
- `executeQuery()` from `$lib/db-operations` must be functional (Tauri IPC)
- The target table must have at least two numeric columns (x and y)
- No aggregation is performed — raw rows are plotted directly

### Generated SQL Pattern

```sql
SELECT "{xField}", "{yField}"[, "{groupBy}"]
FROM "{table}"
[WHERE {filters}]
LIMIT {n}
```

> **Key difference from bar/line/pie:** No `GROUP BY`, no `ORDER BY`, no aggregate function. This chart plots raw data.

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
| **Active** | Data loaded, chart mounted | Picasso SVG scatter plot with axes |

---

## Chart Anatomy — ASCII Node Tree

```
PicassoScatterCanvas
├── chart-shell                              ← border-radius: lg, flex column, fills height
│   ├── chart-header                         ← flex row, align center
│   │   ├── chart-title-text                 ← "Scatter: {yLabel} vs {xLabel}"
│   │   └── [conditional: selection active]
│   │       └── selection-count              ← "{n} selected" badge
│   │
│   ├── [conditional: selectedIndices.size > 0]
│   │   └── filter-bar                       ← flex wrap row, gap, border-bottom
│   │       ├── filter-chip × N (max 20)     ← "{xVal}, {yVal}" + × close
│   │       ├── filter-more                  ← "+{n} more" (if > 20)
│   │       └── filter-clear                 ← "Clear all" dashed button
│   │
│   └── chart-body                           ← flex: 1, cursor: crosshair, position: relative
│       │                                    ← Picasso mounts <svg> here
│       │
│       ├── lasso                            ← absolute div, hidden by default
│       │                                    ← dashed border + accent bg
│       │                                    ← shown on mouse drag (lasso select)
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
│               ├── <g> — y-axis             ← docked left, value scale
│               │   └── <text> × N           ← tick labels
│               │
│               ├── <g> — x-axis             ← docked bottom, value scale
│               │   └── <text> × N           ← tick labels
│               │
│               ├── [conditional: groupBy set && showLegend === true]
│               │   └── <g> — legend-cat     ← docked right, color scale
│               │       ├── <rect> × N       ← color swatches (per unique group)
│               │       └── <text> × N       ← group labels
│               │
│               └── <g> — points             ← Picasso 'point' component
│                   └── <circle> × N         ← one per data row
│                       ├── cx, cy           ← mapped from xField, yField via scales
│                       ├── r                ← pointSize * 0.06
│                       ├── fill             ← color scale (grouped) or PALETTE[0]
│                       ├── stroke           ← #fff, strokeWidth: 2 (selected) | 1.5
│                       └── opacity          ← pointOpacity | 1 (selected) | 0.2 (unselected)
```

### Page-Level Anatomy (consumer at `labs/picasso-charts/scatter`)

```
+page.svelte
└── chart-detail                             ← flex row, height: 100%
    ├── chart-panel                          ← flex: 1, border-right
    │   ├── chart-panel-header               ← badge bar
    │   │   └── section-number               ← "PICASSO-004"
    │   │
    │   ├── chart-panel-body                 ← flex: 1, relative positioning
    │   │   ├── [unconfigured]
    │   │   │   └── chart-placeholder        ← centered icon + heading + description
    │   │   │       ├── ScatterChart (icon)
    │   │   │       ├── h3                   ← "Configure your chart"
    │   │   │       └── p                    ← instruction text
    │   │   │
    │   │   ├── PicassoScatterCanvas         ← (see anatomy above)
    │   │   │
    │   │   └── [conditional: point clicked]
    │   │       └── bar-toast                ← fixed bottom-center popup, 2s timeout
    │   │           ├── bar-toast-key         ← group name (bold)
    │   │           ├── bar-toast-val         ← "x: {value}" (dimmed)
    │   │           └── bar-toast-val         ← "y: {value}" (dimmed)
    │   │
    │   └── aside.drawer                     ← width: 360px, flex column
    │       ├── drawer-header                ← icon + "Chart config"
    │       │   └── Settings (icon)
    │       │
    │       └── drawer-body                  ← scrollable config form
    │           ├── config-section: Data
    │           │   ├── select#chart-table    ← table selector
    │           │   ├── select#chart-x        ← numeric column for X axis
    │           │   ├── select#chart-y        ← numeric column for Y axis
    │           │   └── select#chart-group    ← categorical column (optional, "None" default)
    │           │
    │           ├── config-section: Options
    │           │   ├── input#chart-limit     ← max points (1–10000)
    │           │   ├── input[range]#chart-size ← point size slider (2–24px)
    │           │   ├── input[range]#chart-opacity ← point opacity slider (0.1–1.0)
    │           │   ├── input#chart-fqid      ← chart instance ID
    │           │   └── toggle: showLegend    ← on/off switch
    │           │
    │           └── config-section: Generated SQL
    │               └── sql-preview          ← <pre><code> read-only SQL
```

---

## Data Flow

```
ScatterChartConfig
    │
    ▼
compileScatterChartQuery(config, filterState?)  ──→  SQL string
    │
    ▼
executeScatterChartQuery(config, filterState?)  ──→  { rows: BarChartData[], columns: string[] }
    │                                                  via Tauri IPC → DuckDB
    ▼
PicassoScatterCanvas
    │
    ├─ matrixData()   ──→  { type: 'matrix', data: [[header], ...rows] }
    │   └── columns: [xField, yField, groupBy | '__row_id__']
    │
    ├─ buildSettings()  ──→  Picasso scales + components
    │   ├── scales:
    │   │   ├── x: { data: { field: xField }, expand: 0.1 }
    │   │   ├── y: { data: { field: yField }, expand: 0.15, invert: true }
    │   │   └── col: { type: 'color', range: colors } (only if groupBy)
    │   └── components:
    │       ├── [y-axis]         ← docked left, scale 'y'
    │       ├── [x-axis]         ← docked bottom, scale 'x'
    │       ├── [legend-cat]     ← docked right, scale 'col' (only if groupBy && showLegend)
    │       └── [points]         ← Picasso 'point' component
    │           ├── x: { scale: 'x' }
    │           ├── y: { scale: 'y' }
    │           ├── shape: 'circle'
    │           ├── fill: { scale: 'col', ref: 'group' } | PALETTE[0]
    │           ├── size: pointSize / 20
    │           └── opacity: pointOpacity
    │
    └─ picasso.chart({ element, data, settings })
        └─ Renders <svg> into chart-body container
```

### Reactive Triggers

Data is re-fetched when any of these config fields change:

`table` · `xField.field` · `yField.field` · `groupBy.field` · `limit`

The chart is re-mounted when data rows or visual settings change:

`pointSize` · `pointOpacity` · `showLegend` · `colors`

---

## Interaction: Point Click & Lasso Selection

### Click to select

```
User clicks point
    │
    ▼
chart.shapesAt({x, y, w: hitPad*2, h: hitPad*2})  ──→  hit-test → find closest point
    │
    ▼
Toggle index in selectedIndices Set
    │
    ▼
applySelection()
    ├── chart.update({ settings })  ──→  selected → opacity 1, unselected → opacity 0.2
    └── onSelectionChange([...rows])
    │
    ▼
filter-bar renders chips for selected points
    └── chip click → removeIndex()
        └── "Clear all" → clearAll()
```

### Lasso select (drag)

```
User presses mouse button (mousedown)
    │
    ▼
Record startX, startY, set lassoState.active = true
    │
    ▼
User drags (mousemove)
    │
    ▼
lassoEl: dashed border rectangle follows cursor
    │
    ▼
User releases mouse (mouseup)
    │
    ├── [drag < 5px]  ──→  handleClick() (single point toggle)
    │
    └── [drag >= 5px]  ──→  LASSO SELECTION
        │
        ▼
    chart.shapesAt({x, y, width, height})  ──→  all points in rectangle
        │
        ▼
    Add all hit point indices to selectedIndices Set
        │
        ▼
    applySelection()
        ├── chart.update({ settings })  ──→  dim unselected, highlight selected
        └── onSelectionChange([...rows])
```

---

## Ungrouped vs Grouped Mode

| Aspect | Ungrouped | Grouped (groupBy set) |
|--------|-----------|----------------------|
| `groupBy` | `undefined` | `{ field: '...' }` |
| Point color | Single color (PALETTE[0]) | Color scale by category |
| Legend | Hidden | Shown (right dock) |
| SQL columns | `xField, yField` | `xField, yField, groupBy` |
| Matrix header | `[x, y, '__row_id__']` | `[x, y, groupField]` |
| Best for | Simple correlation check | Cluster analysis, multi-category comparison |

---

## Re-use Checklist

To embed this chart in any page:

1. Import `PicassoScatterCanvas` from `$lib/charts`
2. Import `ScatterChartConfig` from `$lib/charts/types`
3. Build a `ScatterChartConfig` object with at minimum: `id`, `table`, `xField`, `yField`
4. Bind `config` to the component: `<PicassoScatterCanvas {config} onPointClick={handler} />`
5. Ensure the container has a defined height (the chart fills 100% height via flex)
6. Ensure DuckDB is connected and the target table is queryable
7. Optionally handle `onPointClick` for external state updates

Minimal example (ungrouped):

```svelte
<script>
  import { PicassoScatterCanvas } from '$lib/charts';
  import type { ScatterChartConfig } from '$lib/charts/types';

  const config: ScatterChartConfig = {
    id: 'price-vs-quantity',
    table: 'products',
    xField: { field: 'price' },
    yField: { field: 'quantity_sold' },
    limit: 1000,
  };
</script>

<div style="height: 400px;">
  <PicassoScatterCanvas {config} />
</div>
```

Minimal example (grouped):

```svelte
<script>
  import { PicassoScatterCanvas } from '$lib/charts';
  import type { ScatterChartConfig } from '$lib/charts/types';

  const config: ScatterChartConfig = {
    id: 'margin-vs-revenue',
    table: 'sales',
    xField: { field: 'revenue' },
    yField: { field: 'margin' },
    groupBy: { field: 'region' },
    pointSize: 10,
    pointOpacity: 0.7,
    limit: 2000,
  };
</script>

<div style="height: 400px;">
  <PicassoScatterCanvas {config} />
</div>
```
