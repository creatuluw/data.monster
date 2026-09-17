import { describe, it, expect, beforeEach } from 'vitest';
import { registerChartType, getChartType, resolveChartBlock, mergeOptions, needsSetup, resetChartTypes } from '../src/lib/charts/registry';
import type { ChartTypeDefinition } from '../src/lib/charts/registry';
import type { ChartBlockSpec } from '../src/lib/charts/spec-types';

const barDef: ChartTypeDefinition = {
	type: 'bar',
	label: 'Bar chart',
	roles: [
		{ kind: 'dimension', min: 1, max: 1 },
		{ kind: 'measure', min: 1 }
	],
	optionsSchema: [
		{ name: 'orientation', kind: 'enum', label: 'Orientation', default: 'horizontal', options: ['horizontal', 'vertical'] },
		{ name: 'topN', kind: 'number', label: 'Top N', min: 1 }
	],
	hooks: { topN: true, grain: true },
	annotations: ['ruleX', 'ruleY'],
	defaults: { orientation: 'horizontal' }
};

const heatmapDef: ChartTypeDefinition = {
	type: 'heatmap',
	label: 'Heatmap',
	roles: [
		{ kind: 'dimension', min: 2, max: 2 },
		{ kind: 'measure', min: 1, max: 1 }
	],
	optionsSchema: [
		{ name: 'scheme', kind: 'enum', label: 'Color scheme', default: 'orrd', options: ['orrd', 'blues'] },
		{ name: 'threshold', kind: 'number', label: 'Threshold', default: 5, min: 1 }
	],
	hooks: { pivot: true },
	annotations: [],
	defaults: {}
};

const chart = (type: string, over: Partial<ChartBlockSpec> = {}): ChartBlockSpec => ({
	type,
	source: { table: 'bookings' },
	dimensions: [{ col: 'month' }],
	measures: [{ expr: 'sum(hours)', label: 'Hours' }],
	...over
});

describe('registry — register & resolve', () => {
	beforeEach(() => {
		resetChartTypes();
		registerChartType(barDef);
		registerChartType(heatmapDef);
	});

	it('resolves a registered chart type', () => {
		const result = resolveChartBlock(chart('bar'));
		expect(result.definition?.type).toBe('bar');
		expect(result.errors).toEqual([]);
	});

	it('reports an unknown chart type', () => {
		const result = resolveChartBlock(chart('funnel'));
		expect(result.errors[0].path).toBe('type');
	});

	it('getChartType returns undefined for unknown types', () => {
		expect(getChartType('funnel')).toBeUndefined();
	});
});

describe('registry — role validation', () => {
	beforeEach(() => {
		resetChartTypes();
		registerChartType(barDef);
		registerChartType(heatmapDef);
	});

	it('rejects a bar chart with no measures', () => {
		const result = resolveChartBlock(chart('bar', { measures: [] }));
		expect(result.errors[0].path).toBe('measures');
		expect(result.errors[0].message).toContain('at least 1');
	});

	it('rejects a bar chart with two dimensions', () => {
		const result = resolveChartBlock(chart('bar', { dimensions: [{ col: 'a' }, { col: 'b' }] }));
		expect(result.errors[0].path).toBe('dimensions');
		expect(result.errors[0].message).toContain('at most 1');
	});

	it('rejects a heatmap with one dimension', () => {
		const result = resolveChartBlock(chart('heatmap'));
		expect(result.errors[0].path).toBe('dimensions');
		expect(result.errors[0].message).toContain('at least 2');
	});

	it('rejects a heatmap with two measures', () => {
		const result = resolveChartBlock(
			chart('heatmap', { dimensions: [{ col: 'a' }, { col: 'b' }], measures: [{ expr: 'sum(a)' }, { expr: 'sum(b)' }] })
		);
		expect(result.errors[0].path).toBe('measures');
		expect(result.errors[0].message).toContain('at most 1');
	});
});

describe('registry — options defaults', () => {
	beforeEach(() => {
		resetChartTypes();
		registerChartType(barDef);
		registerChartType(heatmapDef);
	});

	it('merges schema defaults when options are absent', () => {
		expect(mergeOptions(chart('bar'))).toEqual({ orientation: 'horizontal' });
	});

	it('keeps explicit options over defaults', () => {
		expect(mergeOptions(chart('bar', { options: { orientation: 'vertical', topN: 5 } }))).toEqual({
			orientation: 'vertical',
			topN: 5
		});
	});
});

describe('needsSetup — empty charts stay skeleton until roles are filled', () => {
	it('true when dimensions or measures are below the role minimum', () => {
		expect(needsSetup({ ...chart('bar'), dimensions: [] })).toBe(true);
		expect(needsSetup({ ...chart('bar'), measures: [] })).toBe(true);
	});

	it('false once every role meets its minimum', () => {
		expect(needsSetup(chart('bar'))).toBe(false);
	});

	it('true for unknown chart types with empty roles', () => {
		expect(needsSetup({ ...chart('bar'), type: 'nope', dimensions: [] })).toBe(true);
	});
});
