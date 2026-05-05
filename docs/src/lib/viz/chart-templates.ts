/**
 * Chart Templates - Raw SveltePlot Code
 * 
 * Each template contains raw SveltePlot code with template variables.
 * Template variables use {variable_name} syntax and are replaced at runtime.
 * 
 * Users write the raw SveltePlot in the Code Editor, which then compiles
 * into a simple <ChartName /> component they can use in analysis views.
 * 
 * Flow:
 * 1. Code Editor: Raw SveltePlot with {variables}
 * 2. Template Compiler: Extracts variables, creates component
 * 3. User Analysis View: <BarChart x="category" y="sales" />
 * 4. Runtime: Injects props, renders final chart
 * 
 * Based on: https://svelteplot.dev/marks/bar
 */

import type { TemplateVariable } from './template-compiler';

export interface ChartTemplate {
	id: string;
	name: string;
	description: string;
	category: 'basic' | 'comparison' | 'trend' | 'distribution' | 'relationship' | 'composition';
	tags: string[];
	rawCode: string; // Raw SveltePlot with template variables
	variables: TemplateVariable[];
	usageExample?: string; // How users will use the compiled component
	thumbnail?: string;
}

export const CHART_TEMPLATES: ChartTemplate[] = [
	// 0. STARTER TEMPLATE (Always show first)
	{
		id: 'start-bar',
		name: '⭐ Start Here: Simple Bar Chart',
		description: 'Perfect starting point with sample data - just copy and modify!',
		category: 'basic',
		tags: ['bar', 'starter', 'beginner', 'simple'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
  
  // Sample data - replace with your data
  const data = [
    { category: 'A', value: 10 },
    { category: 'B', value: 25 },
    { category: 'C', value: 18 },
    { category: 'D', value: 32 },
    { category: 'E', value: 15 }
  ];
</scr` + `ipt>

<Plot
  x={{ label: "Category" }}
  y={{ grid: true, label: "Value" }}
>
  <BarY
    data={data}
    x="category"
    y="value"
    fill="#6366F1"
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x', type: 'string', required: true },
			{ name: 'y', type: 'string', required: true },
			{ name: 'fill', type: 'string', default: '#6366F1' }
		],
		usageExample: `<script>
  import { BarChart } from '$lib/viz/charts/BarChart.svelte';
  
  let data = []; // Your data here
</script>

<BarChart 
  data={data}
  x="category"
  y="value"
  fillColor="#6366F1"
/>`
	},

	// 1. SIMPLE BAR CHART
	{
		id: 'simple-bar',
		name: 'Simple Bar Chart',
		description: 'Basic vertical bar chart for comparing values across categories',
		category: 'basic',
		tags: ['bar', 'basic', 'comparison', 'vertical'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <BarY
    data={{data}}
    x={x_dimension}
    y={y_metric}
    fill={color}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#CA3500' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<BarChart 
  data={revenue_by_category}
  x="category"
  y="revenue"
  color="#6366F1"
/>`
	},

	// 2. HORIZONTAL BAR CHART
	{
		id: 'horizontal-bar',
		name: 'Horizontal Bar Chart',
		description: 'Horizontal bars for long category names and rankings',
		category: 'basic',
		tags: ['bar', 'horizontal', 'ranking'],
		rawCode: `<script lang="ts">
  import { Plot, BarX } from 'svelteplot';
</scr` + `ipt>

<Plot
  marginLeft={120}
  x={{ grid: true, label: {x_label} }}
  y={{ label: {y_label} }}
>
  <BarX
    data={{data}}
    x={x_metric}
    y={y_dimension}
    fill={color}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_metric', type: 'string', required: true },
			{ name: 'y_dimension', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#10B981' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<HorizontalBarChart 
  data={products}
  x="revenue"
  y="product_name"
  color="#10B981"
/>`
	},

	// 3. SORTED BAR CHART
	{
		id: 'sorted-bar',
		name: 'Sorted Bar Chart',
		description: 'Auto-sorted bars showing top performers or rankings',
		category: 'comparison',
		tags: ['bar', 'sorted', 'ranking', 'top-n'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
  
  $: sortedData = [...{data}].sort((a, b) => b[{y_metric}] - a[{y_metric}]);
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <BarY
    data={sortedData}
    x={x_dimension}
    y={y_metric}
    fill={color}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#CA3500' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<SortedBarChart 
  data={sales_by_rep}
  x="rep_name"
  y="total_sales"
  color="#CA3500"
/>`
	},

	// 4. WATERFALL CHART
	{
		id: 'waterfall',
		name: 'Waterfall Chart',
		description: 'Show cumulative effect of sequential values with color-coded changes',
		category: 'trend',
		tags: ['waterfall', 'change', 'variance', 'trend'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{
    label: {x_label}
  }}
  y={{ grid: true, label: {y_label} }}
>
  <BarY
    data={{data}}
    x={x_dimension}
    y1={(d, i) => {data}[i - 1]?.[{y_metric}] ?? 0}
    y2={y_metric}
    fill={(d, i) =>
      i === 0
        ? '#94A3B8'
        : d[{y_metric}] > {data}[i - 1]?.[{y_metric}]
          ? '#10B981'
          : '#EF4444'}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<WaterfallChart 
  data={monthly_changes}
  x="month"
  y="net_change"
/>`
	},

	// 5. LINE CHART
	{
		id: 'line',
		name: 'Line Chart',
		description: 'Track trends over time with smooth connecting lines',
		category: 'trend',
		tags: ['line', 'trend', 'time-series', 'continuous'],
		rawCode: `<script lang="ts">
  import { Plot, LineY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <LineY
    data={{data}}
    x={x_dimension}
    y={y_metric}
    stroke={color}
    strokeWidth={2}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#3B82F6' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<LineChart 
  data={revenue_over_time}
  x="date"
  y="revenue"
  color="#3B82F6"
/>`
	},

	// 6. AREA CHART
	{
		id: 'area',
		name: 'Area Chart',
		description: 'Show magnitude of change over time with filled area',
		category: 'trend',
		tags: ['area', 'trend', 'magnitude', 'time-series'],
		rawCode: `<script lang="ts">
  import { Plot, AreaY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <AreaY
    data={{data}}
    x={x_dimension}
    y={y_metric}
    fill={color}
    fillOpacity={0.3}
    stroke={color}
    strokeWidth={2}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#8B5CF6' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<AreaChart 
  data={traffic_over_time}
  x="date"
  y="visitors"
  color="#8B5CF6"
/>`
	},

	// 7. SCATTER PLOT
	{
		id: 'scatter',
		name: 'Scatter Plot',
		description: 'Explore relationships between two numeric variables',
		category: 'relationship',
		tags: ['scatter', 'correlation', 'relationship', 'dot'],
		rawCode: `<script lang="ts">
  import { Plot, Dot } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ grid: true, label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <Dot
    data={{data}}
    x={x_metric}
    y={y_metric}
    fill={color}
    r={5}
    fillOpacity={0.6}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_metric', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'color', type: 'string', default: '#EC4899' },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<ScatterPlot 
  data={customer_data}
  x="age"
  y="lifetime_value"
  color="#EC4899"
/>`
	},

	// 8. STACKED BAR CHART
	{
		id: 'stacked-bar',
		name: 'Stacked Bar Chart',
		description: 'Compare composition across categories with stacked segments',
		category: 'composition',
		tags: ['bar', 'stacked', 'composition', 'parts-to-whole'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <BarY
    data={{data}}
    x={x_dimension}
    y={y_metric}
    fill={series}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'series', type: 'string', required: true },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<StackedBarChart 
  data={sales_by_category}
  x="month"
  y="revenue"
  series="category"
/>`
	},

	// 9. GROUPED BAR CHART
	{
		id: 'grouped-bar',
		name: 'Grouped Bar Chart',
		description: 'Compare multiple series side-by-side across categories',
		category: 'comparison',
		tags: ['bar', 'grouped', 'multi-series', 'comparison'],
		rawCode: `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ grid: true, label: {y_label} }}
>
  <BarY
    data={{data}}
    x={x_dimension}
    y={y_metric}
    fill={series}
    offset="dodge"
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_metric', type: 'string', required: true },
			{ name: 'series', type: 'string', required: true },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<GroupedBarChart 
  data={actual_vs_target}
  x="region"
  y="value"
  series="type"
/>`
	},

	// 10. HEATMAP
	{
		id: 'heatmap',
		name: 'Heatmap',
		description: 'Show patterns in data using color intensity',
		category: 'distribution',
		tags: ['heatmap', 'density', 'pattern', 'matrix'],
		rawCode: `<script lang="ts">
  import { Plot, Cell } from 'svelteplot';
</scr` + `ipt>

<Plot
  x={{ label: {x_label} }}
  y={{ label: {y_label} }}
  color={{
    type: 'linear',
    scheme: 'BuRd'
  }}
>
  <Cell
    data={{data}}
    x={x_dimension}
    y={y_dimension}
    fill={value_metric}
    inset={1}
  />
</Plot>`,
		variables: [
			{ name: 'data', type: 'data', required: true },
			{ name: 'x_dimension', type: 'string', required: true },
			{ name: 'y_dimension', type: 'string', required: true },
			{ name: 'value_metric', type: 'string', required: true },
			{ name: 'x_label', type: 'string', default: null },
			{ name: 'y_label', type: 'string', default: null }
		],
		usageExample: `<Heatmap 
  data={hourly_activity}
  x="day_of_week"
  y="hour"
  value="activity_count"
/>`
	}
];

/**
 * Get templates by category
 */
export function getTemplatesByCategory(category: ChartTemplate['category']): ChartTemplate[] {
	return CHART_TEMPLATES.filter(t => t.category === category);
}

/**
 * Get templates by tag
 */
export function getTemplatesByTag(tag: string): ChartTemplate[] {
	return CHART_TEMPLATES.filter(t => t.tags.includes(tag));
}

/**
 * Search templates
 */
export function searchTemplates(query: string): ChartTemplate[] {
	const lowercaseQuery = query.toLowerCase();
	return CHART_TEMPLATES.filter(t => 
		t.name.toLowerCase().includes(lowercaseQuery) ||
		t.description.toLowerCase().includes(lowercaseQuery) ||
		t.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery))
	);
}

/**
 * Get all unique tags
 */
export function getAllTags(): string[] {
	const tags = new Set<string>();
	CHART_TEMPLATES.forEach(t => t.tags.forEach(tag => tags.add(tag)));
	return Array.from(tags).sort();
}

/**
 * Get all categories
 */
export function getAllCategories(): ChartTemplate['category'][] {
	return ['basic', 'comparison', 'trend', 'distribution', 'relationship', 'composition'];
}

