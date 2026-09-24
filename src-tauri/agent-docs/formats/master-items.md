# Master items format

> Read when: creating or editing anything under `dm/master-items/`.

A master item is a named, table-bound DuckDB expression reused across charts by
`{"ref": "<id>"}`. The filename (without .json) IS the id — charts point at it, so
renames break references.

Example — `dm/master-items/measures/total_revenue.json`:

```json
{
  "id": "total_revenue",
  "kind": "measure",
  "tableName": "orders",
  "expr": "sum(amount)",
  "label": "Total revenue",
  "fmt": "usd",
  "description": "Sum of order amounts across all regions"
}
```

A dimension — `dm/master-items/dimensions/region.json`:

```json
{
  "id": "region",
  "kind": "dimension",
  "tableName": "orders",
  "expr": "region",
  "label": "Region",
  "fmt": null,
  "description": null
}
```

Rules:

- Folder = kind: `measures/` (aggregations) vs `dimensions/` (grouping columns).
- `id` in the file must equal the filename; `tableName` must be an existing source table.
- `expr` is a raw DuckDB expression; measures usually aggregate, dimensions do not.
- `fmt` (optional): "usd" | "pct" | "hours" | "int" or null.
- No timestamps in the file — the app tracks freshness via file mtime.
- Prefer a master item over repeating an expression across charts: consistent
  formatting, one place to fix, and the auto-JOIN reaches related tables via these
  table bindings.
