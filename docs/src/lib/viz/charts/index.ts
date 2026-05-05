// Chart Registry - Auto-exports all available charts
export { default as BarChart } from './BarChart.svelte';

// Chart metadata for the builder UI
export interface ChartMetadata {
	name: string;
	displayName: string;
	component: any;
	icon: string;
	description: string;
	category: string;
	defaultProps: Record<string, any>;
	propDefinitions: PropDefinition[];
}

export interface PropDefinition {
	name: string;
	displayName: string;
	type: 'string' | 'number' | 'boolean' | 'select' | 'color' | 'array';
	category: string;
	default?: any;
	options?: Array<{ value: any; label: string }>;
	description?: string;
}

// Import the components
import BarChart from './BarChart.svelte';

// Chart registry with metadata
export const CHART_REGISTRY: Record<string, ChartMetadata> = {
	BarChart: {
		name: 'BarChart',
		displayName: 'Bar Chart',
		component: BarChart,
		icon: '📊',
		description: 'Compare values across categories with vertical or horizontal bars',
		category: 'Basic Charts',
		defaultProps: {
			type: 'stacked',
			fillColor: '#CA3500',
			fillOpacity: 1,
			chartAreaHeight: 400,
			yGridlines: true,
			xGridlines: false,
			xAxisLabels: true,
			yAxisLabels: true,
			labels: false,
			legend: true,
			sort: true,
			swapXY: false
		},
		propDefinitions: [
			// Data
			{
				name: 'data',
				displayName: 'Data',
				type: 'array',
				category: 'Data',
				description: 'Array of data objects to visualize'
			},
			{
				name: 'x',
				displayName: 'X Column',
				type: 'string',
				category: 'Data',
				description: 'Column to use for x-axis (categories)'
			},
			{
				name: 'y',
				displayName: 'Y Column',
				type: 'string',
				category: 'Data',
				description: 'Column to use for y-axis (values)'
			},
			{
				name: 'series',
				displayName: 'Series Column',
				type: 'string',
				category: 'Data',
				description: 'Column to group by for multi-series charts'
			},
			// Appearance
			{
				name: 'title',
				displayName: 'Title',
				type: 'string',
				category: 'Labels',
				description: 'Chart title'
			},
			{
				name: 'subtitle',
				displayName: 'Subtitle',
				type: 'string',
				category: 'Labels',
				description: 'Chart subtitle'
			},
			{
				name: 'type',
				displayName: 'Type',
				type: 'select',
				category: 'Appearance',
				default: 'stacked',
				options: [
					{ value: 'stacked', label: 'Stacked' },
					{ value: 'grouped', label: 'Grouped' }
				],
				description: 'Bar grouping method'
			},
			{
				name: 'fillColor',
				displayName: 'Fill Color',
				type: 'color',
				category: 'Appearance',
				default: '#CA3500',
				description: 'Bar fill color'
			},
			{
				name: 'fillOpacity',
				displayName: 'Fill Opacity',
				type: 'number',
				category: 'Appearance',
				default: 1,
				description: 'Bar fill opacity (0-1)'
			},
			{
				name: 'colorPalette',
				displayName: 'Color Palette',
				type: 'array',
				category: 'Appearance',
				description: 'Array of colors for multi-series'
			},
			// Chart Settings
			{
				name: 'chartAreaHeight',
				displayName: 'Height',
				type: 'number',
				category: 'Chart',
				default: 400,
				description: 'Chart height in pixels'
			},
			{
				name: 'swapXY',
				displayName: 'Horizontal',
				type: 'boolean',
				category: 'Chart',
				default: false,
				description: 'Create horizontal bar chart'
			},
			{
				name: 'sort',
				displayName: 'Sort',
				type: 'boolean',
				category: 'Chart',
				default: true,
				description: 'Sort bars by value'
			},
			// Axes
			{
				name: 'xAxisLabels',
				displayName: 'X Axis Labels',
				type: 'boolean',
				category: 'Axes',
				default: true,
				description: 'Show x-axis labels'
			},
			{
				name: 'yAxisLabels',
				displayName: 'Y Axis Labels',
				type: 'boolean',
				category: 'Axes',
				default: true,
				description: 'Show y-axis labels'
			},
			{
				name: 'xGridlines',
				displayName: 'X Gridlines',
				type: 'boolean',
				category: 'Axes',
				default: false,
				description: 'Show vertical gridlines'
			},
			{
				name: 'yGridlines',
				displayName: 'Y Gridlines',
				type: 'boolean',
				category: 'Axes',
				default: true,
				description: 'Show horizontal gridlines'
			},
			{
				name: 'yMin',
				displayName: 'Y Min',
				type: 'number',
				category: 'Axes',
				description: 'Minimum y-axis value'
			},
			{
				name: 'yMax',
				displayName: 'Y Max',
				type: 'number',
				category: 'Axes',
				description: 'Maximum y-axis value'
			},
			// Labels & Legend
			{
				name: 'labels',
				displayName: 'Value Labels',
				type: 'boolean',
				category: 'Labels',
				default: false,
				description: 'Show value labels on bars'
			},
			{
				name: 'legend',
				displayName: 'Legend',
				type: 'boolean',
				category: 'Labels',
				default: true,
				description: 'Show legend'
			},
			{
				name: 'xFmt',
				displayName: 'X Format',
				type: 'string',
				category: 'Formatting',
				description: 'X-axis format (e.g., usd0k, pct0, num2)'
			},
			{
				name: 'yFmt',
				displayName: 'Y Format',
				type: 'string',
				category: 'Formatting',
				description: 'Y-axis format (e.g., usd0k, pct0, num2)'
			}
		]
	}
};

// Get all available charts
export function getAvailableCharts(): ChartMetadata[] {
	return Object.values(CHART_REGISTRY);
}

// Get chart by name
export function getChart(name: string): ChartMetadata | undefined {
	return CHART_REGISTRY[name];
}

// Get charts by category
export function getChartsByCategory(category: string): ChartMetadata[] {
	return Object.values(CHART_REGISTRY).filter((chart) => chart.category === category);
}

// Get all categories
export function getCategories(): string[] {
	const categories = new Set(Object.values(CHART_REGISTRY).map((chart) => chart.category));
	return Array.from(categories);
}

