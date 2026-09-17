/**
 * Bar chart demo data — bundled dummy rows shaped like an aggregated query
 * result (keys = dimension/measure aliases). No workspace or DuckDB needed.
 */
import type { LibraryDemo } from '$lib/library/types';

export const barChartDemo: LibraryDemo = {
	title: 'Sales by brand',
	subtitle: 'Demo data — component library',
	dimensionAliases: ['brand'],
	measureAliases: ['sales'],
	options: {},
	rows: [
		{ brand: 'Apple', sales: 251000 },
		{ brand: 'Samsung', sales: 205000 },
		{ brand: 'Xiaomi', sales: 168000 },
		{ brand: 'Huawei', sales: 117000 },
		{ brand: 'Oppo', sales: 92000 },
		{ brand: 'Vivo', sales: 78000 },
		{ brand: 'Motorola', sales: 49000 },
		{ brand: 'Nothing', sales: 31000 }
	]
};
