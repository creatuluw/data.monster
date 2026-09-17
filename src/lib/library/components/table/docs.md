# Table

Raw rows in a compact, sticky-header table — the block for detail data.

## Options

- `limit` — row limit fetched for the block (default 50)

## Using it on a page

Add via the editor's **+ Table** button, then pick a table in the config drawer.
Columns default to all columns; select specific ones to narrow it.

## Demo wiring

The `/library` preview feeds `demo.ts` rows straight into the same TableRenderer
`/pages` uses.
