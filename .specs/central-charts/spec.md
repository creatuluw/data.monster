# Central Chart System — Phase 1: Spec, Registry, Page Runtime (bar, heatmap, table)

## Feature Overview

Implements the central chart design (`docs/design/central-chart-component-design.md`, locked 2026-09-15) far enough to build real report pages: a **page** is a declarative spec document (JSON) rendered through a shared pipeline, editable through a **dual-mode editor** (no-code Design ⇄ Code) at `/page/<slug>`.

First chart/block citizens, per decision:

- **bar** (chart type, registry) — ported from `src/lib/components/charts/BarChart.svelte`
- **heatmap** (chart type, registry) — ported from `src/lib/components/Heatmap.svelte`
- **table** (block type) — renders query rows, reusing existing table-view machinery
- **text** (block type) — trivial, included since it is free

Out of scope for this phase (later, additive): master-item library, relationship-graph auto-JOIN, per-type custom panels, annotation marks beyond `ruleY`, linked-hover sync, remaining 28 chart types, `plotOptions` escape hatch UI.

## Success Criteria

- A page spec (JSON) with a bar chart, a heatmap, a table and a text block renders at `/page/<slug>` from storage in the internal DuckDB (`d8a_monster_pages`)
- Creating a page at `/pages` opens `/page/<slug>` in the editor
- Design mode: click a block → inspector edits its spec fields (schema-driven); add-block menu offers chart (bar/heatmap), table, text; blocks live-re-render
- Code mode: the same document as editable JSON text with live validation; switching modes is lossless
- Selecting a dimension value on a chart cross-filters the other charts and the table via re-query (source chart excluded, Qlik-style); click-away clears
- Bar and heatmap lab pages (`/labs/bar-chart`, `/labs/heatmap`) render through the registry without losing current behavior
- Shared behaviors work identically on both charts: page-consistent colors, declarative tooltips, `heightVh`, empty/error/missing-role states
- All pure logic (validation, query compilation, registry resolution, color scale, tooltip templating) is unit-tested in `tests/` (vitest, out of `src/`)

## Design Rationale

- **Declarative spec = parity by construction** (interview Q3-A): both surfaces edit one JSON document; nothing expressible lacks a UI representation and vice versa.
- **One query engine + hooks** (Q8-C): bar gets the `topN` hook (existing `buildBars` logic), heatmap gets the `pivot` hook; table uses the same compiler without GROUP BY. No per-chart SQL templates.
- **Expressions not sugar** (Q5): measures are DuckDB expressions (`sum(if(type='class', hours))`); `col+agg` dropdowns are autocomplete that writes expressions. Dimensions are columns with optional `grain`.
- **Registry drives everything per-type** (Q9-B): roles schema + options schema; the inspector is auto-rendered from the schema (no hand-built panels this phase).
- **Storage in internal DuckDB** (Q17-A): `d8a_monster_pages` following the `saved_queries` command pattern.
- **Reuse before build**: card frame, config-drawer machinery, `sameDatum`, `buildBars`, `formatValue`, table viewer — port/wrap, don't rewrite.

## Constraints/Assumptions

- Stack unchanged: Svelte 5 runes, SvelteKit, Tauri v2, DuckDB via Rust commands, Tailwind 4, SveltePlot 0.14.2. **No new npm dependencies** (no zod — hand-rolled validation).
- Test files live in `tests/`, never in `src/` (vite dep-optimizer rule).
- Charts query the workspace DuckDB via the existing `$lib/db-operations` pipeline.
- The old draft `/pages` (charts on first table) is replaced by the list page; its draft `src/lib/components/BarChart.svelte` (top-level) is retired.
- Data roles v1: exactly what bar/heatmap/table need. No master-item refs yet (`ref` field reserved in types, resolver returns a "missing master item" state when encountered).

## Functional Requirements

### FR-1 Spec types + validation

`src/lib/charts/spec-types.ts` (types) and `src/lib/charts/validate.ts` (pure validator).

Types: `PageDoc` (`slug`, `title`, `rows[]` of `blocks[]`), `Block = ChartBlock | TableBlock | TextBlock` (`type`, `span` 1–12), `ChartBlock` (`chart.type`, `source.table`, `dimensions[]` (`col`, `grain?`), `measures[]` (`expr`, `label`, `fmt?` | reserved `ref`), `filters[]` (`col`, `op`, `value`), `sort?`, `limit?`, `options`, `annotations[]`, `tooltip`, `axes`, `legend`, `title`, `subtitle`, `heightVh`), `TableBlock` (`table`, `columns?`, `filters?`, `sort?`, `limit`), `TextBlock` (`text`).

Validator returns a list of `{ path, message }` — never throws; used by code mode, save, and render-time guards.

- **Acceptance**: `tests/validate.test.ts` covers valid doc, unknown block type, unknown chart type, bad span, missing roles, empty expr, bad filter op; all green.

### FR-2 Query compiler

`src/lib/charts/query/compile.ts` — pure string builder (no execution): canonical `SELECT dims, measures FROM table [WHERE …] [GROUP BY dims] [ORDER BY …] [LIMIT n]`; measures render as `expr AS label`; dimensions apply `grain` via `date_trunc`/`strftime` mapping; filters render with escaped literals (quote identifiers, escape strings — trust boundary); sort by `dimension[0]`/`measure[i]` ± dir; chart limit; plus `topN` variant (cap + `Other` bucket) and `pivot` variant (grid query) and plain (table: no aggregation). Column-name validation against a provided table-schema map (rejects unknown columns — injection guard).

- **Acceptance**: `tests/compile.test.ts` asserts generated SQL for: simple bar, expression measure, grain dimension, filter escaping (quotes in values), topN/Other, heatmap pivot, table no-GROUP BY, injection attempt rejected, cross-filter WHERE injection (FR-9 reuse); all green.

### FR-3 Registry core + role validation

`src/lib/charts/registry.ts`: `ChartTypeDefinition` (`type`, `label`, `roles`, `optionsSchema`, `hooks`, `annotations` whitelist, `component`, `defaults`) and the same for block types (`BlockTypeDefinition`). `resolveBlock(block) → { definition, errors }`; role validation (bar: 1 dim + 1–n measures; heatmap: 2 dims + 1 measure; table: 0). Options schema fields: `{ name, kind: 'number'|'boolean'|'enum'|'string'|'color', label, default, options?, min?, max? }`.

- **Acceptance**: `tests/registry.test.ts` covers register/resolve, unknown type error, role under/overflow per type, defaults merging; all green.

### FR-4 Color scale + tooltip resolver

`src/lib/charts/color-scale.ts` — page-level `colorOf(dimension, value)` from DS palette, first-seen assignment, `seriesColors` overrides win. `src/lib/charts/tooltip.ts` — resolve `tooltip.template` placeholders (`{field}`) against a row with `fmt` applied.

- **Acceptance**: `tests/color-scale.test.ts` + `tests/tooltip.test.ts` (stable assignment, override wins, placeholder + fmt + missing-field fallback); all green.

### FR-5 ChartCard shell

`src/lib/components/charts/ChartCard.svelte` — card frame extracted from the current BarChart card: title/subtitle, inline selection label (`·` divider), empty/error/missing-role states, `heightVh` sizing rule, config-toggle slot. Renderers mount inside it.

- **Acceptance**: bar + heatmap render through the shell; states show for empty data, query error, and unfilled roles; labs behavior unchanged.

### FR-6 Bar renderer + registry entry

`src/lib/components/charts/renderers/BarChartRenderer.svelte` — thin SveltePlot `BarX` renderer fed by engine rows (`topN` hook = `buildBars`); horizontal orientation default (option), selection via `sameDatum`, DS-green selected / gray deselected, `scale: null` fill bypass; tooltip = shared renderer anchored at bar; annotations: render `ruleY` entries (whitelist: `ruleX`, `ruleY` for now). Options schema: `orientation`, `topN`, `otherLabel`, `stacked` (visual only if trivially supported, else drop from schema).

- **Acceptance**: `/labs/bar-chart` renders via registry entry with current visuals intact; new-page bar renders from spec; ruleY annotation draws with expression-evaluated value.

### FR-7 Heatmap renderer + registry entry

`src/lib/components/charts/renderers/HeatmapRenderer.svelte` — port of `Heatmap.svelte` internals onto engine rows (`pivot` hook); options schema: `scheme`, `threshold`; annotation whitelist empty.

- **Acceptance**: `/labs/heatmap` renders via registry entry; page heatmap renders from spec; threshold + scheme configurable via inspector.

### FR-8 Table block renderer

`src/lib/components/charts/renderers/TableRenderer.svelte` — renders engine rows (no aggregation) in the app's table style (reuse `TableViewer`/table page patterns; compact page variant); columns from result; respects cross-filter re-query; `limit` default 50.

- **Acceptance**: table block on a page shows rows, re-queries on cross-filter, respects limit.

### FR-9 Page runtime: grid + cross-filter bus

`src/lib/charts/page-runtime.svelte.ts` — resolves page doc → per-block query results (engine), holds `$state` for rows/loading/error; **selection bus**: `{ chartId, dimension, value }` (or clear); affected blocks (≠ source) recompile with injected `WHERE dim = value` and re-query; `colorOf` shared instance. `src/lib/components/charts/PageGrid.svelte` — 12-col rows, mounts blocks via registry.

- **Acceptance**: two charts + table on a demo page: select on bar → heatmap + table re-query and update, bar unchanged; clear restores; loading/error states per block.

### FR-10 Persistence (Rust)

`d8a_monster_pages` table in the internal DB (`slug` PK, `title`, `spec` JSON, `created_at`, `updated_at`); commands `list_pages`, `get_page`, `save_page` (upsert), `delete_page` in a `pages.rs` module following the `saved_queries` pattern; exposed via `$lib` wrapper.

- **Acceptance**: create → reload app → page persists; slug collision upserts; delete removes.

### FR-11 `/pages` list + create

Rebuild `/pages` as the page list (title, block count, updated_at, open/delete); "New page" prompts for name → slug → creates empty doc → routes to `/page/<slug>`. Old draft content removed.

- **Acceptance**: list reflects storage; create → land in editor; delete works; no dead links to old draft.

### FR-12 `/page/[slug]` dual-mode editor

Design mode: `PageGrid` preview + selected-block inspector (schema-driven fields per FR-3 + shared fields per type: roles via column/expression inputs with schema-aware autocomplete, tooltip fields, annotations, heightVh, span) + add-block menu (chart: bar/heatmap; table; text) + row add/remove. Code mode: JSON textarea with live validation (FR-1), errors listed, save disabled while invalid. Mode switch is lossless (same doc). Save → FR-10.

- **Acceptance**: build a page containing bar + heatmap + table + text purely in Design mode; same page edited in Code mode renders identically; invalid JSON shows errors and blocks save.

### FR-13 Labs migration + cleanup

`/labs/bar-chart` + `/labs/heatmap` pages mount registry entries (thin pages); retire old draft `src/lib/components/BarChart.svelte` and the `pages - Copy` route; `ChartConfigDrawer` generalized only as far as the inspector needs (no speculative API).

- **Acceptance**: labs unchanged visually; no dead components/routes left; `svelte-check` clean.

## Task Logging

Per repo convention: each task logs to `.specs/central-charts/tasks-log.json` on completion (`task_id | title | summary | is_working | timestamp | notes | decisions_made`).

## Development Methodology

TDD (RED-GREEN-REFACTOR) for all pure logic (FR-1..4, 9 bus logic) — tests in `tests/`, never `src/`. Component work (FR-5..8, 11..13) is verified via the labs/playground pages and CDP smoke checks. **Every task**: read `tdd-workflow` and `karpathy-guidelines` skills before starting; Karpathy review (minimal diff, no speculative abstraction, surgical changes) before marking done.
