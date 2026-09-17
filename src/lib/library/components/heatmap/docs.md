# Heatmap

Color-grid of one measure across exactly two dimensions (row × column pivot).

## Roles

- **Dimension** (exactly 2) — rows and columns of the grid.
- **Measure** (exactly 1) — cell value → color.

## Options

- `scheme` — color scheme: `orrd` (default), `blues`, `greens`
- `threshold` — number of color stops (default 5, 2–9)

## Using it on a page

Add via the editor's **+ Heatmap** button — the registry supplies the default
block (1st table, first two columns, `count(*)`). The config panel is generated
from the option schema above.

## Demo wiring

The `/library` preview feeds `demo.ts` rows straight into the same renderer
`/pages` uses — replace the data source, not the component.
