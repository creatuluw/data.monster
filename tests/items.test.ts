import { describe, it, expect } from 'vitest';
import { resolveItems } from '../src/lib/charts/items';
import type { MasterItem } from '../src/lib/charts/items';
import type { ChartBlockSpec } from '../src/lib/charts/spec-types';

const library: MasterItem[] = [
	{ id: 'mi_class_hours', kind: 'measure', table: 'bookings', label: 'Class hours', expr: "sum(if(type = 'class', hours))", fmt: 'hours' },
	{ id: 'mi_region', kind: 'dimension', table: 'bookings', label: 'Region', expr: 'region' },
	{ id: 'mi_client_name', kind: 'dimension', table: 'clients', label: 'Client', expr: 'client_name' }
];

const chart = (over: Partial<ChartBlockSpec> = {}): ChartBlockSpec => ({
	type: 'bar',
	source: { table: 'bookings' },
	dimensions: [{ ref: 'mi_region' }],
	measures: [{ ref: 'mi_class_hours' }],
	...over
});

describe('resolveItems — ref vs inline', () => {
	it('resolves measure refs to expressions with label and fmt', () => {
		const { dimensions, measures, missing } = resolveItems(chart(), library);
		expect(measures).toEqual([{ expr: "sum(if(type = 'class', hours))", label: 'Class hours', fmt: 'hours' }]);
		expect(missing).toEqual([]);
		expect(dimensions[0].col).toBe('region');
		expect(dimensions[0].label).toBe('Region');
	});

	it('passes inline entries through unchanged', () => {
		const { measures } = resolveItems(
			chart({ measures: [{ expr: 'sum(hours)', label: 'Total' }] }),
			library
		);
		expect(measures).toEqual([{ expr: 'sum(hours)', label: 'Total' }]);
	});

	it('collects missing refs instead of throwing', () => {
		const { missing, measures } = resolveItems(chart({ measures: [{ ref: 'mi_gone' }] }), library);
		expect(missing).toEqual(['mi_gone']);
		expect(measures).toEqual([]);
	});
});
