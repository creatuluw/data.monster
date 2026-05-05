# Viz Library

A comprehensive visualization library for creating interactive charts with an Evidence.dev-style API.

## Quick Start

### Using the Chart Builder

Navigate to `/warp-lab/chart-lib` in your application to access the visual chart builder:

1. **Select a chart type** from the available options
2. **Configure settings** using the visual settings panel
3. **Preview changes** in real-time on the right panel
4. **Copy the code** from the code view to use in your components

### Using Charts Directly

Import and use charts in your Svelte components:

```svelte
<script>
  import { BarChart } from '$lib/viz';
  
  let data = [
    { category: 'Product A', sales: 12000 },
    { category: 'Product B', sales: 19000 },
    { category: 'Product C', sales: 8500 }
  ];
</script>

<BarChart 
  data={data}
  x="category"
  y="sales"
  title="Sales by Product"
  fillColor="#CA3500"
/>
```

## Available Charts

### BarChart

Vertical or horizontal bar charts for comparing values across categories.

**Minimal Example:**
```svelte
<BarChart data={myData} />
```

**Full Example:**
```svelte
<BarChart 
  data={salesData}
  x="month"
  y="sales"
  title="Monthly Sales"
  subtitle="Last 6 months"
  fillColor="#6366F1"
  fillOpacity={0.8}
  chartAreaHeight={450}
  yFmt="usd0k"
  labels={true}
  sort={true}
/>
```

**Horizontal Bar Chart:**
```svelte
<BarChart 
  data={productData}
  x="product"
  y="revenue"
  swapXY={true}
  fillColor="#10B981"
  yFmt="usd0k"
/>
```

## Core Features

### 🎨 Evidence.dev-Style API

Charts use a declarative, prop-based API inspired by Evidence.dev:
- Simple, readable syntax
- Smart defaults
- Auto-detection of data columns
- Flexible formatting options

### 🔧 Visual Chart Builder

A comprehensive UI for creating charts without writing code:
- Browse chart types
- Configure all settings visually
- Live preview
- Generate copy-paste code

### 📊 Interactive Charts

All charts include:
- Hover tooltips
- Click interactions
- Responsive sizing
- Dark mode support

### 🎯 Smart Defaults

Charts work with minimal configuration:
- Auto-detects x and y columns from data
- Sensible default styling
- Common BI tool conventions
- Only shows non-default props in generated code

## File Structure

```
src/lib/viz/
├── charts/
│   ├── BarChart.svelte       # Bar chart component
│   ├── index.ts              # Chart registry and metadata
│   └── examples.ts           # Usage examples
├── core/
│   └── ChartBuilder.svelte   # Visual chart builder UI
└── index.ts                  # Main exports
```

## Prop Types

### Common Props (All Charts)

```typescript
interface CommonChartProps {
  data: any[];                    // Data array (required)
  title?: string;                 // Chart title
  subtitle?: string;              // Chart subtitle
  chartAreaHeight?: number;       // Height in pixels (default: 400)
  emptyMessage?: string;          // Message for empty data
}
```

### BarChart Props

```typescript
interface BarChartProps extends CommonChartProps {
  // Data
  x?: string;                     // X-axis column (auto-detected)
  y?: string | string[];          // Y-axis column (auto-detected)
  series?: string;                // Series grouping column
  
  // Appearance
  type?: 'stacked' | 'grouped';   // Bar grouping (default: 'stacked')
  fillColor?: string;             // Fill color (default: '#CA3500')
  fillOpacity?: number;           // Opacity 0-1 (default: 1)
  colorPalette?: string[];        // Colors for multi-series
  
  // Chart
  swapXY?: boolean;               // Horizontal chart (default: false)
  sort?: boolean;                 // Sort by value (default: true)
  
  // Axes
  xAxisLabels?: boolean;          // Show x labels (default: true)
  yAxisLabels?: boolean;          // Show y labels (default: true)
  xGridlines?: boolean;           // Show x gridlines (default: false)
  yGridlines?: boolean;           // Show y gridlines (default: true)
  yMin?: number;                  // Y-axis minimum
  yMax?: number;                  // Y-axis maximum
  
  // Labels & Formatting
  labels?: boolean;               // Show value labels (default: false)
  legend?: boolean;               // Show legend (default: true)
  xFmt?: string;                  // X-axis format
  yFmt?: string;                  // Y-axis format
}
```

## Formatting

Format strings follow Evidence.dev conventions:

### Currency
- `usd0` → $1,234
- `usd2` → $1,234.56
- `usd0k` → $1k

### Percentage
- `pct0` → 45%
- `pct2` → 45.67%

### Numbers
- `num0` → 1,234
- `num2` → 1,234.56

## Adding New Charts

1. Create a new chart component in `src/lib/viz/charts/YourChart.svelte`
2. Follow the Evidence.dev API pattern
3. Export from `src/lib/viz/charts/index.ts`
4. Add metadata to `CHART_REGISTRY`
5. Chart will automatically appear in the builder

See `docs/CHART-LIBRARY.md` for detailed instructions.

## Examples

See `src/lib/viz/charts/examples.ts` for more usage examples.

## Technologies

- **Svelte 5**: Component framework with runes
- **svelteplot**: Chart rendering library
- **Lucide Svelte**: Icon library
- **Tailwind CSS**: Styling

