<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	// State
	let tables = $state<string[]>([]);
	let selectedTable = $state<string>('');
	let tableData = $state<any[]>([]);
	let columns = $state<string[]>([]);
	let isLoading = $state(false);
	let error = $state('');
	let rowCount = $state(0);
	let tableSizes = $state<Record<string, string>>({});
	
	// Pagination state
	let currentPage = $state(1);
	let rowsPerPage = $state(10);
	let totalRows = $state(0);
	
	// Computed pagination values
	let totalPages = $derived(Math.ceil(totalRows / rowsPerPage));
	let paginatedData = $derived(tableData.slice((currentPage - 1) * rowsPerPage, currentPage * rowsPerPage));
	let startRow = $derived((currentPage - 1) * rowsPerPage + 1);
	let endRow = $derived(Math.min(currentPage * rowsPerPage, tableData.length));
	
	// Custom dropdown state
	let isDropdownOpen = $state(false);

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
	}

	async function loadTables() {
		try {
			tables = await invoke<string[]>('list_tables');
			
			// Load sizes for all tables
			const sizePromises = tables.map(async (table) => {
				try {
					const size = await invoke<number>('get_table_size', { tableName: table });
					return { table, size: formatBytes(size) };
				} catch {
					return { table, size: '' };
				}
			});
			
			const sizes = await Promise.all(sizePromises);
			tableSizes = sizes.reduce((acc, { table, size }) => {
				acc[table] = size;
				return acc;
			}, {} as Record<string, string>);
			
			// If a table was pre-selected from URL or is already selected, load its data
			if (selectedTable && tables.includes(selectedTable)) {
				await loadTableData();
			} else if (selectedTable && tables.length > 0) {
				// Table from URL doesn't exist - show helpful error and select first table
				error = `Table "${selectedTable}" not found. Available tables: ${tables.join(', ')}`;
				selectedTable = tables[0];
				await loadTableData();
			} else if (tables.length > 0 && !selectedTable) {
				// Otherwise, select the first table if none is selected
				selectedTable = tables[0];
				await loadTableData();
			}
		} catch (err) {
			error = `Error loading tables: ${err}`;
		}
	}

	async function loadTableData() {
		if (!selectedTable) return;

		try {
			isLoading = true;
			error = '';
			
			// Query to get the data
			const query = `SELECT * FROM ${selectedTable} LIMIT 1000`;
			const result = await invoke<string>('execute_query', { query });
			
			// Parse the result
			const parsed = JSON.parse(result);
			
			// Handle new response format: { columns: [...], data: [...] }
			if (parsed.columns && parsed.data) {
				columns = parsed.columns;
				tableData = parsed.data;
			} else {
				// Fallback for old format (backward compatibility)
				tableData = parsed;
				if (tableData.length > 0) {
					columns = Object.keys(tableData[0]);
				} else {
					columns = [];
				}
			}
			
			rowCount = tableData.length;
			totalRows = tableData.length;
			currentPage = 1; // Reset to first page when loading new data
		} catch (err) {
			error = `Error loading table data: ${err}`;
			tableData = [];
			columns = [];
			totalRows = 0;
		} finally {
			isLoading = false;
		}
	}

	async function handleTableChange() {
		await loadTableData();
	}
	
	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}
	
	function handleRowsPerPageChange() {
		currentPage = 1; // Reset to first page when changing rows per page
	}
	
	async function selectTable(table: string) {
		selectedTable = table;
		isDropdownOpen = false;
		await loadTableData();
	}
	
	function toggleDropdown() {
		isDropdownOpen = !isDropdownOpen;
	}
	
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.custom-dropdown')) {
			isDropdownOpen = false;
		}
	}

	onMount(() => {
		// Get parameters from URL
		const urlParams = new URLSearchParams(window.location.search);
		const filenameParam = urlParams.get('filename');
		const folderParam = urlParams.get('folder');
		const tableParam = urlParams.get('table');
		
		// If filename and folder are provided, load that specific file
		if (filenameParam && folderParam) {
			const filename = decodeURIComponent(filenameParam);
			const folder = decodeURIComponent(folderParam);
			
			// Check if this is a database table (not a physical file to upload)
			const isDatabaseTable = folder === 'database' || filename.endsWith('.db');
			
			(async () => {
				try {
					isLoading = true;
					error = '';
					
					if (isDatabaseTable) {
						// This is an attached database table - directly select it
						// The table name is the filename without the .db extension
						selectedTable = filename.replace(/\.db$/, '');
						await loadTables();
					} else {
						// This is a physical file - load it into DuckDB
						const result = await invoke<string>('load_file_by_name', { 
							filename, 
							folder 
						});
						
						// Extract table name from result (format: "Loaded X rows into table 'Y' (Table: Y)")
						const tableMatch = result.match(/Table: (.+)\)$/);
						if (tableMatch) {
							selectedTable = tableMatch[1];
						} else {
							// Fallback: generate table name from filename
							selectedTable = filename
								.replace(/\.[^/.]+$/, '')
								.replace(/[^a-zA-Z0-9_]/g, '_');
						}
						
						await loadTables();
					}
				} catch (err) {
					error = `Error loading file: ${err}`;
				} finally {
					isLoading = false;
				}
			})();
		} else if (tableParam) {
			// Fallback to old behavior with table parameter
			selectedTable = decodeURIComponent(tableParam);
			loadTables();
		} else {
			loadTables();
		}
		
		document.addEventListener('click', handleClickOutside);
		
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});
</script>

<PageLayout>
	<!-- Table Selection -->
		{#if tables.length > 0}
			<div class="bg-white rounded-lg shadow-lg p-6 mb-6">
				<div class="flex items-center gap-4">
					<label for="table-select" class="text-sm font-medium text-slate-700 whitespace-nowrap">
						Select Table:
					</label>
					
					<!-- Custom Dropdown -->
					<div class="relative flex-1 max-w-xs custom-dropdown">
						<button
							type="button"
							onclick={toggleDropdown}
							class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white text-left flex items-center justify-between"
						>
							<span class="flex items-center gap-2">
								<span class="text-slate-900">{selectedTable}</span>
								{#if tableSizes[selectedTable]}
									<span class="text-xs" style="color: #bbb;">({tableSizes[selectedTable]})</span>
								{/if}
							</span>
							<svg class="w-4 h-4 text-slate-500 transition-transform {isDropdownOpen ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
							</svg>
						</button>
						
						{#if isDropdownOpen}
							<div class="absolute z-10 w-full mt-1 bg-white border border-slate-300 rounded-lg shadow-lg max-h-60 overflow-y-auto">
								{#each tables as table}
									<button
										type="button"
										onclick={() => selectTable(table)}
										class="w-full px-4 py-2 text-left hover:bg-slate-50 flex items-center justify-between transition-colors {table === selectedTable ? 'bg-blue-50' : ''}"
									>
										<span class="text-slate-900">{table}</span>
										{#if tableSizes[table]}
											<span class="text-xs" style="color: #bbb;">({tableSizes[table]})</span>
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
					
					<button
						onclick={loadTableData}
						disabled={isLoading || !selectedTable}
						class="btn bg-blue-600 text-white hover:bg-blue-700 disabled:bg-slate-400"
					>
						{#if isLoading}
							Refreshing...
						{:else}
							🔄 Refresh
						{/if}
					</button>
				</div>
			</div>
		{:else}
			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-6 text-center">
				<p class="text-yellow-800 mb-4">No tables found in the database.</p>
				<a href="/" class="btn bg-blue-600 text-white hover:bg-blue-700">
					Upload Data
				</a>
			</div>
		{/if}

		<!-- Error Message -->
		{#if error}
			<div class="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
				<p class="text-red-700 text-sm">{error}</p>
			</div>
		{/if}

		<!-- Loading State -->
		{#if isLoading}
			<div class="bg-white rounded-lg shadow-lg p-12 text-center">
				<div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mb-4"></div>
				<p class="text-slate-600">Loading table data...</p>
			</div>
		{/if}

		<!-- Table Data -->
		{#if !isLoading && tableData.length > 0}
			<div class="bg-white rounded-lg shadow-lg overflow-hidden">
				<!-- Table Info -->
				<div class="px-6 py-4 bg-slate-50 border-b border-slate-200">
					<div class="flex items-center justify-between">
						<div>
							<h2 class="text-lg font-semibold text-slate-900">
								{selectedTable}
							</h2>
							<p class="text-sm text-slate-600">
								{rowCount} {rowCount === 1 ? 'row' : 'rows'} • {columns.length} {columns.length === 1 ? 'column' : 'columns'}
								{#if rowCount === 1000}
									<span class="text-orange-600">(showing first 1,000 rows)</span>
								{/if}
							</p>
						</div>
						<div class="flex items-center gap-3">
							<label for="rows-per-page" class="text-sm font-medium text-slate-700 whitespace-nowrap">
								Rows per page:
							</label>
							<select
								id="rows-per-page"
								bind:value={rowsPerPage}
								onchange={handleRowsPerPageChange}
								class="px-3 py-1.5 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm"
							>
								<option value={5}>5</option>
								<option value={10}>10</option>
								<option value={25}>25</option>
								<option value={50}>50</option>
								<option value={100}>100</option>
							</select>
						</div>
					</div>
				</div>

				<!-- Table -->
				<div class="overflow-x-auto">
					<table class="w-full">
						<thead class="bg-slate-100 border-b border-slate-200">
							<tr>
								{#each columns as column}
									<th class="px-6 py-3 text-left text-xs font-medium text-slate-700 uppercase tracking-wider whitespace-nowrap">
										{column}
									</th>
								{/each}
							</tr>
						</thead>
						<tbody class="divide-y divide-slate-200">
							{#each paginatedData as row, i}
								<tr class="hover:bg-slate-50 transition-colors">
									{#each columns as column}
										<td class="px-6 py-4 text-sm text-slate-900 whitespace-nowrap">
											{row[column] ?? 'NULL'}
										</td>
									{/each}
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
				
				<!-- Pagination Controls -->
				{#if totalPages > 1}
					<div class="px-6 py-4 bg-slate-50 border-t border-slate-200">
						<div class="flex items-center justify-between">
							<!-- Page Info -->
							<p class="text-sm text-slate-700">
								Showing <span class="font-medium">{startRow}</span> to <span class="font-medium">{endRow}</span> of <span class="font-medium">{totalRows}</span> results
							</p>
							
							<!-- Page Controls -->
							<div class="flex items-center gap-2">
								<!-- Previous Button -->
								<button
									onclick={() => goToPage(currentPage - 1)}
									disabled={currentPage === 1}
									class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
								>
									← Previous
								</button>
								
								<!-- Page Numbers -->
								<div class="flex items-center gap-1">
									{#if currentPage > 2}
										<button
											onclick={() => goToPage(1)}
											class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors"
										>
											1
										</button>
										{#if currentPage > 3}
											<span class="px-2 text-slate-500">...</span>
										{/if}
									{/if}
									
									{#if currentPage > 1}
										<button
											onclick={() => goToPage(currentPage - 1)}
											class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors"
										>
											{currentPage - 1}
										</button>
									{/if}
									
									<button
										class="px-3 py-1.5 border-2 border-blue-600 bg-blue-50 rounded-lg text-sm font-medium text-blue-600"
									>
										{currentPage}
									</button>
									
									{#if currentPage < totalPages}
										<button
											onclick={() => goToPage(currentPage + 1)}
											class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors"
										>
											{currentPage + 1}
										</button>
									{/if}
									
									{#if currentPage < totalPages - 1}
										{#if currentPage < totalPages - 2}
											<span class="px-2 text-slate-500">...</span>
										{/if}
										<button
											onclick={() => goToPage(totalPages)}
											class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors"
										>
											{totalPages}
										</button>
									{/if}
								</div>
								
								<!-- Next Button -->
								<button
									onclick={() => goToPage(currentPage + 1)}
									disabled={currentPage === totalPages}
									class="px-3 py-1.5 border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
								>
									Next →
								</button>
							</div>
						</div>
					</div>
				{/if}
			</div>
		{:else if !isLoading && selectedTable && tableData.length === 0}
			<div class="bg-slate-50 border border-slate-200 rounded-lg p-12 text-center">
				<p class="text-slate-600 mb-2">No data found in table "{selectedTable}"</p>
				<p class="text-sm text-slate-500">The table might be empty.</p>
			</div>
		{/if}
</PageLayout>

