# Common task recipes

> Read when: you want a step-by-step for a typical request.

## Build a dashboard for a table

1. Inspect the table (ask the user for columns, or read a saved query's SQL).
2. Read `formats/master-items.md`; create 1–3 measures + dimensions you will reuse.
3. Read `formats/page-doc.md`; create `dm/pages/<table>-overview.json`:
   a text block with the title, a bar chart (dimension + measure), a table block.
4. The open app shows /pages/<table>-overview immediately. Iterate on the JSON.

## Add "profit" everywhere

1. Read `formats/master-items.md`; create `dm/master-items/measures/profit.json`
   with `expr`: `sum(revenue) - sum(cost)`, bound to the right table.
2. Reference `{"ref": "profit"}` in any chart's measures. Done — formatting is central.

## Ingest a CSV the user mentions

1. The CSV lands in `data/main/` (or ask the user to run Connect for a remote source).
2. Table name = filename stem. Create a page + master items for it (see above).

## User reports a broken page

1. Read the page JSON; apply the app's rules (span 1–12, refs resolve, expressions are
   valid DuckDB). Fix the file; the app hot-reloads the fix.
