/**
 * Chart type registry (FR-3). Pure metadata + validation — no component
 * imports here (renderers are wired in PageGrid so vitest stays svelte-free).
 * Registering bar/heatmap happens in registry-setup (called by pages).
 */
import type { ChartBlockSpec, ValidationError } from './spec-types';

export type RoleSpec = { kind: 'dimension' | 'measure'; min: number; max?: number };

export type FieldSpec = {
	name: string;
	kind: 'number' | 'boolean' | 'enum' | 'string' | 'color';
	label: string;
	default?: unknown;
	options?: string[];
	min?: number;
	max?: number;
};

export type QueryHooks = { topN?: boolean; pivot?: boolean; grain?: boolean };

export type ChartTypeDefinition = {
	type: string;
	label: string;
	roles: RoleSpec[];
	optionsSchema: FieldSpec[];
	hooks?: QueryHooks;
	/** Annotation marks this type can host (subset of the basic-mark vocabulary). */
	annotations: string[];
	defaults: Record<string, unknown>;
};

const registry = new Map<string, ChartTypeDefinition>();

export function registerChartType(def: ChartTypeDefinition): void {
	registry.set(def.type, def);
}

export function getChartType(type: string): ChartTypeDefinition | undefined {
	return registry.get(type);
}

export function resetChartTypes(): void {
	registry.clear();
}

/** Resolve a chart block: definition + role errors (unknown type is an error, not a throw). */
export function resolveChartBlock(chart: ChartBlockSpec): {
	definition?: ChartTypeDefinition;
	errors: ValidationError[];
} {
	const definition = getChartType(chart.type);
	if (!definition) return { errors: [{ path: 'type', message: `unknown chart type "${chart.type}"` }] };

	const errors: ValidationError[] = [];
	const counts = { dimension: chart.dimensions.length, measure: chart.measures.length };
	for (const role of definition.roles) {
		const n = counts[role.kind];
		if (n < role.min) errors.push({ path: role.kind + 's', message: `${chart.type} needs at least ${role.min} ${role.kind}(s), got ${n}` });
		else if (role.max !== undefined && n > role.max)
			errors.push({ path: role.kind + 's', message: `${chart.type} takes at most ${role.max} ${role.kind}(s), got ${n}` });
	}
	return { definition: errors.length ? undefined : definition, errors };
}

/** True while the chart's roles are unfilled (below min) — render a setup skeleton, never query. */
export function needsSetup(chart: ChartBlockSpec): boolean {
	const def = getChartType(chart.type);
	if (!def) return chart.dimensions.length === 0 || chart.measures.length === 0;
	return def.roles.some((r) => {
		const n = r.kind === 'dimension' ? chart.dimensions.length : chart.measures.length;
		return n < r.min;
	});
}

/** Merge optionsSchema defaults (then definition.defaults) with the chart's own options. */
export function mergeOptions(chart: ChartBlockSpec): Record<string, unknown> {
	const def = getChartType(chart.type);
	if (!def) return { ...chart.options };
	const merged: Record<string, unknown> = {};
	for (const field of def.optionsSchema) if (field.default !== undefined) merged[field.name] = field.default;
	Object.assign(merged, def.defaults, chart.options ?? {});
	return merged;
}
