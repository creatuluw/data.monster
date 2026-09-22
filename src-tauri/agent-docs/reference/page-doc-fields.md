# Page doc field reference

> Read when: a format doc pointed you here for an exhaustive field list.

Document:

- `slug` (string) — route id; MUST equal the filename stem.
- `title` (string) — display title.
- `rows` (array) — ordered horizontal bands.

Row:

- `columns` (array, preferred) — grid split of this row; sum of spans should be ≤ 12.
- `height` (number, optional) — fixed row height in px (≥ 40); else auto.
- Legacy: `blocks` directly on the row (single implicit column) — still read, not written.

Column:

- `span` (int 1–12) — width on the 12-column grid.
- `height` (number, optional) — fixed column height px (≥ 40).
- `blocks` (array) — stacked blocks in this column.

Block (all kinds):

- `type` — "chart" | "table" | "text".
- `title` (string) — block display title; for charts the RENDERED title is
  `chart.title` (see below) — set that one.

Chart block:

- `chart.type` — "bar" | "heatmap".
- `chart.title` / `chart.subtitle` — rendered heading texts.
- `chart.source.table` — source table (required).
- `chart.dimensions[]` — `{ "col": string, "grain"?: string }` or `{ "ref": "<master-item id>" }`.
- `chart.measures[]` — `{ "expr": string, "label": string, "fmt"?: string }` or `{ "ref": "..." }`.
- `chart.filters[]` — `{ "col", "op" ("=" | "!=" | ">" | "<" | ">=" | "<=" | "in" | "like"), "value" }`.
- `chart.sort` — `{ "col": string, "dir": "asc" | "desc" }`; `chart.limit` (number).
- `chart.options` — per-type extras (bar orientation etc.), optional; omit unless needed.
- `chart.annotations[]` — whitelisted svelteplot marks: arrow, dot, line, ruleX, ruleY, text, rect.
- `chart.tooltip` — declarative `{ "fields": [...], "template": string }`.
- `chart.axes` / `chart.legend` — optional axis/legend overrides.
- `chart.heightVh` (number, optional) — block height as vh units.

Table block:

- `table` (string, required) — source table.
- `columns` (array of column names, optional — default all).
- `filters` / `sort` / `limit` (default 100) — as above.

Text block:

- `text` (string) — paragraph text.

Validation: the app validates on load (`{path, message}` list, never throws).
Unknown chart types, bad spans, unresolved refs and empty expressions are reported,
not coerced.
