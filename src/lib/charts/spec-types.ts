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

/** A dimension: table column (optional grain) or a master-dimension reference. */
export type DimensionSpec = { col: string; grain?: Grain; label?: string } | { ref: string };

/** A measure: DuckDB expression (aggregation included) or a master-measure reference. */
export type MeasureSpec = { expr: string; label?: string; fmt?: string } | { ref: string };

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

export type PageRow = { blocks: Block[] };

export type PageDoc = { slug: string; title: string; rows?: PageRow[] };

export type ValidationError = { path: string; message: string };
