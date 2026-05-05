<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import TablePreviewDrawer from '$lib/components/TablePreviewDrawer.svelte';
	import AutoSuggestCombobox from '$lib/components/AutoSuggestCombobox.svelte';
	import { Database, Table, Trash2, RefreshCw, Eye, Search } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';
	import { parseQueryResult } from '$lib/db-utils';

	// Get database context from layout
	const dbContext = getDbContext();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	interface TableInfo {
		name: string;
		type: 'source' | 'model' | 'system';
		rowCount: number | null;
		isInDatamodels: boolean;
	}

	let allTables = $state<TableInfo[]>([]);
	let isLoading = $state(false);
	
	// Table preview drawer state
	let isPreviewDrawerOpen = $state(false);
	let previewTableName = $state<string | null>(null);

	// Filter state
	let searchQuery = $state('');
	let typeFilter = $state('');

	// Available type options for filter
	const typeOptions = ['All', 'Source', 'Model', 'System'];

	// Filtered tables based on search and type
	const filteredTables = $derived(() => {
		// Filter out system tables by default
		let filtered = allTables.filter(table => table.type !== 'system');

		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(table => 
				table.name.toLowerCase().includes(query)
			);
		}

		// Apply type filter
		if (typeFilter && typeFilter !== 'All') {
			const filterType = typeFilter.toLowerCase() as 'source' | 'model' | 'system';
			filtered = filtered.filter(table => table.type === filterType);
		}

		return filtered;
	});

	async function loadAllTables() {
		try {
			isLoading = true;
			
			// Get all tables from information_schema
			const allTablesJson = await invoke<string>('execute_query', {
				query: 'SELECT table_name FROM information_schema.tables WHERE table_schema = \'main\' ORDER BY table_name'
			});
			const tables: Array<{ table_name: string }> = parseQueryResult(allTablesJson);
			
			// Get datamodels list
			const datamodelsList = await invoke<string[]>('get_saved_tables');
			const datamodelsSet = new Set(datamodelsList);
			
			// Categorize tables
			allTables = await Promise.all(tables.map(async (table) => {
				const tableName = table.table_name;
				
				// Determine type
				let type: 'source' | 'model' | 'system' = 'system';
				if (tableName === 'datamodels' || tableName.startsWith('_warphead')) {
					type = 'system';
				} else if (datamodelsSet.has(tableName)) {
					type = 'model';
				} else {
					type = 'source';
				}
				
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
				
				return {
					name: tableName,
					type,
					rowCount,
					isInDatamodels: datamodelsSet.has(tableName)
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

	function viewTableData(tableName: string) {
		previewTableName = tableName;
		isPreviewDrawerOpen = true;
	}
	
	function closePreviewDrawer() {
		isPreviewDrawerOpen = false;
		previewTableName = null;
	}

	async function dropTable(tableName: string) {
		if (!confirm(`Are you sure you want to drop table '${tableName}'?\n\nThis action cannot be undone!`)) {
			return;
		}
		
		try {
			await invoke<string>('drop_table', { tableName });
			addSystemMessage(`Table '${tableName}' dropped successfully`, 'success');
			await loadAllTables();
			
			// Close drawer if viewing the dropped table
			if (previewTableName === tableName) {
				closePreviewDrawer();
			}
		} catch (error) {
			addSystemMessage(`Error dropping table: ${error}`, 'error');
		}
	}

	function getTableTypeColor(type: 'source' | 'model' | 'system') {
		switch (type) {
			case 'source': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
			case 'model': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300';
			case 'system': return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300';
		}
	}

	function getTableTypeIcon(type: 'source' | 'model' | 'system') {
		switch (type) {
			case 'source': return 'Source';
			case 'model': return 'Model';
			case 'system': return 'System';
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
		<h1 class="font-aspekta text-3xl font-[650] text-slate-900 dark:text-slate-100 mb-2">Manage Tables</h1>
		<p class="text-slate-600 dark:text-slate-400">View and manage all tables in your data model</p>
	</div>
	
	<!-- All Tables Section -->
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

			<!-- Type Filter -->
			<div class="w-full md:w-48">
				<AutoSuggestCombobox
					bind:value={typeFilter}
					options={typeOptions}
					placeholder="Filter by type..."
					onValueChange={(value) => typeFilter = value}
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
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Rows</th>
								<th class="px-6 py-3 text-right text-xs font-medium text-slate-700 dark:text-slate-300 uppercase tracking-wider">Actions</th>
							</tr>
						</thead>
						<tbody class="bg-white dark:bg-slate-900 divide-y divide-slate-200 dark:divide-slate-700">
							{#each filteredTables() as table}
								<tr class="hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors">
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-2">
											<Table class="w-4 h-4 text-slate-400" />
											<span class="text-sm font-medium text-slate-900 dark:text-slate-100">{table.name}</span>
										</div>
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<span class="px-2 py-1 text-xs font-medium rounded-full {getTableTypeColor(table.type)}">
											{getTableTypeIcon(table.type)}
										</span>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
										{table.rowCount !== null ? table.rowCount.toLocaleString() : 'Unknown'}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
										<div class="flex items-center justify-end gap-2">
											<button
												onclick={() => viewTableData(table.name)}
												class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 p-1 rounded hover:bg-blue-50 dark:hover:bg-blue-900/30"
												title="View data"
											>
												<Eye class="w-4 h-4" />
											</button>
											{#if table.type !== 'system'}
												<button
													onclick={() => dropTable(table.name)}
													class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300 p-1 rounded hover:bg-red-50 dark:hover:bg-red-900/30"
													title="Drop table"
												>
													<Trash2 class="w-4 h-4" />
												</button>
											{/if}
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
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
