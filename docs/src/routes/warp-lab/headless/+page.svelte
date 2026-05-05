<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import Label from '$lib/components/Label.svelte';
	import HeadlessChartBarChart from '$lib/components/HeadlessChartBarChart.svelte';
	import { BarChart3, Code, Eye, Play, AlertCircle, Loader, Settings } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';

	// Get database context from layout
	const dbContext = getDbContext();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// Types
	interface Dimension {
		slug: string;
		dimension_name: string;
		field_name: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	interface Metric {
		slug: string;
		metric_name: string;
		formula: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	// State
	let dimensions = $state<Dimension[]>([]);
	let metrics = $state<Metric[]>([]);
	let isLoading = $state(false);
	let isLoadingData = $state(false);

	let selectedDimension = $state<string>('');
	let selectedMetric = $state<string>('');
	let chartData = $state<any[]>([]);
	let errorMessage = $state<string>('');

	// Chart customization options
	let chartDirection = $state<'vertical' | 'horizontal'>('vertical');
	let chartTitle = $state<string>('Headless Chart Bar Chart');

	// Code view state
	let generatedCode = $state<string>('');
	let generatedSQL = $state<string>('');

	// Load dimensions and metrics
	async function loadDimensionsAndMetrics() {
		if (!dbContext.isInitialized) {
			errorMessage = 'Database not initialized';
			return;
		}

		isLoading = true;
		errorMessage = '';

		try {
			const [dimensionsJson, metricsJson] = await Promise.all([
				invoke<string>('list_dimensions'),
				invoke<string>('list_metrics')
			]);

			dimensions = JSON.parse(dimensionsJson);
			metrics = JSON.parse(metricsJson);

			console.log('Loaded dimensions:', dimensions);
			console.log('Loaded metrics:', metrics);
		} catch (error) {
			console.error('Failed to load dimensions/metrics:', error);
			errorMessage = `Failed to load dimensions/metrics: ${error}`;
			addSystemMessage(`Failed to load dimensions/metrics: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	// Execute query and render chart
	async function executeQuery() {
		if (!selectedDimension || !selectedMetric) {
			errorMessage = 'Please select both a dimension and a metric';
			return;
		}

		isLoadingData = true;
		errorMessage = '';
		chartData = [];

		try {
			const selectedDim = dimensions.find(d => d.slug === selectedDimension);
			const selectedMet = metrics.find(m => m.slug === selectedMetric);

			if (!selectedDim || !selectedMet) {
				throw new Error('Selected dimension or metric not found');
			}

			// Update chart title
			chartTitle = `${selectedMet.metric_name} by ${selectedDim.dimension_name}`;

			// Execute metric with dimension
			const resultJson = await invoke<string>('execute_metric_with_dimensions', {
				metricName: selectedMet.metric_name,
				dimensions: [`${selectedDim.source_table}.${selectedDim.field_name}`],
				filters: null
			});

			const result = JSON.parse(resultJson);
			generatedSQL = result.query || '';
			
			const data = result.results || [];
			console.log('Query results:', data);

			if (data.length === 0) {
				errorMessage = 'No data returned from query';
				addSystemMessage('No data returned from query', 'warning');
				return;
			}

			// Transform data for Headless Chart
			chartData = data;

			// Generate code example
			generateCodeExample(selectedDim, selectedMet, data);
			
			addSystemMessage('Chart rendered successfully', 'success');
		} catch (error) {
			console.error('Failed to execute query:', error);
			errorMessage = `Failed to execute query: ${error}`;
			addSystemMessage(`Failed to execute query: ${error}`, 'error');
		} finally {
			isLoadingData = false;
		}
	}

	function generateCodeExample(dim: Dimension, met: Metric, data: any[]) {
		const xKey = dim.field_name;
		const yKey = met.metric_name;

		// Transform to Headless Chart format
		const labels = data.slice(0, 5).map(item => item[xKey]);
		const values = data.slice(0, 5).map(item => item[yKey]);

		generatedCode = `import Widget from '@meursyphus/flitter-svelte';
import { BarChart } from '@meursyphus/headless-chart';
import { Container, BoxDecoration, EdgeInsets, BorderRadius, Text, TextStyle } from '@meursyphus/flitter';

// Your data
const rawData = ${JSON.stringify(data.slice(0, 5), null, 2)}${data.length > 5 ? '\n  // ... and more' : ''};

// Transform to Headless Chart format
const chartData = {
  labels: ${JSON.stringify(labels)},
  datasets: [{
    legend: '${met.metric_name}',
    values: ${JSON.stringify(values)}
  }]
};

// Create chart with customization
const chart = BarChart({
  data: chartData,
  title: '${chartTitle}',
  direction: '${chartDirection}',
  custom: {
    bar: ({ value, datasetIndex }) => {
      const colors = ['#9b00ff', '#00b300', '#5696ff', '#ff9f40', '#ff6384'];
      const color = colors[datasetIndex % colors.length];
      
      return Container({
        width: Infinity,
        height: Infinity,
        margin: EdgeInsets.symmetric({ horizontal: 4 }),
        decoration: new BoxDecoration({
          color: color,
          borderRadius: BorderRadius.circular(4)
        })
      });
    },
    title: ({ title }) => {
      return Text(title || '', {
        style: new TextStyle({
          fontSize: 20,
          fontWeight: 'bold',
          color: '#1a202c'
        })
      });
    }
  }
});

// Render in Svelte component
<Widget widget={chart} />`;
	}

	onMount(() => {
		loadDimensionsAndMetrics();
	});
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div class="p-3 bg-amber-100 dark:bg-amber-900/30 rounded-lg">
				<BarChart3 class="w-8 h-8 text-amber-600 dark:text-amber-400" />
			</div>
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Headless Chart Bar Chart</h1>
				<p class="text-slate-600 dark:text-slate-400">
					Create fully customizable bar charts with widget-based architecture
				</p>
			</div>
		</div>

		<!-- Selection Controls -->
		<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 mb-6">
			<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Chart Configuration</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
				<!-- Dimension Selection -->
				<div>
					<Label for="dimension-select">Select Dimension (X-Axis)</Label>
					{#if isLoading}
						<div class="flex items-center gap-2 text-slate-500 dark:text-slate-400 mt-2">
							<Loader class="w-4 h-4 animate-spin" />
							<span class="text-sm">Loading dimensions...</span>
						</div>
					{:else if dimensions.length === 0}
						<div class="text-sm text-amber-600 dark:text-amber-400 mt-2">
							<AlertCircle class="w-4 h-4 inline mr-1" />
							No dimensions found. Create dimensions in the Models section.
						</div>
					{:else}
						<select
							id="dimension-select"
							bind:value={selectedDimension}
							class="mt-2 block w-full rounded-md border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 shadow-sm focus:border-amber-500 focus:ring-amber-500"
						>
							<option value="">-- Select a dimension --</option>
							{#each dimensions as dim}
								<option value={dim.slug}>
									{dim.dimension_name} ({dim.source_table}.{dim.field_name})
								</option>
							{/each}
						</select>
					{/if}
				</div>

				<!-- Metric Selection -->
				<div>
					<Label for="metric-select">Select Metric (Y-Axis)</Label>
					{#if isLoading}
						<div class="flex items-center gap-2 text-slate-500 dark:text-slate-400 mt-2">
							<Loader class="w-4 h-4 animate-spin" />
							<span class="text-sm">Loading metrics...</span>
						</div>
					{:else if metrics.length === 0}
						<div class="text-sm text-amber-600 dark:text-amber-400 mt-2">
							<AlertCircle class="w-4 h-4 inline mr-1" />
							No metrics found. Create metrics in the Models section.
						</div>
					{:else}
						<select
							id="metric-select"
							bind:value={selectedMetric}
							class="mt-2 block w-full rounded-md border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 shadow-sm focus:border-amber-500 focus:ring-amber-500"
						>
							<option value="">-- Select a metric --</option>
							{#each metrics as met}
								<option value={met.slug}>
									{met.metric_name} ({met.formula})
								</option>
							{/each}
						</select>
					{/if}
				</div>
			</div>

			<!-- Chart Customization -->
			<div class="border-t border-slate-200 dark:border-slate-700 pt-6 mt-6">
				<div class="flex items-center gap-2 mb-4">
					<Settings class="w-5 h-5 text-slate-600 dark:text-slate-400" />
					<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Chart Customization</h3>
				</div>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<!-- Chart Direction -->
					<div>
						<Label for="direction-select">Chart Direction</Label>
						<select
							id="direction-select"
							bind:value={chartDirection}
							class="mt-2 block w-full rounded-md border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 shadow-sm focus:border-amber-500 focus:ring-amber-500"
						>
							<option value="vertical">Vertical</option>
							<option value="horizontal">Horizontal</option>
						</select>
					</div>
				</div>
			</div>

			<!-- Execute Button -->
			<div class="flex justify-end mt-6">
				<Button
					onclick={executeQuery}
					disabled={!selectedDimension || !selectedMetric || isLoadingData}
					variant="primary"
				>
					{#if isLoadingData}
						<Loader class="w-4 h-4 mr-2 animate-spin" />
						Generating Chart...
					{:else}
						<Play class="w-4 h-4 mr-2" />
						Generate Chart
					{/if}
				</Button>
			</div>

			<!-- Error Message -->
			{#if errorMessage}
				<div class="mt-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
					<div class="flex items-start gap-2">
						<AlertCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
						<p class="text-sm text-red-700 dark:text-red-300">{errorMessage}</p>
					</div>
				</div>
			{/if}
		</div>

		<!-- Split View: Code + Preview -->
		{#if chartData.length > 0}
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
				<!-- Left: Code View -->
				<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 overflow-hidden">
					<div class="bg-slate-50 dark:bg-slate-900 px-4 py-3 border-b border-slate-200 dark:border-slate-700">
						<div class="flex items-center gap-2">
							<Code class="w-5 h-5 text-slate-600 dark:text-slate-400" />
							<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Generated Code</h3>
						</div>
					</div>
					
					<div class="p-4 overflow-auto max-h-[600px]">
						<!-- SQL Query -->
						<div class="mb-6">
							<h4 class="text-xs font-semibold text-slate-700 dark:text-slate-300 mb-2 uppercase tracking-wide">SQL Query</h4>
							<pre class="bg-slate-900 text-slate-100 p-4 rounded-lg text-xs overflow-x-auto"><code>{generatedSQL}</code></pre>
						</div>

						<!-- JavaScript Code -->
						<div>
							<h4 class="text-xs font-semibold text-slate-700 dark:text-slate-300 mb-2 uppercase tracking-wide">Svelte / TypeScript</h4>
							<pre class="bg-slate-900 text-slate-100 p-4 rounded-lg text-xs overflow-x-auto"><code>{generatedCode}</code></pre>
						</div>
					</div>
				</div>

				<!-- Right: Chart Preview -->
				<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 overflow-hidden">
					<div class="bg-slate-50 dark:bg-slate-900 px-4 py-3 border-b border-slate-200 dark:border-slate-700">
						<div class="flex items-center gap-2">
							<Eye class="w-5 h-5 text-slate-600 dark:text-slate-400" />
							<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Chart Preview</h3>
						</div>
					</div>
					
					<div class="p-6">
						{#if chartData.length > 0}
							{@const dim = dimensions.find(d => d.slug === selectedDimension)}
							{@const met = metrics.find(m => m.slug === selectedMetric)}
							{#if dim && met}
								<HeadlessChartBarChart
									data={chartData}
									xKey={dim.field_name}
									yKeys={[met.metric_name]}
									width={600}
									height={400}
									chartTitle={chartTitle}
									direction={chartDirection}
								/>
							{/if}
						{:else}
							<div class="flex items-center justify-center h-[400px] text-slate-500 dark:text-slate-400">
								<div class="text-center">
									<BarChart3 class="w-16 h-16 mx-auto mb-4 opacity-50" />
									<p>Select dimension and metric to generate chart</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>
		{:else}
			<!-- Empty State -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-12">
				<div class="text-center">
					<BarChart3 class="w-20 h-20 mx-auto mb-4 text-slate-300 dark:text-slate-600" />
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">No Chart Generated Yet</h3>
					<p class="text-slate-600 dark:text-slate-400 mb-6">
						Select a dimension and metric above, then click "Generate Chart" to see your data visualized.
					</p>
					<div class="text-sm text-slate-500 dark:text-slate-400">
						<p>💡 <strong>Tip:</strong> Headless Chart provides complete control over every chart element through its widget-based architecture!</p>
					</div>
				</div>
			</div>
		{/if}

		<!-- Help Section -->
		<div class="mt-8 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-amber-900 dark:text-amber-300 mb-3">About Headless Chart</h3>
			<div class="text-sm text-amber-800 dark:text-amber-200 space-y-2">
				<p>
					<strong>Headless Chart</strong> is a widget-based charting library that provides complete customization control over every element.
					Built with a Flitter widget system, it allows you to customize 21+ components including bars, axes, legends, and grids.
				</p>
				<p>
					📚 <a href="https://headless-chart.pages.dev/docs/en/chart-guide/bar-chart/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View Headless Chart Documentation</a>
				</p>
				<p>
					🎨 <a href="https://headless-chart.pages.dev/docs/en/getting-started/installation/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">Installation Guide</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

<style>
	select {
		padding: 0.5rem 0.75rem;
	}

	pre {
		font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
		line-height: 1.5;
	}
</style>

