<script lang="ts">
	import { X, Table as TableIcon, ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	
	interface Props {
		isOpen: boolean;
		tableName: string | null;
		onClose: () => void;
	}
	
	let { isOpen = $bindable(), tableName = $bindable(), onClose }: Props = $props();
	
	let isLoading = $state(false);
	let tableData = $state<any[]>([]);
	let columns = $state<string[]>([]);
	let error = $state<string | null>(null);
	
	// Pagination state
	let currentPage = $state(1);
	let rowsPerPage = 15;
	
	// Computed pagination values
	let totalPages = $derived(Math.ceil(tableData.length / rowsPerPage));
	let paginatedData = $derived(
		tableData.slice(
			(currentPage - 1) * rowsPerPage,
			currentPage * rowsPerPage
		)
	);
	let startRow = $derived((currentPage - 1) * rowsPerPage + 1);
	let endRow = $derived(Math.min(currentPage * rowsPerPage, tableData.length));
	
	// Fetch table data when tableName changes
	$effect(() => {
		if (tableName && isOpen) {
			currentPage = 1; // Reset to first page when opening new table
			loadTableData();
		}
	});
	
	async function loadTableData() {
		if (!tableName) return;
		
		isLoading = true;
		error = null;
		
		try {
			// Fetch first 100 records
			const query = `SELECT * FROM "${tableName}" LIMIT 100`;
			const result = await invoke<string>('execute_query', { query });
			const parsed = JSON.parse(result);
			
			// Handle new response format: { columns: [...], data: [...] }
			if (parsed.columns && parsed.data) {
				columns = parsed.columns;
				tableData = parsed.data;
			} else {
				// Fallback for old format (backward compatibility)
				if (parsed.length > 0) {
					columns = Object.keys(parsed[0]);
					tableData = parsed;
				} else {
					columns = [];
					tableData = [];
				}
			}
		} catch (err) {
			console.error('Error loading table data:', err);
			error = err instanceof Error ? err.message : String(err);
		} finally {
			isLoading = false;
		}
	}
	
	// Format cell value for display
	function formatValue(value: any): string {
		if (value === null || value === undefined) {
			return 'NULL';
		}
		if (typeof value === 'object') {
			return JSON.stringify(value);
		}
		return String(value);
	}
	
	// Pagination controls
	function goToFirstPage() {
		currentPage = 1;
	}
	
	function goToPreviousPage() {
		if (currentPage > 1) {
			currentPage--;
		}
	}
	
	function goToNextPage() {
		if (currentPage < totalPages) {
			currentPage++;
		}
	}
	
	function goToLastPage() {
		currentPage = totalPages;
	}
</script>

<!-- Backdrop -->
{#if isOpen}
	<div 
		class="fixed inset-0 bg-black/50 z-40 transition-opacity"
		style="top: 98px;"
		onclick={onClose}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === 'Escape' && onClose()}
	></div>
{/if}

<!-- Drawer -->
<div 
	class="fixed right-0 bg-white dark:bg-slate-900 shadow-2xl z-50 transition-transform duration-300 ease-in-out border-l border-slate-200 dark:border-slate-800"
	class:translate-x-0={isOpen}
	class:translate-x-full={!isOpen}
	style="width: 50vw; top: 98px; height: calc(100vh - 98px);"
>
	<!-- Header -->
	<div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800">
		<div class="flex items-center gap-2">
			<TableIcon class="w-5 h-5 text-blue-500 dark:text-blue-400" />
			<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
				{tableName || 'Table Preview'}
			</h2>
			<span class="text-sm text-slate-500 dark:text-slate-400">
				(First 100 records)
			</span>
		</div>
		<button
			onclick={onClose}
			class="p-2 hover:bg-slate-200 dark:hover:bg-slate-700 rounded transition-colors"
			aria-label="Close preview"
		>
			<X class="w-5 h-5 text-slate-600 dark:text-slate-400" />
		</button>
	</div>
	
	<!-- Content -->
	<div class="h-[calc(100%-65px)] overflow-auto p-4">
		{#if isLoading}
			<div class="flex items-center justify-center h-full">
				<div class="text-center">
					<svg class="animate-spin h-8 w-8 mx-auto mb-4 text-blue-600 dark:text-blue-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400">Loading table data...</p>
				</div>
			</div>
		{:else if error}
			<div class="flex items-center justify-center h-full">
				<div class="text-center max-w-md">
					<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
						<svg class="w-8 h-8 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
						</svg>
					</div>
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">Error loading data</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">{error}</p>
				</div>
			</div>
		{:else if tableData.length === 0}
			<div class="flex items-center justify-center h-full">
				<div class="text-center max-w-md">
					<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center">
						<TableIcon class="w-8 h-8 text-slate-400 dark:text-slate-600" />
					</div>
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">No data</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">This table is empty</p>
				</div>
			</div>
		{:else}
			<!-- Table -->
			<div class="overflow-x-auto">
				<table class="w-full text-sm border-collapse">
					<thead>
						<tr class="bg-slate-100 dark:bg-slate-800 sticky top-0">
							<th class="px-4 py-2 text-left font-semibold text-slate-900 dark:text-slate-100 border border-slate-300 dark:border-slate-700">
								#
							</th>
							{#each columns as column}
								<th class="px-4 py-2 text-left font-semibold text-slate-900 dark:text-slate-100 border border-slate-300 dark:border-slate-700 whitespace-nowrap">
									{column}
								</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each paginatedData as row, idx}
							<tr class="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
								<td class="px-4 py-2 text-slate-500 dark:text-slate-400 border border-slate-200 dark:border-slate-800 font-mono text-xs">
									{startRow + idx}
								</td>
								{#each columns as column}
									<td class="px-4 py-2 text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-800 max-w-xs truncate">
										<span 
											class:text-slate-400={row[column] === null || row[column] === undefined}
											class:italic={row[column] === null || row[column] === undefined}
										>
											{formatValue(row[column])}
										</span>
									</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			
			<!-- Pagination Controls -->
			{#if tableData.length > rowsPerPage}
				<div class="mt-4 flex items-center justify-between p-3 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg">
					<div class="text-sm text-slate-600 dark:text-slate-400">
						Showing <strong>{startRow}</strong> to <strong>{endRow}</strong> of <strong>{tableData.length}</strong> records
					</div>
					
					<div class="flex items-center gap-2">
						<button
							onclick={goToFirstPage}
							disabled={currentPage === 1}
							class="px-3 py-1.5 text-sm font-medium rounded border transition-colors disabled:opacity-50 disabled:cursor-not-allowed
								bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 
								text-slate-700 dark:text-slate-300
								hover:bg-slate-50 dark:hover:bg-slate-600 disabled:hover:bg-white dark:disabled:hover:bg-slate-700"
							title="First page"
						>
							First
						</button>
						
						<button
							onclick={goToPreviousPage}
							disabled={currentPage === 1}
							class="p-1.5 rounded border transition-colors disabled:opacity-50 disabled:cursor-not-allowed
								bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 
								text-slate-700 dark:text-slate-300
								hover:bg-slate-50 dark:hover:bg-slate-600 disabled:hover:bg-white dark:disabled:hover:bg-slate-700"
							title="Previous page"
						>
							<ChevronLeft class="w-4 h-4" />
						</button>
						
						<div class="px-3 py-1.5 text-sm font-medium text-slate-700 dark:text-slate-300 bg-white dark:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded">
							Page {currentPage} of {totalPages}
						</div>
						
						<button
							onclick={goToNextPage}
							disabled={currentPage === totalPages}
							class="p-1.5 rounded border transition-colors disabled:opacity-50 disabled:cursor-not-allowed
								bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 
								text-slate-700 dark:text-slate-300
								hover:bg-slate-50 dark:hover:bg-slate-600 disabled:hover:bg-white dark:disabled:hover:bg-slate-700"
							title="Next page"
						>
							<ChevronRight class="w-4 h-4" />
						</button>
						
						<button
							onclick={goToLastPage}
							disabled={currentPage === totalPages}
							class="px-3 py-1.5 text-sm font-medium rounded border transition-colors disabled:opacity-50 disabled:cursor-not-allowed
								bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 
								text-slate-700 dark:text-slate-300
								hover:bg-slate-50 dark:hover:bg-slate-600 disabled:hover:bg-white dark:disabled:hover:bg-slate-700"
							title="Last page"
						>
							Last
						</button>
					</div>
				</div>
			{/if}
			
			<!-- Footer info -->
			<div class="mt-4 p-3 bg-blue-50 dark:bg-blue-950/30 border border-blue-200 dark:border-blue-800 rounded-lg">
				<p class="text-sm text-blue-700 dark:text-blue-300">
					Total: <strong>{tableData.length}</strong> record{tableData.length === 1 ? '' : 's'} × <strong>{columns.length}</strong> column{columns.length === 1 ? '' : 's'}
				</p>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Ensure table cells don't wrap unnecessarily */
	td, th {
		white-space: nowrap;
	}
	
	/* Add scrollbar styling */
	.overflow-auto {
		scrollbar-width: thin;
		scrollbar-color: #94a3b8 transparent;
	}
	
	.overflow-auto::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}
	
	.overflow-auto::-webkit-scrollbar-track {
		background: transparent;
	}
	
	.overflow-auto::-webkit-scrollbar-thumb {
		background-color: #94a3b8;
		border-radius: 4px;
	}
	
	.overflow-auto::-webkit-scrollbar-thumb:hover {
		background-color: #64748b;
	}
</style>

