// Main export file for the visualization library
export { BarChart } from './charts/index.js';
export { default as ChartBuilder } from './core/ChartBuilder.svelte';
export {
	CHART_REGISTRY,
	getAvailableCharts,
	getChart,
	getChartsByCategory,
	getCategories,
	type ChartMetadata,
	type PropDefinition
} from './charts/index.js';

