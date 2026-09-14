import { describe, it, expect } from 'vitest';
import { sameDatum } from './fundament';

describe('sameDatum — position-based selection matching', () => {
	type Cell = { mx: number; wy: number; cat: string };
	const mx = (d: Cell) => d.mx;
	const cat = (d: Cell) => d.cat;

	it('matches when all key accessors return equal values', () => {
		const match = sameDatum<Cell>([mx, cat]);
		expect(match({ mx: 3, wy: 1, cat: 'a' }, { mx: 3, wy: 9, cat: 'a' })).toBe(true);
	});

	it('differs when any key accessor differs', () => {
		const match = sameDatum<Cell>([mx, cat]);
		expect(match({ mx: 3, wy: 1, cat: 'a' }, { mx: 4, wy: 1, cat: 'a' })).toBe(false);
		expect(match({ mx: 3, wy: 1, cat: 'a' }, { mx: 3, wy: 1, cat: 'b' })).toBe(false);
	});

	it('matches on grid position, not object identity (svelteplot re-copies records)', () => {
		const match = sameDatum<Cell>([mx]);
		expect(match({ mx: 7, wy: 2, cat: 'x' }, { mx: 7, wy: 2, cat: 'x' })).toBe(true);
	});

	it('returns false when either datum is null', () => {
		const match = sameDatum<Cell>([mx]);
		expect(match(null as unknown as Cell, { mx: 1, wy: 1, cat: 'a' })).toBe(false);
		expect(match({ mx: 1, wy: 1, cat: 'a' }, null as unknown as Cell)).toBe(false);
	});
});

import { buildBars } from './fundament';

describe('buildBars — aggregate rows into bar records', () => {
	type Sale = { region: string; amount: number };
	const rows: Sale[] = [
		{ region: 'EU', amount: 10 },
		{ region: 'US', amount: 30 },
		{ region: 'EU', amount: 5 },
		{ region: 'APAC', amount: 20 },
	];

	it('sums values per category, sorted descending', () => {
		expect(buildBars(rows, (d) => d.region, (d) => d.amount)).toEqual([
			{ category: 'US', value: 30 },
			{ category: 'APAC', value: 20 },
			{ category: 'EU', value: 15 },
		]);
	});

	it('returns empty array for empty rows', () => {
		expect(buildBars([], (d: Sale) => d.region, (d: Sale) => d.amount)).toEqual([]);
	});

	it('caps at topN and lumps the rest into an Other bucket (kept last)', () => {
		expect(buildBars(rows, (d) => d.region, (d) => d.amount, { topN: 1 })).toEqual([
			{ category: 'US', value: 30 },
			{ category: 'Other', value: 35 },
		]);
	});

	it('no Other bucket when categories fit within topN', () => {
		expect(buildBars(rows, (d) => d.region, (d) => d.amount, { topN: 3 })).toEqual([
			{ category: 'US', value: 30 },
			{ category: 'APAC', value: 20 },
			{ category: 'EU', value: 15 },
		]);
	});

	it('honors custom otherLabel', () => {
		expect(buildBars(rows, (d) => d.region, (d) => d.amount, { topN: 1, otherLabel: 'Rest' })).toEqual([
			{ category: 'US', value: 30 },
			{ category: 'Rest', value: 35 },
		]);
	});
});
