/**
 * Page-level color scale (FR-4 / Q12-B): color = f(dimension, value) so the
 * same category renders the same color in every chart on the page.
 * First-seen assignment over the DS palette; explicit overrides win.
 */

export type ColorScale = { colorOf: (dimension: string, value: string) => string };

export function createColorScale(palette: string[], opts?: { seriesColors?: Record<string, string> }): ColorScale {
	const assignment = new Map<string, string>();
	const perDimension = new Map<string, number>();

	return {
		colorOf(dimension: string, value: string): string {
			const key = `${dimension}\u0000${value}`;
			const existing = assignment.get(key);
			if (existing) return existing;
			const override = opts?.seriesColors?.[value];
			const color = override ?? palette[(perDimension.get(dimension) ?? 0) % Math.max(palette.length, 1)];
			perDimension.set(dimension, (perDimension.get(dimension) ?? 0) + 1);
			assignment.set(key, color);
			return color;
		}
	};
}
