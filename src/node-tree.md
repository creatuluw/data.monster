# Data Monster - Full Dependency Node Tree

Legend: [EDGE] descriptions below each branch

```
ROUTES (SvelteKit)
|
+-- / (+page.svelte) ........................................................................ HOME
|   +-- [STORE] app.svelte.ts
|   |   +-- [DB-OP] initializeDuckdb
|   |   +-- [DB-OP] shutdownDuckdb
|   |   +-- [DB-OP] getWorkspacePath
|   |   +-- [DB-OP] chooseWorkspaceFolder
|   |   +-- [DB-OP] listTables
|   |   +-- [DB-OP] isTauriAvailable
|   |   +-- [DB-OP] extractErrorMessage
|   |   `-- [EXT] @tauri-apps/api/core (invoke)
|   `-- [EXT] lucide-svelte (PlusCircle, Code, Database, TrendingUp, FileText)
|
+-- / (+layout.svelte) .................................................................... LAYOUT
|   +-- [COMP] Breadcrumb.svelte
|   +-- [STORE] app.svelte.ts
|   |   +-- [DB-OP] initializeDuckdb, shutdownDuckdb, getWorkspacePath
|   |   +-- [DB-OP] chooseWorkspaceFolder, listTables, isTauriAvailable
|   |   +-- [DB-OP] extractErrorMessage
|   |   `-- [EXT] @tauri-apps/api/core (invoke)
|   +-- [STORE] $app/stores (page)
|   +-- [NAV] $app/navigation (goto)
|   +-- [EXT] @tauri-apps/api/event (listen, UnlistenFn)
|   +-- [EXT] lucide-svelte (FolderOpen, Settings, MoreVertical, Link, Check)
|   `-- [CSS] ../app.css
|
+-- / (+layout.ts) ................................................................... SSR=false
|   `-- (no dependencies)
|
+-- /analyst (+page.svelte) ........................................................... TABLE SELECTOR
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [STORE] analyst.svelte.ts
|   |   +-- [DB-OP] runPagedQuery
|   |   +-- [DB-OP] executeQuery
|   |   +-- [DB-OP] getSettings
|   |   +-- [DB-OP] generateChat
|   |   +-- [DB-OP] stopGeneration
|   |   +-- [DB-OP] loadModel
|   |   +-- [DB-OP] unloadModel
|   |   +-- [DB-OP] extractErrorMessage
|   |   +-- [EXT] @tauri-apps/api/core (invoke)
|   |   `-- [EXT] @tauri-apps/api/event (listen)
|   +-- [DB-OP] runPagedQuery (direct)
|   `-- [NAV] $app/navigation (goto)
|
+-- /analyst/chat (+page.svelte) ....................................................... AI CHAT
|   +-- [STORE] analyst.svelte.ts
|   |   +-- (see analyst.svelte.ts subtree above)
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] marked (markdown renderer)
|
+-- /connect (+page.svelte) .......................................................... DATA CONNECT
|   +-- [DB-OP] downloadUrlToWorkspace
|   +-- [DB-OP] generatePgInestSql
|   +-- [DB-OP] connectPostgres
|   +-- [DB-OP] listPostgresTables
|   +-- [DB-OP] extractErrorMessage
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [NAV] $app/navigation (goto)
|   +-- [EXT] @tauri-apps/plugin-dialog (open)
|   `-- [EXT] lucide-svelte (Upload, Link, Database, ChevronRight, Check, LoaderCircle)
|
+-- /data (+page.svelte) ............................................................ TABLE MANAGER
|   +-- [COMP] TableOverview.svelte
|   +-- [COMP] TableDrawer.svelte
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] dropTable
|   +-- [DB-OP] extractErrorMessage
|   `-- [NAV] $app/navigation (goto)
|
+-- /pages (+page.svelte) ........................................................... SAVED PAGES
|   +-- [COMP] BarChart.svelte
|   |   `-- [LIB] $lib/charts
|   |       +-- [CHART-HELPERS] chart-helpers.ts
|   |       |   +-- [TYPE] types.ts (ChartDataPoint, ColorScale)
|   |       |   `-- exports: aggregate, filterData, buildColorScale, toggleGroup,
|   |       |            formatCurrency, formatNumber, formatPercent, PALETTE
|   |       `-- [TYPE] types.ts (standalone -- no imports)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] runPagedQuery
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] lucide-svelte (FileText)
|
+-- /pages/chart/[id] (+page.svelte) ................................................ CHART VIEWER
|   +-- [COMP] BarChart.svelte
|   |   `-- (see BarChart.svelte subtree above)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [STORE] $app/stores (page -> $page.params.id)
|   +-- [DB-OP] runPagedQuery
|   +-- [DB-OP] getTableMeta
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] lucide-svelte (ArrowLeft, XCircle)
|
+-- /preview (+page.svelte) ......................................................... FILE PREVIEW
|   +-- [COMP] PreviewPane.svelte
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] previewFile
|   +-- [DB-OP] getFileSize
|   +-- [DB-OP] extractErrorMessage
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] lucide-svelte (LoaderCircle)
|
+-- /query (+page.svelte) ........................................................... QUERY EDITOR
|   +-- [COMP] TagInput.svelte
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] runPagedQuery
|   +-- [DB-OP] executeQuery
|   +-- [DB-OP] saveTableSource
|   +-- [DB-OP] saveTableLabels
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] getAllTableMeta
|   +-- [DB-OP] listSavedQueries
|   +-- [DB-OP] saveQuery
|   +-- [DB-OP] updateSavedQuery
|   +-- [DB-OP] deleteSavedQuery
|   +-- [DB-OP] extractErrorMessage
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] lucide-svelte (RefreshCw, Share, Table, Bookmark, Copy, Upload, Play,
|                          FileText, XCircle, PlusCircle, Database, ArrowLeft, Trash2, Tag)
|
+-- /settings (+page.svelte) ........................................................ SETTINGS
|   +-- [COMP] ModelManager.svelte
|   +-- [STORE] analyst.svelte.ts
|   |   `-- (see analyst.svelte.ts subtree above)
|   +-- [DB-OP] getSettings
|   +-- [DB-OP] saveSettings
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, Save, CheckCircle, AlertCircle, Database, Cloud, Cpu)
|
+-- /settings/internal-db (+page.svelte) ........................................ INTERNAL DB ADMIN
|   +-- [DB-OP] listInternalTables
|   +-- [DB-OP] queryInternalTable
|   +-- [DB-OP] updateInternalRow
|   +-- [DB-OP] deleteInternalRow
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Database, ArrowLeft, Trash2, Edit3, ChevronLeft,
|                          ChevronRight, RefreshCw)
|
+-- /table/[name] (+page.svelte) .................................................. TABLE VIEWER
|   +-- [COMP] TableViewer.svelte
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [STORE] $app/stores (page -> $page.params.name)
|   +-- [DB-OP] runPagedQuery
|   +-- [DB-OP] executeQuery
|   +-- [DB-OP] dropTable
|   +-- [DB-OP] extractErrorMessage
|   +-- [NAV] $app/navigation (goto)
|   `-- [EXT] lucide-svelte
|
+-- /ui (+page.svelte) ............................................................. DESIGN SYSTEM
|   +-- [COMP] ds/Nav.svelte
|   +-- [COMP] ds/Hero.svelte
|   +-- [COMP] ds/ColorPalette.svelte
|   +-- [COMP] ds/Typography.svelte
|   +-- [COMP] ds/Spacing.svelte
|   +-- [COMP] ds/Buttons.svelte
|   +-- [COMP] ds/Tags.svelte
|   +-- [COMP] ds/Inputs.svelte
|   +-- [COMP] ds/Toggles.svelte
|   +-- [COMP] ds/Cards.svelte
|   +-- [COMP] ds/Accordion.svelte
|   +-- [COMP] ds/Modal.svelte
|   +-- [COMP] ds/SearchAhead.svelte
|   +-- [COMP] ds/Toast.svelte
|   +-- [COMP] ds/Motion.svelte
|   +-- [COMP] ds/Icons.svelte
|   +-- [COMP] ds/Principles.svelte
|   `-- [COMP] ds/Footer.svelte
|
+-- /labs (+layout.svelte) ......................................................... LABS LAYOUT
|   `-- (renders <slot /> only, no dependencies)
|
+-- /labs (+page.svelte) ........................................................... LABS INDEX
|   `-- [EXT] lucide-svelte (BarChart2, Layers, FlaskConical)
|
+-- /labs/charts (+page.svelte) .................................................... CHART LAB
|   +-- [CHART] BarChartCanvas.svelte
|   |   `-- [LIB] $lib/charts
|   |       +-- chart-helpers.ts -> types.ts
|   |       `-- PALETTE, aggregate, filterData, buildColorScale, formatNumber
|   +-- [CHART] PicassoBarChartCanvas.svelte
|   |   `-- [LIB] $lib/charts
|   |       +-- chart-helpers.ts -> types.ts
|   |       `-- engine/DataModelConnector.ts -> db-operations.ts -> @tauri-apps/api/core
|   +-- [EXT] svelteplot (Plot, BarX)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (BarChart2, LineChart, PieChart, AreaChart, ScatterChart)
|
+-- /labs/chart-lib (+page.svelte) ................................................. CHART LIB INDEX
|   `-- [EXT] lucide-svelte (BarChart2, LineChart, AreaChart, PieChart, ScatterChart)
|
+-- /labs/chart-lib/bar (+page.svelte) ............................................. CHART LIB BAR
|   +-- [CHART] BarChartCanvas.svelte
|   |   `-- [LIB] $lib/charts -> chart-helpers.ts -> types.ts
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, BarChart2)
|
+-- /labs/picasso-charts (+page.svelte) ............................................ PICASSO INDEX
|   `-- [EXT] lucide-svelte (BarChart2, LineChart, PieChart, ScatterChart)
|
+-- /labs/picasso-charts/bar (+page.svelte) ........................................ PICASSO BAR
|   +-- [CHART] PicassoBarChartCanvas.svelte
|   |   `-- [ENGINE] DataModelConnector.ts
|   |       +-- types.ts (BarChartConfig, BarChartData, FilterState)
|   |       `-- db-operations.ts (executeQuery) -> @tauri-apps/api/core (invoke)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, BarChart2)
|
+-- /labs/picasso-charts/line (+page.svelte) ....................................... PICASSO LINE
|   +-- [CHART] PicassoLineAreaChartCanvas.svelte
|   |   `-- [ENGINE] DataModelConnector.ts
|   |       +-- types.ts (LineAreaChartConfig, FilterState)
|   |       `-- db-operations.ts (executeQuery) -> @tauri-apps/api/core (invoke)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, LineChart)
|
+-- /labs/picasso-charts/pie (+page.svelte) ........................................ PICASSO PIE
|   +-- [CHART] PicassoPieDonutCanvas.svelte
|   |   `-- [ENGINE] DataModelConnector.ts
|   |       +-- types.ts (PieDonutChartConfig, FilterState)
|   |       `-- db-operations.ts (executeQuery) -> @tauri-apps/api/core (invoke)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, PieChart)
|
+-- /labs/picasso-charts/scatter (+page.svelte) .................................... PICASSO SCATTER
|   +-- [CHART] PicassoScatterCanvas.svelte
|   |   `-- [ENGINE] DataModelConnector.ts
|   |       +-- types.ts (ScatterChartConfig, FilterState)
|   |       `-- db-operations.ts (executeQuery) -> @tauri-apps/api/core (invoke)
|   +-- [STORE] app.svelte.ts
|   |   `-- (see app.svelte.ts subtree above)
|   +-- [DB-OP] getTableMeta
|   +-- [DB-OP] extractErrorMessage
|   `-- [EXT] lucide-svelte (Settings, ScatterChart)
|
+-- /labs/unovis-charts (+page.svelte) ............................................. UNOVIS INDEX
|   `-- [EXT] lucide-svelte (BarChart2, LineChart, AreaChart, PieChart, ScatterChart)
|
`-- /labs/unovis-charts/bar (+page.svelte) ......................................... UNOVIS BAR
    +-- [CHART-ENGINE] compileBarChartQuery, executeBarChartQuery
    |   `-- [ENGINE] DataModelConnector.ts
    |       +-- types.ts (BarChartConfig, BarChartData, FilterState)
    |       `-- db-operations.ts (executeQuery) -> @tauri-apps/api/core (invoke)
    +-- [STORE] app.svelte.ts
    |   `-- (see app.svelte.ts subtree above)
    +-- [DB-OP] getTableMeta
    +-- [DB-OP] extractErrorMessage
    +-- [EXT] @unovis/ts (XYContainer, StackedBar, Axis, Tooltip, BulletLegend)
    `-- [EXT] lucide-svelte (Settings, BarChart2)
```

---

## LIBRARY DEEP SUBTREES

```
$lib/stores/app.svelte.ts .......................................................... APP STATE STORE
|-- $state: dbReady, workspacePath, tables, globalError, pendingSql,
|           pendingFile, pendingPreviewData, pendingBatchIngest
|-- init() -> initializeDuckdb -> @tauri-apps/api/core (invoke)
|-- shutdown() -> shutdownDuckdb -> @tauri-apps/api/core (invoke)
|-- selectWorkspace() -> chooseWorkspaceFolder -> @tauri-apps/api/core (invoke)
|-- refreshTables() -> listTables -> @tauri-apps/api/core (invoke)
`-- extractErrorMessage -> @tauri-apps/api/core (invoke)

$lib/stores/analyst.svelte.ts ..................................................... ANALYST STATE STORE
|-- $state: selectedTables, messages, queries, streaming, streamingContent,
|           activeQueryId, pendingPlan, selectedPlanOptions, customPlanOption,
|           apiUrl, apiKey, apiModel, inferenceMode, localModelId
|-- init() -> @tauri-apps/api/event (listen)
|-- sendMessage() -> generateChat -> @tauri-apps/api/core (invoke)
|                 -> getSettings -> @tauri-apps/api/core (invoke)
|                 -> runPagedQuery -> @tauri-apps/api/core (invoke)
|-- rerunQuery() -> executeQuery -> @tauri-apps/api/core (invoke)
|-- proceedWithPlan() -> executeQuery -> @tauri-apps/api/core (invoke)
|-- stop() -> stopGeneration -> @tauri-apps/api/core (invoke)
|-- getTableSchemas() -> runPagedQuery -> @tauri-apps/api/core (invoke)
`-- loadModel/unloadModel -> @tauri-apps/api/core (invoke)

$lib/db-operations.ts ............................................................. DB OPERATIONS LAYER
|-- ALL functions are IPC calls via @tauri-apps/api/core (invoke)
|-- TYPES: PreviewData, ColumnOverride, QueryResult, DmlResult, PagedQueryResult,
|          ColumnInfo, FileLoadResult, SavedQuery, TableLabels, TableSource,
|          TableTypeEntry, FilePreviewResult, InternalTable, InternalTableData,
|          ModelEntry, DownloadedModel, ModelLoadStatus, DownloadProgress,
|          PgIngestStatement
|-- DB LIFECYCLE: initializeDuckdb, shutdownDuckdb, isTauriAvailable
|-- WORKSPACE: chooseWorkspaceFolder, getWorkspacePath, setWorkspacePath,
|              initializeDataFolders
|-- QUERY: executeQuery, cancelQuery, runPagedQuery
|-- TABLE: listTables, dropTable, createTableFromQuery, renameTable, getTableMeta,
|          getAllTableMeta, getTableTypes
|-- TABLE META: saveTableSource, getTableSource, refreshTableFromSource,
|               saveTableLabels, getTableLabels, getAllTags, getAllGroups
|-- FILE I/O: loadCsvFile, loadParquetFile, loadJsonFile, getFileColumns,
|             getFileSize, downloadUrlToWorkspace, previewFile
|-- SAVED QUERIES: listSavedQueries, saveQuery, updateSavedQuery, deleteSavedQuery
|-- SETTINGS: getSettings, saveSettings
|-- INTERNAL DB: listInternalTables, queryInternalTable, updateInternalRow,
|                deleteInternalRow
|-- POSTGRES: connectPostgres, listPostgresTables, generatePgInestSql
|-- AI/ML: detectSystemRam, listAvailableModels, downloadModel, cancelDownload,
|          listDownloadedModels, deleteModel, loadModel, unloadModel,
|          isModelLoaded, generateChat, stopGeneration
`-- UTIL: extractErrorMessage

$lib/charts/index.ts ............................................................... CHART MODULE
|-- [COMPONENTS]
|   |-- BarChart.svelte
|   |-- BarChartCanvas.svelte
|   |-- PicassoBarChartCanvas.svelte
|   |-- PicassoLineAreaChartCanvas.svelte
|   |-- PicassoPieDonutCanvas.svelte
|   `-- PicassoScatterCanvas.svelte
|-- [ENGINE] engine/DataModelConnector.ts
|   |-- compileBarChartQuery(config, filterState) -> SQL string
|   |-- executeBarChartQuery(config, filterState) -> db-operations.executeQuery
|   |-- compileLineAreaChartQuery(config, filterState) -> SQL string
|   |-- executeLineAreaChartQuery(config, filterState) -> db-operations.executeQuery
|   |-- compilePieDonutChartQuery(config, filterState) -> SQL string
|   |-- executePieDonutChartQuery(config, filterState) -> db-operations.executeQuery
|   |-- compileScatterChartQuery(config, filterState) -> SQL string
|   `-- executeScatterChartQuery(config, filterState) -> db-operations.executeQuery
|-- [HELPERS] chart-helpers.ts
|   |-- PALETTE, BLACK, DIMMED (constants)
|   |-- aggregate(data, groupBy, metric, sortOrder)
|   |-- filterData(data, groupBy, selectedGroups)
|   |-- buildColorScale(aggregated, selectedGroups)
|   |-- toggleGroup(set, group)
|   |-- formatCurrency, formatNumber, formatPercent
|   `-- <- types.ts (ChartDataPoint, ColorScale)
`-- [TYPES] types.ts (standalone, no imports)
    |-- ChartDataPoint
    |-- BarClickDetail
    |-- ChartOptions
    |-- ColorScale
    |-- BarChartDimension, BarChartMetric, BarChartConfig, BarChartData
    |-- FilterState
    |-- LineAreaChartConfig
    |-- PieDonutChartConfig
    `-- ScatterChartConfig
```

---

## EDGE CROSS-REFERENCE

### Routes -> Stores

```
[STORE] app.svelte.ts
    |-> / (Home)
    |-> /+layout
    |-> /analyst
    |-> /connect
    |-> /data
    |-> /pages
    |-> /pages/chart/[id]
    |-> /preview
    |-> /query
    |-> /table/[name]
    |-> /labs/charts
    |-> /labs/chart-lib/bar
    |-> /labs/picasso-charts/bar
    |-> /labs/picasso-charts/line
    |-> /labs/picasso-charts/pie
    |-> /labs/picasso-charts/scatter
    `-> /labs/unovis-charts/bar

[STORE] analyst.svelte.ts
    |-> /analyst
    |-> /analyst/chat
    `-> /settings
```

### Routes -> Components

```
[COMP] Breadcrumb.svelte            -> /+layout
[COMP] TableOverview.svelte         -> /data
[COMP] TableDrawer.svelte           -> /data
[COMP] PreviewPane.svelte           -> /preview
[COMP] BarChart.svelte              -> /pages, /pages/chart/[id]
[COMP] TableViewer.svelte           -> /table/[name]
[COMP] TagInput.svelte              -> /query
[COMP] ModelManager.svelte          -> /settings
[COMP] ds/* (18 components)         -> /ui
```

### Routes -> db-operations

```
[DB-OP] runPagedQuery               -> /analyst, /pages, /pages/chart/[id],
                                        /query, /table/[name]
[DB-OP] executeQuery                -> /query, /table/[name]
[DB-OP] getTableMeta                -> /pages, /pages/chart/[id], /query,
                                        /labs/charts, /labs/chart-lib/bar,
                                        /labs/picasso-charts/{bar,line,pie,scatter},
                                        /labs/unovis-charts/bar
[DB-OP] getAllTableMeta             -> /query
[DB-OP] dropTable                   -> /data, /table/[name]
[DB-OP] previewFile                 -> /preview
[DB-OP] getFileSize                 -> /preview
[DB-OP] downloadUrlToWorkspace      -> /connect
[DB-OP] connectPostgres             -> /connect
[DB-OP] listPostgresTables          -> /connect
[DB-OP] generatePgInestSql          -> /connect
[DB-OP] saveTableSource             -> /query
[DB-OP] saveTableLabels             -> /query
[DB-OP] listSavedQueries            -> /query
[DB-OP] saveQuery                   -> /query
[DB-OP] updateSavedQuery            -> /query
[DB-OP] deleteSavedQuery            -> /query
[DB-OP] getSettings                 -> /settings
[DB-OP] saveSettings                -> /settings
[DB-OP] listInternalTables          -> /settings/internal-db
[DB-OP] queryInternalTable          -> /settings/internal-db
[DB-OP] updateInternalRow           -> /settings/internal-db
[DB-OP] deleteInternalRow           -> /settings/internal-db
[DB-OP] extractErrorMessage         -> /connect, /data, /pages, /preview, /query,
                                        /settings, /settings/internal-db,
                                        /table/[name], /labs/charts,
                                        /labs/chart-lib/bar,
                                        /labs/picasso-charts/{bar,line,pie,scatter},
                                        /labs/unovis-charts/bar
```

### Routes -> External Packages

```
[EXT] @tauri-apps/api/core          -> db-operations.ts (invoke -- ALL db ops)
[EXT] @tauri-apps/api/event         -> /+layout, analyst.svelte.ts
[EXT] @tauri-apps/plugin-dialog     -> /connect
[EXT] lucide-svelte                 -> nearly all routes
[EXT] marked                        -> /analyst/chat
[EXT] svelteplot                    -> /labs/charts
[EXT] @unovis/ts                    -> /labs/unovis-charts/bar
```

---

## DEEP TRACE: db-operations -> Tauri IPC -> Rust Backend

```
$lib/db-operations.ts
`-- @tauri-apps/api/core (invoke)
    `-- src-tauri/ (Rust backend)
        `-- All SQL operations routed through DuckDB via Tauri commands
            |-- DB Lifecycle: initializeDuckdb, shutdownDuckdb
            |-- SQL Execution: executeQuery, cancelQuery, runPagedQuery
            |-- File Ingestion: loadCsvFile, loadParquetFile, loadJsonFile
            |-- Workspace Management: chooseWorkspaceFolder, getWorkspacePath
            |-- PostgreSQL Sync: connectPostgres, listPostgresTables, generatePgInestSql
            |-- AI/ML: generateChat, loadModel, unloadModel, downloadModel
            `-- Internal SQLite: listInternalTables, queryInternalTable,
                                 updateInternalRow, deleteInternalRow
```

## DEEP TRACE: Chart Engine Pipeline

```
Route Page
`-- [CHART COMPONENT] (e.g. PicassoBarChartCanvas.svelte)
    `-- $lib/charts/index.ts
        `-- engine/DataModelConnector.ts
            |-- compile{Type}ChartQuery(config, filterState) -> SQL
            `-- execute{Type}ChartQuery(config, filterState)
                `-- db-operations.executeQuery(sql)
                    `-- @tauri-apps/api/core (invoke)
                        `-- Tauri Rust Backend
                            `-- DuckDB Query Engine
```
