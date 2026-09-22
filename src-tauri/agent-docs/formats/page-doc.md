# Page doc format

> Read when: creating or editing anything under `dm/pages/`.

A page is one JSON document: rows of columns of blocks. Filename = slug = route.
One annotated minimal example — copy, save as `dm/pages/first.json`, watch it appear:

```json
{
  "slug": "first",
  "title": "First page",
  "rows": [
    {
      "columns": [
        {
          "span": 12,
          "height": 320,
          "blocks": [
            {
              "type": "chart",
              "chart": {
                "type": "bar",
                "title": "Revenue by region",
                "source": { "table": "orders" },
                "dimensions": [{ "col": "region" }],
                "measures": [{ "expr": "sum(amount)", "label": "Revenue" }]
              }
            }
          ]
        }
      ]
    }
  ]
}
```

Rules:

- `span` is the 12-column grid width of the column (1–12); `height` is optional pixels (≥ 40).
- `dimensions[].col` is a column of `source.table` (or a related table via a master-item ref).
- `measures[]` are DuckDB expressions; `"fmt"` is an optional format hint ("usd", "pct", ...).
- Prefer master-item references for measures: `{"ref": "total_revenue"}` (see master-items).
- Filters: `"filters": [{"col": "region", "op": "=", "value": "EMEA"}]`; sort/limit supported.
- Kinds: blocks are `chart` | `table` | `text`; chart types `bar` | `heatmap`.
- Block ids are positional (`r{row}-c{col}-b{block}`) — do not hand-assign ids.
- Full field reference: `../reference/page-doc-fields.md`.

The no-code editor writes exactly this document — expect your edits to interleave with
a human's edits. On conflict you both see the file's latest state; last write wins.
