# Library Chart Templates

This directory contains the library chart templates that ship with warphead.dev. These charts are automatically loaded into DuckDB when the application initializes.

## 📁 Files

- **`library-charts.json`** - JSON definition of all library chart templates that ship with the app

## 🔧 How It Works

1. **JSON Definition**: All library chart templates are defined in `library-charts.json`
2. **Runtime Loading (Development)**: When developing, the backend reads the JSON file from disk on each app init
3. **Embedded Fallback (Production)**: In production builds, the JSON is embedded using `include_str!()` macro
4. **Auto-loaded on Init**: When DuckDB initializes, the backend:
   - Parses the JSON file (from disk if available, otherwise from embedded version)
   - Creates the `_warphead_chart_templates` table if it doesn't exist
   - Inserts/updates each chart template using `ON CONFLICT DO UPDATE` (updates library charts when JSON changes)
   - Reports how many charts were created vs updated

## 📝 JSON Schema

Each chart in the JSON file has the following structure:

```json
{
  "chart_name": "Bar Chart",
  "chart_type": "BarChart",
  "description": "Vertical bar chart for comparing values across categories",
  "tags": "basic,comparison,categorical",
  "chart_code": "<script lang=\"ts\">...</script>\n\n<Plot>...</Plot>"
}
```

### Required Fields
- `chart_name` - Display name for the chart template
- `chart_type` - Unique component name (e.g., `BarChart`, `LineChart`)
- `chart_code` - Complete Svelte component code using SveltePlot
- `description` - Human-readable description of what the chart does

### Optional Fields
- `tags` - Comma-separated tags for categorization (e.g., `basic,comparison,categorical`)
- `config_schema` - JSON schema for chart configuration options
- `metrics` - Comma-separated list of metric slugs this chart uses
- `dimensions` - Comma-separated list of dimension slugs this chart uses
- `sample_data` - Sample data for preview/testing

## 📊 Built-in Chart Templates

The library includes these chart templates:

1. **Bar Chart** - Vertical bar chart for comparing categorical values
2. **Line Chart** - Line chart for trends over time or continuous data
3. **Scatter Plot** - Scatter plot for showing relationships between variables
4. **Area Chart** - Area chart for cumulative trends over time
5. **Heatmap** - Heatmap for visualizing data density or matrix values
6. **Data Table** - Simple tabular data display
7. **KPI Tile** - Single value KPI display with optional trend indicator

## 🎨 Chart Template Guidelines

When creating chart templates:

- **Use SveltePlot**: All charts should use the SveltePlot library for consistency
- **Export Props**: Use `export let` for all configurable properties
- **Provide Defaults**: Always provide sensible default values
- **Document Props**: Include comments describing what each prop does
- **Follow Patterns**: Look at existing templates for structure and naming conventions
- **Test Thoroughly**: Test with various data shapes and edge cases

## 🔄 Updating Library Charts

To update library charts in development:

1. Edit `library-charts.json`
2. Restart the app (charts are loaded on initialization)
3. Charts will be automatically updated in the database
4. Check Analysis > Charts to see your changes

## 📚 Adding New Chart Templates

To add a new library chart:

1. Add a new entry to the `charts` array in `library-charts.json`
2. Follow the JSON schema structure above
3. Use SveltePlot components from the [SveltePlot documentation](https://svelteplot.dev)
4. Test your chart with sample data
5. Restart the app to load the new chart

Example based on SveltePlot examples:

```json
{
  "chart_name": "My Custom Chart",
  "chart_type": "MyCustomChart",
  "description": "A custom chart for special use cases",
  "tags": "custom,advanced",
  "chart_code": "<script lang=\"ts\">...</script>\n\n<Plot>...</Plot>"
}
```

## 🐛 Troubleshooting

**Charts not appearing after editing JSON:**
- Restart the application completely
- Check the backend console for error messages
- Verify JSON syntax is valid

**Chart renders incorrectly:**
- Check that all SveltePlot components are imported
- Verify data prop types match expected format
- Look for console errors in browser dev tools

**Duplicate chart_type error:**
- Each `chart_type` must be unique
- Check existing charts in Analysis > Charts
- Use a different component name


