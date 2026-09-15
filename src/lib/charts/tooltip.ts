/**
 * Tooltip template resolver (FR-4 / Q11-B): declarative fields + one template
 * string, shared renderer per chart type. `{field}` placeholders resolve
 * against the hovered row with optional per-field formatters.
 */

export type FieldFormatter = (value: unknown) => string;

/** Named formats available to spec `fmt` fields (v1 set). */
export const NAMED_FMTS: Record<string, FieldFormatter> = {
	hours: (v) => `${v}h`,
	usd: (v) => `$${Number(v).toLocaleString('en-US')}`,
	pct: (v) => `${v}%`,
	int: (v) => `${Math.round(Number(v))}`
};

export function namedFmt(name?: string): FieldFormatter | undefined {
	return name ? NAMED_FMTS[name] : undefined;
}

export function resolveTooltip(
	template: string,
	row: Record<string, unknown> | null | undefined,
	fmts?: Record<string, FieldFormatter>
): string {
	if (!row) return template;
	return template.replace(/\{(\w+)\}/g, (_, field: string) => {
		const value = row[field];
		if (value === undefined || value === null) return '';
		return fmts?.[field] ? fmts[field](value) : String(value);
	});
}
