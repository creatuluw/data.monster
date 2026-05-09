import type { ChartDataPoint, ColorScale } from './types';

export const PALETTE = [
	'oklch(0.69 0.16 41)',
	'oklch(0.58 0.18 250)',
	'oklch(0.68 0.14 160)',
	'oklch(0.62 0.17 145)',
	'oklch(0.78 0.09 41)',
	'oklch(0.58 0.2 280)',
	'oklch(0.72 0.13 190)',
	'oklch(0.66 0.16 130)',
	'oklch(0.58 0.17 22)',
	'oklch(0.76 0.13 80)'
];

export const DIMMED = 'oklch(0.85 0.01 250 / 0.4)';

export function aggregate(
	data: Record<string, any>[],
	groupBy: string,
	metric: string,
	sortOrder: 'asc' | 'desc' = 'asc'
): ChartDataPoint[] {
	const map = new Map<string, number>();
	for (const row of data) {
		const key = row[groupBy];
		map.set(key, (map.get(key) ?? 0) + row[metric]);
	}
	return [...map.entries()]
		.map(([group, value]) => ({ group, value: Math.round(value) }))
		.sort((a, b) => (sortOrder === 'asc' ? a.value - b.value : b.value - a.value));
}

export function filterData(
	data: Record<string, any>[],
	groupBy: string,
	selectedGroups: Set<string>
): Record<string, any>[] {
	if (selectedGroups.size === 0) return data;
	return data.filter((row) => selectedGroups.has(row[groupBy]));
}

export function buildColorScale(
	aggregated: ChartDataPoint[],
	selectedGroups: Set<string>
): ColorScale {
	return Object.fromEntries(
		aggregated.map((d, i) => {
			if (selectedGroups.size > 0 && !selectedGroups.has(d.group)) return [d.group, DIMMED];
			return [d.group, PALETTE[i % PALETTE.length]];
		})
	);
}

export function formatCurrency(value: number): string {
	return '$' + value.toLocaleString();
}

export function formatNumber(value: number): string {
	return value.toLocaleString();
}

export function formatPercent(value: number): string {
	return value.toFixed(1) + '%';
}

export function toggleGroup(set: Set<string>, group: string): Set<string> {
	const next = new Set(set);
	if (next.has(group)) {
		next.delete(group);
	} else {
		next.add(group);
	}
	return next;
}
