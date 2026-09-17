import { describe, it, expect } from 'vitest';
import { dimensionFromPick, measureFromPick, dimensionPickValue, measurePickValue } from '../src/lib/charts/pickers';

describe('pickers — dimension pick codec', () => {
	it('decodes a master ref', () => {
		expect(dimensionFromPick('ref:mi_region', 'bookings')).toEqual({ ref: 'mi_region' });
	});

	it('decodes a source-table field (no table tag)', () => {
		expect(dimensionFromPick('col:bookings:month', 'bookings')).toEqual({ col: 'month' });
	});

	it('decodes a linked-table field (table tag kept for auto-JOIN)', () => {
		expect(dimensionFromPick('col:clients:region', 'bookings')).toEqual({ col: 'region', table: 'clients' });
	});

	it('encodes refs and fields back to pick values', () => {
		expect(dimensionPickValue({ ref: 'mi_x' }, 'bookings')).toBe('ref:mi_x');
		expect(dimensionPickValue({ col: 'month' }, 'bookings')).toBe('col:bookings:month');
		expect(dimensionPickValue({ col: 'region', table: 'clients' }, 'bookings')).toBe('col:clients:region');
	});
});

describe('pickers — measure pick codec', () => {
	it('decodes a master ref', () => {
		expect(measureFromPick('ref:mi_hours', 'bookings')).toEqual({ ref: 'mi_hours' });
	});

	it('decodes a source field into sum(col)', () => {
		expect(measureFromPick('field:bookings:hours', 'bookings')).toEqual({ expr: 'sum(hours)', label: 'sum hours' });
	});

	it('decodes a linked field into a qualified tagged measure', () => {
		expect(measureFromPick('field:clients:spend', 'bookings')).toEqual({
			expr: 'sum("clients"."spend")',
			table: 'clients',
			label: 'sum spend'
		});
	});

	it('encodes refs back; inline exprs encode as custom', () => {
		expect(measurePickValue({ ref: 'mi_x' })).toBe('ref:mi_x');
		expect(measurePickValue({ expr: 'count(*)' })).toBe('custom');
	});
});
