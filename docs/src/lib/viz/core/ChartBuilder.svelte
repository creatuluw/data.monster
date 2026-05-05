<script lang="ts">
	import { getAvailableCharts, type ChartMetadata, type PropDefinition } from '../charts/index.js';
	import { Sparkles, Code, Settings, Eye } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';

	interface Metric {
		slug: string;
		metric_name: string;
		formula: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	interface Dimension {
		slug: string;
		dimension_name: string;
		field_name: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	interface ChartBuilderProps {
		initialData?: any[];
		metrics?: Metric[];
		dimensions?: Dimension[];
		onChartCreate?: (config: any) => void;
	}

	let { initialData = [], metrics = [], dimensions = [], onChartCreate }: ChartBuilderProps = $props();

	// State
	let selectedChart = $state<ChartMetadata | null>(null);
	let chartProps = $state<Record<string, any>>({});
	let viewMode = $state<'gui' | 'code'>('gui');
	let availableCharts = $derived(getAvailableCharts());
	
	// Metric and Dimension selection state
	let selectedMetric = $state<Metric | null>(null);
	let selectedDimensions = $state<string[]>([]);
	let isLoadingData = $state(false);
	
	// Debug state object for console tracking
	let debugState = {
		chartType: '',
		lastUpdated: '',
		props: {} as Record<string, any>
	};

	// Sample data for preview
	let previewData = $state(
		initialData.length > 0
			? initialData
			: [
					{ category: 'Product A', sales: 12000, quantity: 450 },
					{ category: 'Product B', sales: 19000, quantity: 680 },
					{ category: 'Product C', sales: 8500, quantity: 290 },
					{ category: 'Product D', sales: 15000, quantity: 550 },
					{ category: 'Product E', sales: 22000, quantity: 780 }
				]
	);

	// Detect columns from data
	let availableColumns = $derived(
		previewData.length > 0 ? Object.keys(previewData[0]) : []
	);

	// Load data based on selected metric and dimensions
	async function loadMetricData() {
		if (!selectedMetric) {
			console.log('No metric selected, using initial data');
			return;
		}

		try {
			isLoadingData = true;
			console.log('Loading metric data:', {
				metric: selectedMetric.metric_name,
				dimensions: selectedDimensions
			});

			// Execute metric with dimensions
			const result = await invoke<string>('execute_metric_with_dimensions', {
				metricName: selectedMetric.metric_name,
				dimensions: selectedDimensions,
				filters: null
			});

			// Parse the response
			const response = JSON.parse(result);
			const newData = response.results || [];

			console.log('Loaded data:', newData);
			console.log('Response structure:', response);
			console.log('First row of data:', newData[0]);

			if (newData.length > 0) {
				// Get the actual column names from the data
				const columns = Object.keys(newData[0]);
				console.log('Available columns:', columns);
				console.log('Full data array (all rows):', newData);
				
				// Find the metric column - it should match the metric name (case-insensitive)
				// The backend returns it as the metric name, not necessarily snake_case
				const metricName = selectedMetric?.metric_name || '';
				const metricColumn = columns.find(col => 
					col.toLowerCase() === metricName.toLowerCase() ||
					col.toLowerCase().replace(/_/g, ' ') === metricName.toLowerCase()
				) || columns.find(col => typeof newData[0][col] === 'number') || columns[columns.length - 1];
				
				// Find the dimension column - extract just the column name (after the dot)
				let dimensionColumn = columns[0]; // default to first column
				if (selectedDimensions.length > 0) {
					const dimParts = selectedDimensions[0].split('.');
					const dimFieldName = dimParts[dimParts.length - 1]; // Get the last part (column name)
					dimensionColumn = columns.find(col => 
						col.toLowerCase() === dimFieldName.toLowerCase()
					) || columns[0];
				}

				console.log('Column mapping:', {
					metricColumn,
					dimensionColumn,
					allColumns: columns
				});

				// Update preview data FIRST
				previewData = newData;

				// Update chart configuration
				chartProps = {
					...chartProps,
					data: newData,
					y: metricColumn,
					x: dimensionColumn
				};

				console.log('Updated chart props:', { x: chartProps.x, y: chartProps.y });
			} else {
				console.warn('No data returned from metric query');
			}
		} catch (error) {
			console.error('Error loading metric data:', error);
			
			// Show user-friendly error message
			const errorMsg = String(error);
			if (errorMsg.includes('No relationship path found')) {
				const metricTable = selectedMetric?.source_table || 'unknown';
				const dimTable = selectedDimensions.length > 0 
					? dimensions.find(d => d.field_name === selectedDimensions[0])?.source_table 
					: 'none';
					
				console.error('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
				console.error('❌ RELATIONSHIP ERROR');
				console.error('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
				console.error('Metric table:', metricTable);
				console.error('Dimension table:', dimTable);
				console.error('Dimension field:', selectedDimensions[0]);
				console.error('');
				console.error('💡 SOLUTIONS:');
				console.error('1. Make sure a relationship exists in /models/datamodel');
				console.error('2. The dimension must use a field that participates in the JOIN');
				console.error('   (e.g., if tables JOIN on cat_id, dimension should be category.cat_id or all_orders.cat_id)');
				console.error('3. Or create a dimension from the same table as the metric');
				console.error('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
			}
			
			// Keep using existing data on error
		} finally {
			isLoadingData = false;
		}
	}

	// Select a chart
	function selectChart(chart: ChartMetadata) {
		selectedChart = chart;
		chartProps = {
			data: previewData,
			...chart.defaultProps,
			// Auto-detect x and y from data
			x: availableColumns[0] || '',
			y: availableColumns.find((col) => typeof previewData[0]?.[col] === 'number') || availableColumns[1] || ''
		};
		
		// Update debug state
		debugState.chartType = chart.name;
		debugState.lastUpdated = new Date().toLocaleTimeString();
		debugState.props = { ...chartProps };
		delete debugState.props.data; // Remove data for cleaner output
		
		console.clear();
		console.log('📊 CHART BUILDER STATE (Live Updated)');
		console.table(debugState);
		console.log('Full Props:', debugState.props);
	}

	// Update a prop value
	function updateProp(name: string, value: any) {
		chartProps = { ...chartProps, [name]: value };
		
		// Update debug state
		debugState.lastUpdated = new Date().toLocaleTimeString();
		debugState.props = { ...chartProps };
		delete debugState.props.data; // Remove data for cleaner output
		
		console.clear();
		console.log('📊 CHART BUILDER STATE (Live Updated)');
		console.table(debugState);
		console.log('Full Props:', debugState.props);
		console.log(`\n🔄 Last Change: ${name} = ${JSON.stringify(value)}`);
	}

	// Group prop definitions by category
	function groupPropsByCategory(propDefs: PropDefinition[]) {
		const groups: Record<string, PropDefinition[]> = {};
		propDefs.forEach((prop) => {
			if (!groups[prop.category]) {
				groups[prop.category] = [];
			}
			groups[prop.category].push(prop);
		});
		return groups;
	}

	// Generate Evidence.dev-style code
	let generatedCode = $derived.by(() => {
		if (!selectedChart) return '';

		const lines: string[] = [];
		lines.push(`<${selectedChart.name}`);

		// Add props (skip data and defaults)
		Object.entries(chartProps).forEach(([key, value]) => {
			if (key === 'data') return;
			if (!selectedChart) return;
			
			const propDef = selectedChart.propDefinitions.find((p) => p.name === key);
			const defaultValue = selectedChart.defaultProps[key];
			
			// Skip if value equals default
			if (JSON.stringify(value) === JSON.stringify(defaultValue)) return;
			if (value === undefined || value === null || value === '') return;

			if (typeof value === 'boolean') {
				if (value) lines.push(`    ${key}={true}`);
			} else if (typeof value === 'string') {
				lines.push(`    ${key}="${value}"`);
			} else if (typeof value === 'number') {
				lines.push(`    ${key}={${value}}`);
			} else if (Array.isArray(value)) {
				lines.push(`    ${key}={${JSON.stringify(value)}}`);
			}
		});

		lines.push('/>');
		return lines.join('\n');
	});
</script>

<div class="chart-builder">
	<!-- Chart Type Selector -->
	{#if !selectedChart}
		<div class="chart-selector">
			<div class="selector-header">
				<Sparkles class="size-6" />
				<div>
					<h2 class="selector-title">Select a Chart Type</h2>
					<p class="selector-subtitle">Choose from {availableCharts.length} available visualizations</p>
				</div>
			</div>

			<div class="chart-grid">
				{#each availableCharts as chart}
					<button class="chart-card" onclick={() => selectChart(chart)}>
						<div class="chart-icon">{chart.icon}</div>
						<h3 class="chart-name">{chart.displayName}</h3>
						<p class="chart-description">{chart.description}</p>
						<span class="chart-category">{chart.category}</span>
					</button>
				{/each}
			</div>
		</div>
	{:else}
		<!-- Chart Builder Interface -->
		<div class="builder-layout">
			<!-- Header -->
			<div class="builder-header">
				<div class="view-toggle">
					<button
						class="toggle-button"
						class:active={viewMode === 'gui'}
						onclick={() => (viewMode = 'gui')}
					>
						<Settings class="size-4" />
						Settings
					</button>
					<button
						class="toggle-button"
						class:active={viewMode === 'code'}
						onclick={() => (viewMode = 'code')}
					>
						<Code class="size-4" />
						Code
					</button>
				</div>
			</div>

			<!-- Main Content -->
			<div class="builder-content">
				<!-- Left Panel: Settings or Code -->
				<div class="settings-panel">
					{#if viewMode === 'gui'}
						<div class="settings-content">
							<!-- Metrics & Dimensions Section -->
							{#if metrics.length > 0 || dimensions.length > 0}
								<div class="setting-group">
									<h3 class="group-title">Data Source</h3>
									
									{#if metrics.length > 0}
										<div class="setting-item">
											<label class="setting-label" for="metric-selector">
												Metric
												<span class="setting-hint">Select a metric to visualize</span>
											</label>
											<select
												id="metric-selector"
												class="setting-select"
												value={selectedMetric?.metric_name || ''}
												onchange={async (e) => {
													const metricName = e.currentTarget.value;
													const metric = metrics.find(m => m.metric_name === metricName);
													selectedMetric = metric || null;
													if (selectedMetric) {
														console.log('Selected metric:', selectedMetric);
														await loadMetricData();
													}
												}}
												disabled={isLoadingData}
											>
												<option value="">Choose a metric...</option>
												{#each metrics as metric}
													<option value={metric.metric_name}>
														{metric.metric_name}
														{#if metric.description}
															- {metric.description}
														{/if}
													</option>
												{/each}
											</select>
											{#if isLoadingData}
												<span class="setting-hint">Loading data...</span>
											{/if}
										</div>
									{/if}
									
									{#if dimensions.length > 0}
										<div class="setting-item">
											<label class="setting-label" for="dimension-selector">
												Dimension
												<span class="setting-hint">
													{#if selectedMetric}
														Select dimension from {selectedMetric.source_table} table
													{:else}
														Select dimensions to group by
													{/if}
												</span>
											</label>
											<select
												id="dimension-selector"
												class="setting-select"
												value={selectedDimensions[0] || ''}
												onchange={async (e) => {
													const dimensionFieldName = e.currentTarget.value;
													if (dimensionFieldName === '') {
														selectedDimensions = [];
														if (selectedMetric) {
															await loadMetricData();
														}
														return;
													}
													
													const dimension = dimensions.find(d => d.field_name === dimensionFieldName);
													if (dimension) {
														console.log('Selected dimension:', dimension);
														
														selectedDimensions = [dimension.field_name];
														if (selectedMetric) {
															await loadMetricData();
														} else {
															// Update x-axis if no metric selected
															const columnName = dimension.field_name.split('.').pop() || dimension.field_name;
															updateProp('x', columnName);
														}
													}
												}}
												disabled={isLoadingData}
											>
												<option value="">Choose a dimension...</option>
												
												{#if selectedMetric}
													<!-- Show compatible dimensions first (same source table) -->
													{@const compatibleDims = dimensions.filter(d => d.source_table === selectedMetric?.source_table)}
													{@const otherDims = dimensions.filter(d => d.source_table !== selectedMetric?.source_table)}
													
													{#if compatibleDims.length > 0}
														<optgroup label="From {selectedMetric.source_table} (Compatible)">
															{#each compatibleDims as dimension}
																<option value={dimension.field_name}>
																	{dimension.dimension_name} ({dimension.field_name})
																	{#if dimension.description}
																		- {dimension.description}
																	{/if}
																</option>
															{/each}
														</optgroup>
													{/if}
													
													{#if otherDims.length > 0}
														<optgroup label="From Other Tables (May require JOIN)">
															{#each otherDims as dimension}
																<option value={dimension.field_name}>
																	{dimension.dimension_name} ({dimension.field_name})
																	{#if dimension.description}
																		- {dimension.description}
																	{/if}
																</option>
															{/each}
														</optgroup>
													{/if}
												{:else}
													<!-- No metric selected, show all dimensions -->
													{#each dimensions as dimension}
														<option value={dimension.field_name}>
															{dimension.dimension_name} ({dimension.field_name})
															{#if dimension.description}
																- {dimension.description}
															{/if}
														</option>
													{/each}
												{/if}
											</select>
										</div>
									{/if}
								</div>
							{/if}

							<!-- Regular Chart Properties -->
							{#each Object.entries(groupPropsByCategory(selectedChart.propDefinitions)) as [category, props]}
								<div class="setting-group">
									<h3 class="group-title">{category}</h3>
									{#each props as prop}
										{#if prop.name !== 'data' && prop.name !== 'x' && prop.name !== 'y'}
											<div class="setting-item">
												<label class="setting-label" for={prop.name}>
													{prop.displayName}
													{#if prop.description}
														<span class="setting-hint">{prop.description}</span>
													{/if}
												</label>

												{#if prop.type === 'string'}
													<input
														type="text"
														id={prop.name}
														class="setting-input"
														value={chartProps[prop.name] || ''}
														oninput={(e) => updateProp(prop.name, e.currentTarget.value)}
													/>
												{:else if prop.type === 'number'}
													<input
														type="number"
														id={prop.name}
														class="setting-input"
														value={chartProps[prop.name] ?? ''}
														oninput={(e) => updateProp(prop.name, parseFloat(e.currentTarget.value) || 0)}
													/>
												{:else if prop.type === 'boolean'}
													<label class="toggle-switch">
														<input
															type="checkbox"
															id={prop.name}
															checked={chartProps[prop.name] || false}
															onchange={(e) => updateProp(prop.name, e.currentTarget.checked)}
														/>
														<span class="toggle-slider"></span>
													</label>
												{:else if prop.type === 'select' && prop.options}
													<select
														id={prop.name}
														class="setting-select"
														value={chartProps[prop.name] || ''}
														onchange={(e) => updateProp(prop.name, e.currentTarget.value)}
													>
														{#each prop.options as option}
															<option value={option.value}>{option.label}</option>
														{/each}
													</select>
												{:else if prop.type === 'color'}
													<div class="color-input-wrapper">
														<input
															type="color"
															id={prop.name}
															class="color-input"
															value={chartProps[prop.name] || '#CA3500'}
															oninput={(e) => updateProp(prop.name, e.currentTarget.value)}
														/>
														<input
															type="text"
															class="color-text"
															value={chartProps[prop.name] || '#CA3500'}
															oninput={(e) => updateProp(prop.name, e.currentTarget.value)}
														/>
													</div>
												{/if}
											</div>
										{/if}
									{/each}
								</div>
							{/each}
						</div>
					{:else}
						<div class="code-view">
							<div class="code-header">
								<Code class="size-4" />
								<span>Evidence.dev Style Code</span>
							</div>
							<pre class="code-block"><code>{generatedCode}</code></pre>
							<button
								class="copy-button"
								onclick={() => navigator.clipboard.writeText(generatedCode)}
							>
								Copy Code
							</button>
						</div>
					{/if}
				</div>

				<!-- Right Panel: Preview -->
				<div class="preview-panel">
					<div class="preview-header">
						<Eye class="size-4" />
						<span>Live Preview</span>
					</div>
					<div class="preview-content">
						{#if selectedChart}
							{@const Component = selectedChart.component}
							{#key chartProps}
								<Component {...chartProps} />
							{/key}
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.chart-builder {
		width: 100%;
		height: 100%;
		background: rgb(249, 250, 251);
		overflow: auto;
	}

	:global(.dark) .chart-builder {
		background: rgb(17, 24, 39);
	}

	/* Chart Selector */
	.chart-selector {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.selector-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 2rem;
		color: rgb(99, 102, 241);
	}

	.selector-title {
		font-size: 1.875rem;
		font-weight: 700;
		color: rgb(17, 24, 39);
		margin: 0;
	}

	:global(.dark) .selector-title {
		color: rgb(243, 244, 246);
	}

	.selector-subtitle {
		font-size: 1rem;
		color: rgb(107, 114, 128);
		margin: 0.25rem 0 0 0;
	}

	:global(.dark) .selector-subtitle {
		color: rgb(156, 163, 175);
	}

	.chart-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.chart-card {
		background: white;
		border: 2px solid rgb(229, 231, 235);
		border-radius: 0.75rem;
		padding: 1.5rem;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	:global(.dark) .chart-card {
		background: rgb(31, 41, 55);
		border-color: rgb(55, 65, 81);
	}

	.chart-card:hover {
		border-color: rgb(99, 102, 241);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
		transform: translateY(-2px);
	}

	.chart-icon {
		font-size: 2.5rem;
		margin-bottom: 1rem;
	}

	.chart-name {
		font-size: 1.125rem;
		font-weight: 600;
		color: rgb(17, 24, 39);
		margin: 0 0 0.5rem 0;
	}

	:global(.dark) .chart-name {
		color: rgb(243, 244, 246);
	}

	.chart-description {
		font-size: 0.875rem;
		color: rgb(107, 114, 128);
		margin: 0 0 1rem 0;
		line-height: 1.5;
	}

	:global(.dark) .chart-description {
		color: rgb(156, 163, 175);
	}

	.chart-category {
		display: inline-block;
		font-size: 0.75rem;
		font-weight: 500;
		color: rgb(99, 102, 241);
		background: rgb(238, 242, 255);
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
	}

	:global(.dark) .chart-category {
		background: rgba(99, 102, 241, 0.2);
		color: rgb(165, 180, 252);
	}

	/* Builder Layout */
	.builder-layout {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.builder-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.5rem;
		background: white;
		border-bottom: 1px solid rgb(229, 231, 235);
	}

	:global(.dark) .builder-header {
		background: rgb(31, 41, 55);
		border-color: rgb(55, 65, 81);
	}

	.view-toggle {
		display: flex;
		gap: 0.5rem;
		background: rgb(243, 244, 246);
		padding: 0.25rem;
		border-radius: 0.5rem;
	}

	:global(.dark) .view-toggle {
		background: rgb(17, 24, 39);
	}

	.toggle-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: rgb(107, 114, 128);
		background: none;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toggle-button.active {
		background: white;
		color: rgb(99, 102, 241);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	:global(.dark) .toggle-button.active {
		background: rgb(31, 41, 55);
	}

	/* Builder Content */
	.builder-content {
		display: grid;
		grid-template-columns: 400px 1fr;
		flex: 1;
		overflow: hidden;
	}

	.settings-panel {
		background: white;
		border-right: 1px solid rgb(229, 231, 235);
		overflow-y: auto;
	}

	:global(.dark) .settings-panel {
		background: rgb(31, 41, 55);
		border-color: rgb(55, 65, 81);
	}

	.settings-content {
		padding: 1.5rem;
	}

	.setting-group {
		margin-bottom: 2rem;
	}

	.group-title {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: rgb(107, 114, 128);
		margin: 0 0 1rem 0;
	}

	:global(.dark) .group-title {
		color: rgb(156, 163, 175);
	}

	.setting-item {
		margin-bottom: 1.25rem;
	}

	.setting-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: rgb(55, 65, 81);
		margin-bottom: 0.5rem;
	}

	:global(.dark) .setting-label {
		color: rgb(209, 213, 219);
	}

	.setting-hint {
		display: block;
		font-size: 0.75rem;
		font-weight: 400;
		color: rgb(107, 114, 128);
		margin-top: 0.25rem;
	}

	:global(.dark) .setting-hint {
		color: rgb(156, 163, 175);
	}

	.setting-input,
	.setting-select {
		width: 100%;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		border: 1px solid rgb(209, 213, 219);
		border-radius: 0.375rem;
		background: white;
		color: rgb(17, 24, 39);
	}

	:global(.dark) .setting-input,
	:global(.dark) .setting-select {
		background: rgb(17, 24, 39);
		border-color: rgb(55, 65, 81);
		color: rgb(243, 244, 246);
	}

	.setting-input:focus,
	.setting-select:focus {
		outline: none;
		border-color: rgb(99, 102, 241);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	/* Toggle Switch */
	.toggle-switch {
		position: relative;
		display: inline-block;
		width: 44px;
		height: 24px;
	}

	.toggle-switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgb(209, 213, 219);
		border-radius: 24px;
		transition: 0.2s;
	}

	.toggle-slider:before {
		position: absolute;
		content: '';
		height: 18px;
		width: 18px;
		left: 3px;
		bottom: 3px;
		background: white;
		border-radius: 50%;
		transition: 0.2s;
	}

	input:checked + .toggle-slider {
		background: rgb(99, 102, 241);
	}

	input:checked + .toggle-slider:before {
		transform: translateX(20px);
	}

	/* Color Input */
	.color-input-wrapper {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.color-input {
		width: 48px;
		height: 40px;
		border: 1px solid rgb(209, 213, 219);
		border-radius: 0.375rem;
		cursor: pointer;
	}

	:global(.dark) .color-input {
		border-color: rgb(55, 65, 81);
	}

	.color-text {
		flex: 1;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		font-family: monospace;
		border: 1px solid rgb(209, 213, 219);
		border-radius: 0.375rem;
		background: white;
		color: rgb(17, 24, 39);
	}

	:global(.dark) .color-text {
		background: rgb(17, 24, 39);
		border-color: rgb(55, 65, 81);
		color: rgb(243, 244, 246);
	}

	/* Code View */
	.code-view {
		padding: 1.5rem;
	}

	.code-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: rgb(55, 65, 81);
		margin-bottom: 1rem;
	}

	:global(.dark) .code-header {
		color: rgb(209, 213, 219);
	}

	.code-block {
		background: rgb(17, 24, 39);
		color: rgb(209, 213, 219);
		padding: 1.5rem;
		border-radius: 0.5rem;
		overflow-x: auto;
		font-family: 'Courier New', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		margin: 0;
	}

	.copy-button {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: white;
		background: rgb(99, 102, 241);
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: background 0.15s;
	}

	.copy-button:hover {
		background: rgb(79, 70, 229);
	}

	/* Preview Panel */
	.preview-panel {
		background: rgb(249, 250, 251);
		overflow-y: auto;
	}

	:global(.dark) .preview-panel {
		background: rgb(17, 24, 39);
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: rgb(55, 65, 81);
		background: white;
		border-bottom: 1px solid rgb(229, 231, 235);
	}

	:global(.dark) .preview-header {
		background: rgb(31, 41, 55);
		border-color: rgb(55, 65, 81);
		color: rgb(209, 213, 219);
	}

	.preview-content {
		padding: 2rem;
		min-height: 500px;
	}
</style>

