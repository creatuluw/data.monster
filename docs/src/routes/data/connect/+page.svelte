<script lang="ts">
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import ConnectionForm from '$lib/components/ConnectionForm.svelte';
	import ConnectionList from '$lib/components/ConnectionList.svelte';
	import SchemaBrowser from '$lib/components/SchemaBrowser.svelte';
	import ImportDialog from '$lib/components/ImportDialog.svelte';
	import { Cable, Loader2, AlertCircle } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';

	// Get database context from layout
	const dbContext = getDbContext();

	type View = 'list' | 'new-connection' | 'browse';

	let currentView = $state<View>('list');
	let selectedConnectionId = $state<string | null>(null);
	let selectedSchema = $state<string | null>(null);
	let selectedTable = $state<string | null>(null);
	let refreshTrigger = $state(0);
	let showImportDialog = $state(false);
	let isReady = $state(false);

	function handleNewConnection() {
		currentView = 'new-connection';
	}

	function handleConnectionCreated() {
		currentView = 'list';
		refreshTrigger++;
	}

	function handleCancelForm() {
		currentView = 'list';
	}

	function handleSelectConnection(connectionId: string) {
		selectedConnectionId = connectionId;
		currentView = 'browse';
	}

	function handleBackToList() {
		currentView = 'list';
		selectedConnectionId = null;
		selectedSchema = null;
		selectedTable = null;
	}

	function handleSelectTable(schema: string, table: string) {
		selectedSchema = schema;
		selectedTable = table;
		showImportDialog = true;
	}

	function handleImportSuccess() {
		showImportDialog = false;
		selectedSchema = null;
		selectedTable = null;
	}

	function handleCancelImport() {
		showImportDialog = false;
		selectedSchema = null;
		selectedTable = null;
	}

	// Wait for DB initialization
	onMount(() => {
		console.log('📄 CONNECT PAGE onMount');
		console.log('📄 dbContext:', dbContext);
		console.log('📄 dbContext.isInitialized:', dbContext.isInitialized);
		
		// Check if DB is ready
		const checkAndLoad = () => {
			console.log('📄 checkAndLoad() - Checking DB state...');
			console.log('  - isInitialized:', dbContext.isInitialized);
			console.log('  - isInitializing:', dbContext.isInitializing);
			console.log('  - error:', dbContext.error);
			
			if (dbContext.isInitialized) {
				console.log('✅ DB is initialized! Ready to load connections...');
				isReady = true;
			} else if (!dbContext.isInitializing && dbContext.error) {
				// Only show error if initialization is complete and there's an error
				console.error('❌ DB initialization failed with error');
				isReady = false;
			} else if (!dbContext.isInitializing) {
				// Initialization complete but not successful, retry
				console.log('⚠️ Init complete but not successful, retrying in 100ms...');
				setTimeout(checkAndLoad, 100);
			} else {
				// Still initializing, wait
				console.log('⏳ Still initializing, waiting 100ms...');
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

	<div class="max-w-6xl mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div class="p-3 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
				<Cable class="w-8 h-8 text-blue-600 dark:text-blue-400" />
			</div>
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Database Connections</h1>
				<p class="text-slate-600 dark:text-slate-400">
					Connect to PostgreSQL databases and import data
				</p>
			</div>
		</div>

		<!-- Main Content -->
		{#if currentView === 'new-connection'}
			<!-- New Connection Form -->
			<div class="mb-6">
				<button
					onclick={handleBackToList}
					class="text-sm text-blue-600 dark:text-blue-400 hover:underline mb-4"
				>
					← Back to Connections
				</button>
			</div>
			<ConnectionForm
				onSuccess={handleConnectionCreated}
				onCancel={handleCancelForm}
			/>
		{:else if currentView === 'browse' && selectedConnectionId}
			<!-- Schema Browser -->
			<div class="mb-6">
				<button
					onclick={handleBackToList}
					class="text-sm text-blue-600 dark:text-blue-400 hover:underline mb-4"
				>
					← Back to Connections
				</button>
			</div>
			<SchemaBrowser
				connectionId={selectedConnectionId}
				onSelectTable={handleSelectTable}
			/>
		{:else}
			<!-- Connection List -->
			<ConnectionList
				onSelectConnection={handleSelectConnection}
				onNewConnection={handleNewConnection}
				refreshTrigger={refreshTrigger}
			/>
		{/if}

		<!-- Info Box -->
		{#if currentView === 'list'}
			<div class="mt-8 p-6 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
				<h3 class="text-lg font-semibold text-blue-900 dark:text-blue-300 mb-2">PostgreSQL Support</h3>
				<div class="text-sm text-blue-800 dark:text-blue-400 space-y-2">
					<p>
						Connect to PostgreSQL databases and import data directly into your local DuckDB database.
					</p>
					<ul class="list-disc list-inside space-y-1 ml-2">
						<li><strong>Live Mode:</strong> Query PostgreSQL directly without importing (always fresh, zero storage)</li>
						<li><strong>Cached Mode:</strong> Import data and keep connection for easy refresh</li>
						<li><strong>Imported Mode:</strong> One-time import for fastest queries</li>
					</ul>
					<p class="mt-3 text-blue-700 dark:text-blue-300">
						<strong>Note:</strong> Connections use DuckDB's native PostgreSQL extension for optimal performance and security.
					</p>
				</div>
			</div>
		{/if}
	</div>

	<!-- Import Dialog Modal -->
	{#if showImportDialog && selectedConnectionId && selectedSchema && selectedTable}
		<ImportDialog
			connectionId={selectedConnectionId}
			schemaName={selectedSchema}
			tableName={selectedTable}
			onSuccess={handleImportSuccess}
			onCancel={handleCancelImport}
		/>
	{/if}
</PageLayout>
