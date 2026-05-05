<script lang="ts">
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import Label from '$lib/components/Label.svelte';
	import { Download, FileOutput } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';

	// Get database context from layout
	const dbContext = getDbContext();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// State
	let isLoading = $state(false);
	let selectedTable = $state('');
	let availableTables = $state<string[]>([]);
	let selectedFormat = $state('csv');
	
	const exportFormats = [
		{ value: 'csv', label: 'CSV', description: 'Comma-separated values' },
		{ value: 'parquet', label: 'Parquet', description: 'Columnar format' },
		{ value: 'json', label: 'JSON', description: 'JSON array' },
		{ value: 'jsonl', label: 'JSONL', description: 'JSON Lines' }
	];

	async function loadTables() {
		// TODO: Implement table loading from DuckDB
		addSystemMessage('Loading tables...', 'info');
	}

	async function exportData() {
		if (!selectedTable) {
			addSystemMessage('Please select a table to export', 'warning');
			return;
		}

		try {
			isLoading = true;
			addSystemMessage(`Exporting ${selectedTable} as ${selectedFormat.toUpperCase()}...`, 'info');
			
			// TODO: Implement export functionality
			await new Promise(resolve => setTimeout(resolve, 1000)); // Placeholder delay
			
			addSystemMessage('Export functionality coming soon!', 'warning');
		} catch (error) {
			addSystemMessage(`Export failed: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		// Wait for DB initialization, then load tables
		const checkAndLoad = () => {
			if (dbContext.isInitialized) {
				loadTables();
			} else if (!dbContext.isInitializing) {
				// Initialization complete but not successful
				return;
			} else {
				// Still initializing, wait
				setTimeout(checkAndLoad, 100);
			}
		};
		
		checkAndLoad();
	});
</script>

<PageLayout>
	{#if !dbContext.isInitializing && dbContext.error && !dbContext.isTauriAvailable}
		<div class="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg mb-8">
			<h3 class="text-sm font-semibold text-red-900 dark:text-red-300 mb-2">⚠️ Tauri Runtime Not Detected</h3>
			<p class="text-sm text-red-700 dark:text-red-400 mb-2">
				The Tauri API is not available. You need to run the full Tauri application.
			</p>
			<div class="bg-red-100 dark:bg-red-900/40 p-3 rounded font-mono text-xs text-red-800 dark:text-red-300">
				npm run tauri:dev
			</div>
		</div>
	{/if}

	<!-- Main Convert Section -->
	<div class="max-w-3xl mx-auto">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-slate-900 dark:text-slate-100 mb-2">Convert & Export Data</h1>
			<p class="text-slate-600 dark:text-slate-400">Export your data tables to various file formats</p>
		</div>

		<!-- Table Selection -->
		<div class="mb-8">
			<Label for="table-select">
				Select Table to Export
			</Label>
			<select
				id="table-select"
				bind:value={selectedTable}
				disabled={!dbContext.isInitialized || availableTables.length === 0}
				class="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-slate-100 dark:disabled:bg-slate-800 disabled:cursor-not-allowed"
			>
				<option value="">-- Select a table --</option>
				{#each availableTables as table}
					<option value={table}>{table}</option>
				{/each}
			</select>
			{#if availableTables.length === 0 && dbContext.isInitialized}
				<p class="text-sm text-slate-500 dark:text-slate-400 mt-2">
					No tables available. Upload or connect to data sources first.
				</p>
			{/if}
		</div>

		<!-- Format Selection -->
		<div class="mb-8">
			<Label>
				Export Format
			</Label>
			<div class="grid grid-cols-2 gap-4 mt-2">
				{#each exportFormats as format}
					<button
						type="button"
						onclick={() => selectedFormat = format.value}
						class="flex items-center gap-3 p-4 border rounded-lg text-left transition-all {
							selectedFormat === format.value
								? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
								: 'border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600'
						}"
					>
						<div class="flex-grow">
							<div class="text-sm font-semibold text-slate-900 dark:text-slate-100">{format.label}</div>
							<div class="text-xs text-slate-500 dark:text-slate-400">{format.description}</div>
						</div>
						{#if selectedFormat === format.value}
							<div class="w-4 h-4 rounded-full bg-blue-500 flex items-center justify-center">
								<div class="w-2 h-2 rounded-full bg-white"></div>
							</div>
						{:else}
							<div class="w-4 h-4 rounded-full border-2 border-slate-300 dark:border-slate-600"></div>
						{/if}
					</button>
				{/each}
			</div>
		</div>

		<!-- Export Button -->
		<div class="mb-8">
			<Button
				variant="primary"
				size="lg"
				onclick={exportData}
				disabled={!dbContext.isInitialized || !selectedTable || isLoading}
				class="w-full"
			>
				{#if isLoading}
					<span class="flex items-center justify-center gap-3">
						<svg class="animate-spin h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						<span class="text-lg">Exporting...</span>
					</span>
				{:else}
					<span class="flex items-center justify-center gap-3">
						<Download class="w-6 h-6" />
						<span class="text-lg">Export Data</span>
					</span>
				{/if}
			</Button>
		</div>

		<!-- Info Box -->
		<div class="p-6 bg-cyan-50 dark:bg-cyan-900/20 rounded-lg border border-cyan-200 dark:border-cyan-800">
			<h3 class="text-base font-semibold text-cyan-900 dark:text-cyan-300 mb-3">📦 Export & Convert</h3>
			<ul class="text-sm text-cyan-800 dark:text-cyan-400 space-y-2">
				<li>• Select a table from your loaded data sources</li>
				<li>• Choose your preferred export format</li>
				<li>• Data will be converted and saved to your chosen location</li>
				<li>• Supports CSV, Parquet, JSON, and JSONL formats</li>
			</ul>
			<div class="mt-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded">
				<p class="text-sm text-yellow-800 dark:text-yellow-400">
					⚠️ <strong>Coming Soon:</strong> This feature is currently under development. Export functionality will be available in a future update.
				</p>
			</div>
		</div>
	</div>
</PageLayout>

