/**
 * Central chart system — page spec types (FR-1).
 * The page document is the single source of truth edited by both surfaces
 * (Design inspector ⇄ Code JSON). Everything here is plain serializable data.
 */

export type Grain =
	| 'hour'
	| 'day'
	| 'week'
	| 'month'
	| 'quarter'
	| 'year'
	| 'day_of_week'
	| 'month_of_year';

export type FilterOp = '=' | '!=' | '>' | '<' | '>=' | '<=' | 'in' | 'like';

/** A dimension: table column (optional grain, optional linked-table binding) or a master-dimension reference. */
/** A dimension: a column (optionally from a linked table) with optional time grain,
 *  a resolved master-item expression (raw: true — authored in ExprEditor, not a column),
 *  or a master-item reference pre-resolution. */
export type DimensionSpec = { col: string; table?: string; grain?: Grain; label?: string; raw?: boolean } | { ref: string };

/** A measure: DuckDB expression (aggregation included, optional linked-table binding) or a master-measure reference. */
export type MeasureSpec = { expr: string; table?: string; label?: string; fmt?: string } | { ref: string };

export type FilterSpec = { col: string; op: FilterOp; value: unknown };

export type SortSpec = { by: string; dir: 'asc' | 'desc' };

export type AnnotationMark = 'arrow' | 'dot' | 'line' | 'ruleX' | 'ruleY' | 'text' | 'rect';

export type AnnotationSpec = {
	mark: AnnotationMark;
	at: number | { expr: string };
	label?: string;
	color?: string;
};

export type TooltipSpec = {
	title?: string;
	fields?: { col: string; label?: string; fmt?: string }[];
	template?: string;
};

export type AxisOptions = {
	title?: string | null;
	gridlines?: boolean;
	ticks?: boolean;
	labels?: boolean;
	min?: number;
	max?: number;
	fitToData?: boolean;
	rotate?: number;
};

export type ChartBlockSpec = {
	type: string;
	source: { table: string };
	dimensions: DimensionSpec[];
	measures: MeasureSpec[];
	filters?: FilterSpec[];
	sort?: SortSpec;
	limit?: number;
	options?: Record<string, unknown>;
	annotations?: AnnotationSpec[];
	tooltip?: TooltipSpec;
	axes?: { x?: AxisOptions; y?: AxisOptions };
	legend?: { show?: boolean; position?: 'top' | 'bottom' };
	title?: string;
	subtitle?: string;
	heightVh?: number;
};

export type ChartBlock = { type: 'chart'; span?: number; chart: ChartBlockSpec };

export type TableBlock = {
	type: 'table';
	span?: number;
	table: string;
	columns?: string[];
	filters?: FilterSpec[];
	sort?: SortSpec;
	limit?: number;
	title?: string;
};

export type TextBlock = { type: 'text'; span?: number; text: string };

export type Block = ChartBlock | TableBlock | TextBlock;

export type PageColumn = { span?: number; height?: number; blocks: Block[] };

/** New shape: explicit columns that own the horizontal split. Legacy `blocks` kept valid. */
export type PageRow = { blocks?: Block[]; columns?: PageColumn[]; height?: number };

export type PageDoc = { slug: string; title: string; rows?: PageRow[] };

/** Row → columns. Legacy rows (flat blocks) map each block to its own column so old side-by-side spans keep rendering. */
export function rowColumns(row: PageRow): PageColumn[] {
	if (row.columns) return row.columns;
	return (row.blocks ?? []).map((b) => ({ span: b.span ?? 12, blocks: [b] }));
}

/** Convert every row to the explicit-columns shape (used at load/apply so editing is uniform). */
export function normalizePageDoc(doc: PageDoc): PageDoc {
	return { ...doc, rows: (doc.rows ?? []).map((row) => ({ height: row.height, columns: rowColumns(row) })) };
}

export type ValidationError = { path: string; message: string };
