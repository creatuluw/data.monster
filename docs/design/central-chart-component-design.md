# Central Chart Component Design — data.monster

One design for every chart in the app. Derived from the Evidence.dev prop taxonomy, adapted to our stack (Tauri + DuckDB + SvelteKit/Svelte 5 + SveltePlot) and a 17-question design interview (2026-09-15).

## The one-line model

> Every chart on a report page is a **declarative spec block**, resolved through **one shared pipeline** (semantic layer → query engine → chart type registry → shell) into a **SveltePlot render**. A **page editor with two surfaces** (no-code Design, code mode) edits the same spec document — parity by construction.

---

## 1. Locked decisions

| # | Question | Decision |
|---|----------|----------|
| 1 | Architecture shape | **C — composition layers**: shell + shared contract + per-type components + escape hatch |
| 3 | Code surface | **A — constrained declarative spec**; no arbitrary Svelte in pages; perfect parity |
| 4 | Data binding | **B refined** — each chart is query-bound; dimensions/measures are the dynamic part |
| 5 | Measure/dimension form | **Expressions, not sugar** — `sum(if(type='class', hours))`; `{col, agg}` dropdowns are UI autocomplete that writes expressions |
| 6 | Master items | **A — workspace-level library** in internal DB, referenced by stable id, each bound to its table |
| 7 | Smart association | **B — relationship graph** drives which master items a chart may use, and auto-JOINs |
| 8 | Query generation | **C — one canonical query engine** + per-type hooks (grain, top-N, pivot, ordering) |
| 9 | Chart-specific options | **C — schema-driven panels** from the registry + custom-panel escape hatch |
| 10 | Interactivity | **B — selection cross-filters** other charts via re-query (Qlik-style, transient) |
| 11 | Tooltips | **B — declarative fields + one template string**, shared renderer |
| 12 | Colors | **B — page-consistent shared scale** (same value = same color) + manual overrides |
| 13 | Layout | **B — explicit grid**: spec declares rows, blocks take 12-col spans |
| 14 | Page blocks | **B — general block envelope** + block registry (`chart`, `text` first; `table`/`kpi` later) |
| 15 | Annotations | **A — SveltePlot basic-mark vocabulary** (Arrow, Dot, Line, RuleX/RuleY, Text, Rect), whitelisted per type, values literal or measure expression |
| 16 | Editing surface | **A — `/pages` lists; `/page/<slug>` is the dual-mode editor** (Design ⇄ Code over one document) |
| 17 | Storage | **A — internal DuckDB** (`d8a_monster_pages`); code mode is the in-app editor |

Deferred (explicitly): linked-hover sync (Q10-C), master-dimension value colors (Q12-C), arbitrary layered-mark DSL (Q15-C), files-as-storage (Q17-B/C), non-chart blocks beyond text (Q14-C).

---

## 2. The layers

Bottom-up, each layer only knows the one below:

```
L7  Editor surfaces      /page/<slug>: Design mode (inspector) ⇄ Code mode (text) — same doc
L6  Page runtime         grid, cross-filter bus, shared color scale, shared tooltip renderer, states
L5  Chart shell          card frame, title/subtitle, selection label, config toggle, empty/error/missing states
L4  Chart type registry  per type: roles schema, options schema, hooks, annotation whitelist, renderer
L3  Chart spec contract  shared props every type consumes (roles, fmt, legend, axes, size, tooltip, …)
L2  Semantic layer       master items (dims/measures as expressions, table-bound) + relationship graph
L1  Query engine         canonical aggregation query + per-type hooks; runs against DuckDB
L0  Chart fundament      pure TS (sameDatum, aggregation utils) — already exists
```

## 3. The spec documents

### 3.1 Page document (stored in `d8a_monster_pages`)

```json
{
  "slug": "monthly-utilization",
  "title": "Monthly utilization",
  "rows": [
    {
      "blocks": [
        {
          "type": "chart",
          "span": 8,
          "chart": {
            "type": "bar",
            "source": { "table": "bookings" },
            "dimensions": [
              { "col": "month", "grain": "month" }
            ],
            "measures": [
              { "ref": "mi_class_hours" },
              { "expr": "sum(hours)", "label": "Total hours", "fmt": "hours" }
            ],
            "filters": [{ "col": "year", "op": "=", "value": 2025 }],
            "sort": { "by": "measure[0]", "dir": "desc" },
            "limit": 12,
            "options": { "orientation": "horizontal", "topN": 10, "otherLabel": "Other" },
            "annotations": [
              { "mark": "ruleY", "at": { "expr": "avg(hours) * 1.1" }, "label": "Ø +10%" }
            ],
            "tooltip": {
              "title": "month",
              "fields": [{ "col": "hours", "label": "Hours", "fmt": "hours" }],
              "template": "{month}: {hours} h"
            },
            "axes": { "x": { "title": null, "gridlines": true }, "y": { "fitToData": false } },
            "legend": { "show": true, "position": "top" },
            "title": "Booked hours per month",
            "subtitle": null,
            "heightVh": 0.3
          }
        },
        { "type": "text", "span": 4, "text": "## Insight\nAmsterdam leads on class hours." }
      ]
    }
  ]
}
```

Rules:

- A **measure/dimension entry is either** `{ "ref": "<master-item-id>" }` (resolves in the library) **or inline** `{ "expr": "...", "label": "...", "fmt": "..." }`.
- Unrepresentable things never exist — if a field is unknown, validation errors (Evidence's `echarts_options` pattern shows raw JSON in the UI at worst).
- Escape hatch: every chart block accepts `plotOptions` (raw SveltePlot props object) merged over computed config — the power-user door; shown as raw JSON in Design mode.

### 3.2 Master item (stored in `d8a_monster_master_items`)

```json
{
  "id": "mi_class_hours",
  "kind": "measure",
  "table": "bookings",
  "label": "Class hours",
  "expr": "sum(if(type = 'class', hours))",
  "fmt": "hours",
  "description": "All hours booked on class-type activities"
}
```

- Stable ids survive renames; deleting one leaves charts in a **"missing master item"** state (never silent breakage).
- Availability: a chart on table X sees items bound to X **and any table joinable to X** via the relationship graph. Using an item from a related table adds the auto-JOIN along the declared relationship.

### 3.3 Query engine (L1)

Canonical shape for every type:

```sql
SELECT {{dimensions}}, {{measures}}
FROM {{table}} {{auto_joins}}
{{filters}}
GROUP BY {{dimensions}}
ORDER BY {{order}}
LIMIT {{limit}}
```

Per-type hooks (registry-declared, shared spec fields drive them):

| Hook | Used by | Effect |
|------|---------|--------|
| `grain` | any dimension declared `date` | trunc/bucket (`month`, `week`, `day of week`, …) |
| `topN` | bar | cap + `Other` bucket (existing `buildBars` logic) |
| `pivot` | heatmap | full 2-D grid pre-aggregation |
| `order` | funnel | ordering pass |

Cross-filter mechanics: page runtime holds the current selection `{ chartId, dimension, value }`; every other chart re-queries with the selection injected as a `WHERE dim = value`; the source chart stays untouched; click-away clears. Selections are transient — never persisted.

---

## 4. Shared vs chart-specific — the contract

### Every chart type gets (shared, L3)

| Concern | Spec fields | Notes |
|---------|-------------|-------|
| Data roles | `source`, `dimensions`, `measures`, `filters`, `sort`, `limit` | filled/validated per the type's role schema |
| Titles | `title`, `subtitle` | shell renders |
| Formatting | `fmt` per measure/field | one shared format library (`usd`, `hours`, `%`, dates) |
| Legend | `legend.show`, `legend.position` | |
| Axes | `axes.x/y…` groups: title, ticks, gridlines, min/max/fit, labels, rotate | types map roles → axes |
| Size | `heightVh` (default 0.3), `span` (layout) | existing app rule |
| Tooltip | `tooltip.title/fields/template` | shared renderer; type only anchors it |
| Annotations | `annotations[]` | from the type's whitelist only |
| Colors | via page shared scale | `seriesColors` overrides optional |
| States | empty / error / loading / missing master item | shell renders |
| Selection | click-to-select, positional matching (`sameDatum`) | cross-filters page |

### Each chart type declares (registry, L4)

```ts
interface ChartTypeDefinition {
  type: string;                      // "bar", "line", "heatmap", …
  label: string;
  roles: RoleSpec[];                 // e.g. bar: 1 dimension, 1+ measures, optional series dim
  optionsSchema: FieldSpec[];        // drives the schema-based panel in BOTH surfaces
  customPanel?: SvelteComponent;     // escape hatch for bespoke option UI
  hooks?: QueryHooks;                // grain / topN / pivot / order
  annotations: MarkName[];           // whitelist: subset of arrow,dot,line,ruleX,ruleY,text,rect
  component: SvelteComponent<ChartRendererProps>; // thin SveltePlot renderer
  defaults: object;                  // merged into options
}
```

What genuinely differs per chart (and *only* this differs):

- **Role schema** — which dimensions/measures are required (scatter: 2 measures; heatmap: 2 dims + 1 measure)
- **Option fields** — marks' own knobs (scatter `size`, line `missingValues`/`curve`, heatmap `scheme`/`threshold`, bar `orientation`/`stacked`)
- **Query hooks** — the special shapes
- **Annotation whitelist** — cartesian types host rules/dots/text; heatmap hosts none
- **The renderer** — which SveltePlot marks it draws, tooltip anchoring, how selection looks
- **Optional custom panel** — for options deserving bespoke UI

The renderer itself receives everything resolved: rows (engine output), shared scales (color), selection bindings, annotation marks pre-resolved to values, tooltip renderer, and its typed options. It stays a thin mark-layer — the same discipline as today's `Heatmap`/`BarChart`.

---

## 5. Editor surfaces (L7)

- `/pages` — list + create; creating opens `/page/<slug>`.
- `/page/<slug>` — dual mode over one document:
  - **Design**: live grid; click a block → inspector panel (schema-driven controls, same component family as the labs bolt drawer); add-block menu (chart → pick type → map roles → place); annotation menu per type whitelist.
  - **Code**: the page document as editable text with live validation; switching modes is lossless (same document).
- `/labs` remains the per-type playground; labs pages mount registry entries directly (proof: the 32-card catalog = the registry).

## 6. Migration from today's code

| Exists today | Becomes |
|--------------|---------|
| `src/lib/charts/fundament.ts` (`sameDatum`, `buildBars`) | L0 stays; `buildBars` = the `topN` hook implementation |
| `Heatmap.svelte`, `BarChart.svelte` | registry entries + thin renderers; shell extracts their shared card frame |
| ChartConfigDrawer + bolt toggle | generalized schema-driven panel, reused by labs and page inspector |
| `heightVh` rule, DS colors, mono labels | carried into shell/base options unchanged |

## 7. Build order

1. **Registry + spec types + validation + shell** — port bar & heatmap onto it (labs keeps working)
2. **Query engine + master items + relationship integration** (internal DB tables)
3. **Page runtime** — grid renderer, cross-filter bus, shared color scale, tooltip renderer
4. **`/page/<slug>` editor** — Design ⇄ Code, inspector, add-block flows
5. **Catalog rollout** — move the 30 placeholder types onto the registry one at a time, each gaining roles/options as it's built
