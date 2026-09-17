import { describe, it, expect } from 'vitest';
import { validatePageDoc } from '../src/lib/charts/validate';
import type { PageDoc } from '../src/lib/charts/spec-types';
import { rowColumns, normalizePageDoc } from '../src/lib/charts/spec-types';

const validDoc: PageDoc = {
	slug: 'monthly-utilization',
	title: 'Monthly utilization',
	rows: [
		{
			blocks: [
				{
					type: 'chart',
					span: 8,
					chart: {
						type: 'bar',
						source: { table: 'bookings' },
						dimensions: [{ col: 'month', grain: 'month' }],
						measures: [{ ref: 'mi_class_hours' }, { expr: 'sum(hours)', label: 'Total hours', fmt: 'hours' }],
						filters: [{ col: 'year', op: '=', value: 2025 }],
						sort: { by: 'measure[0]', dir: 'desc' },
						limit: 12,
						annotations: [{ mark: 'ruleY', at: { expr: 'avg(hours)' }, label: 'Ø' }],
						tooltip: { title: 'month', fields: [{ col: 'hours', fmt: 'hours' }], template: '{month}: {hours} h' }
					}
				},
				{ type: 'text', span: 4, text: '## Insight' }
			]
		},
		{
			blocks: [
				{
					type: 'chart',
					span: 12,
					chart: {
						type: 'heatmap',
						source: { table: 'bookings' },
						dimensions: [{ col: 'week' }, { col: 'day' }],
						measures: [{ expr: 'sum(hours)', label: 'Hours' }]
					}
				},
				{ type: 'table', span: 12, table: 'bookings', limit: 50 }
			]
		}
	]
};

describe('validatePageDoc — happy paths', () => {
	it('accepts a full valid document', () => {
		expect(validatePageDoc(validDoc)).toEqual([]);
	});

	it('accepts a page with no rows yet', () => {
		expect(validatePageDoc({ slug: 'empty', title: 'Empty', rows: [] })).toEqual([]);
	});
});

describe('validatePageDoc — document level', () => {
	it('rejects a slug that is not a slug', () => {
		const errors = validatePageDoc({ ...validDoc, slug: 'My Page!' });
		expect(errors).toEqual([{ path: 'slug', message: 'slug must be lowercase letters, numbers and hyphens' }]);
	});

	it('rejects a missing title', () => {
		const errors = validatePageDoc({ slug: 'x', title: '' });
		expect(errors.some((e) => e.path === 'title' && /required/.test(e.message))).toBe(true);
	});

	it('returns a single not-an-object error for garbage input instead of throwing', () => {
		const errors = validatePageDoc('nonsense');
		expect(errors.length).toBe(1);
		expect(errors[0].path).toBe('');
	});
});

describe('validatePageDoc — blocks', () => {
	it('rejects an unknown block type', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'video', span: 6 }] }] };
		const errors = validatePageDoc(doc);
		expect(errors[0].path).toBe('rows[0].blocks[0].type');
	});

	it('rejects span 0', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'text', span: 0, text: 'a' }] }] };
		expect(validatePageDoc(doc)[0].path).toBe('rows[0].blocks[0].span');
	});

	it('rejects span above 12', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'text', span: 13, text: 'a' }] }] };
		expect(validatePageDoc(doc)[0].path).toBe('rows[0].blocks[0].span');
	});

	it('rejects a non-integer span', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'text', span: 6.5, text: 'a' }] }] };
		expect(validatePageDoc(doc)[0].path).toBe('rows[0].blocks[0].span');
	});

	it('rejects a text block without text', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'text', span: 6 }] }] };
		expect(validatePageDoc(doc)[0].path).toBe('rows[0].blocks[0].text');
	});

	it('rejects a table block without a table', () => {
		const doc = { slug: 'x', title: 'X', rows: [{ blocks: [{ type: 'table', span: 6 }] }] };
		expect(validatePageDoc(doc)[0].path).toBe('rows[0].blocks[0].table');
	});
});

describe('validatePageDoc — chart blocks', () => {
	const chartBlock = (chart: Record<string, unknown>) => ({
		slug: 'x',
		title: 'X',
		rows: [{ blocks: [{ type: 'chart', span: 6, chart }] }]
	});

	it('rejects an unknown chart type', () => {
		const errors = validatePageDoc(chartBlock({ type: 'funnel', source: { table: 't' }, dimensions: [], measures: [] }));
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.type');
	});

	it('rejects a missing source table', () => {
		const errors = validatePageDoc(chartBlock({ type: 'bar', source: {}, dimensions: [], measures: [] }));
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.source.table');
	});

	it('rejects a missing dimensions array', () => {
		const errors = validatePageDoc(chartBlock({ type: 'bar', source: { table: 't' }, measures: [] }));
		expect(errors.some((e) => e.path === 'rows[0].blocks[0].chart.dimensions')).toBe(true);
	});

	it('rejects a missing measures array', () => {
		const errors = validatePageDoc(chartBlock({ type: 'bar', source: { table: 't' }, dimensions: [] }));
		expect(errors.some((e) => e.path === 'rows[0].blocks[0].chart.measures')).toBe(true);
	});

	it('rejects an empty measure expression', () => {
		const errors = validatePageDoc(
			chartBlock({ type: 'bar', source: { table: 't' }, dimensions: [{ col: 'd' }], measures: [{ expr: '' }] })
		);
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.measures[0].expr');
	});

	it('rejects a measure carrying both ref and expr', () => {
		const errors = validatePageDoc(
			chartBlock({
				type: 'bar',
				source: { table: 't' },
				dimensions: [{ col: 'd' }],
				measures: [{ ref: 'mi_x', expr: 'sum(a)' }]
			})
		);
		expect(errors[0].message).toContain('either ref or expr');
	});

	it('rejects a measure carrying neither ref nor expr', () => {
		const errors = validatePageDoc(
			chartBlock({ type: 'bar', source: { table: 't' }, dimensions: [{ col: 'd' }], measures: [{ label: 'x' }] })
		);
		expect(errors[0].message).toContain('either ref or expr');
	});

	it('rejects a dimension with an invalid grain', () => {
		const errors = validatePageDoc(
			chartBlock({
				type: 'bar',
				source: { table: 't' },
				dimensions: [{ col: 'd', grain: 'fortnight' }],
				measures: [{ expr: 'sum(a)' }]
			})
		);
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.dimensions[0].grain');
	});
});

describe('validatePageDoc — filters', () => {
	const withFilter = (op: string, value: unknown) =>
		validatePageDoc({
			slug: 'x',
			title: 'X',
			rows: [
				{
					blocks: [
						{
							type: 'chart',
							span: 6,
							chart: {
								type: 'bar',
								source: { table: 't' },
								dimensions: [{ col: 'd' }],
								measures: [{ expr: 'sum(a)' }],
								filters: [{ col: 'c', op, value }]
							}
						}
					]
				}
			]
		});

	it('rejects an unknown filter op', () => {
		const errors = withFilter('~~', 1);
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.filters[0].op');
	});

	it('rejects a filter without a value', () => {
		const errors = withFilter('=', undefined);
		expect(errors[0].path).toBe('rows[0].blocks[0].chart.filters[0].value');
	});

	it('rejects a filter without a column', () => {
		const errors = validatePageDoc({
			slug: 'x',
			title: 'X',
			rows: [
				{
					blocks: [
						{
							type: 'chart',
							span: 6,
							chart: {
								type: 'bar',
								source: { table: 't' },
								dimensions: [{ col: 'd' }],
								measures: [{ expr: 'sum(a)' }],
								filters: [{ col: '', op: '=', value: 1 }]
								}
							}
						]
					}
				]
			});
		expect(errors.some((e) => e.path === 'rows[0].blocks[0].chart.filters[0].col')).toBe(true);
	});
});

describe('validatePageDoc — columns', () => {
	const withColumns = (columns: unknown[]) => validatePageDoc({ slug: 'x', title: 'X', rows: [{ columns }] });

	it('accepts a row with explicit columns', () => {
		expect(withColumns([{ span: 6, blocks: [{ type: 'text', text: 'a' }] }, { span: 6, blocks: [] }])).toEqual([]);
	});

	it('rejects a column span above 12', () => {
		expect(withColumns([{ span: 13, blocks: [] }])[0].path).toBe('rows[0].columns[0].span');
	});

	it('rejects a column without a blocks array', () => {
		expect(withColumns([{ span: 6 }])[0].path).toBe('rows[0].columns[0].blocks');
	});

	it('validates blocks inside columns', () => {
		expect(withColumns([{ span: 6, blocks: [{ type: 'text' }] }])[0].path).toBe('rows[0].columns[0].blocks[0].text');
	});

	it('rejects a row with neither columns nor blocks', () => {
		expect(validatePageDoc({ slug: 'x', title: 'X', rows: [{}] })[0].path).toBe('rows[0]');
	});
});

describe('rowColumns — legacy mapping', () => {
	it('maps each legacy block to its own column, keeping spans', () => {
		const cols = rowColumns({ blocks: [{ type: 'text', span: 4, text: 'a' }, { type: 'text', span: 8, text: 'b' }] });
		expect(cols).toEqual([
			{ span: 4, blocks: [{ type: 'text', span: 4, text: 'a' }] },
			{ span: 8, blocks: [{ type: 'text', span: 8, text: 'b' }] }
		]);
	});

	it('defaults a legacy spanless block to a full-width column', () => {
		expect(rowColumns({ blocks: [{ type: 'text', text: 'a' }] })[0].span).toBe(12);
	});

	it('explicit columns pass through untouched', () => {
		const columns = [{ span: 3, blocks: [] }];
		expect(rowColumns({ columns })).toBe(columns);
	});

	it('normalizePageDoc converts every row to columns', () => {
		const doc = normalizePageDoc({
			slug: 'x',
			title: 'X',
			rows: [{ blocks: [{ type: 'text', span: 6, text: 'a' }] }, { columns: [{ span: 12, blocks: [] }] }]
		});
		expect(doc.rows!.every((r) => Array.isArray(r.columns) && r.blocks === undefined)).toBe(true);
		expect(doc.rows![0].columns![0].span).toBe(6);
	});
});

describe('validatePageDoc — row/column height', () => {
	const withColumns = (columns: unknown[]) => validatePageDoc({ slug: 'x', title: 'X', rows: [{ columns }] });
	it('accepts a column height of 40+', () => {
		expect(withColumns([{ span: 6, height: 300, blocks: [] }])).toEqual([]);
	});
	it('rejects a column height below 40', () => {
		expect(withColumns([{ span: 6, height: 20, blocks: [] }])[0].path).toBe('rows[0].columns[0].height');
	});
	it('rejects a non-numeric row height', () => {
		expect(validatePageDoc({ slug: 'x', title: 'X', rows: [{ columns: [], height: 'tall' }] })[0].path).toBe('rows[0].height');
	});
});
