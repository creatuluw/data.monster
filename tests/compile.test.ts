import { describe, it, expect } from 'vitest';
import { compileChartQuery, compileTableQuery, toGrid } from '../src/lib/charts/query/compile';
import type { ChartBlockSpec, TableBlock } from '../src/lib/charts/spec-types';

const schema = { bookings: ['month', 'week', 'day', 'hours', 'type', 'year', 'region'] };

const bar = (over: Partial<ChartBlockSpec> = {}): ChartBlockSpec => ({
	type: 'bar',
	source: { table: 'bookings' },
	dimensions: [{ col: 'month' }],
	measures: [{ expr: 'sum(hours)', label: 'Total hours' }],
	...over
});

describe('compileChartQuery — canonical shape', () => {
	it('compiles a simple bar chart', () => {
		const { sql } = compileChartQuery(bar(), { schema });
		expect(sql).toBe(
			'SELECT "month", sum(hours) AS "Total hours" FROM "bookings" GROUP BY "month"'
		);
	});

	it('applies date grains via date_trunc', () => {
		const { sql } = compileChartQuery(
			bar({ dimensions: [{ col: 'month', grain: 'month' }] }),
			{ schema }
		);
		expect(sql).toContain("date_trunc('month', \"month\") AS \"month\"");
	});

	it('applies seasonality grains via strftime', () => {
		const { sql } = compileChartQuery(
			bar({ dimensions: [{ col: 'month', grain: 'day_of_week' }] }),
			{ schema }
		);
		expect(sql).toContain("strftime('%w', \"month\") AS \"month\"");
	});

	it('aliases unlabeled measures as m0, m1, …', () => {
		const { sql } = compileChartQuery(bar({ measures: [{ expr: 'sum(hours)' }] }), { schema });
		expect(sql).toContain('sum(hours) AS "m0"');
	});

	it('sorts by measure index and direction', () => {
		const { sql } = compileChartQuery(bar({ sort: { by: 'measure[0]', dir: 'asc' } }), { schema });
		expect(sql.trim().endsWith('ORDER BY "Total hours" ASC')).toBe(true);
	});

	it('applies the limit', () => {
		const { sql } = compileChartQuery(bar({ limit: 12 }), { schema });
		expect(sql.trim().endsWith('LIMIT 12')).toBe(true);
	});
});

describe('compileChartQuery — filters', () => {
	it('escapes single quotes in string values', () => {
		const { sql } = compileChartQuery(
			bar({ filters: [{ col: 'region', op: '=', value: "O'Brien" }] }),
			{ schema }
		);
		expect(sql).toContain(`WHERE "region" = 'O''Brien'`);
	});

	it('renders IN filters with a value list', () => {
		const { sql } = compileChartQuery(
			bar({ filters: [{ col: 'region', op: 'in', value: ['NL', 'BE'] }] }),
			{ schema }
		);
		expect(sql).toContain(`"region" IN ('NL', 'BE')`);
	});

	it('renders LIKE filters', () => {
		const { sql } = compileChartQuery(
			bar({ filters: [{ col: 'region', op: 'like', value: 'A%' }] }),
			{ schema }
		);
		expect(sql).toContain(`"region" LIKE 'A%'`);
	});

	it('ANDs a cross-filter with existing filters', () => {
		const { sql } = compileChartQuery(
			bar({ filters: [{ col: 'year', op: '=', value: 2025 }] }),
			{ schema, crossFilter: { col: 'region', value: 'NL' } }
		);
		expect(sql).toContain(`WHERE "year" = 2025 AND "region" = 'NL'`);
	});

	it('adds a lone WHERE for a cross-filter without filters', () => {
		const { sql } = compileChartQuery(bar(), { schema, crossFilter: { col: 'region', value: 5 } });
		expect(sql).toContain(`WHERE "region" = 5`);
	});
});

describe('compileChartQuery — topN + Other', () => {
	it('wraps in a CASE/row_number bucket query', () => {
		const { sql } = compileChartQuery(
			bar({ options: { topN: 3 }, sort: { by: 'measure[0]', dir: 'desc' } }),
			{ schema }
		);
		expect(sql).toContain('row_number() OVER (ORDER BY "Total hours" DESC) AS rnk');
		expect(sql).toContain('CASE WHEN rnk <= 3 THEN "month" ELSE \'Other\' END AS "month"');
		expect(sql).toContain('GROUP BY 1');
	});
});

describe('compileChartQuery — schema guard', () => {
	it('rejects an unknown dimension column', () => {
		expect(() =>
			compileChartQuery(bar({ dimensions: [{ col: 'month; DROP TABLE x' }] }), { schema })
		).toThrow(/unknown column/i);
	});

	it('rejects an unknown filter column', () => {
		expect(() =>
			compileChartQuery(bar({ filters: [{ col: 'evil', op: '=', value: 1 }] }), { schema })
		).toThrow(/unknown column/i);
	});

	it('rejects an unknown source table', () => {
		expect(() => compileChartQuery(bar({ source: { table: 'nope' } }), { schema })).toThrow(
			/unknown table/i
		);
	});
});

describe('compileTableQuery', () => {
	const table: TableBlock = { type: 'table', span: 12, table: 'bookings', limit: 50 };

	it('selects all columns without aggregation', () => {
		const { sql } = compileTableQuery(table, { schema });
		expect(sql).toBe('SELECT * FROM "bookings" LIMIT 50');
	});

	it('selects explicit columns', () => {
		const { sql } = compileTableQuery({ ...table, columns: ['month', 'hours'] }, { schema });
		expect(sql).toBe('SELECT "month", "hours" FROM "bookings" LIMIT 50');
	});

	it('applies filters and sorting without GROUP BY', () => {
		const { sql } = compileTableQuery(
			{ ...table, filters: [{ col: 'year', op: '=', value: 2025 }], sort: { by: 'hours', dir: 'desc' } },
			{ schema }
		);
		expect(sql).toBe('SELECT * FROM "bookings" WHERE "year" = 2025 ORDER BY "hours" DESC LIMIT 50');
		expect(sql).not.toContain('GROUP BY');
	});
});

describe('toGrid — heatmap pivot', () => {
	const rows = [
		{ x: 'a', y: '1', v: 1 },
		{ x: 'b', y: '2', v: 2 },
		{ x: 'a', y: '2', v: 3 }
	];

	it('builds first-seen ordered axes and a cell map', () => {
		const grid = toGrid(rows, (r) => r.x, (r) => r.y, (r) => r.v);
		expect(grid.xValues).toEqual(['a', 'b']);
		expect(grid.yValues).toEqual(['1', '2']);
		expect(grid.get('a', '2')).toBe(3);
		expect(grid.get('b', '1')).toBeUndefined();
	});

	it('handles empty rows', () => {
		const grid = toGrid<Record<string, never>, number>([], () => '', () => '', () => 0);
		expect(grid.xValues).toEqual([]);
		expect(grid.yValues).toEqual([]);
	});
});

describe('compileChartQuery — linked-table dimensions', () => {
	const linkedSchema = {
		bookings: ['month', 'hours'],
		clients: ['region', 'spend']
	};

	it('qualifies a linked-table dimension column and groups by the qualified name', () => {
		const { sql } = compileChartQuery(
			bar({ dimensions: [{ col: 'region', table: 'clients' }] }),
			{ schema: linkedSchema }
		);
		expect(sql).toContain('"clients"."region"');
		expect(sql).toContain('GROUP BY "clients"."region"');
	});

	it('rejects a linked-table column missing on its own table', () => {
		expect(() =>
			compileChartQuery(bar({ dimensions: [{ col: 'nope', table: 'clients' }] }), { schema: linkedSchema })
		).toThrow(/unknown column "nope" on table "clients"/);
	});
});
