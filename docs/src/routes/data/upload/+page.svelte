<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import Label from '$lib/components/Label.svelte';
	import { Upload, FolderPlus, CheckCircle } from 'lucide-svelte';
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
	let selectedFolder = $state('main');
	let showCreateFolderModal = $state(false);
	let newFolderName = $state('');
	let folderError = $state('');
	let uploadSuccess = $state(false);
	let lastUploadedFile = $state('');
	let uploadedFiles = $state<Array<{ name: string; status: 'success' | 'error'; message: string }>>([]);
	let currentFileProgress = $state('');
	let totalFiles = $state(0);
	let processedFiles = $state(0);

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

	async function processFile(pathStr: string): Promise<{ name: string; status: 'success' | 'error'; message: string }> {
		const fileName = pathStr.split(/[\\\/]/).pop() || 'data';
		
		try {
			const extension = fileName.split('.').pop()?.toLowerCase();
			const tableName = fileName.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

			// Save file to selected folder
			const savedPath = await invoke<string>('save_file_to_folder', {
				sourcePath: pathStr,
				folderName: selectedFolder
			});

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

			return {
				name: fileName,
				status: 'success',
				message: result
			};
		} catch (error) {
			return {
				name: fileName,
				status: 'error',
				message: String(error)
			};
		}
	}

	async function uploadFile() {
		try {
			isLoading = true;
			uploadSuccess = false;
			uploadedFiles = [];
			currentFileProgress = '';
			totalFiles = 0;
			processedFiles = 0;
			
			addSystemMessage('Opening file dialog...', 'info');
			
			const filePaths = await open({
				multiple: true,
				filters: [
					{
						name: 'Data Files',
						extensions: ['csv', 'parquet', 'json', 'jsonl']
					}
				]
			});

			if (!filePaths || (Array.isArray(filePaths) && filePaths.length === 0)) {
				addSystemMessage('No files selected', 'warning');
				isLoading = false;
				return;
			}

			// Convert to array if single file
			const fileArray = Array.isArray(filePaths) ? filePaths : [filePaths];
			totalFiles = fileArray.length;
			
			addSystemMessage(`Processing ${totalFiles} file${totalFiles > 1 ? 's' : ''}...`, 'info');

			// Process each file
			for (const filePath of fileArray) {
				const pathStr = typeof filePath === 'string' ? filePath : String(filePath);
				const fileName = pathStr.split(/[\\\/]/).pop() || 'data';
				
				processedFiles++;
				currentFileProgress = `Processing ${fileName} (${processedFiles}/${totalFiles})`;
				
				const result = await processFile(pathStr);
				uploadedFiles.push(result);
			}

			// Show summary
			const successCount = uploadedFiles.filter(f => f.status === 'success').length;
			const errorCount = uploadedFiles.filter(f => f.status === 'error').length;
			
			if (successCount > 0) {
				uploadSuccess = true;
				if (errorCount === 0) {
					addSystemMessage(`Successfully uploaded ${successCount} file${successCount > 1 ? 's' : ''} to folder: ${selectedFolder}`, 'success');
				} else {
					addSystemMessage(`Uploaded ${successCount} file${successCount > 1 ? 's' : ''}, ${errorCount} failed`, 'warning');
				}
				
				// Dispatch event to refresh file lists globally
				window.dispatchEvent(new CustomEvent('file-uploaded'));
			} else {
				addSystemMessage('All files failed to upload', 'error');
				uploadSuccess = false;
			}
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
			uploadSuccess = false;
		} finally {
			isLoading = false;
			currentFileProgress = '';
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
	{#if uploadSuccess && uploadedFiles.length > 0}
		<div class="mb-8 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
			<div class="flex items-start gap-3 mb-3">
				<CheckCircle class="w-5 h-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
				<div class="flex-grow">
					<h3 class="text-sm font-semibold text-green-900 dark:text-green-300 mb-1">Upload Complete!</h3>
					<p class="text-sm text-green-700 dark:text-green-400">
						{uploadedFiles.filter(f => f.status === 'success').length} of {uploadedFiles.length} file{uploadedFiles.length > 1 ? 's' : ''} uploaded successfully.
					</p>
				</div>
			</div>
			
			<!-- File Results List -->
			<div class="space-y-2 mt-3">
				{#each uploadedFiles as file}
					<div class="flex items-start gap-2 text-sm p-2 rounded {file.status === 'success' ? 'bg-green-100 dark:bg-green-900/30' : 'bg-red-100 dark:bg-red-900/30'}">
						<span class="font-mono text-xs {file.status === 'success' ? 'text-green-900 dark:text-green-300' : 'text-red-900 dark:text-red-300'} flex-grow">
							{file.status === 'success' ? '✓' : '✗'} {file.name}
						</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Main Upload Section -->
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
					class="flex-grow px-4 py-3 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-green-500 disabled:bg-slate-100 dark:disabled:bg-slate-800 disabled:cursor-not-allowed"
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

		<!-- Upload Button -->
		<div class="mb-8">
		<Button
			variant="primary"
			size="lg"
			onclick={uploadFile}
			disabled={!dbContext.isInitialized || isLoading}
			class="w-full"
		>
				{#if isLoading}
					<span class="flex items-center justify-center gap-3">
						<svg class="animate-spin h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						<span class="text-lg">{currentFileProgress || 'Processing...'}</span>
					</span>
				{:else}
					<span class="flex items-center justify-center gap-3">
						<Upload class="w-6 h-6" />
						<span class="text-lg">Choose File(s) to Upload</span>
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
		<div class="mt-6 p-6 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800">
			<h3 class="text-base font-semibold text-green-900 dark:text-green-300 mb-3">📤 How File Upload Works</h3>
			<ul class="text-sm text-green-800 dark:text-green-400 space-y-2">
				<li>• Select a folder to organize your files</li>
				<li>• Click the upload button to choose one or multiple files from your computer</li>
				<li>• Files will be copied to the data directory</li>
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


