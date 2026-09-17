/**
 * Heatmap demo data — bundled dummy rows shaped like a pivoted query result
 * (two dimension aliases + one measure alias). No workspace or DuckDB needed.
 */
import type { LibraryDemo } from '$lib/library/types';

const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'];
const shifts = ['Morning', 'Afternoon', 'Evening', 'Night'];

const rows = days.flatMap((day) =>
	shifts.map((shift) => ({
		day,
		shift,
		tickets: Math.round(20 + Math.abs(days.indexOf(day) - shifts.indexOf(shift)) * 9 + ((day + shift).length % 7))
	}))
);

export const heatmapDemo: LibraryDemo = {
	title: 'Support tickets by day and shift',
	subtitle: 'Demo data — component library',
	dimensionAliases: ['day', 'shift'],
	measureAliases: ['tickets'],
	options: {},
	rows
};
