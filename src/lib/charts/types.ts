export interface ChartDataPoint {
	group: string;
	value: number;
}

export interface BarClickDetail {
	group: string;
	index: number;
}

export interface ChartOptions {
	title?: string;
	height?: string;
	sortOrder?: 'asc' | 'desc';
	barHeight?: number;
	barGap?: number;
	labelWidth?: number;
	valueFormatter?: (value: number) => string;
}

export type ColorScale = Record<string, string>;

export interface BarChartDimension {
	field: string;
	label?: string;
}

export interface BarChartMetric {
	field: string;
	label?: string;
	aggregate?: 'COUNT' | 'SUM' | 'AVG' | 'MIN' | 'MAX';
}

export interface BarChartConfig {
	id: string;
	table: string;
	dimension: BarChartDimension;
	metric: BarChartMetric;
	orientation?: 'vertical' | 'horizontal';
	filters?: string[];
	sortDirection?: 'ASC' | 'DESC';
	limit?: number;
	clickToFilter?: boolean;
	colors?: string[];
	showValues?: boolean;
	valueFormat?: 'number' | 'currency' | 'percent' | 'compact';
}

export interface BarChartData {
	[key: string]: string | number | null;
}

export type FilterState = Map<string, Set<string>>;
