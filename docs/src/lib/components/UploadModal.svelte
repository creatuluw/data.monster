<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import Button from '$lib/components/Button.svelte';
	import { Upload } from 'lucide-svelte';
	import { loadCSVFile, loadParquetFile, loadJSONFile, saveFileToFolder, createFolder as createFolderDB, listFolders } from '$lib/db-operations';

	let { 
		isOpen = $bindable(false)
	} = $props();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// State
	let isDbInitialized = $state(false);
	let isLoading = $state(false);
	let isTauriAvailable = $state(false);
	
	// Folder management state
	let folders = $state<string[]>(['main']);
	let selectedFolder = $state('main');
	let showCreateFolderModal = $state(false);
	let newFolderName = $state('');
	let folderError = $state('');

	async function loadFolders() {
		try {
			folders = await listFolders();
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
			const result = await createFolderDB(folderNameToCreate);
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
			const savedPath = await saveFileToFolder(pathStr, selectedFolder);

			addSystemMessage('Processing file...', 'info');
			
			// File loading is already wrapped in critical operation
			const result = await (async () => {
				switch (extension) {
					case 'csv':
						return await loadCSVFile(savedPath, tableName);
					case 'parquet':
						return await loadParquetFile(savedPath, tableName);
					case 'json':
					case 'jsonl':
						return await loadJSONFile(savedPath, tableName);
					default:
						throw new Error(`Unsupported file type: ${extension}`);
				}
			})();

			addSystemMessage(`${result} (saved to folder: ${selectedFolder})`, 'success');
			
			// Dispatch event to refresh file lists
			window.dispatchEvent(new CustomEvent('file-uploaded'));
			
			// Close modal on success
			isOpen = false;
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function initializeDb() {
		try {
			await invoke('initialize_duckdb');
			isTauriAvailable = true;
			isDbInitialized = true;
			await loadFolders();
		} catch (error) {
			if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
				isTauriAvailable = false;
			} else {
				isTauriAvailable = true;
			}
		}
	}

	// Watch for modal opening to initialize
	$effect(() => {
		if (isOpen && !isDbInitialized) {
			initializeDb();
		}
	});

	function closeModal() {
		isOpen = false;
	}
</script>

<!-- Upload File Modal -->
{#if isOpen}
	<div 
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[60] p-4" 
		role="dialog" 
		aria-modal="true" 
		tabindex="-1"
		style="padding-top: 100px;"
		onclick={(e) => { if (e.target === e.currentTarget) closeModal(); }} 
		onkeydown={(e) => { if (e.key === 'Escape') closeModal(); }}
	>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
			<div class="p-6 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
				<div>
					<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Upload File</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">Upload CSV, Parquet, JSON, or JSONL files</p>
				</div>
				<Button
					variant="ghost"
					size="sm"
					onclick={closeModal}
					aria-label="Close"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
					</svg>
				</Button>
			</div>
			
			<div class="flex-1 overflow-y-auto p-6">
				{#if !isTauriAvailable}
					<div class="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg mb-6">
						<h3 class="text-sm font-semibold text-red-900 dark:text-red-300 mb-2">⚠️ Tauri Runtime Not Detected</h3>
						<p class="text-sm text-red-700 dark:text-red-400 mb-2">
							The Tauri API is not available. You need to run the full Tauri application.
						</p>
						<div class="bg-red-100 dark:bg-red-900/40 p-3 rounded font-mono text-xs text-red-800 dark:text-red-300">
							npm run dev
						</div>
					</div>
				{/if}

				<!-- Folder Selection -->
				<div class="mb-6">
					<label for="upload-folder-select" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Save to Folder
					</label>
					<div class="flex gap-2">
						<select
							id="upload-folder-select"
							bind:value={selectedFolder}
							disabled={!isTauriAvailable}
							class="flex-grow px-3 py-2 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-slate-100 dark:disabled:bg-slate-800 disabled:cursor-not-allowed"
						>
							{#each folders as folder}
								<option value={folder}>{folder}</option>
							{/each}
						</select>
						<Button
							variant="primary"
							size="md"
							onclick={() => { showCreateFolderModal = true; newFolderName = ''; folderError = ''; }}
							disabled={!isTauriAvailable}
							title="Create new folder"
						>
							<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
							</svg>
						</Button>
					</div>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Files will be saved to: <span class="font-mono">{selectedFolder}</span>
					</p>
				</div>

				<!-- Supported File Types -->
				<div class="mb-6 p-4 bg-slate-50 dark:bg-slate-800/50 rounded-lg border border-slate-200 dark:border-slate-700">
					<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100 mb-3">Supported File Types</h3>
					<div class="grid grid-cols-2 gap-3">
						<div class="flex items-center gap-2">
							<span class="text-xs font-semibold px-2 py-1 rounded bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400">CSV</span>
							<span class="text-xs text-slate-600 dark:text-slate-400">Comma-separated values</span>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-xs font-semibold px-2 py-1 rounded bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400">Parquet</span>
							<span class="text-xs text-slate-600 dark:text-slate-400">Columnar format</span>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-xs font-semibold px-2 py-1 rounded bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">JSON</span>
							<span class="text-xs text-slate-600 dark:text-slate-400">JSON documents</span>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-xs font-semibold px-2 py-1 rounded bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">JSONL</span>
							<span class="text-xs text-slate-600 dark:text-slate-400">JSON Lines</span>
						</div>
					</div>
				</div>
			</div>
			
			<div class="p-4 border-t border-slate-200 dark:border-slate-700 flex justify-end gap-3">
				<Button
					variant="ghost"
					onclick={closeModal}
				>
					Cancel
				</Button>
				<Button
					variant="primary"
					onclick={async () => {
						await uploadFile();
					}}
					disabled={!isDbInitialized || isLoading || !isTauriAvailable}
				>
					{#if isLoading}
						<span class="flex items-center gap-2">
							<svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Processing...
						</span>
					{:else}
						<span class="flex items-center gap-2">
							<Upload class="w-5 h-5" />
							Choose File to Upload
						</span>
					{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Create Folder Modal -->
{#if showCreateFolderModal}
	<div 
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[60]" 
		role="dialog" 
		aria-modal="true" 
		tabindex="-1"
		style="padding-top: 100px;"
		onclick={(e) => { if (e.target === e.currentTarget) showCreateFolderModal = false; }} 
		onkeydown={(e) => { if (e.key === 'Escape') showCreateFolderModal = false; }}
	>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl p-6 w-full max-w-md mx-4">
			<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-4">Create New Folder</h2>
			
			<div class="mb-4">
				<label for="folder-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Folder Name
				</label>
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

