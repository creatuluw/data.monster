/**
 * Bar chart — central-charts type definition (moved from registry-setup).
 * roles/optionsSchema drive the /pages inspector and query compilation.
 */
import type { ChartTypeDefinition } from '$lib/charts/registry';

export const barChartDefinition: ChartTypeDefinition = {
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
