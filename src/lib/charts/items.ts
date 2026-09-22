/**
 * Master-item resolution (FR-14). The workspace library lives in
 * d8a_monster_items (Rust commands, items.rs); this module resolves chart
 * spec refs against the loaded library. Missing refs are collected, never
 * thrown — blocks render a missing-master-item state.
 */
import type { ChartBlockSpec, DimensionSpec, MeasureSpec } from './spec-types';

export type MasterItem = {
	id: string;
	kind: 'measure' | 'dimension';
	table: string;
	label: string;
	expr: string;
	fmt?: string;
	description?: string;
};

export type ResolvedDimension = { col: string; table?: string; label?: string; grain?: string; raw?: boolean };
export type ResolvedMeasure = { expr: string; table?: string; label?: string; fmt?: string };

export type ResolvedItems = {
	dimensions: ResolvedDimension[];
	measures: ResolvedMeasure[];
	/** Master-item ids referenced by the chart but absent from the library. */
	missing: string[];
	/** Tables beyond the source table that the resolved items pull in (auto-JOIN input). */
	involvedTables: string[];
};

export function resolveItems(chart: ChartBlockSpec, library: MasterItem[]): ResolvedItems {
	const byId = new Map(library.map((item) => [item.id, item]));
	const missing: string[] = [];
	const dimensions: ResolvedDimension[] = [];
	const measures: ResolvedMeasure[] = [];
	const involved = new Set<string>();

	for (const d of chart.dimensions) {
		if (!('ref' in d)) {
			dimensions.push({ col: d.col, table: d.table, label: d.label, grain: d.grain as string | undefined });
			if (d.table && d.table !== chart.source.table) involved.add(d.table);
			continue;
		}
		const item = byId.get(d.ref);
		if (!item || item.kind !== 'dimension') {
			missing.push(d.ref);
			continue;
		}
		dimensions.push({ col: item.expr, label: item.label, raw: true });
		if (item.table !== chart.source.table) involved.add(item.table);
	}

	for (const m of chart.measures) {
		if (!('ref' in m)) {
			measures.push({ expr: m.expr, table: m.table, label: m.label, fmt: m.fmt });
			if (m.table && m.table !== chart.source.table) involved.add(m.table);
			continue;
		}
		const item = byId.get(m.ref);
		if (!item || item.kind !== 'measure') {
			missing.push(m.ref);
			continue;
		}
		measures.push({ expr: item.expr, label: item.label, fmt: item.fmt });
		if (item.table !== chart.source.table) involved.add(item.table);
	}

	return { dimensions, measures, missing, involvedTables: [...involved] };
}
