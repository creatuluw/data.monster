<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import TablePreviewDrawer from '$lib/components/TablePreviewDrawer.svelte';
	import AutoSuggestCombobox from '$lib/components/AutoSuggestCombobox.svelte';
	import Select from '$lib/components/Select.svelte';
	import Pagination from '$lib/components/Pagination.svelte';
	import { Database, Table, RefreshCw, Eye, Search, PlayCircle, FileCode } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';
	import { parseQueryResult } from '$lib/db-utils';
	import { goto } from '$app/navigation';

	// Get database context from layout
	const dbContext = getDbContext();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	interface TableIngestionInfo {
		table_name: string;
		ingestion_strategy: string;
		is_ingested: boolean;
		source_path: string | null;
		source_type: string | null;
		ingested_at: string | null;
		ingested_row_count: number | null;
	}

	interface TableInfo {
		name: string;
		type: 'source' | 'model';
		rowCount: number | null;
		columnCount: number | null;
		ingestionStrategy: string;
		isIngested: boolean;
		ingestedAt: string | null;
		ingestedRowCount: number | null;
	}

	let allTables = $state<TableInfo[]>([]);
	let isLoading = $state(false);
	let ingestingTables = $state<string[]>([]);
	
	// Table preview drawer state
	let isPreviewDrawerOpen = $state(false);
	let previewTableName = $state<string | null>(null);

	// Filter state
	let searchQuery = $state('');
	let strategyFilter = $state('');
	
	// Pagination state
	let currentPage = $state(1);
	let pageSize = $state(20);
	const pageSizeOptions = [
		{ value: '10', label: '10 per page' },
		{ value: '20', label: '20 per page' },
		{ value: '50', label: '50 per page' },
		{ value: '100', label: '100 per page' }
	];

	// Available filter options
	const typeOptions = ['All', 'Model'];
	const strategyOptions = ['All', 'Manual', 'On App Load', 'On Query'];

	// Strategy select options
	const strategySelectOptions = [
		{ value: 'manual', label: 'Manual' },
		{ value: 'on_app_load', label: 'On App Load' },
		{ value: 'on_query', label: 'On Query' }
	];

	// Filtered tables based on search and type
	const filteredTables = $derived(() => {
		let filtered = allTables;

		// Always filter out source tables - only show model tables
		filtered = filtered.filter(table => table.type === 'model');

		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(table => 
				table.name.toLowerCase().includes(query)
			);
		}

		// Apply type filter (now only 'All' or 'Model', but 'All' still means model tables only)
		// No need to apply type filter since we're already filtering to models only

		// Apply strategy filter
		if (strategyFilter && strategyFilter !== 'All') {
			const filterStrategy = strategyFilter.toLowerCase().replace(/ /g, '_');
			filtered = filtered.filter(table => table.ingestionStrategy === filterStrategy);
		}

		return filtered;
	});
	
	// Paginated tables
	const paginatedTables = $derived(() => {
		const startIndex = (currentPage - 1) * pageSize;
		const endIndex = startIndex + pageSize;
		return filteredTables().slice(startIndex, endIndex);
	});
	
	// Total pages
	const totalPages = $derived(Math.ceil(filteredTables().length / pageSize));
	
	// Reset to page 1 when filters change
	$effect(() => {
		// Watch these values
		searchQuery;
		strategyFilter;
		// Reset page
		currentPage = 1;
	});
	
	// Reset to page 1 when page size changes
	function handlePageSizeChange(newSize: string) {
		pageSize = parseInt(newSize);
		currentPage = 1;
	}

	async function loadAllTables() {
		try {
			isLoading = true;
			
			// Get all tables from information_schema (excluding system tables)
			const allTablesJson = await invoke<string>('execute_query', {
				query: `SELECT table_name FROM information_schema.tables 
						WHERE table_schema = 'main' 
						AND table_name != 'datamodels' 
						AND table_name NOT LIKE '_warphead%'
						ORDER BY table_name`
			});
			const tables: Array<{ table_name: string }> = parseQueryResult(allTablesJson);
			
			// Get datamodels list
			const datamodelsList = await invoke<string[]>('get_saved_tables');
			const datamodelsSet = new Set(datamodelsList);
			
			// Get ingestion info
			const ingestionInfoJson = await invoke<string>('get_tables_ingestion_info');
			const ingestionInfo: TableIngestionInfo[] = JSON.parse(ingestionInfoJson);
			const ingestionMap = new Map(ingestionInfo.map(info => [info.table_name, info]));
			
			// Categorize tables and get additional info
			allTables = await Promise.all(tables.map(async (table) => {
				const tableName = table.table_name;
				
				// Determine type
				const type: 'source' | 'model' = datamodelsSet.has(tableName) ? 'model' : 'source';
				
				// Get ingestion info
				const ingestionData = ingestionMap.get(tableName);
				
				// Get row count
				let rowCount: number | null = null;
				try {
					const countResult = await invoke<string>('execute_query', {
						query: `SELECT COUNT(*) as count FROM ${tableName}`
					});
					const parsed = parseQueryResult<{ count: number }>(countResult);
					rowCount = parsed[0]?.count || 0;
				} catch (error) {
					console.error(`Failed to get row count for ${tableName}:`, error);
				}
				
				// Get column count
				let columnCount: number | null = null;
				try {
					const columnsResult = await invoke<string>('execute_query', {
						query: `SELECT COUNT(*) as count FROM information_schema.columns 
								WHERE table_schema = 'main' AND table_name = '${tableName}'`
					});
					const parsed = parseQueryResult<{ count: number }>(columnsResult);
					columnCount = parsed[0]?.count || 0;
				} catch (error) {
					console.error(`Failed to get column count for ${tableName}:`, error);
				}
				
				return {
					name: tableName,
					type,
					rowCount,
					columnCount,
					ingestionStrategy: ingestionData?.ingestion_strategy || 'manual',
					isIngested: ingestionData?.is_ingested || false,
					ingestedAt: ingestionData?.ingested_at || null,
					ingestedRowCount: ingestionData?.ingested_row_count || null,
				};
			}));
			
			addSystemMessage(`Loaded ${allTables.length} tables`, 'success');
		} catch (error) {
			console.error('Error loading tables:', error);
			addSystemMessage(`Error loading tables: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function ingestTable(tableName: string) {
		try {
			ingestingTables = [...ingestingTables, tableName];
			
			const result = await invoke<string>('ingest_table', { tableName });
			addSystemMessage(result, 'success');
			await loadAllTables();
		} catch (error) {
			addSystemMessage(`Failed to ingest table: ${error}`, 'error');
		} finally {
			ingestingTables = ingestingTables.filter(t => t !== tableName);
		}
	}

	async function setIngestionStrategy(tableName: string, strategy: string) {
		try {
			await invoke('set_ingestion_strategy', { tableName, strategy });
			addSystemMessage(`Ingestion strategy for '${tableName}' set to '${strategy}'`, 'success');
			await loadAllTables();
		} catch (error) {
			addSystemMessage(`Failed to set ingestion strategy: ${error}`, 'error');
		}
	}

	function viewTableData(tableName: string) {
		previewTableName = tableName;
		isPreviewDrawerOpen = true;
	}
	
	function openInSqlEditor(tableName: string) {
		// Navigate to SQL editor with pre-filled query
		const query = `SELECT * FROM ${tableName} LIMIT 100`;
		goto(`/sql-studio?query=${encodeURIComponent(query)}`);
	}
	
	function closePreviewDrawer() {
		isPreviewDrawerOpen = false;
		previewTableName = null;
	}

	function getTableTypeColor(type: 'source' | 'model') {
		switch (type) {
			case 'source': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
			case 'model': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300';
		}
	}

	function getTableTypeIcon(type: 'source' | 'model') {
		switch (type) {
			case 'source': return 'Source';
			case 'model': return 'Model';
		}
	}

	function getStrategyColor(strategy: string) {
		switch (strategy) {
			case 'manual': return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300';
			case 'on_app_load': return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300';
			case 'on_query': return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300';
			default: return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300';
		}
	}

	function getStrategyLabel(strategy: string) {
		switch (strategy) {
			case 'manual': return 'Manual';
			case 'on_app_load': return 'On App Load';
			case 'on_query': return 'On Query';
			default: return strategy;
		}
	}

	function formatDate(dateStr: string | null) {
		if (!dateStr) return 'Never';
		try {
			return new Date(dateStr).toLocaleString();
		} catch {
			return dateStr;
		}
	}

	onMount(() => {
		// Wait for DB initialization, then load tables
		const checkAndLoad = () => {
			if (dbContext.isInitialized) {
				loadAllTables();
			} else if (!dbContext.isInitializing && dbContext.error) {
				// Only show error if initialization is complete and there's an error
				addSystemMessage('Database not available', 'error');
			} else {
				// Still initializing or needs retry, wait
				setTimeout(checkAndLoad, 100);
			}
		};
		
		checkAndLoad();
	});
</script>

<PageLayout>
	<!-- Header -->
	<div class="mb-8">
		<h1 class="font-aspekta text-3xl font-[650] text-slate-900 dark:text-slate-100 mb-2">Data Loads</h1>
		<p class="text-slate-600 dark:text-slate-400">View and manage data model tables</p>
	</div>
	
	<!-- Tables Section -->
	<section class="mb-8">
		<!-- Filters -->
		<div class="flex flex-col md:flex-row gap-4 mb-6">
			<!-- Search Input -->
			<div class="flex-1 relative">
				<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
					<Search class="w-5 h-5 text-slate-400" />
				</div>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search tables..."
					class="w-full pl-10 pr-4 py-2 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				/>
			</div>

			<!-- Strategy Filter -->
			<div class="w-full md:w-48">
				<AutoSuggestCombobox
					bind:value={strategyFilter}
					options={strategyOptions}
					placeholder="Filter by strategy..."
					onValueChange={(value) => strategyFilter = value}
				/>
			</div>

			<!-- Refresh Button -->
			<Button variant="secondary" onclick={loadAllTables} disabled={isLoading}>
				{#if isLoading}
					<span class="flex items-center gap-2">
						<svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Refreshing...
					</span>
				{:else}
					<span class="flex items-center gap-2">
						<RefreshCw class="w-4 h-4" />
						Refresh
					</span>
				{/if}
			</Button>
		</div>
		
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg border border-slate-200 dark:border-slate-800 overflow-hidden">
			{#if isLoading}
				<div class="p-8 text-center">
					<svg class="animate-spin h-8 w-8 mx-auto text-blue-600 dark:text-blue-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-2">Loading tables...</p>
				</div>
			{:else if filteredTables().length === 0}
				<div class="p-8 text-center">
					<Database class="w-12 h-12 mx-auto text-slate-300 dark:text-slate-600 mb-3" />
					{#if allTables.length === 0}
						<p class="text-slate-600 dark:text-slate-400">No tables found in database</p>
					{:else}
						<p class="text-slate-600 dark:text-slate-400">No tables match your filters</p>
						<p class="text-sm text-slate-500 dark:text-slate-500 mt-1">Try adjusting your search or filter criteria</p>
					{/if}
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-slate-200 dark:divide-slate-700">
						<thead class="bg-slate-50 dark:bg-slate-800">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Table Name</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Type</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Status</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Strategy</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Rows</th>
								<th class="px-6 py-3 text-right text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Actions</th>
							</tr>
						</thead>
						<tbody class="bg-white dark:bg-slate-900 divide-y divide-slate-200 dark:divide-slate-700">
							{#each paginatedTables() as table}
								<tr class="hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors">
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-2">
											<Table class="w-4 h-4 text-slate-400" />
											<div class="flex flex-col">
												<span class="text-sm font-medium text-slate-900 dark:text-slate-100">{table.name}</span>
												<span class="text-xs text-slate-500 dark:text-slate-400">{table.columnCount || 0} columns</span>
											</div>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<span class="px-2 py-1 text-xs font-medium rounded-full {getTableTypeColor(table.type)}">
											{getTableTypeIcon(table.type)}
										</span>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										{#if table.isIngested}
											<span class="text-xs font-medium text-green-600 dark:text-green-400 flex items-center gap-1">
												<div class="w-2 h-2 bg-green-500 rounded-full"></div>
												Ingested
											</span>
										{:else}
											<span class="text-xs font-medium text-slate-500 dark:text-slate-400 flex items-center gap-1">
												<div class="w-2 h-2 bg-slate-400 rounded-full"></div>
												Not Ingested
											</span>
										{/if}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<Select
											options={strategySelectOptions}
											bind:value={table.ingestionStrategy}
											size="sm"
											onchange={(newStrategy) => setIngestionStrategy(table.name, newStrategy)}
										/>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
										{table.rowCount !== null ? table.rowCount.toLocaleString() : 'Unknown'}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
										<div class="flex items-center justify-end gap-2">
											<button
												onclick={() => ingestTable(table.name)}
												disabled={ingestingTables.includes(table.name)}
												class="text-green-600 hover:text-green-900 dark:text-green-400 dark:hover:text-green-300 p-1 rounded hover:bg-green-50 dark:hover:bg-green-900/30 disabled:opacity-50 disabled:cursor-not-allowed"
												title="Load Now"
											>
												{#if ingestingTables.includes(table.name)}
													<svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
														<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
														<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
													</svg>
												{:else}
													<PlayCircle class="w-4 h-4" />
												{/if}
											</button>
											{#if table.isIngested}
												<button
													onclick={() => openInSqlEditor(table.name)}
													class="text-purple-600 hover:text-purple-900 dark:text-purple-400 dark:hover:text-purple-300 p-1 rounded hover:bg-purple-50 dark:hover:bg-purple-900/30"
													title="Open in SQL Editor"
												>
													<FileCode class="w-4 h-4" />
												</button>
											{/if}
											<button
												onclick={() => viewTableData(table.name)}
												class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 p-1 rounded hover:bg-blue-50 dark:hover:bg-blue-900/30"
												title="View data"
											>
												<Eye class="w-4 h-4" />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
				
				<!-- Pagination Controls -->
				<div class="border-t border-slate-200 dark:border-slate-700">
					<Pagination
						bind:currentPage
						bind:pageSize
						totalItems={filteredTables().length}
						pageSizeOptions={pageSizeOptions}
						size="sm"
						onPageChange={(page) => currentPage = page}
						onPageSizeChange={(size) => handlePageSizeChange(size.toString())}
					/>
				</div>
			{/if}
		</div>
	</section>
</PageLayout>

<!-- Table Preview Drawer -->
<TablePreviewDrawer 
	bind:isOpen={isPreviewDrawerOpen} 
	bind:tableName={previewTableName}
	onClose={closePreviewDrawer}
/>
