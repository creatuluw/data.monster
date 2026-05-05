<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { Play, Save, X, Loader, BarChart3, AlertCircle, Sparkles, BookOpen, Database } from 'lucide-svelte';
	import Button from './Button.svelte';
	import SplitPane from './SplitPane.svelte';
	import CodeMirror from 'svelte-codemirror-editor';
	import { html } from '@codemirror/lang-html';
	import { oneDark } from '@codemirror/theme-one-dark';
	import { BarChart } from '$lib/viz/charts';
	import { Plot, BarY, BarX, LineY, AreaY, Dot } from 'svelteplot';
	import ChartTemplateBrowser from './ChartTemplateBrowser.svelte';
	import type { ChartTemplate } from '$lib/viz/chart-templates';
	import { invoke } from '@tauri-apps/api/core';
	import Chart from './Chart.svelte';

	interface Props {
		initialCode?: string;
		initialMetrics?: string[];
		initialDimensions?: string[];
		onSave?: (code: string, metrics: string[], dimensions: string[]) => Promise<void>;
		onClose?: () => void;
		availableMetrics?: Array<{ slug: string; metric_name: string }>;
		availableDimensions?: Array<{ slug: string; dimension_name: string; field_name: string; source_table: string }>;
		editingTemplate?: {
			chart_type: string;
			chart_name: string;
			slug: string;
			min_metrics?: number;
			max_metrics?: number;
			min_dimensions?: number;
			max_dimensions?: number;
		} | null;
	}

	let {
		initialCode = `<script lang="ts">
  import { Plot, BarY } from 'svelteplot';
  
  // Sample data - replace with your data
  const data = [
    { category: 'A', value: 10 },
    { category: 'B', value: 25 },
    { category: 'C', value: 18 },
    { category: 'D', value: 32 },
    { category: 'E', value: 15 }
  ];
</scr` + `ipt>

<Plot
  x={{ label: "Category" }}
  y={{ grid: true, label: "Value" }}
>
  <BarY
    data={data}
    x="category"
    y="value"
    fill="#6366F1"
  />
</Plot>`,
		initialMetrics = [],
		initialDimensions = [],
		onSave,
		onClose,
		availableMetrics = [],
		availableDimensions = [],
		editingTemplate = null
	}: Props = $props();

	let code = $state(initialCode);
	let selectedMetrics = $state<string[]>(initialMetrics);
	let selectedDimensions = $state<string[]>(initialDimensions);
	let previewComponent = $state<any>(null);
	let previewData = $state<any>(null);
	let isRendering = $state(false);
	let isSaving = $state(false);
	let renderError = $state('');
	let isDark = $state(false);
	let lastRenderTime = $state(0);
	let autoPreviewTimeout: ReturnType<typeof setTimeout> | null = null;
	let showTemplateBrowser = $state(false);
	let isLoadingData = $state(false);
	let hasDataLoaded = $state(false);
	let loadedData = $state<any[]>([]);
	let loadedDataColumns = $state<{ dimension: string; metric: string }>({ dimension: '', metric: '' });
	let showDataDrawer = $state(false);
	let dataViewTab = $state<'json' | 'table'>('table');
	let editorTab = $state<'create' | 'usage'>('create');
	let usageCode = $state('');

	// Watch for theme changes
	onMount(() => {
		isDark = document.documentElement.classList.contains('dark');

		// Listen for theme changes
		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains('dark');
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		// Auto-render on mount
		renderPreview();

		return () => {
			observer.disconnect();
			if (autoPreviewTimeout) clearTimeout(autoPreviewTimeout);
		};
	});

	// Auto-preview with debounce when code changes
	$effect(() => {
		// Watch code changes
		code;
		
		if (autoPreviewTimeout) clearTimeout(autoPreviewTimeout);
		autoPreviewTimeout = setTimeout(() => {
			renderPreview();
		}, 800); // Debounce 800ms for hot reload feel
	});

	async function renderPreview() {
		isRendering = true;
		renderError = '';
		const startTime = Date.now();

		try {
			// If we have loaded data, use it directly
			if (hasDataLoaded && loadedData.length > 0) {
				previewData = loadedData;
				previewComponent = BarChart;
				await tick();
				lastRenderTime = Date.now() - startTime;
				isRendering = false;
				return;
			}

			// Otherwise, parse the code to extract component and data
			const parsed = parseChartCode(code);
			
			if (!parsed) {
				// Don't throw error, just clear preview
				previewComponent = null;
				previewData = null;
				lastRenderTime = Date.now() - startTime;
				isRendering = false;
				return;
			}

			// Set preview data
			previewData = parsed.data;
			previewComponent = BarChart; // For now, we support BarChart
			
			await tick();
			lastRenderTime = Date.now() - startTime;
		} catch (error) {
			renderError = error instanceof Error ? error.message : String(error);
			previewComponent = null;
			previewData = null;
		} finally {
			isRendering = false;
		}
	}

	function parseChartCode(code: string): { data: any; props: Record<string, any> } | null {
		try {
			// Extract data array from script tag
			const scriptMatch = code.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
			if (!scriptMatch) return null;

			const scriptContent = scriptMatch[1];
			
			// Extract data array
			const dataMatch = scriptContent.match(/let\s+data\s*=\s*(\[[\s\S]*?\]);/);
			if (!dataMatch) return null;

			// eslint-disable-next-line no-eval
			const data = eval(`(${dataMatch[1]})`);

			// Extract component props
			const componentMatch = code.match(/<BarChart[^>]*\/?>|<BarChart[^>]*>([\s\S]*?)<\/BarChart>/i);
			if (!componentMatch) return null;

			const propsString = componentMatch[0];
			const props: Record<string, any> = { data };

			// Parse common props
			const propPatterns = [
				{ name: 'x', regex: /x=["']([^"']+)["']/ },
				{ name: 'y', regex: /y=["']([^"']+)["']/ },
				{ name: 'title', regex: /title=["']([^"']+)["']/ },
				{ name: 'subtitle', regex: /subtitle=["']([^"']+)["']/ },
				{ name: 'fillColor', regex: /fillColor=["']([^"']+)["']/ },
				{ name: 'swapXY', regex: /swapXY=\{(true|false)\}/ },
				{ name: 'labels', regex: /labels=\{(true|false)\}/ },
				{ name: 'legend', regex: /legend=\{(true|false)\}/ }
			];

			for (const { name, regex } of propPatterns) {
				const match = propsString.match(regex);
				if (match) {
					const value = match[1];
					props[name] = value === 'true' ? true : value === 'false' ? false : value;
				}
			}

			return { data, props };
		} catch (error) {
			console.error('Parse error:', error);
			return null;
		}
	}

	function parseAdditionalProps(code: string): Record<string, any> {
		const props: Record<string, any> = {};
		
		try {
			// Extract component props (excluding x, y, data)
			const componentMatch = code.match(/<BarChart[^>]*\/?>|<BarChart[^>]*>([\s\S]*?)<\/BarChart>/i);
			if (!componentMatch) return props;

			const propsString = componentMatch[0];

			// Parse common props
			const propPatterns = [
				{ name: 'title', regex: /title=["']([^"']+)["']/ },
				{ name: 'subtitle', regex: /subtitle=["']([^"']+)["']/ },
				{ name: 'fillColor', regex: /fillColor=["']([^"']+)["']/ },
				{ name: 'swapXY', regex: /swapXY=\{(true|false)\}/ },
				{ name: 'labels', regex: /labels=\{(true|false)\}/ },
				{ name: 'legend', regex: /legend=\{(true|false)\}/ },
				{ name: 'sort', regex: /sort=\{(true|false)\}/ },
				{ name: 'chartAreaHeight', regex: /chartAreaHeight=\{(\d+)\}/ }
			];

			for (const { name, regex } of propPatterns) {
				const match = propsString.match(regex);
				if (match) {
					const value = match[1];
					if (value === 'true') props[name] = true;
					else if (value === 'false') props[name] = false;
					else if (!isNaN(Number(value))) props[name] = Number(value);
					else props[name] = value;
				}
			}

			return props;
		} catch (error) {
			console.error('Parse additional props error:', error);
			return props;
		}
	}

	async function handleSave() {
		if (!onSave) return;

		try {
			isSaving = true;
			await onSave(code, selectedMetrics, selectedDimensions);
		} finally {
			isSaving = false;
		}
	}

	function toggleMetric(slug: string) {
		if (selectedMetrics.includes(slug)) {
			selectedMetrics = selectedMetrics.filter(m => m !== slug);
		} else {
			selectedMetrics = [...selectedMetrics, slug];
		}
		// Auto-populate chart with selected metric
		loadMetricData();
	}

	function toggleDimension(slug: string) {
		if (selectedDimensions.includes(slug)) {
			selectedDimensions = selectedDimensions.filter(d => d !== slug);
		} else {
			selectedDimensions = [...selectedDimensions, slug];
		}
		// Auto-populate chart with selected dimension
		loadMetricData();
	}

	async function loadMetricData() {
		const metric = availableMetrics.find(m => selectedMetrics.includes(m.slug));
		const dimension = availableDimensions.find(d => selectedDimensions.includes(d.slug));

		// Check template requirements from metadata, fallback to code inference
		const minDimensions = editingTemplate?.min_dimensions ?? (code.includes('<div') && code.includes('export let') && !code.includes('<Plot') ? 0 : 1);
		const requiresDimension = minDimensions > 0;

		// For KPI Tile and similar custom HTML components, only metric is needed
		if (!metric) {
			hasDataLoaded = false;
			loadedData = [];
			return;
		}

		// For charts that require dimensions, check if dimension is selected
		if (requiresDimension && !dimension) {
			hasDataLoaded = false;
			loadedData = [];
			return;
		}

		try {
			isLoadingData = true;
			
			// If no dimension is needed (KPI Tile), execute metric without dimensions
			if (!requiresDimension || !dimension) {
				// Execute aggregate metric (no GROUP BY) using execute_metric command
				try {
					const result = await invoke<string>('execute_metric', { metricName: metric.metric_name });
					const newData = JSON.parse(result);
					
					console.log('Loaded KPI data:', newData);
					
					if (newData && newData.length > 0) {
						loadedData = newData;
						loadedDataColumns = {
							dimension: '',
							metric: metric.metric_name
						};
						hasDataLoaded = true;
					} else {
						hasDataLoaded = false;
						loadedData = [];
					}
				} catch (error) {
					console.error('Error loading KPI metric:', error);
					hasDataLoaded = false;
					loadedData = [];
				}
				
				isLoadingData = false;
				return;
			}
			
			// Check if field_name already includes table prefix (format: table.column)
			// If not, add the source_table prefix
			const dimensionFullName = dimension.field_name.includes('.') 
				? dimension.field_name 
				: `${dimension.source_table}.${dimension.field_name}`;
			
			console.log('Loading metric data:', {
				metric: metric.metric_name,
				dimension: dimensionFullName,
				dimensionSourceTable: dimension.source_table,
				dimensionField: dimension.field_name,
				dimensionFullNameComputed: dimensionFullName
			});

			// Debug: Check what relationships exist in the database
			try {
				const relsQuery = `SELECT * FROM _warphead_relationships ORDER BY from_table, to_table`;
				const relsResult = await invoke<string>('execute_query', { query: relsQuery });
				const relationships = JSON.parse(relsResult);
				console.log('📊 All relationships in database:', relationships);
			} catch (e) {
				console.warn('Could not load relationships for debugging:', e);
			}

			// Execute metric with dimensions - same as chart-lib
			const result = await invoke<string>('execute_metric_with_dimensions', {
				metricName: metric.metric_name,
				dimensions: [dimensionFullName],
				filters: null
			});

			// Parse the response
			const response = JSON.parse(result);
			const newData = response.results || [];

			console.log('Loaded data:', newData);

			if (newData.length > 0) {
				// Get the actual column names from the data
				const columns = Object.keys(newData[0]);
				console.log('Available columns:', columns);
				
				// Find the metric column
				const metricColumn = columns.find(col => 
					col.toLowerCase() === metric.metric_name.toLowerCase() ||
					col.toLowerCase().replace(/_/g, ' ') === metric.metric_name.toLowerCase()
				) || columns.find(col => typeof newData[0][col] === 'number') || columns[columns.length - 1];
				
				// Find the dimension column
				const dimParts = dimension.field_name.split('.');
				const dimFieldName = dimParts[dimParts.length - 1];
				const dimensionColumn = columns.find(col => 
					col.toLowerCase() === dimFieldName.toLowerCase()
				) || columns[0];

				console.log('Column mapping:', {
					metricColumn,
					dimensionColumn,
					allColumns: columns
				});

				// Store data separately (NOT in code)
				loadedData = newData;
				loadedDataColumns = {
					dimension: dimensionColumn,
					metric: metricColumn
				};

				// Update template code WITHOUT data
				updateTemplateCode(dimensionColumn, metricColumn, metric, dimension);
				hasDataLoaded = true;
			} else {
				console.warn('No data returned from metric query');
				hasDataLoaded = false;
				loadedData = [];
			}
		} catch (error) {
			console.error('Error loading metric data:', error);
			hasDataLoaded = false;
			loadedData = [];
			
			// Provide helpful error messages
			const errorStr = String(error);
			if (errorStr.includes('No relationship path found')) {
				const match = errorStr.match(/from '([^']+)' to '([^']+)'/);
				if (match) {
					const [_, fromTable, toTable] = match;
					renderError = `⚠️ No relationship found between "${fromTable}" and "${toTable}".\n\nTo fix this:\n1. Go to @datamodel to define a relationship between these tables\n2. Or select a dimension from the "${fromTable}" table\n3. Or choose a different metric`;
				} else {
					renderError = `⚠️ ${errorStr}\n\nPlease check that your tables are related in @datamodel`;
				}
			} else {
				renderError = `Error loading data: ${error}`;
			}
		} finally {
			isLoadingData = false;
		}
	}

	function updateTemplateCode(
		dimensionColumn: string,
		metricColumn: string,
		metric: any,
		dimension: any
	) {
		// Parse current code to preserve structure
		const scriptMatch = code.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
		
		if (!scriptMatch) {
			console.warn('No script tag found in code');
			return;
		}

		let scriptContent = scriptMatch[1];

		// Remove any existing data array from script - keep it clean
		scriptContent = scriptContent.replace(/let\s+data\s*=\s*\[[\s\S]*?\];?\s*/g, '');
		scriptContent = scriptContent.replace(/const\s+data\s*=\s*\[[\s\S]*?\];?\s*/g, '');
		// Also remove any data comments
		scriptContent = scriptContent.replace(/\/\/.*data.*\n/gi, '');
		// Clean up extra whitespace
		scriptContent = scriptContent.trim();

		// Get the rest of the code after script tag
		const scriptEndTag = '<' + '/script>';
		const scriptEndIndex = code.indexOf(scriptEndTag);
		const afterScript = scriptEndIndex >= 0 ? code.substring(scriptEndIndex + 9) : '';

		// Update x and y props in chart components more carefully
		// We need to handle both simple x="value" and component x={{ object }}
		let updatedAfterScript = afterScript;
		
		// Find and update component x/y props (for LineY, BarY, AreaY, Dot, etc.)
		// Match x="something" or x={something} but not x={{ which is for Plot component
		const xRegex = /(<(?:LineY|BarY|BarX|AreaY|Dot|Cell)[^>]*)\s+x=(?:"[^"]*"|'[^']*'|\{[^{][^}]*\})/g;
		updatedAfterScript = updatedAfterScript.replace(xRegex, `$1 x="${dimensionColumn}"`);
		
		const yRegex = /(<(?:LineY|BarY|BarX|AreaY|Dot|Cell)[^>]*)\s+y=(?:"[^"]*"|'[^']*'|\{[^{][^}]*\})/g;
		updatedAfterScript = updatedAfterScript.replace(yRegex, `$1 y="${metricColumn}"`);

		// Update Plot component labels
		updatedAfterScript = updatedAfterScript.replace(
			/x=\{\{\s*label:\s*(?:\{[^}]+\}|"[^"]*"|'[^']*')\s*\}\}/g,
			`x={{ label: "${dimension.dimension_name}" }}`
		);
		
		updatedAfterScript = updatedAfterScript.replace(
			/y=\{\{\s*grid:\s*true,\s*label:\s*(?:\{[^}]+\}|"[^"]*"|'[^']*')\s*\}\}/g,
			`y={{ grid: true, label: "${metric.metric_name}" }}`
		);

		// Reconstruct the code without data
		const newCode = `<script>\n${scriptContent}\n</scr` + `ipt>\n${updatedAfterScript}`;

		code = newCode;
		renderPreview();
	}

	function updateChartCodeWithData(
		data: any[],
		dimensionColumn: string,
		metricColumn: string,
		metric: any,
		dimension: any
	) {
		// This function is deprecated - kept for backwards compatibility
		// Use updateTemplateCode instead
		updateTemplateCode(dimensionColumn, metricColumn, metric, dimension);
	}

	// Parse additional props for loaded data charts
	const additionalChartProps = $derived(parseAdditionalProps(code));

	// Extract props from parsed data for rendering (when not using loaded data)
	const chartProps = $derived.by(() => {
		if (!previewData) return {};
		
		const parsed = parseChartCode(code);
		return parsed?.props || {};
	});

	function loadTemplate(template: ChartTemplate) {
		code = template.rawCode;
		if (template.usageExample) {
			usageCode = template.usageExample;
		}
		showTemplateBrowser = false;
		renderPreview();
	}

	// Generate usage code from template variables
	function generateUsageCode() {
		if (!code) return '';
		
		// Extract the component name from the template if available (from chart_type)
		const componentName = editingTemplate?.chart_type || 'MyChart';
		
		// Get actual dimension/metric if selected, otherwise use placeholders
		const dimension = selectedDimensions.length > 0 
			? availableDimensions.find(d => selectedDimensions.includes(d.slug))
			: null;
		const metric = selectedMetrics.length > 0
			? availableMetrics.find(m => selectedMetrics.includes(m.slug))
			: null;
		
		// Extract just the column name from dimension field_name (e.g., "category.category" -> "category")
		let xDimensionValue = 'x_dimension';
		if (dimension?.field_name) {
			const parts = dimension.field_name.split('.');
			xDimensionValue = parts[parts.length - 1]; // Get last part (column name)
		}
		
		const yMetricValue = metric?.metric_name || 'y_metric';
		
		// Build clean usage example showing how to use Chart component
		let usage = `<script lang="ts">\n`;
		usage += `  import { Chart } from '$lib/viz/dynamic-charts';\n`;
		usage += `</scr` + `ipt>\n\n`;
		usage += `<Chart\n`;
		usage += `  component_name="${componentName}"\n`;
		usage += `  data={data}\n`;
		usage += `  x_dimension="${xDimensionValue}"\n`;
		usage += `  y_metric="${yMetricValue}"\n`;
		usage += `/>\n`;
		
		return usage;
	}

	// Parse usage code to extract Chart props for preview (no data parsing needed)
	function parseUsageCode(usageCode: string): {
		component_name: string;
		x_dimension?: string;
		y_metric?: string;
		x_label?: string;
		y_label?: string;
	} | null {
		try {
			// Extract Chart component props
			const chartMatch = usageCode.match(/<Chart[\s\S]*?\/>/);
			if (!chartMatch) return null;
			
			const chartCode = chartMatch[0];
			
			// Extract individual props
			const componentNameMatch = chartCode.match(/component_name=["']([^"']+)["']/);
			const xDimensionMatch = chartCode.match(/x_dimension=["']([^"']+)["']/);
			const yMetricMatch = chartCode.match(/y_metric=["']([^"']+)["']/);
			const xLabelMatch = chartCode.match(/x_label=["']([^"']+)["']/);
			const yLabelMatch = chartCode.match(/y_label=["']([^"']+)["']/);
			
			return {
				component_name: componentNameMatch?.[1] || editingTemplate?.chart_type || 'MyChart',
				x_dimension: xDimensionMatch?.[1],
				y_metric: yMetricMatch?.[1],
				x_label: xLabelMatch?.[1],
				y_label: yLabelMatch?.[1]
			};
		} catch (e) {
			console.error('Error parsing usage code:', e);
			return null;
		}
	}

	// State for parsed usage props
	let usageProps = $state<{
		component_name: string;
		x_dimension?: string;
		y_metric?: string;
		x_label?: string;
		y_label?: string;
	} | null>(null);

	// Watch for changes in usage code and parse it
	$effect(() => {
		if (editorTab === 'usage' && usageCode) {
			usageProps = parseUsageCode(usageCode);
		}
	});

	// Update usage code when selections change or tab switches
	$effect(() => {
		// Always generate usage code when relevant data changes
		usageCode = generateUsageCode();
	});
</script>

<div class="flex flex-col h-full bg-white dark:bg-slate-900">
	<!-- Header -->
	<div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center gap-3">
			<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Chart Builder</h2>
			{#if lastRenderTime > 0 && !renderError}
				<span class="text-xs text-slate-500 dark:text-slate-400 flex items-center gap-1">
					<Sparkles class="w-3 h-3" />
					Rendered in {lastRenderTime}ms
				</span>
			{/if}
		</div>
		<div class="flex items-center gap-2">
			<Button variant="ghost" size="sm" onclick={() => showTemplateBrowser = true}>
				<BookOpen class="w-4 h-4 mr-2" />
				Browse Templates
			</Button>
			<Button variant="ghost" size="sm" onclick={renderPreview} disabled={isRendering}>
				{#if isRendering}
					<Loader class="w-4 h-4 mr-2 animate-spin" />
					Rendering...
				{:else}
					<Play class="w-4 h-4 mr-2" />
					Preview
				{/if}
			</Button>
			{#if onSave}
				<Button variant="primary" size="sm" onclick={handleSave} disabled={isSaving}>
					{#if isSaving}
						<Loader class="w-4 h-4 mr-2 animate-spin" />
						Saving...
					{:else}
						<Save class="w-4 h-4 mr-2" />
						Save
					{/if}
				</Button>
			{/if}
			{#if onClose}
				<Button variant="ghost" size="sm" onclick={onClose}>
					<X class="w-4 h-4" />
				</Button>
			{/if}
		</div>
	</div>

	<!-- Main Content: Code Editor + Preview + Data Bindings -->
	<div class="flex flex-1 overflow-hidden">
		<SplitPane initialSize={50} minSize={30} maxSize={70}>
			{#snippet leftPanel()}
				<!-- Left: Code Editor with Tabs -->
				<div class="flex flex-col h-full">
					<!-- Editor Tabs -->
					<div class="flex border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800">
						<button
							type="button"
							onclick={() => editorTab = 'create'}
							class="px-4 py-2.5 text-sm font-medium border-b-2 transition-colors {editorTab === 'create'
								? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
								: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
						>
							Create
						</button>
						<button
							type="button"
							onclick={() => { editorTab = 'usage'; usageCode = generateUsageCode(); }}
							class="px-4 py-2.5 text-sm font-medium border-b-2 transition-colors {editorTab === 'usage'
								? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
								: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
						>
							Usage
						</button>
					</div>

					<!-- CodeMirror Editor -->
					<div class="flex-1 overflow-auto codemirror-wrapper">
						{#if editorTab === 'create'}
							<CodeMirror
								bind:value={code}
								lang={html()}
								theme={isDark ? oneDark : undefined}
								styles={{
									'&': {
										width: '100%',
										height: '100%',
										fontSize: '14px'
									}
								}}
							/>
						{:else}
							<!-- Usage Tab - Editable -->
							<CodeMirror
								bind:value={usageCode}
								lang={html()}
								theme={isDark ? oneDark : undefined}
								styles={{
									'&': {
										width: '100%',
										height: '100%',
										fontSize: '14px'
									}
								}}
							/>
						{/if}
					</div>
				</div>
			{/snippet}

			{#snippet rightPanel()}
				<!-- Right: Preview + Data Bindings (Nested Vertical Split) -->
				<div class="flex flex-col h-full">
					<SplitPane initialSize={65} minSize={40} maxSize={85} orientation="vertical">
						{#snippet topPanel()}
							<!-- Preview Section -->
							<div class="flex flex-col h-full">
							<!-- Preview Toolbar -->
							<div class="px-4 py-2 bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
								<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Preview</span>
								<div class="flex items-center gap-3">
									<!-- Debug info -->
									<span class="text-xs text-slate-500">
										Data: {hasDataLoaded ? 'loaded' : 'not loaded'} | Rows: {loadedData.length}
									</span>
									
									{#if selectedMetrics.length > 0 || selectedDimensions.length > 0}
										<div class="text-xs text-slate-500 dark:text-slate-400 flex items-center gap-2">
											{#if selectedMetrics.length > 0}
												<span>{selectedMetrics.length} metric{selectedMetrics.length !== 1 ? 's' : ''}</span>
											{/if}
											{#if selectedDimensions.length > 0}
												<span>{selectedDimensions.length} dimension{selectedDimensions.length !== 1 ? 's' : ''}</span>
											{/if}
										</div>
									{/if}
								</div>
							</div>

								<!-- Preview Area -->
								<div class="flex-1 p-6 overflow-auto bg-slate-50 dark:bg-slate-950">
				{#if isLoadingData}
					<!-- Loading State -->
					<div class="flex items-center justify-center h-full">
						<div class="text-center">
							<Loader class="w-12 h-12 text-indigo-500 animate-spin mx-auto mb-4" />
							<p class="text-sm text-slate-600 dark:text-slate-400">
								Loading data from database...
							</p>
						</div>
					</div>
				{:else if !hasDataLoaded && selectedMetrics.length === 0 && selectedDimensions.length === 0}
					<!-- Initial Empty State - No Selections -->
					{@const minDimensions = editingTemplate?.min_dimensions ?? (code.includes('<div') && code.includes('export let') && !code.includes('<Plot') ? 0 : 1)}
					{@const requiresDimension = minDimensions > 0}
					<div class="flex items-center justify-center h-full">
						<div class="text-center max-w-md">
							<div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-slate-100 dark:bg-slate-800 mb-4">
								<BarChart3 class="w-8 h-8 text-slate-400" />
							</div>
							<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
								Select Data to Visualize
							</h3>
							<p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
								{#if requiresDimension}
									Choose a metric and dimension from the Data Bindings section below to load real data and see your chart.
								{:else}
									Choose a metric from the Data Bindings section below to load real data and see your chart.
								{/if}
							</p>
							<div class="flex flex-col gap-2 text-xs text-slate-500 dark:text-slate-400 text-left bg-slate-100 dark:bg-slate-800 px-4 py-3 rounded-lg">
								<div class="flex items-center gap-2">
									<span class="text-indigo-500">1.</span>
									<span>Click a <strong class="text-indigo-600 dark:text-indigo-400">metric</strong> (e.g., "quantity", "revenue")</span>
								</div>
								{#if requiresDimension}
									<div class="flex items-center gap-2">
										<span class="text-purple-500">2.</span>
										<span>Click a <strong class="text-purple-600 dark:text-purple-400">dimension</strong> (e.g., "Category")</span>
									</div>
									<div class="flex items-center gap-2">
										<span class="text-green-500">3.</span>
										<span>Real data will load and your chart will render automatically</span>
									</div>
								{:else}
									<div class="flex items-center gap-2">
										<span class="text-green-500">2.</span>
										<span>Your KPI value will load automatically (no dimension needed)</span>
									</div>
								{/if}
							</div>
						</div>
					</div>
				{:else if !hasDataLoaded && (selectedMetrics.length > 0 || selectedDimensions.length > 0)}
					<!-- Partial Selection State -->
					{@const minDimensions = editingTemplate?.min_dimensions ?? (code.includes('<div') && code.includes('export let') && !code.includes('<Plot') ? 0 : 1)}
					{@const requiresDimension = minDimensions > 0}
					<div class="flex items-center justify-center h-full">
						<div class="text-center max-w-md">
							<div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-amber-100 dark:bg-amber-900/20 mb-4">
								<AlertCircle class="w-8 h-8 text-amber-600 dark:text-amber-400" />
							</div>
							<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
								Almost There!
							</h3>
							<p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
								{#if selectedMetrics.length === 0}
									Select a <strong class="text-indigo-600 dark:text-indigo-400">metric</strong> to complete your chart.
								{:else if requiresDimension && selectedDimensions.length === 0}
									Select a <strong class="text-purple-600 dark:text-purple-400">dimension</strong> to complete your chart.
								{:else if !requiresDimension}
									<!-- KPI Tile is ready with just a metric -->
									<span class="text-green-600 dark:text-green-400">Loading your KPI data...</span>
								{/if}
							</p>
							{#if requiresDimension}
								<div class="text-xs text-slate-500 dark:text-slate-400 bg-slate-100 dark:bg-slate-800 px-4 py-3 rounded-lg">
									Both a metric and dimension are required to load data.
								</div>
							{:else}
								<div class="text-xs text-slate-500 dark:text-slate-400 bg-slate-100 dark:bg-slate-800 px-4 py-3 rounded-lg">
									This chart only needs a metric. Dimension is optional.
								</div>
							{/if}
						</div>
					</div>
				{:else if renderError}
					<!-- Error State -->
					<div class="flex items-center justify-center h-full">
						<div class="max-w-md text-center">
							<div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-red-100 dark:bg-red-900/20 mb-4">
								<AlertCircle class="w-8 h-8 text-red-600 dark:text-red-400" />
							</div>
							<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
								Unable to render chart
							</h3>
							<p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
								There's an issue with your chart code or data. Check the details below.
							</p>
							<div class="bg-red-50 dark:bg-red-900/10 border border-red-200 dark:border-red-800 rounded-lg p-3 text-left">
								<p class="text-xs font-mono text-red-700 dark:text-red-400 break-words whitespace-pre-wrap">
									{renderError}
								</p>
							</div>
						</div>
					</div>
				{:else if isRendering}
					<!-- Loading State -->
					<div class="flex items-center justify-center h-full">
						<div class="text-center">
							<Loader class="w-12 h-12 text-indigo-500 animate-spin mx-auto mb-4" />
							<p class="text-sm text-slate-600 dark:text-slate-400">
								Rendering your chart...
							</p>
						</div>
					</div>
				{:else if hasDataLoaded && loadedData.length > 0}
					<!-- Rendered Chart with Loaded Data - Using Chart Component -->
					<div class="h-full min-h-[400px] bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800 p-4">
						{#if editorTab === 'usage' && usageProps}
							<!-- Preview from Usage Tab - using edited props with loaded data -->
							<Chart
								component_name={usageProps.component_name}
								data={loadedData}
								x_dimension={usageProps.x_dimension || loadedDataColumns.dimension}
								y_metric={usageProps.y_metric || loadedDataColumns.metric}
								x_label={usageProps.x_label}
								y_label={usageProps.y_label}
							/>
						{:else}
							<!-- Preview from Create Tab with loaded data -->
							<Chart
								component_name={editingTemplate?.chart_type || 'MyChart'}
								data={loadedData}
								x_dimension={loadedDataColumns.dimension}
								y_metric={loadedDataColumns.metric}
							/>
						{/if}
					</div>
				{:else if previewComponent && previewData}
					<!-- Rendered Chart with Parsed Data -->
					<div class="h-full min-h-[400px] bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800 p-4">
						{#if previewComponent && previewData}
							<BarChart data={previewData} {...chartProps} />
						{/if}
					</div>
				{:else}
					<!-- Empty State (shouldn't reach here but fallback) -->
					<div class="flex items-center justify-center h-full">
						<div class="text-center max-w-md">
							<div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-slate-100 dark:bg-slate-800 mb-4">
								<BarChart3 class="w-8 h-8 text-slate-400" />
							</div>
							<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
								Chart Preview
							</h3>
							<p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
								Select a metric and dimension to load data and see your chart.
							</p>
						</div>
					</div>
				{/if}
			</div>
						</div>
					{/snippet}

					{#snippet bottomPanel()}
						<!-- Data Bindings Panel (Below Preview in Second Snippet) -->
						<div class="border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 p-4 max-h-64 overflow-y-auto">
								<div class="flex items-center justify-between mb-3">
									<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Data Bindings</h3>
									{#if hasDataLoaded && loadedData.length > 0}
										<button
											type="button"
											onclick={() => showDataDrawer = true}
											class="text-xs px-2 py-1 rounded bg-indigo-50 dark:bg-indigo-900/20 text-indigo-600 dark:text-indigo-400 hover:bg-indigo-100 dark:hover:bg-indigo-900/30 transition-colors font-medium flex items-center gap-1"
										>
											<Database class="w-3 h-3" />
											View Data ({loadedData.length} rows)
										</button>
									{/if}
								</div>
								
								<!-- Metrics -->
								<div class="mb-4">
									<span class="text-xs font-medium text-slate-600 dark:text-slate-400 mb-2 block">
										Metrics {selectedMetrics.length > 0 ? `(${selectedMetrics.length} selected)` : ''}
									</span>
									{#if availableMetrics.length === 0}
										<p class="text-xs text-slate-500 dark:text-slate-500 italic">No metrics available</p>
									{:else}
										<div class="flex flex-wrap gap-2">
											{#each availableMetrics as metric}
												<button
													type="button"
													onclick={() => toggleMetric(metric.slug)}
													class="px-2 py-1 text-xs rounded border transition-colors {selectedMetrics.includes(
														metric.slug
													)
														? 'bg-indigo-100 dark:bg-indigo-900/30 border-indigo-500 text-indigo-700 dark:text-indigo-400 font-medium'
														: 'bg-white dark:bg-slate-900 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:border-indigo-400'}"
													title={metric.metric_name}
												>
													{metric.metric_name}
												</button>
											{/each}
										</div>
									{/if}
								</div>

								<!-- Dimensions -->
								<div>
									<span class="text-xs font-medium text-slate-600 dark:text-slate-400 mb-2 block">
										Dimensions {selectedDimensions.length > 0 ? `(${selectedDimensions.length} selected)` : ''}
									</span>
									{#if availableDimensions.length === 0}
										<p class="text-xs text-slate-500 dark:text-slate-500 italic">No dimensions available</p>
									{:else}
										<div class="flex flex-wrap gap-2">
											{#each availableDimensions as dimension}
												<button
													type="button"
													onclick={() => toggleDimension(dimension.slug)}
													class="px-2 py-1 text-xs rounded border transition-colors {selectedDimensions.includes(
														dimension.slug
													)
														? 'bg-purple-100 dark:bg-purple-900/30 border-purple-500 text-purple-700 dark:text-purple-400 font-medium'
														: 'bg-white dark:bg-slate-900 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:border-purple-400'}"
													title={dimension.dimension_name}
												>
													{dimension.dimension_name}
												</button>
											{/each}
										</div>
									{/if}
								</div>

								{#if selectedMetrics.length > 0 && selectedDimensions.length > 0}
									<div class="mt-3 pt-3 border-t border-slate-200 dark:border-slate-700">
										<p class="text-xs text-slate-600 dark:text-slate-400">
											💡 Click a metric or dimension to automatically update the chart code with sample data.
											The chart will use: <strong class="text-purple-600 dark:text-purple-400">{availableDimensions.find(d => selectedDimensions.includes(d.slug))?.field_name}</strong> × <strong class="text-indigo-600 dark:text-indigo-400">{availableMetrics.find(m => selectedMetrics.includes(m.slug))?.metric_name}</strong>
										</p>
									</div>
								{/if}
						</div>
					{/snippet}
				</SplitPane>
			</div>
		{/snippet}
	</SplitPane>
</div>
</div>

<!-- Data Viewer Drawer -->
{#if showDataDrawer}
	<div 
		class="fixed inset-0 bg-black/50 flex items-end justify-end z-[70]" 
		onclick={() => showDataDrawer = false}
		onkeydown={(e) => e.key === 'Escape' && (showDataDrawer = false)}
		role="button"
		tabindex="-1"
	>
		<div 
			class="bg-white dark:bg-slate-900 h-full w-[600px] shadow-2xl flex flex-col"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-label="Data viewer"
			tabindex="0"
		>
			<!-- Drawer Header -->
			<div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
				<div>
					<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">Data Viewer</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						{loadedData.length} rows × {Object.keys(loadedData[0] || {}).length} columns
					</p>
				</div>
				<button
					type="button"
					onclick={() => showDataDrawer = false}
					class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Tabs -->
			<div class="flex border-b border-slate-200 dark:border-slate-700 px-6">
				<button
					type="button"
					onclick={() => dataViewTab = 'table'}
					class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {dataViewTab === 'table'
						? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
						: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
				>
					Table
				</button>
				<button
					type="button"
					onclick={() => dataViewTab = 'json'}
					class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {dataViewTab === 'json'
						? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
						: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
				>
					JSON
				</button>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-auto p-6">
				{#if dataViewTab === 'table'}
					<!-- Table View -->
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b border-slate-200 dark:border-slate-700">
									{#each Object.keys(loadedData[0] || {}) as column}
										<th class="text-left py-3 px-4 font-semibold text-slate-900 dark:text-slate-100 bg-slate-50 dark:bg-slate-800">
											{column}
										</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each loadedData as row, i}
									<tr class="border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
										{#each Object.values(row) as value}
											<td class="py-3 px-4 text-slate-700 dark:text-slate-300">
												{typeof value === 'number' ? value.toLocaleString() : value}
											</td>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{:else}
					<!-- JSON View -->
					<div class="relative">
						<button
							type="button"
							onclick={() => navigator.clipboard.writeText(JSON.stringify(loadedData, null, 2))}
							class="absolute top-2 right-2 px-3 py-1 text-xs font-medium text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 bg-slate-100 dark:bg-slate-800 rounded hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
						>
							Copy JSON
						</button>
						<pre class="bg-slate-900 dark:bg-black text-slate-100 p-4 rounded-lg overflow-x-auto text-xs leading-relaxed"><code>{JSON.stringify(loadedData, null, 2)}</code></pre>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Template Browser Modal -->
{#if showTemplateBrowser}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[70] p-4">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-2xl w-full max-w-6xl h-[80vh] overflow-hidden">
			<ChartTemplateBrowser 
				onSelect={loadTemplate}
				onClose={() => showTemplateBrowser = false}
			/>
		</div>
	</div>
{/if}

<style>
	:global(.codemirror-wrapper .cm-editor) {
		height: 100% !important;
	}
	
	:global(.codemirror-wrapper .cm-scroller) {
		overflow: auto;
	}
</style>
