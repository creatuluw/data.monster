/**
 * Pick-value codec shared by the block inspector and the in-chart skeleton setup:
 * encodes/decodes dimension/measure picker options (`ref:<id>` | `col:<table>:<col>`
 * | `field:<table>:<col>`) into chart spec entries.
 */
import type { DimensionSpec, MeasureSpec } from './spec-types';

export function dimensionFromPick(v: string, sourceTable: string): DimensionSpec {
	if (v.startsWith('ref:')) return { ref: v.slice(4) };
	const [, table, col] = v.split(':');
	return table === sourceTable ? { col } : { col, table };
}

export function dimensionPickValue(d: DimensionSpec, sourceTable: string): string {
	if ('ref' in d) return `ref:${d.ref}`;
	return `col:${d.table ?? sourceTable}:${d.col}`;
}

export function measureFromPick(v: string, sourceTable: string): MeasureSpec {
	if (v.startsWith('ref:')) return { ref: v.slice(4) };
	const [, table, col] = v.split(':');
	const linked = table !== sourceTable;
	return {
		expr: linked ? `sum("${table}"."${col}")` : `sum(${col})`,
		table: linked ? table : undefined,
		label: `sum ${col}`
	};
}

export function measurePickValue(m: MeasureSpec): string {
	return 'ref' in m ? `ref:${m.ref}` : 'custom';
}
