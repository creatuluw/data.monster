# Re-usable Chart Component Architecture

> A unified chart system for Data Monster that can be dropped onto any page, configured visually, and
> wired into the DuckDB data model with full interactivity (click-to-filter, expressions, cross-table joins).

---

## Context

The draft chart in `data.monster/src/routes/+page.svelte` proved the concept: a bar chart built with
`@carbon/charts-svelte` where clicking bars filters the underlying data, group-by and metric dropdowns
drive aggregation, and summary KPI cards reflect the filtered state. However, it is tightly coupled to a
single page, hardcodes fields/metrics, and has no configurability beyond the two dropdowns.

This document defines the architecture for a **re-usable, configurable chart system** that generalises
that pattern into something we can place on any page in the app.

---

## ASCII Node Tree — Chart Component Architecture

```
ChartSystem
│
├── ChartCanvas                          ← top-level host component, dropped onto any page
│   ├── chartInstanceId: string          ← unique id for state isolation
│   ├── config: ChartConfig              ← resolved config (defaults + user overrides)
│   ├── data: ChartDataResult            ← the query result feeding the visual
│   ├── filterState: FilterState         ← active click-to-filter selections
│   ├── isLoading: boolean
│   ├── error: string | null
│   │
│   ├── ChartToolbar                     ← per-chart action bar (collapsible)
│   │   ├── ChartTypeSwitcher            ← swap between bar / line / scatter / etc.
│   │   ├── QuickActions
│   │   │   ├── RefreshData
│   │   │   ├── ResetFilters
│   │   │   ├── ToggleFullscreen
│   │   │   └── ExportChart              ← PNG / SVG / CSV
│   │   └── EditConfigButton             ← opens ChartConfigEditor sidebar
│   │
│   ├── ChartRenderer                    ← renders the selected chart library component
│   │   ├── chartType: ChartType         ← 'bar' | 'line' | 'area' | 'scatter' | 'pie' | ...
│   │   ├── chartLibrary: ChartLibrary   ← 'layerchart' | 'svelteplot' | 'picasso' (auto-selected or override)
│   │   ├── data: any[]
│   │   ├── options: RenderOptions       ← axes, colors, labels, grid, legend, etc.
│   │   ├── onElementClick: handler      ← bubbles up to FilterEngine
│   │   ├── onElementHover: handler      ← tooltips
│   │   ├── width / height / responsive
│   │   │
│   │   ├── Renderers (library adapters)
│   │   │   ├── LayerChartRenderer       ← wraps `layerchart` components
│   │   │   ├── SveltePlotRenderer       ← wraps `svelteplot` <Plot> + marks
│   │   │   └── PicassoRenderer          ← wraps picasso.js canvas charts
│   │   │
│   │   └── ChartVisual (what actually renders)
│   │       ├── Axes                     ← x-axis, y-axis, labels, ticks, grid lines
│   │       │   ├── x: AxisConfig        ← field, label, scale type, format, inverted, domain
│   │       │   └── y: AxisConfig        ← field, label, scale type, format, inverted, domain
│   │       ├── Marks                    ← the data-driven visual elements
│   │       │   ├── bars / lines / dots / areas / slices / cells
│   │       │   ├── color: ColorConfig   ← palette, scale, conditional coloring
│   │       │   ├── size: SizeConfig     ← bar width, dot radius, stroke width
│   │       │   └── opacity: number
│   │       ├── Legend                   ← show/hide, position, interactive toggle
│   │       ├── Tooltip                 ← hover tooltip template, fields, formatting
│   │       ├── Annotations             ← reference lines, labels, thresholds
│   │       └── InteractionLayer        ← click, brush, zoom, pan overlays
│   │
│   ├── FilterEngine                     ← manages click-to-filter state
│   │   ├── activeFilters: Map<string, Set<string>>
│   │   │   └── dimension → selected values
│   │   ├── filterMode: 'include' | 'exclude' | 'single'
│   │   ├── crossFilterSync: boolean     ← propagate filters to other charts on same page
│   │   ├── onFilterChange → re-queries data with WHERE clause appended
│   │   └── FilterChipBar                ← shows active filters as removable chips
│   │       ├── FilterChip[]             ← one per active filter value
│   │       └── ClearAllButton
│   │
│   └── SummaryStrip                     ← optional KPI summary cards below chart
│       └── SummaryCard[]                ← count, sum, avg, top-N — driven by same data
│
├── ChartLibrary                         ← pick-a-chart browser (modal / sidebar panel)
│   ├── categories: ChartCategory[]
│   │   ├── Comparison                   ← bar, grouped bar, lollipop, bullet
│   │   ├── Trend                        ← line, area, sparkline
│   │   ├── Distribution                 ← histogram, box plot, violin
│   │   ├── Correlation                  ← scatter, bubble
│   │   ├── Composition                  ← pie, donut, treemap, stacked bar/area
│   │   ├── Table                        ← data table with sorting/paging
│   │   └── KPI                          ← single value tile, gauge
│   │
│   ├── ChartTemplate                    ← a selectable chart type definition
│   │   ├── name: string                 ← "Bar Chart"
│   │   ├── chartType: string            ← "BarChart"
│   │   ├── description: string
│   │   ├── tags: string[]
│   │   ├── thumbnail: string            ← preview image or SVG
│   │   ├── minMetrics / maxMetrics
│   │   ├── minDimensions / maxDimensions
│   │   ├── defaultConfig: Partial<ChartConfig>
│   │   └── sampleData: any[]
│   │
│   ├── SearchBar                         ← filter templates by name / tag
│   └── TemplateGrid                      ← visual grid of chart thumbnails to pick from
│
├── ChartConfigEditor                    ← sidebar / modal for editing all chart properties
│   │
│   ├── DataSourcePanel                  ← wire chart to data model
│   │   ├── TableSelector                ← pick base table from DuckDB
│   │   ├── FieldMapper
│   │   │   ├── DimensionPicker[]        ← x-axis, group-by, color-by fields
│   │   │   │   └── shows columns from selected table (with type icons)
│   │   │   ├── MetricPicker[]           ← y-axis, size, value fields
│   │   │   │   └── shows numeric columns + expression option
│   │   │   └── RelatedTableJoiner       ← traverse foreign keys to pull in fields from linked tables
│   │   │       ├── detectedRelations: Relation[]    ← auto-detected from DuckDB schema
│   │   │       ├── joinConfig: { table, on, type }  ← LEFT / INNER, join clause
│   │   │       └── joinedFields become available in DimensionPicker / MetricPicker
│   │   │
│   │   ├── FilterClause[]               ← pre-filter the data (WHERE)
│   │   │   └── field + operator + value
│   │   │
│   │   └── SortClause[]                 ← ORDER BY
│   │       └── field + direction
│   │
│   ├── ExpressionEditor                 ← metric formula / UDF input
│   │   ├── expressionText: string       ← the raw expression string
│   │   ├── FunctionLibrary
│   │   │   ├── Aggregates               ← SUM, AVG, COUNT, MIN, MAX, MEDIAN, STDDEV
│   │   │   ├── Window Functions         ← ROW_NUMBER, RANK, LAG, LEAD, SUM OVER
│   │   │   ├── String Functions         ← CONCAT, SUBSTR, UPPER, LOWER, REGEX_MATCHES
│   │   │   ├── Date Functions           ← YEAR, MONTH, DATE_TRUNC, DATEDIFF
│   │   │   ├── Math Functions           ← ROUND, CEIL, FLOOR, ABS, LOG, SQRT
│   │   │   └── Custom UDFs              ← user-defined SQL macros stored in DuckDB
│   │   │
│   │   ├── Autocomplete                 ← token-aware suggestions (columns, functions, operators)
│   │   ├── SyntaxValidation             ← validates against DuckDB SQL dialect
│   │   └── PreviewResult                ← shows a few sample rows from the expression
│   │
│   ├── VisualPanel                      ← all visual/customization properties
│   │   ├── General
│   │   │   ├── title: string            ← chart title text
│   │   │   ├── subtitle: string
│   │   │   ├── description: string      ← alt-text / accessible description
│   │   │   └── aspectRatio: number      ← width:height ratio
│   │   │
│   │   ├── Axes
│   │   │   ├── x-axis config
│   │   │   │   ├── label: string
│   │   │   │   ├── scaleType: 'linear' | 'log' | 'band' | 'time' | 'point'   [default: auto-detected]
│   │   │   │   ├── format: string        ← number/date format pattern              [default: auto]
│   │   │   │   ├── domain: [min, max]    ← override auto domain                    [default: auto]
│   │   │   │   ├── gridVisible: boolean                                            [default: false]
│   │   │   │   ├── inverted: boolean                                               [default: false]
│   │   │   │   └── tickCount: number                                               [default: auto]
│   │   │   └── y-axis config              ← same shape as x-axis
│   │   │       └── gridVisible default is true for y-axis
│   │   │
│   │   ├── Colors
│   │   │   ├── palette: string           ← 'categorical' | 'sequential' | 'diverging' | custom
│   │   │   ├── scheme: string            ← named color scheme (e.g. 'viridis', 'tableau10')
│   │   │   ├── customColors: string[]    ← explicit color array
│   │   │   └── colorByField: string      ← field used for color segmentation
│   │   │
│   │   ├── Legend
│   │   │   ├── visible: boolean                                                    [default: true]
│   │   │   ├── position: 'top' | 'right' | 'bottom' | 'left'                      [default: 'right']
│   │   │   └── interactive: boolean      ← clicking legend items toggles visibility [default: true]
│   │   │
│   │   ├── Tooltip
│   │   │   ├── enabled: boolean                                                    [default: true]
│   │   │   ├── template: string         ← custom HTML template with {{field}} tokens
│   │   │   └── fields: string[]         ← which fields to show
│   │   │
│   │   ├── Marks (chart-type-specific)
│   │   │   ├── bar
│   │   │   │   ├── cornerRadius: number                                            [default: 0]
│   │   │   │   ├── paddingInner: number                                            [default: 0.1]
│   │   │   │   ├── paddingOuter: number                                            [default: 0.05]
│   │   │   │   └── orientation: 'vertical' | 'horizontal'                         [default: 'vertical']
│   │   │   ├── line
│   │   │   │   ├── strokeWidth: number                                             [default: 2]
│   │   │   │   ├── curve: 'linear' | 'monotone' | 'step' | 'natural'              [default: 'monotone']
│   │   │   │   ├── showDots: boolean                                               [default: false]
│   │   │   │   └── showArea: boolean                                               [default: false]
│   │   │   ├── scatter
│   │   │   │   ├── dotRadius: number                                               [default: 4]
│   │   │   │   ├── opacity: number                                                 [default: 0.7]
│   │   │   │   └── sizeByField: string | null                                      [default: null]
│   │   │   ├── pie / donut
│   │   │   │   ├── innerRadius: number                  (0 = pie, >0 = donut)     [default: 0]
│   │   │   │   ├── padAngle: number                                                [default: 0]
│   │   │   │   └── showLabels: boolean                                             [default: true]
│   │   │   └── heatmap
│   │   │       ├── colorScheme: string                                             [default: 'blues']
│   │   │       ├── showValues: boolean                                             [default: false]
│   │   │       └── borderRadius: number                                            [default: 2]
│   │   │
│   │   └── Annotations
│   │       ├── ReferenceLine[]         ← value, label, color, dash pattern
│   │       └── ReferenceArea[]         ← x/y range, fill, label
│   │
│   ├── InteractionPanel                 ← click-to-filter and cross-filter config
│   │   ├── clickToFilter: boolean                                                 [default: true]
│   │   ├── filterMode: 'include' | 'exclude' | 'single'                          [default: 'include']
│   │   ├── crossFilterWith: chartInstanceId[]     ← sync with other charts on page
│   │   ├── brushSelection: boolean                                                [default: false]
│   │   └── zoomPan: boolean                                                       [default: false]
│   │
│   └── ExportPanel                      ← export options
│       ├── format: 'png' | 'svg' | 'csv' | 'json'
│       ├── includeTitle: boolean
│       └── resolution: number
│
├── ChartConfig                          ← the serializable config object (JSON)
│   ├── id: string                       ← chart instance unique id
│   ├── pageId: string                   ← which page this chart lives on
│   ├── chartType: ChartType
│   ├── dataConfig: DataConfig
│   │   ├── baseTable: string
│   │   ├── joins: JoinConfig[]
│   │   │   └── { table: string, on: string, type: 'LEFT' | 'INNER' }
│   │   ├── dimensions: DimensionConfig[]
│   │   │   └── { field: string, alias?: string, label?: string }
│   │   ├── metrics: MetricConfig[]
│   │   │   └── { field: string, alias?: string, label?: string, expression?: string, aggregate?: string }
│   │   ├── filters: FilterConfig[]
│   │   │   └── { field: string, operator: string, value: any }
│   │   ├── sorts: SortConfig[]
│   │   │   └── { field: string, direction: 'ASC' | 'DESC' }
│   │   └── limit: number | null
│   ├── visualConfig: VisualConfig       ← all the props from VisualPanel
│   ├── interactionConfig: InteractionConfig
│   └── version: number                  ← for config migration
│
├── DataModelConnector                   ← bridges chart config → DuckDB SQL execution
│   ├── resolveConfig(config) → SQL      ← compiles ChartConfig into a SQL query
│   │   ├── builds SELECT clause from dimensions + metric expressions
│   │   ├── builds FROM + JOINs from baseTable + joins
│   │   ├── builds WHERE from filters + active filter state
│   │   ├── builds GROUP BY from dimensions
│   │   ├── builds ORDER BY from sorts
│   │   └── builds LIMIT
│   │
│   ├── executeQuery(sql) → ChartDataResult
│   │   └── calls Tauri invoke('execute_query', { sql })
│   │
│   ├── getSchema() → TableSchema[]      ← introspects DuckDB for table/column metadata
│   │   └── uses DESCRIBE + information_schema
│   │
│   ├── detectRelations() → Relation[]   ← finds foreign-key-like relationships
│   │   ├── column name matching (e.g. orders.customer_id → customers.id)
│   │   └── explicit FK constraints if defined
│   │
│   └── validateExpression(expr) → { valid: boolean, error?: string }
│       └── dry-run EXPLAIN on the expression
│
├── ExpressionEngine                     ← formula / UDF subsystem
│   ├── parseExpression(text) → AST
│   ├── resolveFields(ast, schema) → string[]   ← which columns are referenced
│   ├── compileToSQL(ast) → string               ← generates SQL fragment
│   ├── FunctionRegistry
│   │   ├── builtIn: Map<string, FunctionDef>    ← DuckDB built-in functions
│   │   └── customUDFs: Map<string, UDFFef>      ← user-created SQL macros
│   │
│   ├── UDFManager
│   │   ├── createUDF(name, params, body)        ← CREATE MACRO
│   │   ├── listUDFs() → UDF[]
│   │   └── deleteUDF(name)
│   │
│   └── AutocompleteProvider
│       ├── columnSuggestions(context) → string[]
│       ├── functionSuggestions(prefix) → string[]
│       └── operatorSuggestions(context) → string[]
│
└── ChartStateStore                      ← global chart state (Svelte 5 runes store)
    ├── instances: Map<string, ChartInstanceState>
    │   └── ChartInstanceState
    │       ├── config: ChartConfig
    │       ├── data: any[]
    │       ├── filterState: FilterState
    │       ├── isLoading: boolean
    │       └── error: string | null
    │
    ├── registerChart(config) → chartInstanceId
    ├── removeChart(id)
    ├── updateConfig(id, partial)
    ├── refreshData(id)
    └── syncFilters(sourceId, filters)    ← cross-chart filter propagation
```

---

## Default Values Strategy (80/20 Rule)

Every configurable property above has a sensible default. Users should only need to specify:

| What the user picks | Everything else defaults to |
|---|---|
| Chart type | Best renderer for that type |
| Base table | All columns listed |
| X field (dimension) | Auto-detected scale type, format, label |
| Y field (metric) | Auto-aggregation (SUM), number formatting, y-axis grid on |
| — | Color palette: categorical-10 from design tokens |
| — | Legend: visible, right-aligned, interactive |
| — | Tooltip: enabled, shows all mapped fields |
| — | Click-to-filter: enabled, include mode |
| — | Height: 400px, responsive width |
| — | Title: auto-generated from "{metric} by {dimension}" |
| — | Margins: 60/40/20/20 (left/bottom/top/right) |

### Defaults by Chart Type

```
Bar Chart:
  ├── x.scaleType: 'band'
  ├── y.gridVisible: true
  ├── x.gridVisible: false
  ├── bar.cornerRadius: 0
  ├── bar.paddingInner: 0.1
  ├── bar.paddingOuter: 0.05
  └── legend.visible: false (single series)

Line Chart:
  ├── x.scaleType: 'time' or 'point' (auto)
  ├── y.gridVisible: true
  ├── line.curve: 'monotone'
  ├── line.strokeWidth: 2
  ├── line.showDots: false
  └── legend.visible: false (single series)

Scatter Plot:
  ├── x.gridVisible: true
  ├── y.gridVisible: true
  ├── scatter.dotRadius: 4
  ├── scatter.opacity: 0.7
  └── tooltip.showCoordinates: true

Pie / Donut:
  ├── no axes
  ├── legend.visible: true, position: 'right'
  ├── innerRadius: 0 (pie) or 0.6 (donut, if toggled)
  └── showLabels: true

Heatmap:
  ├── colorScheme: 'blues'
  ├── x.gridVisible: false
  ├── y.gridVisible: false
  └── showValues: false (auto-enable if cells > 20px)
```

---

## Data Flow

```
 User picks chart from ChartLibrary
        │
        ▼
 ChartConfigEditor opens → user maps fields
        │
        ▼
 ChartConfig is compiled
        │
        ▼
 DataModelConnector.resolveConfig(config)
        │
        ├── builds SQL from config:
        │     SELECT dimension, SUM(metric_expr) as value
        │     FROM base_table
        │     [LEFT JOIN related_table ON ...]
        │     WHERE [filters + active filter state]
        │     GROUP BY dimension
        │     ORDER BY value DESC
        │     LIMIT N
        │
        ▼
 DataModelConnector.executeQuery(sql)
        │
        ▼ (via Tauri invoke → DuckDB native)
        │
        ▼
 ChartDataResult → { rows: any[], schema: ColumnDef[] }
        │
        ▼
 ChartRenderer receives data + options
        │
        ├── selects renderer adapter (layerchart / svelteplot / picasso)
        │
        ▼
 Visual renders with interaction layer
        │
        ▼ (user clicks a bar)
        │
 FilterEngine.onElementClick(element)
        │
        ├── toggles value in activeFilters
        │
        ▼
 DataModelConnector re-queries with updated WHERE
        │
        ▼
 Chart re-renders with filtered data
        │
        ▼ (if crossFilterSync enabled)
        │
 ChartStateStore.syncFilters(sourceId, filters)
        │
        └── other charts on same page re-query
```

---

## Interaction: Click-to-Filter

This was the core interaction in the draft. The architecture preserves and generalises it:

```
 Click on bar/segment/dot
        │
        ▼
 InteractionLayer captures element + metadata
        │
        ├── extracts: { dimension: string, value: string }
        │
        ▼
 FilterEngine.toggleFilter(dimension, value)
        │
        ├── If value already in activeFilters[dimension]:
        │     └── remove it (deselect)
        ├── Else:
        │     └── add it (select)
        │
        ▼
 Visual update (immediate, optimistic):
        │
        ├── Selected elements: full color from palette
        ├── Unselected elements: dimmed (oklch(0.9 0.005 250 / 0.45))
        │   (matches the DIMMED constant from the draft)
        │
        ▼
 FilterChipBar updates with removable chips
        │
        ▼
 DataModelConnector re-queries (WHERE dimension IN (...))
        │
        ▼
 SummaryStrip recalculates KPIs from filtered data
```

---

## Cross-Table Data via Relationships

The Data Monster data model lives in DuckDB. Tables are loaded from user files and can reference
each other through naming conventions or explicit relationships. The chart system leverages this:

```
 Example: Sales dashboard chart spanning orders + customers + products

 ChartConfig:
   baseTable: 'orders'
   joins:
     - table: 'customers'
       on: 'orders.customer_id = customers.id'
       type: 'LEFT'
     - table: 'products'
       on: 'orders.product_id = products.id'
       type: 'LEFT'
   dimensions:
     - field: 'customers.region',     label: 'Region'
   metrics:
     - field: 'orders.net_amount',    aggregate: 'SUM', label: 'Revenue'
     - field: 'products.category',    expression: 'COUNT(DISTINCT {{field}})', label: 'Product Count'

 Generated SQL:
   SELECT
     customers.region AS "Region",
     SUM(orders.net_amount) AS "Revenue",
     COUNT(DISTINCT products.category) AS "Product Count"
   FROM orders
   LEFT JOIN customers ON orders.customer_id = customers.id
   LEFT JOIN products ON orders.product_id = products.id
   GROUP BY customers.region
   ORDER BY "Revenue" DESC
```

The `RelatedTableJoiner` in the config editor auto-detects these relationships:
- Scans `information_schema` for column names matching `{table_singular}_id` patterns
- Shows detected relations as suggested joins the user can accept or modify
- Once a join is added, all columns from the joined table become available in dimension/metric pickers

---

## Expression Editor

The expression editor lets users go beyond simple column references to write DuckDB SQL expressions:

### Supported Patterns

```
┌─────────────────────────────────────────────────────────────────────┐
│ Expression Input                                                     │
│                                                                      │
│  SUM(net_amount * (1 - discount_pct / 100))                         │
│                                                                      │
├─────────────────────────────────────────────────────────────────────┤
│ Autocomplete ────────────────────────────────────────────────────── │
│                                                                      │
│  Functions        Columns (orders)       Operators                   │
│  ├─ SUM()         ├─ id                  ├─ +  -  *  /              │
│  ├─ AVG()         ├─ net_amount          ├─ =  !=  <  >            │
│  ├─ COUNT()       ├─ quantity            ├─ AND  OR  NOT            │
│  ├─ COUNT(DIST    ├─ discount_pct        ├─ IS NULL                  │
│  ├─ MAX()         ├─ rating              ├─ LIKE  ILIKE             │
│  ├─ MIN()         ├─ product_id          ├─ CASE WHEN ... END       │
│  ├─ ROUND()       ├─ customer_id         └─ IN (...)                │
│  ├─ CEIL()        └─ status                                         │
│  ├─ ABS()                                                           │
│  ├─ COALESCE()     Custom UDFs                                     │
│  ├─ NULLIF()       ├─ calculate_margin()                            │
│  └─ ...            └─ format_currency()                            │
│                                                                      │
├─────────────────────────────────────────────────────────────────────┤
│ Preview (top 5 rows)                                                │
│                                                                      │
│  Region        │ Revenue After Discount                              │
│  ──────────────┼────────────────────────                             │
│  North America │         2,847,320                                   │
│  Europe        │         1,923,150                                   │
│  Asia Pacific  │         1,456,890                                   │
│  Latin America │           892,340                                   │
│  Middle East   │           567,210                                   │
│                                                                      │
├─────────────────────────────────────────────────────────────────────┤
│ ✓ Valid DuckDB expression                                 [Insert] │
└─────────────────────────────────────────────────────────────────────┘
```

### UDF Lifecycle

```
 User creates a UDF in the Expression Editor
        │
        ▼
 CREATE MACRO calculate_margin(cost, price) AS (price - cost) / price * 100
        │
        ▼
 Stored in DuckDB (persists across sessions — native DB on disk)
        │
        ▼
 Appears in autocomplete under "Custom UDFs"
        │
        ▼
 Can be used in any metric expression:
        calculate_margin(orders.unit_cost, orders.unit_price)
```

---

## Component File Structure

```
src/lib/charts/
├── index.ts                           ← barrel export
├── types.ts                           ← all TypeScript interfaces & types
│
├── ChartCanvas.svelte                 ← top-level host component
├── ChartToolbar.svelte                ← per-chart actions
├── ChartRenderer.svelte               ← renderer switch/router
├── FilterEngine.svelte                ← filter state management
├── FilterChipBar.svelte               ← active filter chips
├── SummaryStrip.svelte                ← KPI cards strip
│
├── renderers/                         ← library adapter layer
│   ├── index.ts
│   ├── layerchart/
│   │   ├── LayerChartRenderer.svelte
│   │   ├── BarChart.svelte
│   │   ├── LineChart.svelte
│   │   ├── ScatterChart.svelte
│   │   ├── PieChart.svelte
│   │   └── ...
│   ├── svelteplot/
│   │   ├── SveltePlotRenderer.svelte
│   │   ├── BarChart.svelte
│   │   ├── LineChart.svelte
│   │   └── ...
│   └── picasso/
│       ├── PicassoRenderer.svelte
│       ├── ScatterLarge.svelte        ← for >10K points (canvas)
│       └── ...
│
├── library/                           ← chart template library
│   ├── ChartLibrary.svelte            ← pick-a-chart modal/sidebar
│   ├── ChartTemplateCard.svelte
│   ├── templates.ts                   ← built-in template definitions
│   └── categories.ts
│
├── editor/                            ← config editor panels
│   ├── ChartConfigEditor.svelte       ← sidebar host with tabs
│   ├── DataSourcePanel.svelte
│   ├── FieldMapper.svelte
│   ├── DimensionPicker.svelte
│   ├── MetricPicker.svelte
│   ├── RelatedTableJoiner.svelte
│   ├── ExpressionEditor.svelte
│   ├── VisualPanel.svelte
│   ├── InteractionPanel.svelte
│   ├── ExportPanel.svelte
│   └── AutocompleteDropdown.svelte
│
├── engine/                            ← data & logic layer
│   ├── DataModelConnector.ts          ← config → SQL → result
│   ├── ExpressionEngine.ts            ← expression parsing & validation
│   ├── FunctionRegistry.ts            ← built-in + UDF function defs
│   ├── UDFManager.ts                  ← CRUD for user-defined macros
│   └── AutocompleteProvider.ts        ← context-aware suggestions
│
└── state/
    └── ChartStateStore.svelte.ts      ← Svelte 5 runes-based global store
```

---

## TypeScript Type Definitions (Key Interfaces)

```typescript
// --- Chart Types ---
type ChartType =
  | 'bar' | 'grouped-bar' | 'stacked-bar' | 'horizontal-bar'
  | 'line' | 'multi-line'
  | 'area' | 'stacked-area'
  | 'scatter' | 'bubble'
  | 'pie' | 'donut' | 'treemap'
  | 'heatmap'
  | 'histogram' | 'boxplot'
  | 'table' | 'kpi-tile';

type ChartLibrary = 'layerchart' | 'svelteplot' | 'picasso';

// --- Data Config ---
interface DataConfig {
  baseTable: string;
  joins: JoinConfig[];
  dimensions: DimensionConfig[];
  metrics: MetricConfig[];
  filters: FilterConfig[];
  sorts: SortConfig[];
  limit: number | null;
}

interface JoinConfig {
  table: string;
  on: string;
  type: 'LEFT' | 'INNER';
}

interface DimensionConfig {
  field: string;
  alias?: string;
  label?: string;
}

interface MetricConfig {
  field: string;
  alias?: string;
  label?: string;
  expression?: string;      // raw DuckDB expression, e.g. "SUM(net_amount * (1 - discount_pct))"
  aggregate?: 'SUM' | 'AVG' | 'COUNT' | 'MIN' | 'MAX' | 'MEDIAN' | 'STDDEV';
}

interface FilterConfig {
  field: string;
  operator: '=' | '!=' | '>' | '<' | '>=' | '<=' | 'IN' | 'NOT IN' | 'LIKE' | 'IS NULL' | 'IS NOT NULL';
  value: any;
}

interface SortConfig {
  field: string;
  direction: 'ASC' | 'DESC';
}

// --- Visual Config ---
interface VisualConfig {
  title: string;
  subtitle?: string;
  description?: string;
  aspectRatio?: number;
  axes: {
    x: AxisConfig;
    y: AxisConfig;
  };
  colors: ColorConfig;
  legend: LegendConfig;
  tooltip: TooltipConfig;
  marks: MarkConfigs;       // chart-type-specific (bar, line, scatter, etc.)
  annotations: AnnotationConfig[];
}

interface AxisConfig {
  label?: string;
  scaleType?: 'linear' | 'log' | 'band' | 'time' | 'point';
  format?: string;
  domain?: [number | null, number | null];
  gridVisible?: boolean;
  inverted?: boolean;
  tickCount?: number;
}

// --- Interaction Config ---
interface InteractionConfig {
  clickToFilter: boolean;
  filterMode: 'include' | 'exclude' | 'single';
  crossFilterWith: string[];     // other chart instance IDs
  brushSelection: boolean;
  zoomPan: boolean;
}

// --- Filter State ---
interface FilterState {
  activeFilters: Map<string, Set<string>>;
}

// --- Full Chart Config ---
interface ChartConfig {
  id: string;
  pageId: string;
  chartType: ChartType;
  dataConfig: DataConfig;
  visualConfig: VisualConfig;
  interactionConfig: InteractionConfig;
  version: number;
}
```

---

## Lessons from the Draft (`data.monster/src/routes/+page.svelte`)

The draft implementation revealed several patterns worth preserving and several that need fixing:

### What Worked (Preserve)
- **Click-on-bar to toggle filter** — the `handleBarClick` + `selectedGroups` Set pattern is exactly right
- **Dimmed unselected bars** — `DIMMED = 'oklch(0.9 0.005 250 / 0.45)'` gives a clear visual distinction
- **Group-by + metric dropdowns** — simple UX that immediately changes the chart's data mapping
- **Filter chip bar** — removable chips showing active filters, with a "Clear all" button
- **Summary cards recalculated** from filtered data — top group, top value, count, total
- **Derived reactivity** — `$derived` chains from raw data → filtered → aggregated → chart data

### What Needs Fixing (Generalise)
- **Hardcoded fields** — `fields = ['product', 'category', 'region', ...]` → should introspect from DuckDB schema
- **Hardcoded metrics** — `['netAmount', 'quantity', 'unitPrice', ...]` → should be auto-detected numeric columns
- **Single chart type** — only `BarChartSimple` from Carbon → needs type switching
- **No config persistence** — state is ephemeral → ChartConfig should be serializable and saveable
- **Tight page coupling** — chart logic mixed with page layout → extract into reusable `ChartCanvas`
- **No cross-table joins** — reads from a single `data.json` → needs SQL-based data connector
- **No expression support** — simple column references only → needs expression editor
- **No chart library abstraction** — directly imports Carbon → needs renderer adapter layer
- **Chart loaded in onMount** — dynamic `import()` of Carbon is fine but should be managed by the renderer layer

---

## Chart Library Shortlist

See [chart-library-candidates.md](./chart-library-candidates.md) for the full research report. Summary:

| Library | Role | Why |
|---|---|---|
| **LayerChart** (`layerchart`) | Primary renderer | Native Svelte 5, all chart types, built-in interactions, Tailwind-compatible |
| **SveltePlot** (`svelteplot`) | Analytical / faceted charts | Grammar-of-graphics, multi-layer compositions, great for complex analytical views |
| **Picasso.js** (`picasso.js`) | High-performance scatter / large datasets | Canvas rendering, built-in brushing, handles >10K points smoothly |

**Drop**: `@carbon/charts-svelte` — heavy, IBM ecosystem lock-in, Svelte 5 migration incomplete.

---

## Implementation Priority

```
Phase 1 — Core (MVP)
  ├── ChartCanvas.svelte                 ← host component
  ├── ChartRenderer + LayerChart adapter ← bar, line, scatter first
  ├── DataModelConnector                 ← config → SQL → execute
  ├── FilterEngine                       ← click-to-filter
  └── basic ChartConfigEditor            ← table picker, field mapper, 2 dropdowns

Phase 2 — Library & Templates
  ├── ChartLibrary modal                 ← browse and pick templates
  ├── ChartTemplate definitions          ← 7 built-in types from library-charts.json
  ├── VisualPanel                        ← axes, colors, legend, marks config
  └── SummaryStrip                       ← KPI cards

Phase 3 — Expressions & Cross-Table
  ├── ExpressionEditor                   ← formula input with autocomplete
  ├── UDFManager                         ← create / list / delete macros
  ├── RelatedTableJoiner                 ← auto-detect + manual join config
  └── AutocompleteProvider               ← context-aware suggestions

Phase 4 — Polish & Advanced
  ├── Cross-chart filter sync            ← charts on same page share filter state
  ├── SveltePlot adapter                 ← faceted / multi-layer views
  ├── Picasso adapter                    ← large scatter plots
  ├── Annotation system                  ← reference lines, areas
  └── Export                             ← PNG, SVG, CSV
```
