<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import { 
		Calculator, 
		Plus, 
		Edit, 
		Trash2, 
		Play, 
		X, 
		Check, 
		AlertCircle,
		Copy,
		Tag as TagIcon
	} from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { parseQueryResult } from '$lib/db-utils';

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// Metric Types
	interface Metric {
		slug: string;
		metric_name: string;
		formula: string;
		source_table: string;
		description: string | null;
		tags: string | null;
		created_at: string;
		updated_at: string;
	}

	// State
	let metrics = $state<Metric[]>([]);
	let isLoading = $state(false);
	let isTauriAvailable = $state(false);

	// Form state for creating/editing metrics
	let showCreateModal = $state(false);
	let editingMetric = $state<string | null>(null);
	let metricSlug = $state('');
	let metricName = $state('');
	let formula = $state('');
	let sourceTable = $state('');
	let description = $state('');
	let tags = $state('');
	let isSaving = $state(false);

	// Test state
	let testingMetric = $state<Metric | null>(null);
	let testQuery = $state('');
	let testDimensions = $state<string[]>([]);
	let testFilters = $state('');
	let testUseDimensions = $state(true); // Toggle between dimension mode and custom query mode
	let testResults = $state<any[]>([]);
	let testColumns = $state<string[]>([]);
	let isTestingMetric = $state(false);
	let testError = $state('');
	let generatedQuery = $state(''); // Store the generated SQL query
	
	// Dimension selector state for test panel
	let availableDimensionFields = $state<FieldOption[]>([]);
	let savedDimensions = $state<any[]>([]);
	let isLoadingDimensionFields = $state(false);
	let selectedDimensionToAdd = $state('');

	// Delete confirmation
	let deletingMetric = $state<string | null>(null);

	// Filter state
	let searchTerm = $state('');
	let selectedTag = $state('');

	// Field selector state
	interface FieldOption {
		table: string;
		column: string;
		type: string;
		fullName: string; // table.column
	}
	let availableFields = $state<FieldOption[]>([]);
	let selectedField = $state('');
	let isLoadingFields = $state(false);
	let formulaTextarea: HTMLTextAreaElement | undefined = $state();

	async function loadMetrics() {
		try {
			isLoading = true;
			const result = await invoke<string>('list_metrics');
			metrics = parseQueryResult(result);
		} catch (error) {
			console.error('Error loading metrics:', error);
			addSystemMessage(`Error loading metrics: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function loadAvailableFields() {
		try {
			isLoadingFields = true;
			
			// Get all tables from the data model (model type only)
			const datamodelsList = await invoke<string[]>('get_saved_tables');
			
			// Get columns for each model table
			const fields: FieldOption[] = [];
			
			for (const tableName of datamodelsList) {
				try {
					const columnsResult = await invoke<string>('execute_query', {
						query: `SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '${tableName}' ORDER BY ordinal_position`
					});
					const columns: Array<{ column_name: string; data_type: string }> = parseQueryResult(columnsResult);
					
					for (const col of columns) {
						fields.push({
							table: tableName,
							column: col.column_name,
							type: col.data_type,
							fullName: `${tableName}.${col.column_name}`
						});
					}
				} catch (error) {
					console.error(`Error loading columns for ${tableName}:`, error);
				}
			}
			
			availableFields = fields;
		} catch (error) {
			console.error('Error loading fields:', error);
			addSystemMessage(`Error loading fields: ${error}`, 'error');
		} finally {
			isLoadingFields = false;
		}
	}

	function insertFieldIntoFormula() {
		if (!selectedField || !formulaTextarea) return;
		
		// Extract table and column name from the selected field (table.column)
		const [tableName, columnName] = selectedField.split('.');
		
		// Auto-set source table if not already set
		if (!sourceTable && tableName) {
			sourceTable = tableName;
		}
		
		const textarea = formulaTextarea;
		const start = textarea.selectionStart;
		const end = textarea.selectionEnd;
		const text = formula;
		
		// Insert the column name at cursor position
		const before = text.substring(0, start);
		const after = text.substring(end);
		formula = before + columnName + after;
		
		// Set cursor position after inserted text
		setTimeout(() => {
			textarea.focus();
			const newPos = start + columnName.length;
			textarea.setSelectionRange(newPos, newPos);
		}, 0);
		
		// Reset selection
		selectedField = '';
	}

	function openCreateModal() {
		showCreateModal = true;
		editingMetric = null;
		metricSlug = '';
		metricName = '';
		formula = '';
		sourceTable = '';
		description = '';
		tags = '';
		loadAvailableFields();
	}

	function openEditModal(metric: Metric) {
		showCreateModal = true;
		editingMetric = metric.metric_name;
		metricSlug = metric.slug;
		metricName = metric.metric_name;
		formula = metric.formula;
		sourceTable = metric.source_table;
		description = metric.description || '';
		tags = metric.tags || '';
		loadAvailableFields();
	}

	function closeModal() {
		showCreateModal = false;
		editingMetric = null;
		metricSlug = '';
		metricName = '';
		formula = '';
		sourceTable = '';
		description = '';
		tags = '';
	}

	async function saveMetric() {
		if (!metricName.trim() || !formula.trim() || !sourceTable.trim()) {
			addSystemMessage('Metric name, formula, and source table are required', 'warning');
			return;
		}

		try {
			isSaving = true;
			const result = await invoke<string>('create_metric', {
				metricName: metricName.trim(),
				formula: formula.trim(),
				sourceTable: sourceTable.trim(),
				description: description.trim(),
				tags: tags.trim(),
				existingSlug: metricSlug || null  // Pass slug when editing
			});

			addSystemMessage(result, 'success');
			closeModal();
			await loadMetrics();
		} catch (error) {
			addSystemMessage(`Error saving metric: ${error}`, 'error');
		} finally {
			isSaving = false;
		}
	}

	async function deleteMetric(metricName: string) {
		try {
			const result = await invoke<string>('delete_metric', { metricName });
			addSystemMessage(result, 'success');
			deletingMetric = null;
			await loadMetrics();
		} catch (error) {
			addSystemMessage(`Error deleting metric: ${error}`, 'error');
		}
	}

	function openTestPanel(metric: Metric) {
		testingMetric = metric;
		testUseDimensions = true;
		testDimensions = [];
		testFilters = '';
		// Generate example test query
		testQuery = `SELECT ${metric.formula} AS result FROM ${metric.source_table} LIMIT 10`;
		testResults = [];
		testColumns = [];
		testError = '';
		generatedQuery = '';
		loadDimensionFields(metric.source_table);
	}
	
	async function loadDimensionFields(sourceTable: string) {
		try {
			isLoadingDimensionFields = true;
			
			// Load saved dimensions first
			try {
				const dimensionsResult = await invoke<string>('list_dimensions');
				savedDimensions = parseQueryResult(dimensionsResult);
			} catch (error) {
				console.error('Error loading saved dimensions:', error);
				savedDimensions = [];
			}
			
			// Get all tables from the data model
			const datamodelsList = await invoke<string[]>('get_saved_tables');
			
			// Get columns for each table
			const fields: FieldOption[] = [];
			
			for (const tableName of datamodelsList) {
				try {
					const columnsResult = await invoke<string>('execute_query', {
						query: `SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '${tableName}' ORDER BY ordinal_position`
					});
					const columns: Array<{ column_name: string; data_type: string }> = parseQueryResult(columnsResult);
					
					for (const col of columns) {
						fields.push({
							table: tableName,
							column: col.column_name,
							type: col.data_type,
							fullName: `${tableName}.${col.column_name}`
						});
					}
				} catch (error) {
					console.error(`Error loading columns for ${tableName}:`, error);
				}
			}
			
			availableDimensionFields = fields;
		} catch (error) {
			console.error('Error loading dimension fields:', error);
			addSystemMessage(`Error loading dimension fields: ${error}`, 'error');
		} finally {
			isLoadingDimensionFields = false;
		}
	}
	
	function addDimension() {
		console.log('addDimension called', { selectedDimensionToAdd, currentDimensions: testDimensions });
		if (selectedDimensionToAdd && !testDimensions.includes(selectedDimensionToAdd)) {
			testDimensions = [...testDimensions, selectedDimensionToAdd];
			console.log('Dimension added, new list:', testDimensions);
			selectedDimensionToAdd = '';
		} else {
			console.log('Dimension not added - already exists or empty');
		}
	}
	
	function removeDimension(dimension: string) {
		testDimensions = testDimensions.filter(d => d !== dimension);
	}

	async function testMetric() {
		if (testUseDimensions) {
			// Use dimension mode
			if (!testingMetric) return;
			
			try {
				isTestingMetric = true;
				testError = '';
				testResults = [];
				testColumns = [];
				generatedQuery = '';

				const result = await invoke<string>('execute_metric_with_dimensions', {
					metricName: testingMetric.metric_name,
					dimensions: testDimensions,  // Can be empty array for no dimensions
					filters: testFilters.trim() || null
				});

				// Parse the response which now contains both query and results
				const response = JSON.parse(result);
				generatedQuery = response.query || '';
				const parsed = response.results || [];

				if (parsed.length > 0) {
					testColumns = Object.keys(parsed[0]);
					testResults = parsed;
					addSystemMessage(`Test returned ${parsed.length} row(s)`, 'success');
				} else {
					testColumns = [];
					testResults = [];
					addSystemMessage('Test returned no results', 'info');
				}
			} catch (error) {
				testError = `${error}`;
				addSystemMessage(`Test error: ${error}`, 'error');
			} finally {
				isTestingMetric = false;
			}
		} else {
			// Use custom query mode
			if (!testQuery.trim()) {
				addSystemMessage('Please enter a test query', 'warning');
				return;
			}

			try {
				isTestingMetric = true;
				testError = '';
				testResults = [];
				testColumns = [];
				generatedQuery = testQuery; // In custom mode, show the user's query

				const result = await invoke<string>('test_metric', {
					formula: testingMetric?.formula || '',
					testQuery: testQuery.trim()
				});

				const parsed = parseQueryResult(result);

				if (parsed.length > 0) {
					testColumns = Object.keys(parsed[0]);
					testResults = parsed;
					addSystemMessage(`Test returned ${parsed.length} row(s)`, 'success');
				} else {
					testColumns = [];
					testResults = [];
					addSystemMessage('Test returned no results', 'info');
				}
			} catch (error) {
				testError = `${error}`;
				addSystemMessage(`Test error: ${error}`, 'error');
			} finally {
				isTestingMetric = false;
			}
		}
	}

	function closeTestPanel() {
		testingMetric = null;
		testQuery = '';
		testResults = [];
		testColumns = [];
		testError = '';
		generatedQuery = '';
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		addSystemMessage('Copied to clipboard', 'success');
	}

	// Get unique tags
	const allTags = $derived(
		Array.from(new Set(
			metrics
				.filter(m => m.tags)
				.flatMap(m => m.tags!.split(',').map(t => t.trim()).filter(t => t))
		)).sort()
	);

	// Filtered metrics
	const filteredMetrics = $derived(
		metrics.filter(m => {
			const matchesSearch = !searchTerm || 
				m.metric_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
				(m.description && m.description.toLowerCase().includes(searchTerm.toLowerCase())) ||
				(m.formula && m.formula.toLowerCase().includes(searchTerm.toLowerCase()));
			
			const matchesTag = !selectedTag || 
				(m.tags && m.tags.split(',').map(t => t.trim()).includes(selectedTag));
			
			return matchesSearch && matchesTag;
		})
	);

	onMount(async () => {
		try {
			await invoke('initialize_duckdb');
			isTauriAvailable = true;
			addSystemMessage('Connected to database', 'success');
			await loadMetrics();
		} catch (error) {
			if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
				isTauriAvailable = false;
				addSystemMessage('Tauri API not available. Please run: npm run tauri:dev', 'error');
			} else {
				isTauriAvailable = true;
				addSystemMessage('Database connection issue. Some features may not work.', 'warning');
				console.error('DuckDB initialization error:', error);
			}
		}
	});
</script>

<PageLayout>
	<div class="flex items-start justify-between mb-6 gap-4">
		<div class="flex items-center gap-3">
			<Button variant="ghost" onclick={() => window.location.href = '/models/functions'}>
				View Functions
			</Button>
			<Button variant="primary" onclick={openCreateModal} disabled={!isTauriAvailable}>
				<Plus class="w-4 h-4 mr-2" />
				New Metric
			</Button>
		</div>
	</div>

	{#if isLoading}
		<div class="flex items-center justify-center h-64">
			<div class="text-slate-500 dark:text-slate-400">Loading metrics...</div>
		</div>
	{:else if metrics.length === 0}
		<div class="flex flex-col items-center justify-center h-64 text-center">
			<Calculator class="w-16 h-16 text-slate-300 dark:text-slate-600 mb-4" />
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				No metrics yet
			</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-6 max-w-md">
				Create your first metric using DuckDB's built-in aggregations (<code class="px-1 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs">sum</code>, <code class="px-1 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs">avg</code>, <code class="px-1 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs">count</code>) or custom functions.
			</p>
			<Button variant="primary" onclick={openCreateModal} disabled={!isTauriAvailable}>
				<Plus class="w-4 h-4 mr-2" />
				Create Metric
			</Button>
		</div>
	{:else}
		<!-- Search and Filter -->
		<div class="mb-6 flex items-center gap-4 flex-wrap">
			<input
				type="text"
				bind:value={searchTerm}
				placeholder="Search metrics..."
				class="flex-1 min-w-[200px] px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
			/>
			{#if allTags.length > 0}
				<select
					bind:value={selectedTag}
					class="px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
				>
					<option value="">All Tags</option>
					{#each allTags as tag}
						<option value={tag}>{tag}</option>
					{/each}
				</select>
			{/if}
			<span class="text-sm text-slate-500 dark:text-slate-400">
				{filteredMetrics.length} metric{filteredMetrics.length === 1 ? '' : 's'}
			</span>
		</div>

		<!-- Metrics Grid -->
		<div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
			{#each filteredMetrics as metric}
				<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-5 hover:shadow-md transition-shadow">
					<!-- Header -->
					<div class="flex items-start justify-between mb-3">
						<div class="flex-1">
							<div class="flex items-center gap-2 mb-1">
								<Calculator class="w-5 h-5 text-indigo-500" />
								<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
									{metric.metric_name}
								</h3>
							</div>
							{#if metric.description}
								<p class="text-sm text-slate-600 dark:text-slate-400 mb-2">
									{metric.description}
								</p>
							{/if}
						</div>
						<div class="flex items-center gap-2">
							<button
								type="button"
								onclick={() => openTestPanel(metric)}
								class="p-2 text-slate-400 hover:text-indigo-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
								title="Test metric"
							>
								<Play class="w-4 h-4" />
							</button>
							<button
								type="button"
								onclick={() => openEditModal(metric)}
								class="p-2 text-slate-400 hover:text-blue-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
								title="Edit metric"
							>
								<Edit class="w-4 h-4" />
							</button>
							<button
								type="button"
								onclick={() => deletingMetric = metric.metric_name}
								class="p-2 text-slate-400 hover:text-red-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
								title="Delete metric"
							>
								<Trash2 class="w-4 h-4" />
							</button>
						</div>
					</div>

					<!-- Formula -->
					<div class="bg-slate-50 dark:bg-slate-950 rounded p-3 mb-3">
						<div class="flex items-center justify-between mb-2">
							<span class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase">Formula</span>
							<button
								type="button"
								onclick={() => copyToClipboard(metric.formula)}
								class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
								title="Copy formula"
							>
								<Copy class="w-3.5 h-3.5" />
							</button>
						</div>
						<pre class="text-sm font-mono text-slate-700 dark:text-slate-300 whitespace-pre-wrap break-words">{metric.formula}</pre>
					</div>

					<!-- Source Table -->
					<div class="mb-3">
						<span class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase">Source Table:</span>
						<span class="ml-2 text-sm font-mono text-slate-700 dark:text-slate-300">{metric.source_table}</span>
					</div>

					<!-- Tags -->
					{#if metric.tags}
						<div class="flex items-center gap-2 flex-wrap mb-3">
							{#each metric.tags.split(',').map(t => t.trim()).filter(t => t) as tag}
								<span class="inline-flex items-center gap-1 text-xs px-2 py-1 rounded-full bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400 border border-blue-200 dark:border-blue-800">
									<TagIcon class="w-3 h-3" />
									{tag}
								</span>
							{/each}
						</div>
					{/if}

					<!-- Metadata -->
					<div class="text-xs text-slate-500 dark:text-slate-400">
						Updated: {new Date(metric.updated_at).toLocaleDateString()}
					</div>

					<!-- Delete Confirmation -->
					{#if deletingMetric === metric.metric_name}
						<div class="mt-3 pt-3 border-t border-slate-200 dark:border-slate-800">
							<div class="flex items-center gap-2 text-sm">
								<AlertCircle class="w-4 h-4 text-red-500" />
								<span class="text-slate-700 dark:text-slate-300 flex-1">Delete this metric?</span>
								<button
									type="button"
									onclick={() => deleteMetric(metric.metric_name)}
									class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
								>
									<Check class="w-4 h-4" />
								</button>
								<button
									type="button"
									onclick={() => deletingMetric = null}
									class="px-3 py-1 bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300 rounded hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors"
								>
									<X class="w-4 h-4" />
								</button>
							</div>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</PageLayout>

<!-- Create/Edit Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
					{editingMetric ? 'Edit Metric' : 'Create New Metric'}
				</h2>
			</div>

			<div class="p-6 space-y-4">
				<!-- Metric Name -->
				<div>
					<label for="metric-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Title *
					</label>
				<input
					id="metric-name"
					type="text"
					bind:value={metricName}
					placeholder="Total Sales"
					class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
				/>
				</div>

				<!-- Slug (read-only, only shown when editing) -->
				{#if editingMetric && metricSlug}
					<div>
						<label for="metric-slug" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
							Slug (Unique Identifier)
						</label>
						<input
							id="metric-slug"
							type="text"
							value={metricSlug}
							class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-slate-50 dark:bg-slate-800 text-slate-500 dark:text-slate-400 font-mono text-sm cursor-not-allowed"
							readonly
							disabled
						/>
						<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
							The slug is auto-generated and cannot be changed
						</p>
					</div>
				{/if}

				<!-- Formula -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<label for="metric-formula" class="block text-sm font-medium text-slate-700 dark:text-slate-300">
							Calculation / Formula *
						</label>
						<div class="flex items-center gap-2">
							<select
								bind:value={selectedField}
								class="text-xs px-2 py-1 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
								disabled={isLoadingFields || availableFields.length === 0}
							>
								<option value="">
									{isLoadingFields ? 'Loading fields...' : availableFields.length === 0 ? 'No model tables' : 'Select field'}
								</option>
								{#if availableFields.length > 0}
									{@const tableGroups = availableFields.reduce((acc, field) => {
										if (!acc[field.table]) acc[field.table] = [];
										acc[field.table].push(field);
										return acc;
									}, {} as Record<string, FieldOption[]>)}
									{#each Object.entries(tableGroups) as [tableName, fields]}
										<optgroup label={tableName}>
											{#each fields as field}
												<option value={field.fullName}>
													{field.column} ({field.type})
												</option>
											{/each}
										</optgroup>
									{/each}
								{/if}
							</select>
							<button
								type="button"
								onclick={insertFieldIntoFormula}
								disabled={!selectedField}
								class="p-1.5 rounded bg-indigo-500 text-white hover:bg-indigo-600 disabled:bg-slate-300 dark:disabled:bg-slate-700 disabled:cursor-not-allowed transition-colors"
								title="Insert field into formula"
							>
								<Plus class="w-4 h-4" />
							</button>
						</div>
					</div>
					<textarea
						id="metric-formula"
						bind:this={formulaTextarea}
						bind:value={formula}
						placeholder="sum(sales_amount)"
						rows="4"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
					></textarea>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Use DuckDB built-in functions: sum(), avg(), count(), min(), max(). Or use custom functions from Functions library.
					</p>
				</div>

				<!-- Source Table -->
				<div>
					<label for="source-table" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Source Table *
					</label>
					<input
						id="source-table"
						type="text"
						bind:value={sourceTable}
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-slate-50 dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
						readonly
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Automatically set when you insert a field from the formula builder.
					</p>
				</div>

				<!-- Description -->
				<div>
					<label for="metric-description" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Description
					</label>
					<textarea
						id="metric-description"
						bind:value={description}
						placeholder="What does this metric measure?"
						rows="2"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					></textarea>
				</div>

				<!-- Tags -->
				<div>
					<label for="metric-tags" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Tags
					</label>
					<input
						id="metric-tags"
						type="text"
						bind:value={tags}
						placeholder="sales, revenue, kpi"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Comma-separated tags for organizing metrics
					</p>
				</div>
			</div>

			<div class="p-6 border-t border-slate-200 dark:border-slate-800 flex items-center justify-end gap-3">
				<Button variant="ghost" onclick={closeModal}>
					Cancel
				</Button>
				<Button variant="primary" onclick={saveMetric} disabled={isSaving}>
					{#if isSaving}
						Saving...
					{:else}
						{editingMetric ? 'Update Metric' : 'Create Metric'}
					{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Test Panel -->
{#if testingMetric}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-5xl w-full max-h-[90vh] flex flex-col">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<Play class="w-5 h-5 text-indigo-500" />
					Test Metric: {testingMetric.metric_name}
				</h2>
				<button
					type="button"
					onclick={closeTestPanel}
					class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
				>
					<X class="w-6 h-6" />
				</button>
			</div>

			<div class="p-6 space-y-4 flex-1 overflow-auto">
				<!-- Formula and Source Table Display -->
				<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
					<div class="text-sm font-medium text-blue-900 dark:text-blue-100 mb-2">
						Formula: <code class="font-mono">{testingMetric.formula}</code>
					</div>
					<div class="text-sm text-blue-800 dark:text-blue-200">
						Source Table: <code class="font-mono">{testingMetric.source_table}</code>
					</div>
				</div>

				<!-- Mode Toggle -->
				<div class="flex items-center gap-4 p-3 bg-slate-50 dark:bg-slate-800 rounded-lg">
					<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Test Mode:</span>
					<label class="flex items-center gap-2 cursor-pointer">
						<input
							type="radio"
							bind:group={testUseDimensions}
							value={true}
							class="w-4 h-4 text-indigo-600"
						/>
						<span class="text-sm text-slate-900 dark:text-slate-100">With Dimensions (Automatic JOINs)</span>
					</label>
					<label class="flex items-center gap-2 cursor-pointer">
						<input
							type="radio"
							bind:group={testUseDimensions}
							value={false}
							class="w-4 h-4 text-indigo-600"
						/>
						<span class="text-sm text-slate-900 dark:text-slate-100">Custom Query</span>
					</label>
				</div>

				{#if testUseDimensions}
					<!-- Dimension Selector Mode -->
					<div>
						<label for="dimension-selector" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
							Select Dimensions
						</label>
						<div class="flex items-start gap-2">
							<select
								id="dimension-selector"
								bind:value={selectedDimensionToAdd}
								class="flex-1 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
								disabled={isLoadingDimensionFields}
							>
								<option value="">
									{isLoadingDimensionFields ? 'Loading fields...' : 'Select a dimension field'}
								</option>
								
								<!-- Saved Dimensions (shown first) -->
								{#if savedDimensions.length > 0}
									<optgroup label="📌 Saved Dimensions">
										{#each savedDimensions as dimension}
											<option value={dimension.field_name}>
												{dimension.dimension_name}
											</option>
										{/each}
									</optgroup>
								{/if}
								
								<!-- Regular Table Fields -->
								{#if availableDimensionFields.length > 0}
									{@const tableGroups = availableDimensionFields.reduce((acc, field) => {
										if (!acc[field.table]) acc[field.table] = [];
										acc[field.table].push(field);
										return acc;
									}, {} as Record<string, FieldOption[]>)}
									{#each Object.entries(tableGroups) as [tableName, fields]}
										<optgroup label={tableName}>
											{#each fields as field}
												<option value={field.fullName}>
													{field.column} ({field.type})
												</option>
											{/each}
										</optgroup>
									{/each}
								{/if}
							</select>
							<button
								type="button"
								onclick={addDimension}
								disabled={!selectedDimensionToAdd}
								class="px-4 py-2 rounded bg-indigo-500 text-white hover:bg-indigo-600 disabled:bg-slate-300 dark:disabled:bg-slate-700 disabled:cursor-not-allowed transition-colors whitespace-nowrap"
							>
								<Plus class="w-4 h-4 inline mr-1" />
								Add
							</button>
						</div>
						<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
							Saved dimensions appear first, followed by all table fields. The system will automatically JOIN related tables.
						</p>
					</div>

					<!-- Selected Dimensions -->
					{#if testDimensions.length > 0}
						<div>
							<span class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
								Selected Dimensions:
							</span>
							<div class="flex flex-wrap gap-2">
								{#each testDimensions as dimension}
									<span class="inline-flex items-center gap-2 px-3 py-1.5 bg-indigo-50 dark:bg-indigo-900/20 text-indigo-700 dark:text-indigo-400 border border-indigo-200 dark:border-indigo-800 rounded-full text-sm">
										<code class="font-mono">{dimension}</code>
										<button
											type="button"
											onclick={() => removeDimension(dimension)}
											class="hover:text-indigo-900 dark:hover:text-indigo-200"
											title="Remove dimension"
										>
											<X class="w-3.5 h-3.5" />
										</button>
									</span>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Filters (WHERE clause) -->
					<div>
						<label for="test-filters-input" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
							Filters (Optional WHERE conditions)
						</label>
						<input
							id="test-filters-input"
							type="text"
							bind:value={testFilters}
							placeholder="e.g., year = 2024 AND status = 'active'"
							class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
						/>
						<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
							Add WHERE conditions without the "WHERE" keyword
						</p>
					</div>
				{:else}
					<!-- Custom Query Mode -->
					<div>
						<label for="test-query-input" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
							Custom SQL Query
						</label>
						<textarea
							id="test-query-input"
							bind:value={testQuery}
							rows="4"
							placeholder="SELECT {testingMetric.formula} AS result FROM {testingMetric.source_table} LIMIT 10"
							class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
						></textarea>
						<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
							Write a complete SQL query to test the metric formula
						</p>
					</div>
				{/if}

				<div class="flex gap-2">
					<Button variant="primary" onclick={testMetric} disabled={isTestingMetric}>
						{#if isTestingMetric}
							Running...
						{:else}
							<Play class="w-4 h-4 mr-2" />
							Run Test
						{/if}
					</Button>
				</div>

				<!-- Generated Query Display -->
				{#if generatedQuery}
					<div class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
						<div class="flex items-center justify-between mb-2">
							<span class="text-sm font-semibold text-slate-700 dark:text-slate-300">Generated SQL Query</span>
							<button
								type="button"
								onclick={() => copyToClipboard(generatedQuery)}
								class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
								title="Copy query"
							>
								<Copy class="w-4 h-4" />
							</button>
						</div>
						<pre class="text-sm font-mono text-slate-700 dark:text-slate-300 whitespace-pre-wrap break-words bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded p-3 overflow-x-auto">{generatedQuery}</pre>
					</div>
				{/if}

				<!-- Test Results -->
				{#if testError}
					<div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
						<p class="text-sm text-red-700 dark:text-red-400 font-mono">{testError}</p>
					</div>
				{:else if testResults.length > 0}
					<div class="border border-slate-200 dark:border-slate-800 rounded-lg overflow-hidden">
						<div class="overflow-x-auto max-h-96">
							<table class="w-full text-sm">
								<thead class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 sticky top-0">
									<tr>
										{#each testColumns as column}
											<th class="px-4 py-2 text-left font-medium text-slate-900 dark:text-slate-100 whitespace-nowrap">
												{column}
											</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each testResults as row}
										<tr class="border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
											{#each testColumns as column}
												<td class="px-4 py-2 text-slate-700 dark:text-slate-300 whitespace-nowrap font-mono text-xs">
													{row[column] ?? 'NULL'}
												</td>
											{/each}
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				{:else if !isTestingMetric}
					<div class="flex items-center justify-center h-32 text-slate-500 dark:text-slate-400 text-sm">
						Edit the query and run test to see results
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}
