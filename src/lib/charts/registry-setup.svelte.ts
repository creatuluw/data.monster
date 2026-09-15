/**
 * Registers the chart types with their metadata + renderers (FR-6/7 wiring).
 * Import this module from any route that renders registry charts.
 * Pure metadata duplicates live in tests/registry.test.ts; this module adds
 * the Svelte components on top.
 */
import { registerChartType, type ChartTypeDefinition } from './registry';
import BarChartRenderer from '$lib/components/charts/renderers/BarChartRenderer.svelte';
import HeatmapRenderer from '$lib/components/charts/renderers/HeatmapRenderer.svelte';

let registered = false;

export function setupChartRegistry(): void {
	if (registered) return;
	registered = true;

	const bar: ChartTypeDefinition = {
		type: 'bar',
		label: 'Bar chart',
		roles: [
			{ kind: 'dimension', min: 1, max: 1 },
			{ kind: 'measure', min: 1 }
		],
		optionsSchema: [
			{ name: 'orientation', kind: 'enum', label: 'Orientation', default: 'horizontal', options: ['horizontal', 'vertical'] },
			{ name: 'topN', kind: 'number', label: 'Top N', min: 1 },
			{ name: 'otherLabel', kind: 'string', label: 'Other label', default: 'Other' }
		],
		hooks: { topN: true, grain: true },
		annotations: ['ruleX', 'ruleY'],
		defaults: { orientation: 'horizontal' }
	};

	const heatmap: ChartTypeDefinition = {
		type: 'heatmap',
		label: 'Heatmap',
		roles: [
			{ kind: 'dimension', min: 2, max: 2 },
			{ kind: 'measure', min: 1, max: 1 }
		],
		optionsSchema: [
			{ name: 'scheme', kind: 'enum', label: 'Color scheme', default: 'orrd', options: ['orrd', 'blues', 'greens'] },
			{ name: 'threshold', kind: 'number', label: 'Threshold stops', default: 5, min: 2, max: 9 }
		],
		hooks: { pivot: true, grain: true },
		annotations: [],
		defaults: {}
	};

	registerChartType(bar);
	registerChartType(heatmap);
}

/** Renderer components keyed by chart type (kept out of the pure registry). */
export const chartRenderers: Record<string, typeof BarChartRenderer> = {
	bar: BarChartRenderer,
	heatmap: HeatmapRenderer
};
