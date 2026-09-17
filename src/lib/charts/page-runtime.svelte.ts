/**
 * Page runtime (FR-9): resolves a page doc into per-block query state,
 * runs the canonical queries, and owns the cross-filter selection bus.
 * One $state object + derived functions — no event framework.
 */
import type { Block, PageDoc } from './spec-types';
import { rowColumns } from './spec-types';
import type { TableSchemas } from './query/compile';
import { compileChartQuery, compileTableQuery } from './query/compile';
import { resolveItems, type MasterItem } from './items';
import type { Relationship } from './relationships';
import { buildJoins } from './relationships';
import { createColorScale, type ColorScale } from './color-scale';
import { mergeOptions } from './registry';
import { namedFmt, type FieldFormatter } from './tooltip';
import type { ResolvedAnnotation } from './renderer-types';

export type RunQuery = (sql: string) => Promise<Record<string, unknown>[]>;

export type RuntimeDeps = {
	schemas: TableSchemas;
	items: MasterItem[];
	relationships: Relationship[];
	runQuery: RunQuery;
	palette?: string[];
};

export type PageSelection = { blockId: string; dimension: string; value: string } | null;

export type BlockState = {
	id: string;
	block: Block;
	loading: boolean;
	error: string;
	missing: string[];
	rows: Record<string, unknown>[];
	columns: string[];
	dimensionAliases: string[];
	measureAliases: string[];
	options: Record<string, unknown>;
	annotations: ResolvedAnnotation[];
};

/** Default page palette: DS green led categorical set (Q12-B). */
const DEFAULT_PALETTE = [
	'oklch(0.44 0.1 158)',
	'#2563eb',
	'#d97706',
	'#9333ea',
	'#dc2626',
	'#0891b2',
	'#65a30d',
	'#db2777'
];

/** Which blocks re-query for a selection: every other block whose table carries the column. Pure. */
export function blocksAffectedBySelection(
	blocks: Block[],
	selection: PageSelection,
	schemas: TableSchemas
): Block[] {
	if (!selection) return [];
	return blocks.filter((b) => {
		if (isSource(b, selection.blockId)) return false;
		const table = b.type === 'chart' ? b.chart.source.table : b.type === 'table' ? b.table : null;
		return table !== null && (schemas[table] ?? []).includes(selection.dimension);
	});
}

/** Cross-filter injection for a block, or null when its table lacks the column. Pure. */
export function crossFilterFor(
	block: Block,
	selection: PageSelection,
	schemas: TableSchemas
): { col: string; value: unknown } | null {
	if (!selection) return null;
	const table = block.type === 'chart' ? block.chart.source.table : block.type === 'table' ? block.table : null;
	if (table === null || !(schemas[table] ?? []).includes(selection.dimension)) return null;
	return { col: selection.dimension, value: selection.value };
}

function isSource(block: Block, blockId: string): boolean {
	// blocks flatten with stable ids "r{row}-b{block}" — compare via stored id at call site
	return (block as Block & { __id?: string }).__id === blockId;
}

export function createPageRuntime(doc: PageDoc, deps: RuntimeDeps) {
	const flat =
		doc.rows?.flatMap((row, ri) =>
			rowColumns(row).flatMap((col, ci) =>
				col.blocks.map((block, bi) => ({ ...block, __id: `r${ri}-c${ci}-b${bi}` } as Block & { __id: string }))
			)
		) ?? [];

	let selection = $state<PageSelection>(null);
	let states = $state<Record<string, BlockState>>({});
	const colorScale: ColorScale = createColorScale(deps.palette ?? DEFAULT_PALETTE);

	function initial(block: Block & { __id: string }): BlockState {
		return {
			id: block.__id,
			block,
			loading: true,
			error: '',
			missing: [],
			rows: [],
			columns: [],
			dimensionAliases: [],
			measureAliases: [],
			options: {},
			annotations: []
		};
	}

	async function load(block: Block & { __id: string }): Promise<void> {
		const id = block.__id;
		states[id] = initial(block);
		// re-read: states is $state — only mutations through the proxy are reactive
		const state = states[id]!

		try {
			if (block.type === 'text') {
				state.loading = false;
				return;
			}
			const cross = crossFilterFor(block, selection, deps.schemas);

			if (block.type === 'table') {
				const { sql } = compileTableQuery(block, { schema: deps.schemas, crossFilter: cross });
				state.rows = await deps.runQuery(sql);
				state.columns = block.columns?.length ? block.columns : Object.keys(state.rows[0] ?? {});
				state.loading = false;
				return;
			}

			// chart
			const chart = block.chart;
			const resolved = resolveItems(chart, deps.items);
			if (resolved.missing.length) {
				state.missing = resolved.missing;
				state.loading = false;
				return;
			}
			const joins = buildJoins(chart.source.table, resolved.involvedTables, deps.relationships);
			state.options = mergeOptions(chart);
			const sorted = chart.sort ?? (chart.type === 'bar' ? { by: 'measure[0]', dir: 'desc' as const } : undefined);
			const compiled = compileChartQuery(
				{ ...chart, dimensions: resolved.dimensions, measures: resolved.measures, sort: sorted } as typeof chart,
				{ schema: deps.schemas, joins, crossFilter: cross }
			);
			state.rows = await deps.runQuery(compiled.sql);
			state.dimensionAliases = compiled.dimensionAliases;
			state.measureAliases = compiled.measureAliases;
			state.loading = false;

			// annotations: evaluate expression values as scalars (literal numbers pass through)
			const annotationValues: ResolvedAnnotation[] = [];
			for (const a of chart.annotations ?? []) {
				if (typeof a.at === 'number') annotationValues.push({ mark: a.mark, value: a.at, label: a.label });
				else {
					const scalar = await deps.runQuery(
						`SELECT (${a.at.expr}) AS v FROM "${chart.source.table}"${joins.length ? ' ' + joins.join(' ') : ''} LIMIT 1`
					);
					annotationValues.push({ mark: a.mark, value: Number(scalar[0]?.v ?? 0), label: a.label });
				}
			}
			state.annotations = annotationValues;
		} catch (err) {
			state.loading = false;
			state.error = err instanceof Error ? err.message : String(err);
		}
	}

	async function select(id: string, sel: { dimension: string; value: string } | null): Promise<void> {
		selection = sel ? { blockId: id, ...sel } : null;
		const affected = blocksAffectedBySelection(flat, selection, deps.schemas).filter(
			(b) => !(b as Block & { __id?: string }).__id || !isSource(b, id)
		);
		// reload affected blocks (all non-source blocks that carry the column)
		await Promise.all(affected.map((b) => load(b as Block & { __id: string })));
		// source block itself never re-queries (Qlik-style exclusion) — but if the
		// selection was cleared, the previously filtered blocks already reloaded above
	}

	async function init(): Promise<void> {
		await Promise.all(flat.map(load));
	}

	function fmtsFor(state: BlockState): Record<string, FieldFormatter> {
		const fmts: Record<string, FieldFormatter> = {};
		if (state.block.type !== 'chart') return fmts;
		for (const m of state.block.chart.measures) {
			if (!('ref' in m) && m.fmt) {
				const f = namedFmt(m.fmt);
				if (f) fmts[state.measureAliases[state.block.chart.measures.indexOf(m)] ?? m.label ?? ''] = f;
			}
		}
		for (const field of state.block.chart.tooltip?.fields ?? []) {
			const f = namedFmt(field.fmt);
			if (f) fmts[field.col] = f;
		}
		return fmts;
	}

	return {
		get selection() {
			return selection;
		},
		states,
		colorScale,
		init,
		select,
		fmtsFor,
		blocks: flat
	};
}
