/**
 * Heatmap — central-charts type definition (moved from registry-setup).
 */
import type { ChartTypeDefinition } from '$lib/charts/registry';

export const heatmapDefinition: ChartTypeDefinition = {
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
