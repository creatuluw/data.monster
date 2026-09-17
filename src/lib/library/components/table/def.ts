/**
 * Table block — built-in /pages block kind (not a chart type; rendered by
 * PageGrid via TableRenderer). Definition is display metadata for /library.
 */
import type { ChartTypeDefinition } from '$lib/charts/registry';

export const tableDefinition: ChartTypeDefinition = {
	type: 'table',
	label: 'Table',
	roles: [], // any columns
	optionsSchema: [{ name: 'limit', kind: 'number', label: 'Row limit', default: 50, min: 1 }],
	hooks: {},
	annotations: [],
	defaults: {}
};
