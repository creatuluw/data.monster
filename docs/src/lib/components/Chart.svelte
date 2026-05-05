<script lang="ts">
	import { onMount } from 'svelte';
	import { Plot, BarY, BarX, LineY, AreaY, Dot, Cell } from 'svelteplot';
	import { loadChartTemplate, compileTemplate, parseTemplateProps } from '$lib/viz/chart-template-runtime';
	import type { ChartTemplateProps } from '$lib/viz/chart-template-runtime';
	import { Loader, AlertCircle } from 'lucide-svelte';

	interface Props {
		component_name: string; // Component name to load (e.g., "MyChart", "SalesBarChart")
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
		strokeWidth?: number;
		fillOpacity?: number;
		r?: number;
		[key: string]: any;
	}

	let {
		component_name,
		data = [],
		x_dimension,
		y_dimension,
		y_dimension_2,
		x_metric,
		y_metric,
		value_metric,
		x_label,
		y_label,
		color,
		fill,
		stroke,
		strokeWidth,
		fillOpacity,
		r,
		...restProps
	}: Props = $props();

	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let templateCode = $state<string>('');
	let componentType = $state<string | null>(null);
	let compiledProps = $state<ChartTemplateProps>({});

	onMount(async () => {
		await loadTemplate();
	});

	async function loadTemplate() {
		isLoading = true;
		error = null;

		try {
			const templateData = await loadChartTemplate(component_name);
			
			if (!templateData) {
				error = `Chart component "${component_name}" not found`;
				isLoading = false;
				return;
			}

			templateCode = templateData.chart_code;
			
			// Parse template to determine component type
			const parsed = parseTemplateProps(templateCode);
			componentType = parsed.componentType;

			// Compile props
			compiledProps = {
				data,
				x_dimension,
				y_dimension,
				y_dimension_2,
				x_metric,
				y_metric,
				value_metric,
				x_label: x_label || x_dimension || 'X',
				y_label: y_label || y_metric || 'Y',
				color,
				fill,
				stroke,
				...restProps
			};

			isLoading = false;
		} catch (err) {
			error = err instanceof Error ? err.message : String(err);
			isLoading = false;
		}
	}

	// Reactive: recompile when props change
	$effect(() => {
		if (templateCode) {
			compiledProps = {
				data,
				x_dimension,
				y_dimension,
				y_dimension_2,
				x_metric,
				y_metric,
				value_metric,
				x_label: x_label || x_dimension || 'X',
				y_label: y_label || y_metric || 'Y',
				color,
				fill,
				stroke,
				...restProps
			};
		}
	});

	/**
	 * Compile custom HTML template by extracting the HTML portion
	 * and replacing prop references with actual values
	 */
	function compileCustomHTMLTemplate(_template: string, props: ChartTemplateProps): string {
		// For KPI Tile: format the value from data if available
		let displayValue: any = props.value || 0;
		let displayLabel = props.label || props.y_label || 'KPI';
		
		// If data array is provided, extract the aggregated value
		if (props.data && props.data.length > 0) {
			if (props.y_metric && props.data[0][props.y_metric] !== undefined) {
				// Single value (aggregate query result)
				displayValue = props.data[0][props.y_metric];
			}
			// Use metric name as label if available
			if (props.y_metric) {
				displayLabel = props.y_label || props.y_metric;
			}
		}
		
		// Format the value
		let formattedValue = displayValue;
		if (typeof displayValue === 'number') {
			const format = props.format || 'number';
			if (format === 'currency') {
				formattedValue = `$${displayValue.toLocaleString()}`;
			} else if (format === 'percent') {
				formattedValue = `${displayValue.toFixed(1)}%`;
			} else {
				formattedValue = displayValue.toLocaleString();
			}
		}
		
		// Simple KPI Tile render
		return `
			<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6">
				<div class="text-sm font-medium text-slate-600 dark:text-slate-400 mb-2">
					${displayLabel}
				</div>
				<div class="flex items-baseline gap-3">
					<div class="text-3xl font-bold text-slate-900 dark:text-slate-100">
						${formattedValue}
					</div>
				</div>
			</div>
		`;
	}
</script>

{#if isLoading}
	<div class="flex items-center justify-center h-64 bg-slate-50 dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700">
		<div class="text-center">
			<Loader class="w-8 h-8 text-indigo-500 animate-spin mx-auto mb-2" />
			<p class="text-sm text-slate-600 dark:text-slate-400">Loading chart component...</p>
		</div>
	</div>
{:else if error}
	<div class="flex items-center justify-center h-64 bg-red-50 dark:bg-red-900/10 rounded-lg border border-red-200 dark:border-red-800">
		<div class="text-center max-w-md px-4">
			<AlertCircle class="w-12 h-12 text-red-500 mx-auto mb-3" />
			<h3 class="text-lg font-semibold text-red-900 dark:text-red-100 mb-2">
				Chart Component Error
			</h3>
			<p class="text-sm text-red-700 dark:text-red-300">
				{error}
			</p>
		</div>
	</div>
{:else if componentType === 'BarY'}
	<Plot>
		<BarY
			data={compiledProps.data || []}
			x={compiledProps.x_dimension || 'x'}
			y={compiledProps.y_metric || 'y'}
			fill={compiledProps.fill || compiledProps.color || '#6366F1'}
			fillOpacity={fillOpacity}
		/>
	</Plot>
{:else if componentType === 'BarX'}
	<Plot marginLeft={120}>
		<BarX
			data={compiledProps.data || []}
			x={compiledProps.x_metric || compiledProps.y_metric || 'y'}
			y={compiledProps.y_dimension || compiledProps.x_dimension || 'x'}
			fill={compiledProps.fill || compiledProps.color || '#10B981'}
			fillOpacity={fillOpacity}
		/>
	</Plot>
{:else if componentType === 'LineY'}
	<Plot>
		<!-- TypeScript type issue with SveltePlot - works at runtime -->
		{@const xProp = compiledProps.x_dimension || 'x'}
		{@const yProp = compiledProps.y_metric || 'y'}
		<LineY
			data={compiledProps.data || []}
			x={xProp}
			y={yProp}
			stroke={compiledProps.stroke || compiledProps.color || '#3B82F6'}
			strokeWidth={strokeWidth || 2}
		/>
	</Plot>
{:else if componentType === 'AreaY'}
	<Plot>
		<AreaY
			data={compiledProps.data || []}
			x={compiledProps.x_dimension || 'x'}
			y={compiledProps.y_metric || 'y'}
			fill={compiledProps.fill || compiledProps.color || '#8B5CF6'}
			fillOpacity={fillOpacity || 0.3}
			stroke={compiledProps.stroke || compiledProps.color || '#8B5CF6'}
			strokeWidth={strokeWidth || 2}
		/>
	</Plot>
{:else if componentType === 'Dot'}
	<Plot>
		<Dot
			data={compiledProps.data || []}
			x={compiledProps.x_dimension || compiledProps.x_metric || 'x'}
			y={compiledProps.y_metric || 'y'}
			fill={compiledProps.fill || compiledProps.color || '#EC4899'}
			r={r || 5}
			fillOpacity={fillOpacity || 0.6}
		/>
	</Plot>
{:else if componentType === 'Cell'}
	<Plot>
		<Cell
			data={compiledProps.data || []}
			x={compiledProps.x_dimension || 'x'}
			y={compiledProps.y_dimension_2 || compiledProps.y_dimension || 'y'}
			fill={compiledProps.value_metric || 'value'}
			inset={1}
		/>
	</Plot>
{:else if componentType === 'CustomHTML'}
	<!-- Custom HTML component (KPI Tile, Data Table, etc.) -->
	{@html compileCustomHTMLTemplate(templateCode, compiledProps)}
{:else}
	<div class="flex items-center justify-center h-64 bg-slate-50 dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700">
		<div class="text-center max-w-md px-4">
			<AlertCircle class="w-12 h-12 text-slate-400 mx-auto mb-3" />
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				Unknown Chart Type
			</h3>
			<p class="text-sm text-slate-600 dark:text-slate-400">
				Component type "{componentType}" is not supported yet.
			</p>
		</div>
	</div>
{/if}

