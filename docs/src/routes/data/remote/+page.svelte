<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import Label from '$lib/components/Label.svelte';
	import { Globe, FolderPlus, CheckCircle } from 'lucide-svelte';
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
	let folders = $state<string[]>(['main']);
	let selectedFolder = $state('remote');
	let showCreateFolderModal = $state(false);
	let newFolderName = $state('');
	let folderError = $state('');
	let remoteUrl = $state('');
	let remoteError = $state('');
	let uploadSuccess = $state(false);
	let lastUploadedFile = $state('');
	let downloadResult = $state({ name: '', status: 'success' as 'success' | 'error', message: '' });

	async function loadFolders() {
		try {
			folders = await invoke<string[]>('list_folders');
			// Default to 'remote' folder if it exists, otherwise 'main'
			if (folders.includes('remote')) {
				selectedFolder = 'remote';
			} else if (!selectedFolder || !folders.includes(selectedFolder)) {
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

	async function fetchRemoteFile() {
		try {
			isLoading = true;
			remoteError = '';
			uploadSuccess = false;
			
			// Validate inputs
			if (!remoteUrl.trim()) {
				remoteError = 'URL cannot be empty';
				isLoading = false;
				return;
			}

			addSystemMessage('Fetching remote file...', 'info');
			
			// Call the Tauri command to fetch and save the file
			const savedPath = await invoke<string>('fetch_remote_file', {
				url: remoteUrl.trim(),
				folderName: selectedFolder
			});

			addSystemMessage('Processing file...', 'info');

			// Extract filename and extension from the saved path
			const fileName = savedPath.split(/[\\\/]/).pop() || 'data';
			const extension = fileName.split('.').pop()?.toLowerCase();
			const tableName = fileName.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

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

			lastUploadedFile = fileName;
			downloadResult = {
				name: fileName,
				status: 'success',
				message: result
			};
			uploadSuccess = true;
			addSystemMessage(`Successfully downloaded and loaded ${fileName} to folder: ${selectedFolder}`, 'success');
			
			// Clear URL input
			remoteUrl = '';
			
			// Dispatch event to refresh file lists globally
			window.dispatchEvent(new CustomEvent('file-uploaded'));
		} catch (error) {
			const fileName = remoteUrl.split('/').pop() || 'file';
			remoteError = `${error}`;
			downloadResult = {
				name: fileName,
				status: 'error',
				message: String(error)
			};
			addSystemMessage(`Error: ${error}`, 'error');
			uploadSuccess = false;
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		// Wait for DB initialization, then load folders
		const checkAndLoad = () => {
			if (dbContext.isInitialized) {
				loadFolders();
			} else if (!dbContext.isInitializing) {
				// Initialization complete but not successful, stop trying
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

	<!-- Success Message -->
	{#if uploadSuccess && downloadResult.name}
		<div class="mb-8 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
			<div class="flex items-start gap-3 mb-3">
				<CheckCircle class="w-5 h-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
				<div class="flex-grow">
					<h3 class="text-sm font-semibold text-green-900 dark:text-green-300 mb-1">Download Complete!</h3>
					<p class="text-sm text-green-700 dark:text-green-400">
						File downloaded and loaded successfully.
					</p>
				</div>
			</div>
			
			<!-- Download Result -->
			<div class="mt-3">
				<div class="flex items-start gap-2 text-sm p-2 rounded {downloadResult.status === 'success' ? 'bg-green-100 dark:bg-green-900/30' : 'bg-red-100 dark:bg-red-900/30'}">
					<span class="font-mono text-xs {downloadResult.status === 'success' ? 'text-green-900 dark:text-green-300' : 'text-red-900 dark:text-red-300'} flex-grow">
						{downloadResult.status === 'success' ? '✓' : '✗'} {downloadResult.name}
					</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Main Form Section -->
	<div class="max-w-3xl mx-auto">
		<!-- Folder Selection -->
		<div class="mb-8">
			<Label for="folder-select">
				Save to Folder
			</Label>
			<div class="flex gap-3">
				<select
					id="folder-select"
					bind:value={selectedFolder}
					disabled={!dbContext.isInitialized}
					class="flex-grow px-4 py-3 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-slate-100 dark:disabled:bg-slate-800 disabled:cursor-not-allowed"
				>
					{#each folders as folder}
						<option value={folder}>{folder}</option>
					{/each}
				</select>
				<Button
					variant="primary"
					size="md"
					onclick={() => { showCreateFolderModal = true; newFolderName = ''; folderError = ''; }}
					disabled={!dbContext.isInitialized}
					title="Create new folder"
				>
					<FolderPlus class="w-5 h-5" />
				</Button>
			</div>
			<p class="text-sm text-slate-500 dark:text-slate-400 mt-2">
				Files will be saved to: <span class="font-mono font-semibold">{selectedFolder}</span>
			</p>
		</div>

		<!-- URL Input -->
		<div class="mb-8">
			<Label for="remote-url">
				Data File URL
			</Label>
			<input
				id="remote-url"
				type="url"
				bind:value={remoteUrl}
				disabled={!dbContext.isInitialized}
				placeholder="https://example.com/data/file.csv"
				onkeydown={(e) => { if (e.key === 'Enter' && remoteUrl.trim()) fetchRemoteFile(); }}
				class="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-slate-100 dark:disabled:bg-slate-800 disabled:cursor-not-allowed"
			/>
			<p class="text-sm text-slate-500 dark:text-slate-400 mt-2">
				Enter the direct URL to a CSV, Parquet, JSON, or JSONL file
			</p>
		</div>

		<!-- Error Message -->
		{#if remoteError}
			<div class="mb-6 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
				<p class="text-sm text-red-700 dark:text-red-400">{remoteError}</p>
			</div>
		{/if}

		<!-- Download Button -->
		<div class="mb-8">
			<Button
				variant="primary"
				size="lg"
				onclick={fetchRemoteFile}
				disabled={!dbContext.isInitialized || isLoading || !remoteUrl.trim()}
				class="w-full"
			>
				{#if isLoading}
					<span class="flex items-center justify-center gap-3">
						<svg class="animate-spin h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						<span class="text-lg">Downloading...</span>
					</span>
				{:else}
					<span class="flex items-center justify-center gap-3">
						<Globe class="w-6 h-6" />
						<span class="text-lg">Download from URL</span>
					</span>
				{/if}
			</Button>
		</div>

		<!-- Supported File Types -->
		<div class="p-6 bg-slate-50 dark:bg-slate-800/50 rounded-lg border border-slate-200 dark:border-slate-700">
			<h3 class="text-base font-semibold text-slate-900 dark:text-slate-100 mb-4">Supported File Types</h3>
			<div class="grid grid-cols-2 gap-4">
				<div class="flex items-center gap-3">
					<span class="text-sm font-semibold px-3 py-1.5 rounded bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400">CSV</span>
					<span class="text-sm text-slate-600 dark:text-slate-400">Comma-separated values</span>
				</div>
				<div class="flex items-center gap-3">
					<span class="text-sm font-semibold px-3 py-1.5 rounded bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400">Parquet</span>
					<span class="text-sm text-slate-600 dark:text-slate-400">Columnar format</span>
				</div>
				<div class="flex items-center gap-3">
					<span class="text-sm font-semibold px-3 py-1.5 rounded bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">JSON</span>
					<span class="text-sm text-slate-600 dark:text-slate-400">JSON documents</span>
				</div>
				<div class="flex items-center gap-3">
					<span class="text-sm font-semibold px-3 py-1.5 rounded bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">JSONL</span>
					<span class="text-sm text-slate-600 dark:text-slate-400">JSON Lines</span>
				</div>
			</div>
		</div>

		<!-- Info Box -->
		<div class="mt-6 p-6 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
			<h3 class="text-base font-semibold text-blue-900 dark:text-blue-300 mb-3">📡 How Remote Download Works</h3>
			<ul class="text-sm text-blue-800 dark:text-blue-400 space-y-2">
				<li>• Select a folder to organize your downloaded files</li>
				<li>• Enter the URL of a publicly accessible data file</li>
				<li>• The file will be downloaded and saved to the selected folder</li>
				<li>• Data will be automatically loaded into DuckDB</li>
				<li>• You can then query and analyze your data</li>
			</ul>
		</div>
	</div>

	<!-- Create Folder Modal -->
	{#if showCreateFolderModal}
		<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[60]" role="dialog" aria-modal="true" tabindex="-1" style="padding-top: 100px;" onclick={(e) => { if (e.target === e.currentTarget) showCreateFolderModal = false; }} onkeydown={(e) => { if (e.key === 'Escape') showCreateFolderModal = false; }}>
			<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl p-6 w-full max-w-md mx-4">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-4">Create New Folder</h2>
				
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
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
					{#if folderError}
						<p class="text-xs text-red-600 dark:text-red-400 mt-1">{folderError}</p>
					{/if}
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
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
</PageLayout>


