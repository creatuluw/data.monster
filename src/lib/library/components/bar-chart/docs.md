# Bar chart

Horizontal (default) or vertical bars over one dimension and one or more measures.

## Roles

- **Dimension** (exactly 1) — the band axis category.
- **Measure** (1+) — bar length. The first measure is plotted; extras feed tooltips.

## Options

- `orientation` — `horizontal` (default) or `vertical`
- `topN` — keep only the top N categories (rest bucketed into `otherLabel`)
- `otherLabel` — label for the aggregated remainder (default `Other`)

## Annotations

Supports `ruleX` / `ruleY` reference rules (dashed line at a value).

## Using it on a page

Add via the editor's **+ Bar chart** button — the registry supplies the default
block (1st table, 1st column, `count(*)`). The config panel is generated from
the option schema above.

## Demo wiring

The `/library` preview feeds `demo.ts` rows straight into the same renderer
`/pages` uses — replace the data source, not the component.
