<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import { Database, RefreshCw, Download, FileUp, Globe, HardDrive } from 'lucide-svelte';
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
	let connections = $state<any[]>([]);
	let remoteFiles = $state<any[]>([]);
	let uploadedFiles = $state<any[]>([]);

	// Placeholder data - will be replaced with actual data from backend
	async function loadDataSources() {
		try {
			isLoading = true;
			
			// TODO: Load actual PostgreSQL connections from _warphead_connections table
			// connections = await invoke<any[]>('list_postgres_connections');
			
			// TODO: Load remote files from data/remote folder
			// remoteFiles = await invoke<any[]>('list_remote_files');
			
			// TODO: Load uploaded files from data folder
			// uploadedFiles = await invoke<any[]>('list_uploaded_files');
			
			// Placeholder data
			connections = [
				{ id: '1', name: 'Production DB', type: 'postgresql', host: 'localhost', database: 'myapp' },
				{ id: '2', name: 'Analytics DB', type: 'postgresql', host: 'analytics.example.com', database: 'analytics' }
			];
			
			remoteFiles = [
				{ name: 'sales_data.csv', url: 'https://example.com/sales_data.csv', lastLoaded: '2024-10-20' },
				{ name: 'customers.parquet', url: 'https://example.com/customers.parquet', lastLoaded: '2024-10-19' }
			];
			
			uploadedFiles = [
				{ name: 'orders.csv', folder: 'main', uploadedAt: '2024-10-25' },
				{ name: 'products.json', folder: 'main', uploadedAt: '2024-10-24' }
			];
			
		} catch (error) {
			addSystemMessage(`Error loading data sources: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function loadFromConnection(connectionId: string) {
		addSystemMessage('Loading data from connection...', 'info');
		// TODO: Implement actual loading logic
		addSystemMessage('Feature coming soon!', 'warning');
	}

	async function refreshRemoteFile(fileName: string) {
		addSystemMessage(`Refreshing ${fileName}...`, 'info');
		// TODO: Implement actual refresh logic
		addSystemMessage('Feature coming soon!', 'warning');
	}

	async function reloadUploadedFile(fileName: string) {
		addSystemMessage(`Reloading ${fileName}...`, 'info');
		// TODO: Implement actual reload logic
		addSystemMessage('Feature coming soon!', 'warning');
	}

	onMount(() => {
		// Wait for DB initialization, then load data sources
		const checkAndLoad = () => {
			if (dbContext.isInitialized) {
				loadDataSources();
			} else if (!dbContext.isInitializing) {
				return;
			} else {
				setTimeout(checkAndLoad, 100);
			}
		};
		
		checkAndLoad();
	});
</script>

<PageLayout>
	<div class="max-w-6xl mx-auto">
		<!-- Header -->
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-slate-900 dark:text-slate-100 mb-2">Load Data from Sources</h1>
			<p class="text-slate-600 dark:text-slate-400">
				Manage and refresh data from your defined sources
			</p>
		</div>

		{#if !dbContext.isInitializing && dbContext.error && !dbContext.isTauriAvailable}
			<div class="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg mb-8">
				<h3 class="text-sm font-semibold text-red-900 dark:text-red-300 mb-2">⚠️ Tauri Runtime Not Detected</h3>
				<p class="text-sm text-red-700 dark:text-red-400 mb-2">
					The Tauri API is not available. You need to run the full Tauri application.
				</p>
			</div>
		{/if}

		<!-- Database Connections Section -->
		<div class="mb-8">
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<Database class="w-5 h-5" />
					Database Connections
				</h2>
				<Button
					variant="primary"
					size="sm"
					href="/data/connect"
				>
					Add Connection
				</Button>
			</div>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#if connections.length === 0}
					<div class="col-span-2 p-8 text-center bg-slate-50 dark:bg-slate-800/50 rounded-lg border-2 border-dashed border-slate-300 dark:border-slate-700">
						<Database class="w-12 h-12 mx-auto mb-3 text-slate-400" />
						<p class="text-slate-600 dark:text-slate-400 mb-4">No database connections configured</p>
						<Button variant="primary" href="/data/connect">
							Configure Database Connection
						</Button>
					</div>
				{:else}
					{#each connections as connection}
						<div class="p-4 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 shadow-sm hover:shadow-md transition-shadow">
							<div class="flex items-start justify-between mb-3">
								<div class="flex-grow">
									<h3 class="font-semibold text-slate-900 dark:text-slate-100 mb-1">{connection.name}</h3>
									<p class="text-sm text-slate-600 dark:text-slate-400">
										{connection.type} • {connection.host}
									</p>
									<p class="text-sm text-slate-500 dark:text-slate-500 font-mono">{connection.database}</p>
								</div>
								<span class="px-2 py-1 text-xs font-semibold rounded bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400">
									Connected
								</span>
							</div>
							<div class="flex gap-2">
								<Button
									variant="primary"
									size="sm"
									onclick={() => loadFromConnection(connection.id)}
									fullWidth
								>
									<RefreshCw class="w-4 h-4 mr-2" />
									Load Data
								</Button>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>

		<!-- Remote Files Section -->
		<div class="mb-8">
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<Globe class="w-5 h-5" />
					Remote Files
				</h2>
				<Button
					variant="primary"
					size="sm"
					href="/data/remote"
				>
					Add Remote File
				</Button>
			</div>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#if remoteFiles.length === 0}
					<div class="col-span-2 p-8 text-center bg-slate-50 dark:bg-slate-800/50 rounded-lg border-2 border-dashed border-slate-300 dark:border-slate-700">
						<Globe class="w-12 h-12 mx-auto mb-3 text-slate-400" />
						<p class="text-slate-600 dark:text-slate-400 mb-4">No remote files configured</p>
						<Button variant="primary" href="/data/remote">
							Add Remote File Source
						</Button>
					</div>
				{:else}
					{#each remoteFiles as file}
						<div class="p-4 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 shadow-sm hover:shadow-md transition-shadow">
							<div class="flex items-start justify-between mb-3">
								<div class="flex-grow">
									<h3 class="font-semibold text-slate-900 dark:text-slate-100 mb-1">{file.name}</h3>
									<p class="text-sm text-slate-600 dark:text-slate-400 truncate">
										{file.url}
									</p>
									<p class="text-xs text-slate-500 dark:text-slate-500 mt-1">
										Last loaded: {file.lastLoaded}
									</p>
								</div>
							</div>
							<div class="flex gap-2">
								<Button
									variant="primary"
									size="sm"
									onclick={() => refreshRemoteFile(file.name)}
									fullWidth
								>
									<Download class="w-4 h-4 mr-2" />
									Refresh
								</Button>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>

		<!-- Uploaded Files Section -->
		<div class="mb-8">
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<HardDrive class="w-5 h-5" />
					Uploaded Files
				</h2>
				<Button
					variant="primary"
					size="sm"
					href="/data/upload"
				>
					Upload File
				</Button>
			</div>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#if uploadedFiles.length === 0}
					<div class="col-span-2 p-8 text-center bg-slate-50 dark:bg-slate-800/50 rounded-lg border-2 border-dashed border-slate-300 dark:border-slate-700">
						<FileUp class="w-12 h-12 mx-auto mb-3 text-slate-400" />
						<p class="text-slate-600 dark:text-slate-400 mb-4">No files uploaded</p>
						<Button variant="primary" href="/data/upload">
							Upload Your First File
						</Button>
					</div>
				{:else}
					{#each uploadedFiles as file}
						<div class="p-4 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 shadow-sm hover:shadow-md transition-shadow">
							<div class="flex items-start justify-between mb-3">
								<div class="flex-grow">
									<h3 class="font-semibold text-slate-900 dark:text-slate-100 mb-1">{file.name}</h3>
									<p class="text-sm text-slate-600 dark:text-slate-400">
										Folder: {file.folder}
									</p>
									<p class="text-xs text-slate-500 dark:text-slate-500 mt-1">
										Uploaded: {file.uploadedAt}
									</p>
								</div>
							</div>
							<div class="flex gap-2">
								<Button
									variant="secondary"
									size="sm"
									onclick={() => reloadUploadedFile(file.name)}
									fullWidth
								>
									<RefreshCw class="w-4 h-4 mr-2" />
									Reload
								</Button>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>

		<!-- Info Box -->
		<div class="p-6 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
			<h3 class="text-base font-semibold text-blue-900 dark:text-blue-300 mb-3">🔄 Data Loading & Refresh</h3>
			<ul class="text-sm text-blue-800 dark:text-blue-400 space-y-2">
				<li>• <strong>Database Connections:</strong> Load and refresh data from connected PostgreSQL databases</li>
				<li>• <strong>Remote Files:</strong> Re-download and update data from remote URLs</li>
				<li>• <strong>Uploaded Files:</strong> Reload previously uploaded files into DuckDB</li>
				<li>• <strong>Coming Soon:</strong> Automated refresh schedules, incremental updates, and data versioning</li>
			</ul>
		</div>
	</div>
</PageLayout>


