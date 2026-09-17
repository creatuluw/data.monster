/**
 * Pick-value codec shared by the block inspector and the in-chart skeleton setup:
 * encodes/decodes dimension/measure picker options (`ref:<id>` | `col:<table>:<col>`
 * | `field:<table>:<col>`) into chart spec entries.
 */
import type { ChartBlockSpec, DimensionSpec, MeasureSpec } from './spec-types';
import type { MasterItem } from './items';

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

/** Display label for an assigned chart member: refs resolve via master items, raw fields show col/expr (+ linked-table meta). */
export function roleLabels(
	chart: ChartBlockSpec,
	kind: 'dimension' | 'measure',
	items: MasterItem[]
): { label: string; meta: string }[] {
	const table = chart.source.table;
	const members = kind === 'dimension' ? chart.dimensions : chart.measures;
	return members.map((m) => {
		if ('ref' in m) {
			const it = items.find((i) => i.id === m.ref);
			return { label: `⭐ ${it?.label ?? m.ref}`, meta: it?.table ?? table };
		}
		if (kind === 'dimension') {
			const d = m as { col: string; table?: string; label?: string };
			return { label: d.label ?? d.col, meta: d.table ? `${d.table} ⤳` : table };
		}
		const x = m as { expr: string; table?: string; label?: string };
		return { label: x.label ?? x.expr, meta: x.table ? `${x.table} ⤳` : table };
	});
}
