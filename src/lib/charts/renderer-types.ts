/**
 * Props every chart renderer receives from the page runtime (FR-6/7/8).
 * Kept in a .ts module so the runtime stays importable without Svelte.
 */
import type { TooltipSpec } from './spec-types';
import type { ColorScale } from './color-scale';
import type { FieldFormatter } from './tooltip';

export type ResolvedAnnotation = { mark: string; value: number; label?: string };

export type ChartRendererProps = {
	/** engine rows (already aggregated; keys = dimension/measure aliases) */
	rows: Record<string, unknown>[];
	/** runtime status — when omitted, renderers derive empty/ok from rows */
	status?: 'ok' | 'loading' | 'error' | 'empty';
	error?: string;
	dimensionAliases: string[];
	measureAliases: string[];
	/** merged registry defaults + chart options */
	options: Record<string, unknown>;
	/** annotations with evaluated values */
	annotations: ResolvedAnnotation[];
	title?: string;
	subtitle?: string;
	tooltip?: TooltipSpec;
	selected: { dimension: string; value: string } | null;
	onSelect: (selection: { dimension: string; value: string } | null) => void;
	colorScale: ColorScale;
	fmts: Record<string, FieldFormatter>;
	heightVh: number;
};
