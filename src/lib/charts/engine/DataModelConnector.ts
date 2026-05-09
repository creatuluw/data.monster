import type { BarChartConfig, BarChartData, FilterState } from '../types';
import { executeQuery, type QueryResult } from '$lib/db-operations';

export function compileBarChartQuery(config: BarChartConfig, filterState?: FilterState): string {
	const dimension = config.dimension.field;
	const metric = config.metric.field;
	const aggregate = config.metric.aggregate ?? 'SUM';
	const sortDir = config.sortDirection ?? 'DESC';
	const limit = config.limit ?? 30;
	const table = `"${config.table}"`;

	const clauses: string[] = [];

	const configFilters = (config.filters ?? []).filter(Boolean);
	if (configFilters.length > 0) {
		clauses.push(configFilters.join(' AND '));
	}

	if (filterState && filterState.size > 0) {
		for (const [field, values] of filterState) {
			if (values.size > 0) {
				const quoted = [...values].map((v) => `'${String(v).replace(/'/g, "''")}'`).join(', ');
				clauses.push(`"${field}" IN (${quoted})`);
			}
		}
	}

	const where = clauses.length > 0 ? ` WHERE ${clauses.join(' AND ')}` : '';

	return `SELECT "${dimension}", ${aggregate}("${metric}") as value FROM ${table}${where} GROUP BY "${dimension}" ORDER BY value ${sortDir} LIMIT ${limit}`;
}

export async function executeBarChartQuery(
	config: BarChartConfig,
	filterState?: FilterState
): Promise<{ rows: BarChartData[]; columns: string[] }> {
	const sql = compileBarChartQuery(config, filterState);
	const result = await executeQuery(sql);

	if (!('columns' in result)) {
		throw new Error('Unexpected result from bar chart query');
	}

	const { columns, data } = result as QueryResult;
	const rows: BarChartData[] = data.map((row: unknown[]) => {
		const obj: BarChartData = {};
		columns.forEach((col, i) => {
			obj[col] = (row[i] as string | number | null) ?? null;
		});
		return obj;
	});

	return { rows, columns };
}
