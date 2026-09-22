# Relationships format

> Read when: creating or editing `dm/relationships.json`.

One JSON document describing joins between source tables. The query compiler walks
this graph to auto-JOIN related tables, so charts can use fields from tables that are
not their direct source.

Example:

```json
{
  "relationships": [
    {
      "id": "rel-orders-customers",
      "fromTable": "orders",
      "fromColumn": "customer_id",
      "toTable": "customers",
      "toColumn": "id"
    }
  ]
}
```

Rules:

- `id` is a stable slug (`rel-<from>-<to>` is the app's convention).
- Direction matters only for readability; the compiler matches either way.
- A corrupt document FAILS LOUD and the app refuses to overwrite it with an empty
  graph — fix the JSON rather than deleting the file.
- Add a relationship when a chart needs a column from a table two hops away;
  chained relationships are traversed.
