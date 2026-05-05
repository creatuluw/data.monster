/**
 * Chart Template Runtime
 * 
 * Dynamically loads chart templates from DuckDB and converts them into
 * usable Svelte components at runtime.
 * 
 * Flow:
 * 1. Load templates from DB (chart_templates table)
 * 2. Parse template code and extract variables
 * 3. Replace template variables with actual props
 * 4. Render as Svelte component
 * 
 * Usage:
 *   <DynamicChart template="MyChart" data={salesData} x_dimension="category" y_metric="revenue" />
 */

import { invoke } from '@tauri-apps/api/core';

export interface ChartTemplateFromDB {
	slug: string;
	chart_name: string;
	chart_type: string; // This is the component name (MyChart, SalesBarChart, etc.)
	chart_code: string;
	config_schema: string | null;
	description: string | null;
	tags: string | null;
	metrics: string | null;
	dimensions: string | null;
	sample_data: string | null;
	min_metrics: number | null;
	max_metrics: number | null;
	min_dimensions: number | null;
	max_dimensions: number | null;
	created_at: string;
	updated_at: string;
}

export interface ChartTemplateProps {
	[key: string]: any;
	data?: any[];
	x_dimension?: string;
	y_dimension?: string;
	y_dimension_2?: string;
	x_metric?: string;
	y_metric?: string;
	value_metric?: string;
	x_label?: string;
	y_label?: string;
	color?: string;
	fill?: string;
	stroke?: string;
}

/**
 * Load all chart templates from DuckDB
 */
export async function loadChartTemplates(): Promise<ChartTemplateFromDB[]> {
	try {
		const result = await invoke<string>('list_chart_templates');
		return JSON.parse(result);
	} catch (error) {
		console.error('Error loading chart templates:', error);
		return [];
	}
}

/**
 * Load a specific chart template by slug or chart_type
 */
export async function loadChartTemplate(
	identifier: string
): Promise<ChartTemplateFromDB | null> {
	try {
		const templates = await loadChartTemplates();
		return (
			templates.find((t) => t.slug === identifier || t.chart_type === identifier) || null
		);
	} catch (error) {
		console.error('Error loading chart template:', error);
		return null;
	}
}

/**
 * Replace template variables in code with actual values
 * 
 * Template variables:
 * - {{data}} or {data} - data array
 * - {x_dimension} - dimension field name
 * - {y_metric} - metric field name
 * - {x_label} - x axis label
 * - {y_label} - y axis label
 * - {color}, {fill}, {stroke} - color values
 */
export function compileTemplate(templateCode: string, props: ChartTemplateProps): string {
	let compiled = templateCode;

	// Replace template variables
	const replacements: Record<string, any> = {
		'{{data}}': props.data ? 'data' : '[]',
		'{data}': props.data ? 'data' : '[]',
		'{x_dimension}': props.x_dimension ? `"${props.x_dimension}"` : '"x"',
		'{y_dimension}': props.y_dimension ? `"${props.y_dimension}"` : '"y"',
		'{y_dimension_2}': props.y_dimension_2 ? `"${props.y_dimension_2}"` : '"y2"',
		'{x_metric}': props.x_metric ? `"${props.x_metric}"` : '"x"',
		'{y_metric}': props.y_metric ? `"${props.y_metric}"` : '"value"',
		'{value_metric}': props.value_metric ? `"${props.value_metric}"` : '"value"',
		'{x_label}': props.x_label ? `"${props.x_label}"` : '""',
		'{y_label}': props.y_label ? `"${props.y_label}"` : '""',
		'{color}': props.color ? `"${props.color}"` : '"#6366F1"',
		'{fill}': props.fill ? `"${props.fill}"` : '"#6366F1"',
		'{stroke}': props.stroke ? `"${props.stroke}"` : '"#6366F1"'
	};

	// Perform replacements
	for (const [variable, value] of Object.entries(replacements)) {
		compiled = compiled.replace(new RegExp(escapeRegExp(variable), 'g'), value);
	}

	return compiled;
}

/**
 * Extract component type from template code
 * (e.g., "BarY", "LineY", "AreaY", etc.)
 */
export function extractComponentType(templateCode: string): string | null {
	// Check for SveltePlot components first
	const componentMatch = templateCode.match(/<(BarY|BarX|LineY|AreaY|Dot|Cell|Line|Area|Bar)/);
	if (componentMatch) return componentMatch[1];
	
	// Check for custom HTML-based components (KPI Tile, Data Table, etc.)
	// If it has <div class= and export let but no SveltePlot components, it's custom HTML
	if (templateCode.includes('<div') && templateCode.includes('export let') && !templateCode.includes('<Plot')) {
		return 'CustomHTML';
	}
	
	return null;
}

/**
 * Parse template code to extract component props
 */
export function parseTemplateProps(templateCode: string): {
	componentType: string | null;
	hasData: boolean;
	hasX: boolean;
	hasY: boolean;
	requiresData: boolean;
} {
	const componentType = extractComponentType(templateCode);
	const hasData = templateCode.includes('data=');
	const hasX = templateCode.includes('x=');
	const hasY = templateCode.includes('y=');
	const requiresData = templateCode.includes('{data}') || templateCode.includes('{{data}}');

	return {
		componentType,
		hasData,
		hasX,
		hasY,
		requiresData
	};
}

/**
 * Create a registry of chart templates by component name
 */
export class ChartTemplateRegistry {
	private templates: Map<string, ChartTemplateFromDB> = new Map();
	private loaded: boolean = false;

	async load() {
		if (this.loaded) return;

		const templates = await loadChartTemplates();
		for (const template of templates) {
			// Index by both chart_type (component name) and slug
			this.templates.set(template.chart_type, template);
			this.templates.set(template.slug, template);
		}

		this.loaded = true;
	}

	get(identifier: string): ChartTemplateFromDB | undefined {
		return this.templates.get(identifier);
	}

	getAll(): ChartTemplateFromDB[] {
		return Array.from(this.templates.values());
	}

	has(identifier: string): boolean {
		return this.templates.has(identifier);
	}

	clear() {
		this.templates.clear();
		this.loaded = false;
	}
}

// Global registry instance
export const chartTemplateRegistry = new ChartTemplateRegistry();

/**
 * Helper function to escape regex special characters
 */
function escapeRegExp(string: string): string {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

