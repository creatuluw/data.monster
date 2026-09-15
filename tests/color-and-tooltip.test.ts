import { describe, it, expect } from 'vitest';
import { createColorScale } from '../src/lib/charts/color-scale';
import { resolveTooltip } from '../src/lib/charts/tooltip';

describe('createColorScale — page-consistent assignment', () => {
	it('assigns palette colors in first-seen order', () => {
		const scale = createColorScale(['green', 'blue', 'red']);
		expect(scale.colorOf('region', 'AMS')).toBe('green');
		expect(scale.colorOf('region', 'RTM')).toBe('blue');
	});

	it('is stable for the same dimension+value across calls', () => {
		const scale = createColorScale(['green', 'blue', 'red']);
		expect(scale.colorOf('region', 'AMS')).toBe(scale.colorOf('region', 'AMS'));
	});

	it('keeps assignments independent per dimension', () => {
		const scale = createColorScale(['green', 'blue']);
		expect(scale.colorOf('region', 'X')).toBe('green');
		expect(scale.colorOf('category', 'X')).toBe('green'); // separate dimension, own first-seen
		expect(scale.colorOf('region', 'Y')).toBe('blue');
	});

	it('wraps around the palette when values exceed its length', () => {
		const scale = createColorScale(['green']);
		expect(scale.colorOf('d', 'a')).toBe('green');
		expect(scale.colorOf('d', 'b')).toBe('green');
	});

	it('lets explicit overrides win', () => {
		const scale = createColorScale(['green'], { seriesColors: { AMS: 'red' } });
		expect(scale.colorOf('region', 'AMS')).toBe('red');
		expect(scale.colorOf('region', 'RTM')).toBe('green');
	});
});

describe('resolveTooltip — template + fmt', () => {
	it('replaces {field} placeholders with row values', () => {
		const out = resolveTooltip('{month}: {hours} h', { month: 'May', hours: 122 });
		expect(out).toBe('May: 122 h');
	});

	it('applies per-field formatters', () => {
		const out = resolveTooltip('{month}: {hours}', { month: 'May', hours: 122.4 }, {
			hours: (v) => `${Math.round(v as number)}h`
		});
		expect(out).toBe('May: 122h');
	});

	it('renders an empty string for missing fields', () => {
		const out = resolveTooltip('{a} / {b}', { a: 1 });
		expect(out).toBe('1 / ');
	});

	it('returns the raw template for null rows', () => {
		expect(resolveTooltip('{a}', null)).toBe('{a}');
	});
});
