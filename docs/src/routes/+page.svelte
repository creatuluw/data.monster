<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount, getContext } from 'svelte';
	import Card from '$lib/components/Card.svelte';
	import Button from '$lib/components/Button.svelte';
	import Label from '$lib/components/Label.svelte';
	import { Database, Network, LineChart, CircleCheckBig } from 'lucide-svelte';

	// Get DB context from layout
	const dbContext = getContext<{
		isInitialized: boolean;
		isTauriAvailable: boolean;
		error: string | null;
		isInitializing: boolean;
	}>('db');

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// DuckDB state - use context values
	let tables = $state<string[]>([]);
	let uploadStatus = $state('');
	let isLoading = $state(false);
	let showUiOptions = $state(false);
	
	// Folder management state
	let folders = $state<string[]>(['main']);
	let selectedFolder = $state('main');
	let showCreateFolderModal = $state(false);
	let newFolderName = $state('');
	let folderError = $state('');
	
	// File browsing state
	interface FileMetadata {
		filename: string;
		folder: string;
		file_path: string;
		file_type: string;
		size_bytes: number;
		uploaded_at: number;
		row_count: number | null;
		source_type: string;
		source_format: string;
	}
	
	let recentFiles = $state<FileMetadata[]>([]);
	let showBrowseModal = $state(false);
	let browseFolder = $state<string | null>(null);
	let folderFiles = $state<FileMetadata[]>([]);

	async function initializeDuckDb() {
		try {
			isLoading = true;
			addSystemMessage('Initializing DuckDB...', 'info');
			const result = await invoke<string>('initialize_duckdb');
			addSystemMessage(result, 'success');
			tables = await invoke<string[]>('list_tables');
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function loadFolders() {
		try {
			folders = await invoke<string[]>('list_folders');
			if (!selectedFolder || !folders.includes(selectedFolder)) {
				selectedFolder = 'main';
			}
		} catch (error) {
			console.error('Failed to load folders:', error);
		}
	}

	async function createFolder() {
		if (!newFolderName.trim()) {
			folderError = 'Folder name cannot be empty';
			return;
		}

		try {
			const folderNameToCreate = newFolderName.trim();
			const result = await invoke<string>('create_folder', { folderName: folderNameToCreate });
			addSystemMessage(result, 'success');
			folderError = '';
			newFolderName = '';
			showCreateFolderModal = false;
			await loadFolders();
			selectedFolder = folderNameToCreate;
		} catch (error) {
			folderError = `${error}`;
		}
	}
	
	async function loadRecentFiles() {
		try {
			recentFiles = await invoke<FileMetadata[]>('get_recent_files', { limit: 6 });
		} catch (error) {
			console.error('Failed to load recent files:', error);
		}
	}
	
	async function browseFolderFiles(folder: string) {
		try {
			browseFolder = folder;
			const allFiles = await invoke<FileMetadata[]>('get_all_files_metadata');
			folderFiles = allFiles.filter(f => f.folder === folder);
			showBrowseModal = true;
		} catch (error) {
			console.error('Failed to load folder files:', error);
		}
	}
	
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
	}
	
	function formatDate(timestamp: number): string {
		if (!timestamp) return 'Unknown';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}
	
	function formatRowCount(count: number | null): string {
		if (count === null) return 'N/A';
		return count.toLocaleString();
	}

	async function uploadFile() {
		try {
			isLoading = true;
			addSystemMessage('Opening file dialog...', 'info');
			
			const filePath = await open({
				multiple: false,
				filters: [
					{
						name: 'Data Files',
						extensions: ['csv', 'parquet', 'json', 'jsonl']
					}
				]
			});

			if (!filePath) {
				addSystemMessage('No file selected', 'warning');
				isLoading = false;
				return;
			}

			addSystemMessage('Saving file to folder...', 'info');
			
			// Extract file extension and name
			const pathStr = typeof filePath === 'string' ? filePath : String(filePath);
			const fileName = pathStr.split(/[\\\/]/).pop() || 'data';
			const extension = fileName.split('.').pop()?.toLowerCase();
			const tableName = fileName.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

			// Save file to selected folder
			const savedPath = await invoke<string>('save_file_to_folder', {
				sourcePath: pathStr,
				folderName: selectedFolder
			});

			addSystemMessage('Processing file...', 'info');

			let result: string;
			
			switch (extension) {
				case 'csv':
					result = await invoke<string>('load_csv_file', { 
						filePath: savedPath, 
						tableName 
					});
					break;
				case 'parquet':
					result = await invoke<string>('load_parquet_file', { 
						filePath: savedPath, 
						tableName 
					});
					break;
				case 'json':
				case 'jsonl':
					result = await invoke<string>('load_json_file', { 
						filePath: savedPath, 
						tableName 
					});
					break;
				default:
					throw new Error(`Unsupported file type: ${extension}`);
			}

			addSystemMessage(`${result} (saved to folder: ${selectedFolder})`, 'success');
			
			// Refresh tables list
			tables = await invoke<string[]>('list_tables');
			
			// Refresh recent files
			await loadRecentFiles();
			
			// Show UI options after successful upload
			showUiOptions = true;
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	onMount(async () => {
		// Wait for DB to be initialized by the layout
		if (!dbContext.isTauriAvailable) {
			addSystemMessage('Tauri API not available. Please run: npm run dev', 'error');
			return;
		}

		// Wait for DB initialization to complete
		const waitForDb = async () => {
			let attempts = 0;
			while (attempts < 50 && (dbContext.isInitializing || !dbContext.isInitialized)) {
				await new Promise(resolve => setTimeout(resolve, 100));
				attempts++;
			}
			return dbContext.isInitialized;
		};

		const isReady = await waitForDb();
		if (!isReady) {
			addSystemMessage('Database initialization timed out', 'error');
			return;
		}

		// Load data after DB is ready
		try {
			await loadFolders();
			await loadRecentFiles();
		} catch (error) {
			addSystemMessage(`Error loading data: ${error}`, 'error');
		}
	});
</script>

<main class="flex-grow">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="pt-12 pb-8" style="margin-top: 10vh;">
			<!-- Page Title -->
			<div class="text-center mb-12">
				<h1 class="text-4xl md:text-5xl lg:text-6xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-3">Welcome to Warphead</h1>
				<p class="text-lg text-slate-600 dark:text-slate-400">Choose where you'd like to start</p>
			</div>

		<!-- Navigation Cards -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<Card
				title="Data"
				description="Upload, manage, and explore your datasets. Import CSV, Parquet, JSON, and JSONL files."
				href="/data"
				icon={Database}
			/>
			<Card
				title="Models"
				description="Create data models with reusable measures, dimensions, and calculated fields for consistent analytics."
				href="/models"
				icon={Network}
			/>
			<Card
				title="Analysis"
				description="Perform advanced analytics, create visualizations, and gain insights from your data."
				href="/analysis"
				icon={LineChart}
			/>
		<Card
			title="Findings"
			description="Explore actionable insights and recommendations to improve team performance and results."
			href="/findings"
			icon={CircleCheckBig}
		/>
		</div>

		<!-- Development Link -->
		<div class="text-center mt-8">
			<a 
				href="/warp-lab"
				class="inline-flex items-center text-sm font-medium text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
			>
				WarpLab &rarr;
			</a>
		</div>
	</div>
</div>

<!-- Hidden Upload Form (Legacy - Keeping for future reference) -->
<div class="hidden">
	{#if false}

		<!-- Upload Form -->
		<div class="bg-white rounded-lg shadow-lg p-6">
			{#if !dbContext.isTauriAvailable}
				<div class="p-4 bg-red-50 border border-red-200 rounded-lg mb-6">
					<h3 class="text-sm font-semibold text-red-900 mb-2">⚠️ Tauri Runtime Not Detected</h3>
					<p class="text-sm text-red-700 mb-2">
							The Tauri API is not available. You need to run the full Tauri application.
						</p>
						<div class="bg-red-100 p-3 rounded font-mono text-xs text-red-800">
							npm run dev
						</div>
						<p class="text-xs text-red-600 mt-2">
							Note: If you're seeing this in development, make sure you're not just viewing the Vite server directly in a browser.
						</p>
					</div>
				{/if}

				<!-- Folder Selection -->
				<div class="mb-6">
					<Label for="folder-select">
						Save to Folder
					</Label>
					<div class="flex gap-2">
					<select
						id="folder-select"
						bind:value={selectedFolder}
						disabled={!dbContext.isTauriAvailable}
						class="flex-grow px-3 py-2 border border-slate-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-slate-100 disabled:cursor-not-allowed"
					>
							{#each folders as folder}
								<option value={folder}>{folder}</option>
							{/each}
						</select>
						<Button
							variant="primary"
							size="md"
							onclick={() => { showCreateFolderModal = true; newFolderName = ''; folderError = ''; }}
							disabled={!dbContext.isTauriAvailable}
							title="Create new folder"
						>
							<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
							</svg>
						</Button>
					</div>
					<p class="text-xs text-slate-500 mt-1">
						Files will be saved to: <span class="font-mono">{selectedFolder}</span>
					</p>
				</div>

				<div class="mb-6">
					<Label for="file-upload">
						Upload File
					</Label>
					<p class="text-sm text-slate-600 mb-4">
						Select a CSV, Parquet, JSON, or JSONL file to upload
					</p>
					
				<Button
					variant="primary"
					onclick={uploadFile}
					disabled={!dbContext.isInitialized || isLoading || !dbContext.isTauriAvailable}
					fullWidth
				>
						{#if isLoading}
							<span>Processing...</span>
						{:else}
							<span>Choose File</span>
						{/if}
					</Button>
				</div>


				<!-- Recent Uploads Section -->
				{#if recentFiles.length > 0}
					<div class="mt-6">
						<div class="flex items-center justify-between mb-3">
							<h3 class="text-sm font-semibold text-slate-900">📁 Recent Uploads</h3>
							<button
								onclick={() => browseFolderFiles(selectedFolder)}
								class="text-xs text-blue-600 hover:text-blue-700 font-medium"
							>
								Browse All Files →
							</button>
						</div>
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						{#each recentFiles.slice(0, 6) as file}
							<a 
								href="/table-view?filename={encodeURIComponent(file.filename)}&folder={encodeURIComponent(file.folder)}"
								class="bg-white border border-slate-200 rounded-lg p-3 hover:shadow-md hover:border-blue-300 transition-all cursor-pointer"
							>
								<div class="flex items-start justify-between mb-2">
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2 mb-1">
											<span class="text-xs font-semibold px-2 py-0.5 rounded {
												file.file_type === 'CSV' ? 'bg-green-100 text-green-700' :
												file.file_type === 'Parquet' ? 'bg-purple-100 text-purple-700' :
												file.file_type === 'JSON' ? 'bg-blue-100 text-blue-700' :
												'bg-slate-100 text-slate-700'
											}">
												{file.file_type}
											</span>
											<span class="text-xs text-slate-500 truncate">{file.folder}</span>
										</div>
										<h4 class="text-sm font-medium text-slate-900 truncate" title={file.filename}>
											{file.filename}
										</h4>
									</div>
								</div>
								<div class="space-y-1 text-xs text-slate-600">
									<div class="flex justify-between">
										<span>Size:</span>
										<span class="font-medium">{formatBytes(file.size_bytes)}</span>
									</div>
									{#if file.row_count !== null}
										<div class="flex justify-between">
											<span>Rows:</span>
											<span class="font-medium">{formatRowCount(file.row_count)}</span>
										</div>
									{/if}
									<div class="flex justify-between">
										<span>Uploaded:</span>
										<span class="font-medium text-slate-500 text-[10px]">{formatDate(file.uploaded_at)}</span>
									</div>
								</div>
							</a>
						{/each}
					</div>
					</div>
				{/if}
				
				<!-- Folder Navigation -->
				{#if folders.length > 1}
					<div class="mt-6 p-4 bg-slate-50 rounded-lg">
						<h3 class="text-sm font-semibold text-slate-900 mb-3">📂 Browse Folders</h3>
						<div class="flex flex-wrap gap-2">
						{#each folders as folder}
							<Button
								variant="ghost"
								size="sm"
								onclick={() => browseFolderFiles(folder)}
								class="flex items-center gap-2"
							>
								<span class="font-medium">{folder}</span>
								<span class="text-blue-600">→</span>
							</Button>
						{/each}
						</div>
					</div>
				{/if}
				
				<!-- Explore Data Options -->
				{#if showUiOptions}
					<div class="mt-6 border-t border-slate-200 pt-6">
						<h3 class="text-lg font-semibold text-slate-900 mb-4">📊 Explore Your Data</h3>
						<p class="text-sm text-slate-600 mb-4">
							Choose how you want to interact with your data:
						</p>
						
						<div class="grid grid-cols-1 gap-3">
							<!-- SQL Query Interface -->
							<a 
								href="/query"
								class="group p-4 bg-gradient-to-r from-blue-50 to-blue-100 hover:from-blue-100 hover:to-blue-200 border border-blue-200 rounded-lg transition-all duration-200 hover:shadow-md"
							>
								<div class="flex items-start justify-between">
									<div>
										<h4 class="font-semibold text-blue-900 group-hover:text-blue-700 mb-1">
											🔍 SQL Query Editor
										</h4>
										<p class="text-sm text-blue-700">
											Write custom SQL queries to explore and analyze your data
										</p>
									</div>
									<span class="text-blue-600 group-hover:translate-x-1 transition-transform">→</span>
								</div>
							</a>

						<!-- Data Table Viewer -->
						<a 
							href="/table-view"
							class="group p-4 bg-gradient-to-r from-green-50 to-green-100 hover:from-green-100 hover:to-green-200 border border-green-200 rounded-lg transition-all duration-200 hover:shadow-md"
						>
								<div class="flex items-start justify-between">
									<div>
										<h4 class="font-semibold text-green-900 group-hover:text-green-700 mb-1">
											📋 Table Viewer
										</h4>
										<p class="text-sm text-green-700">
											Browse your data in an interactive table with sorting and filtering
										</p>
									</div>
									<span class="text-green-600 group-hover:translate-x-1 transition-transform">→</span>
								</div>
							</a>

							<!-- Data Visualization -->
							<a 
								href="/visualize"
								class="group p-4 bg-gradient-to-r from-purple-50 to-purple-100 hover:from-purple-100 hover:to-purple-200 border border-purple-200 rounded-lg transition-all duration-200 hover:shadow-md"
							>
								<div class="flex items-start justify-between">
									<div>
										<h4 class="font-semibold text-purple-900 group-hover:text-purple-700 mb-1">
											📈 Visualizations
										</h4>
										<p class="text-sm text-purple-700">
											Create charts and graphs to visualize your data insights
										</p>
									</div>
									<span class="text-purple-600 group-hover:translate-x-1 transition-transform">→</span>
								</div>
							</a>

							<!-- Data Export -->
							<a 
								href="/export"
								class="group p-4 bg-gradient-to-r from-orange-50 to-orange-100 hover:from-orange-100 hover:to-orange-200 border border-orange-200 rounded-lg transition-all duration-200 hover:shadow-md"
							>
								<div class="flex items-start justify-between">
									<div>
										<h4 class="font-semibold text-orange-900 group-hover:text-orange-700 mb-1">
											💾 Export Data
										</h4>
										<p class="text-sm text-orange-700">
											Export your data or query results to various formats
										</p>
									</div>
									<span class="text-orange-600 group-hover:translate-x-1 transition-transform">→</span>
								</div>
							</a>
						</div>
					</div>
				{/if}
			</div>
	{/if}
</div>

	<!-- Create Folder Modal -->
	{#if showCreateFolderModal}
		<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[60]" role="dialog" aria-modal="true" tabindex="-1" style="padding-top: 100px;" onclick={(e) => { if (e.target === e.currentTarget) showCreateFolderModal = false; }} onkeydown={(e) => { if (e.key === 'Escape') showCreateFolderModal = false; }}>
			<div class="bg-white rounded-lg shadow-xl p-6 w-full max-w-md mx-4">
				<h2 class="text-xl font-semibold text-slate-900 mb-4">Create New Folder</h2>
				
				<div class="mb-4">
					<Label for="folder-name">
						Folder Name
					</Label>
					<input
						id="folder-name"
						type="text"
						bind:value={newFolderName}
						onkeydown={(e) => { if (e.key === 'Enter') createFolder(); }}
						placeholder="Enter folder name"
						class="w-full px-3 py-2 border border-slate-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
					{#if folderError}
						<p class="text-xs text-red-600 mt-1">{folderError}</p>
					{/if}
					<p class="text-xs text-slate-500 mt-1">
						Allowed characters: letters, numbers, spaces, hyphens, and underscores
					</p>
				</div>

				<div class="flex gap-3 justify-end">
					<Button
						variant="ghost"
						onclick={() => { showCreateFolderModal = false; newFolderName = ''; folderError = ''; }}
					>
						Cancel
					</Button>
					<Button
						variant="primary"
						onclick={createFolder}
						disabled={!newFolderName.trim()}
					>
						Create Folder
					</Button>
				</div>
			</div>
		</div>
	{/if}
	
	<!-- Browse Files Modal -->
	{#if showBrowseModal && browseFolder}
		<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[60] p-4" role="dialog" aria-modal="true" tabindex="-1" style="padding-top: 100px;" onclick={(e) => { if (e.target === e.currentTarget) showBrowseModal = false; }} onkeydown={(e) => { if (e.key === 'Escape') showBrowseModal = false; }}>
			<div class="bg-white rounded-lg shadow-xl w-full max-w-4xl max-h-[90vh] overflow-hidden flex flex-col">
				<div class="p-6 border-b border-slate-200 flex items-center justify-between">
					<div>
						<h2 class="text-xl font-semibold text-slate-900">Files in "{browseFolder}"</h2>
						<p class="text-sm text-slate-600 mt-1">{folderFiles.length} file{folderFiles.length !== 1 ? 's' : ''} found</p>
					</div>
					<Button
						variant="ghost"
						size="sm"
						onclick={() => showBrowseModal = false}
						aria-label="Close"
					>
						<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
						</svg>
					</Button>
				</div>
				
				<div class="flex-1 overflow-y-auto p-6">
					{#if folderFiles.length === 0}
						<div class="text-center py-12">
							<svg class="w-16 h-16 mx-auto text-slate-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
							</svg>
							<p class="text-slate-600 text-lg mb-2">No files in this folder</p>
							<p class="text-slate-500 text-sm">Upload some data files to get started</p>
						</div>
					{:else}
						<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
							{#each folderFiles as file}
								<a 
									href="/table-view?filename={encodeURIComponent(file.filename)}&folder={encodeURIComponent(file.folder)}"
									class="bg-slate-50 border border-slate-200 rounded-lg p-4 hover:shadow-md hover:border-blue-300 transition-all cursor-pointer"
								>
									<div class="flex items-start justify-between mb-3">
										<span class="text-xs font-semibold px-2 py-1 rounded {
											file.file_type === 'CSV' ? 'bg-green-100 text-green-700' :
											file.file_type === 'Parquet' ? 'bg-purple-100 text-purple-700' :
											file.file_type === 'JSON' ? 'bg-blue-100 text-blue-700' :
											'bg-slate-100 text-slate-700'
										}">
											{file.file_type}
										</span>
									</div>
									
									<h4 class="text-sm font-semibold text-slate-900 mb-3 break-words" title={file.filename}>
										{file.filename}
									</h4>
									
									<div class="space-y-2 text-xs">
										<div class="flex justify-between items-center py-1.5 px-2 bg-white rounded">
											<span class="text-slate-600">Size:</span>
											<span class="font-medium text-slate-900">{formatBytes(file.size_bytes)}</span>
										</div>
										
										{#if file.row_count !== null}
											<div class="flex justify-between items-center py-1.5 px-2 bg-white rounded">
												<span class="text-slate-600">Rows:</span>
												<span class="font-medium text-slate-900">{formatRowCount(file.row_count)}</span>
											</div>
										{/if}
										
										<div class="flex justify-between items-center py-1.5 px-2 bg-white rounded">
											<span class="text-slate-600">Uploaded:</span>
											<span class="font-medium text-slate-900 text-[10px]">{formatDate(file.uploaded_at)}</span>
										</div>
									</div>
								</a>
							{/each}
						</div>
					{/if}
				</div>
				
				<div class="p-4 border-t border-slate-200 flex justify-end">
					<Button
						variant="ghost"
						onclick={() => showBrowseModal = false}
					>
						Close
					</Button>
				</div>
			</div>
		</div>
	{/if}
</main>


