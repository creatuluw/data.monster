/**
 * Text block — built-in /pages block kind (PageGrid renders it inline).
 */
import type { ChartTypeDefinition } from '$lib/charts/registry';

export const textDefinition: ChartTypeDefinition = {
	type: 'text',
	label: 'Text',
	roles: [],
	optionsSchema: [],
	hooks: {},
	annotations: [],
	defaults: {}
};
