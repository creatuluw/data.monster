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
