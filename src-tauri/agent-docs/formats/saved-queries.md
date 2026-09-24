# Saved queries format

> Read when: creating or editing anything under `dm/saved-queries/`.

One `.sql` file per query; the filename (without .sql) is the slug. The SQL body is
verbatim — write real SQL, not JSON-escaped strings. Optional single-line meta header
on line 1; everything after it is the query.

Example — `dm/saved-queries/top-customers.sql`:

```sql
-- dm: {"name":"Top customers","description":"Revenue per customer, top 20","tags":["sales","revenue"]}
SELECT c.name, sum(o.amount) AS revenue
FROM orders o JOIN customers c ON o.customer_id = c.id
GROUP BY c.name
ORDER BY revenue DESC
LIMIT 20
```

Rules:

- Header keys: `name` (display name), `description`, `tags` (JSON array; the UI shows
  them comma-separated). All optional; headerless files get name = filename.
- The header must be ONE line starting exactly with `-- dm:` followed by valid JSON.
  A malformed header is reported as a problem — the query is skipped, never half-run.
- Filename = slug = identity; renaming changes the slug.
- Any DuckDB SELECT is fine; the /query UI runs it and paginates results.
