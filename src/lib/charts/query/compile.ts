/**
 * Canonical query compiler (FR-2). Pure SQL string builder — never executes.
 * Trust boundary: dimension/filter columns are validated against the table
 * schema; filter literals are escaped; measure expressions are trusted SQL
 * by design (authored in the editors). Auto-JOIN clauses (FR-15) are passed
 * in pre-built via opts.joins.
 */
import type { ChartBlockSpec, DimensionSpec, FilterSpec, Grain, TableBlock } from '../spec-types';

export type TableSchemas = Record<string, string[]>;

export type CompileOptions = {
	schema: TableSchemas;
	/** Pre-built JOIN clauses (FR-15 auto-JOIN), appended after FROM. */
	joins?: string[];
	crossFilter?: { col: string; value: unknown } | null;
};

export type Grid<T> = {
	xValues: string[];
	yValues: string[];
	get: (x: string, y: string) => T | undefined;
};

const TEMPORAL_GRAINS: Record<string, string> = {
	hour: 'hour',
	day: 'day',
	week: 'week',
	month: 'month',
	quarter: 'quarter',
	year: 'year'
};
const SEASONAL_GRAINS: Record<string, string> = {
	day_of_week: '%w',
	month_of_year: '%m'
};

const ident = (name: string): string => `"${name.replaceAll('"', '""')}"`;

const literal = (value: unknown): string => {
	if (typeof value === 'number') return String(value);
	if (typeof value === 'boolean') return value ? 'TRUE' : 'FALSE';
	if (Array.isArray(value)) return `(${value.map(literal).join(', ')})`;
	const s = String(value).replaceAll("'", "''");
	return `'${s}'`;
};

function checkColumn(schema: TableSchemas, table: string, col: string, path: string): void {
	const cols = schema[table];
	if (!cols) throw new Error(`unknown table "${table}" (${path})`);
	if (!cols.includes(col)) throw new Error(`unknown column "${col}" on table "${table}" (${path})`);
}

function dimSelect(d: DimensionSpec, i: number, schema: TableSchemas, table: string): string {
	if ('ref' in d) throw new Error('unresolved master-item ref in dimensions — resolve first (FR-14)');
	// linked-table field: validate + qualify against its own table
	const dimTable = 'table' in d && d.table ? d.table : table;
	const alias = ident(d.label ?? d.col);
	let col: string;
	if (d.raw) {
		// resolved master-item expression dim — authored in ExprEditor, same
		// trust level as measures (which compile raw); not a column name
		col = `(${d.col})`;
	} else {
		checkColumn(schema, dimTable, d.col, `dimensions[${i}]`);
		col = dimTable !== table ? `${ident(dimTable)}.${ident(d.col)}` : ident(d.col);
	}
	if (d.grain === undefined) return d.label ? `${col} AS ${alias}` : col;
	const temporal = TEMPORAL_GRAINS[d.grain as keyof typeof TEMPORAL_GRAINS];
	if (temporal) return `date_trunc('${temporal}', ${col}) AS ${alias}`;
	const fmt = SEASONAL_GRAINS[d.grain as keyof typeof SEASONAL_GRAINS];
	return `strftime('${fmt}', ${col}) AS ${alias}`;
}

function dimAlias(d: DimensionSpec): string {
	return 'ref' in d ? '' : d.label ?? d.col;
}

function whereClause(filters: FilterSpec[] | undefined, cross: CompileOptions['crossFilter'], schema: TableSchemas, table: string): string {
	const parts: string[] = [];
	[...(filters ?? []), ...(cross ? [{ col: cross.col, op: '=' as const, value: cross.value }] : [])].forEach(
		(f) => {
			checkColumn(schema, table, f.col, 'filters');
			const op = f.op === 'in' ? 'IN' : f.op === 'like' ? 'LIKE' : f.op;
			parts.push(`${ident(f.col)} ${op} ${literal(f.value)}`);
		}
	);
	return parts.length ? ` WHERE ${parts.join(' AND ')}` : '';
}

function orderBy(sort: ChartBlockSpec['sort'], dims: DimensionSpec[], measureAliases: string[]): string | null {
	if (!sort) return null;
	let expr: string | null = null;
	const m = /^measure\[(\d+)\]$/.exec(sort.by);
	if (m) expr = ident(measureAliases[Number(m[1])] ?? '');
	else {
		const d = /^dimension\[(\d+)\]$/.exec(sort.by);
		if (d && dims[Number(d[1])] && !('ref' in dims[Number(d[1])])) expr = ident(dimAlias(dims[Number(d[1])]));
		else expr = ident(sort.by);
	}
	return ` ORDER BY ${expr} ${sort.dir === 'asc' ? 'ASC' : 'DESC'}`;
}

export function compileChartQuery(chart: ChartBlockSpec, opts: CompileOptions): { sql: string; dimensionAliases: string[]; measureAliases: string[] } {
	const table = chart.source.table;
	if (!opts.schema[table]) throw new Error(`unknown table "${table}" (source)`);

	const dimSql = chart.dimensions.map((d, i) => dimSelect(d, i, opts.schema, table));
	const dimensionAliases = chart.dimensions.map(dimAlias);
	const measureAliases = chart.measures.map((m, i) =>
		'ref' in m ? `r${i}` : (m.label ?? `m${i}`)
	);
	if (chart.measures.some((m) => 'ref' in m))
		throw new Error('unresolved master-item ref in measures — resolve first (FR-14)');
	const measureSql = chart.measures.map((m, i) => `${(m as { expr: string }).expr} AS ${ident(measureAliases[i])}`);

	const where = whereClause(chart.filters, opts.crossFilter, opts.schema, table);
	const joins = opts.joins?.length ? ' ' + opts.joins.join(' ') : '';
	const order = orderBy(chart.sort, chart.dimensions, measureAliases) ?? '';
	const limit = chart.limit !== undefined ? ` LIMIT ${chart.limit}` : '';

	const topN = typeof chart.options?.topN === 'number' ? chart.options.topN : null;

	if (topN !== null && dimensionAliases.length === 1) {
		const dim = ident(dimensionAliases[0]);
		const measure = ident(measureAliases[0]);
		const innerOrder = order || ` ORDER BY ${measure} DESC`;
		const inner = `SELECT ${dimSql.join(', ')}, ${measureSql.join(', ')}, row_number() OVER (${innerOrder.replace(' ORDER BY ', 'ORDER BY ')}) AS rnk FROM ${ident(table)}${joins}${where} GROUP BY ${dimSql.map((d) => d.split(' AS ')[0]).join(', ')}`;
		const outerMeasures = measureSql
			.map((_, i) => `sum(${ident(measureAliases[i])}) AS ${ident(measureAliases[i])}`)
			.join(', ');
		return {
			sql: `SELECT CASE WHEN rnk <= ${topN} THEN ${dim} ELSE 'Other' END AS ${dim}, ${outerMeasures} FROM (${inner}) GROUP BY 1 ORDER BY 2 DESC${limit}`,
			dimensionAliases,
			measureAliases
		};
	}

	const sql = `SELECT ${[...dimSql, ...measureSql].join(', ')} FROM ${ident(table)}${joins}${where} GROUP BY ${dimSql.map((d) => d.split(' AS ')[0]).join(', ')}${order}${limit}`;
	return { sql, dimensionAliases, measureAliases };
}

export function compileTableQuery(block: TableBlock, opts: CompileOptions): { sql: string } {
	const table = block.table;
	const cols = opts.schema[table];
	if (!cols) throw new Error(`unknown table "${table}" (table block)`);
	const select = block.columns?.length
		? block.columns.map((c) => {
				checkColumn(opts.schema, table, c, 'columns');
				return ident(c);
			}).join(', ')
		: '*';
	const where = whereClause(block.filters, opts.crossFilter, opts.schema, table);
	const order = block.sort ? ` ORDER BY ${ident(block.sort.by)} ${block.sort.dir === 'asc' ? 'ASC' : 'DESC'}` : '';
	const limit = ` LIMIT ${block.limit ?? 50}`;
	return { sql: `SELECT ${select} FROM ${ident(table)}${where}${order}${limit}` };
}

/** Heatmap pivot (client-side): long rows → first-seen-ordered axes + cell map. */
export function toGrid<Row, V>(
	rows: Row[],
	x: (r: Row) => string,
	y: (r: Row) => string,
	v: (r: Row) => V
): Grid<V> {
	const xValues: string[] = [];
	const yValues: string[] = [];
	const cells = new Map<string, V>();
	for (const row of rows) {
		const xv = x(row);
		const yv = y(row);
		if (!xValues.includes(xv)) xValues.push(xv);
		if (!yValues.includes(yv)) yValues.push(yv);
		cells.set(`${xv}\u0000${yv}`, v(row));
	}
	return { xValues, yValues, get: (a, b) => cells.get(`${a}\u0000${b}`) };
}
