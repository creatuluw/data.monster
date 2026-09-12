import type { FieldFunction, ColumnInfo } from '$lib/db-operations';

export function typeCategory(colType: string): string {
	const t = colType.toUpperCase();
	return t;
}

export function matchesType(fn: FieldFunction, colType: string): boolean {
	const patterns = fn.applies_to.split(',').map((s) => s.trim().toUpperCase());
	return patterns.some((p) => {
		if (p.endsWith('%')) {
			return colType.toUpperCase().startsWith(p.slice(0, -1));
		}
		return colType.toUpperCase() === p;
	});
}

export function getFunctionsForType(
	functions: FieldFunction[],
	colType: string
): FieldFunction[] {
	return functions.filter((fn) => matchesType(fn, colType));
}

export function quoteName(name: string): string {
	return `"${name.replace(/"/g, '""')}"`;
}

export function generateFunctionSQL(
	fn: FieldFunction,
	columnName: string,
	alias: string
): string {
	const col = quoteName(columnName);
	const aliasQuoted = quoteName(alias);
	let expr = fn.sql_template
		.replace(/{column}/g, col)
		.replace(/{alias}/g, aliasQuoted);
	if (fn.output_type) {
		expr = `CAST(${expr} AS ${fn.output_type})`;
	}
	if (!fn.sql_template.includes('{alias}')) {
		expr += ` AS ${aliasQuoted}`;
	}
	return expr;
}

export interface ActiveFunction {
	columnName: string;
	functionId: string;
}

export function buildSelectClause(
	columns: ColumnInfo[],
	activeFunctions: ActiveFunction[],
	functionDefs: FieldFunction[]
): string {
	const activeByCol = new Map<string, ActiveFunction[]>();
	for (const af of activeFunctions) {
		const list = activeByCol.get(af.columnName) || [];
		list.push(af);
		activeByCol.set(af.columnName, list);
	}

	const fnById = new Map<string, FieldFunction>();
	for (const fn of functionDefs) {
		fnById.set(fn.id, fn);
	}

	const parts: string[] = [];
	for (const col of columns) {
		parts.push(`  ${quoteName(col.name)}`);
		const colActive = activeByCol.get(col.name) || [];
		for (const af of colActive) {
			const fn = fnById.get(af.functionId);
			if (fn) {
				const alias = `${col.name}_${fn.id}`;
				parts.push(`  ${generateFunctionSQL(fn, col.name, alias)}`);
			}
		}
	}

	return parts.join(',\n');
}
