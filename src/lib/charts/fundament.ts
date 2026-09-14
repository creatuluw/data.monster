/**
 * Chart fundament — the shared, tested core for all /labs chart components.
 *
 * Conventions every chart built on this fundament follows:
 * - accessor-based props API (generic over the caller's row type)
 * - selection is matched by grid/category position, never object identity
 *   (svelteplot re-copies records on each transform pass)
 * - data prep (aggregation, sorting) happens in pure functions here,
 *   chart components stay thin renderers
 */

/** Predicate: do two data share the same position on every key accessor? */
export function sameDatum<T>(keys: ((d: T) => unknown)[]): (a: T, b: T) => boolean {
	return (a, b) => {
		if (a == null || b == null) return false;
		return keys.every((k) => k(a) === k(b));
	};
}

export interface BarDatum {
	category: string;
	value: number;
}

/** Aggregate rows into sorted bar records; optionally cap at topN with an Other bucket. */
export function buildBars<T>(
	rows: T[],
	category: (d: T) => string,
	value: (d: T) => number,
	opts?: { topN?: number; otherLabel?: string }
): BarDatum[] {
	const sums = new Map<string, number>();
	for (const d of rows) {
		const c = category(d);
		sums.set(c, (sums.get(c) ?? 0) + value(d));
	}
	const bars: BarDatum[] = [...sums.entries()]
		.map(([category, value]) => ({ category, value }))
		.sort((a, b) => b.value - a.value);
	const topN = opts?.topN;
	if (topN != null && bars.length > topN) {
		const rest = bars.splice(topN);
		const other = rest.reduce((s, b) => s + b.value, 0);
		bars.push({ category: opts?.otherLabel ?? 'Other', value: other });
	}
	return bars;
}
