<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { tabs } from '$lib/stores/tabs';
	import Button from '$lib/components/Button.svelte';
	import { Trash2, Edit2, FolderOpen, FileText, X, Check, Database } from 'lucide-svelte';
	
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
	
	interface Props {
		onFileDeleted?: () => void;
		onFileRenamed?: () => void;
	}
	
	let { onFileDeleted, onFileRenamed }: Props = $props();
	
	// Helper to check if an item is a database table (not a physical file)
	function isDatabaseTable(file: FileMetadata): boolean {
		return file.source_type === 'database' || file.folder === 'database';
	}
	
	let allFiles = $state<FileMetadata[]>([]);
	let isLoading = $state(false);
	let filterFolder = $state<string>('all');
	let folders = $state<string[]>(['all']);
	let editingFile = $state<string | null>(null);
	let newFilename = $state('');
	let deleteConfirm = $state<string | null>(null);
	
	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// Handle opening in new tab
	function openInNewTab(e: MouseEvent, file: FileMetadata) {
		e.preventDefault();
		e.stopPropagation();
		
		const href = `/table-view?filename=${encodeURIComponent(file.filename)}&folder=${encodeURIComponent(file.folder)}`;
		const title = isDatabaseTable(file) ? file.filename.replace('.db', '') : file.filename;
		
		console.log('➕ Opening file in new tab:', title);
		
		// Add new tab to the tab bar
		tabs.addTab(title, href);
	}

	// Handle right-click to open in new tab
	function handleContextMenu(e: MouseEvent, file: FileMetadata) {
		e.preventDefault();
		openInNewTab(e, file);
	}
	
	async function loadAllFiles() {
		try {
			isLoading = true;
			allFiles = await invoke<FileMetadata[]>('get_all_files_metadata');
			
			// Extract unique folders
			const uniqueFolders = ['all', ...new Set(allFiles.map(f => f.folder))];
			folders = uniqueFolders;
		} catch (error) {
			addSystemMessage(`Error loading files: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}
	
	const filteredFiles = $derived(() => {
		if (filterFolder === 'all') {
			return allFiles;
		}
		return allFiles.filter(f => f.folder === filterFolder);
	});
	
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
	
	function startEdit(file: FileMetadata) {
		editingFile = file.filename;
		newFilename = file.filename;
	}
	
	function cancelEdit() {
		editingFile = null;
		newFilename = '';
	}
	
	async function saveRename(file: FileMetadata) {
		if (!newFilename.trim() || newFilename === file.filename) {
			cancelEdit();
			return;
		}
		
		try {
			const result = await invoke<string>('rename_file', {
				filename: file.filename,
				folder: file.folder,
				newFilename: newFilename.trim()
			});
			
			addSystemMessage(result, 'success');
			cancelEdit();
			await loadAllFiles();
			
			if (onFileRenamed) {
				onFileRenamed();
			}
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
		}
	}
	
	function startDelete(filename: string) {
		deleteConfirm = filename;
	}
	
	function cancelDelete() {
		deleteConfirm = null;
	}
	
	async function confirmDelete(file: FileMetadata) {
		try {
			const result = await invoke<string>('delete_file', {
				filename: file.filename,
				folder: file.folder
			});
			
			addSystemMessage(result, 'success');
			deleteConfirm = null;
			await loadAllFiles();
			
			if (onFileDeleted) {
				onFileDeleted();
			}
		} catch (error) {
			addSystemMessage(`Error: ${error}`, 'error');
		}
	}
	
	// Load files on mount
	$effect(() => {
		loadAllFiles();
	});
</script>

<div class="bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700 overflow-hidden">
	<!-- Header -->
	<div class="p-6 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100">Data Manager</h2>
				<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
					Manage all data sources ({filteredFiles().length} {filterFolder === 'all' ? 'total' : `in ${filterFolder}`})
				</p>
			</div>
			
			<!-- Folder Filter -->
			<div class="flex items-center gap-3">
				<label for="folder-filter" class="text-sm text-slate-600 dark:text-slate-400">
					Filter by folder:
				</label>
				<select
					id="folder-filter"
					bind:value={filterFolder}
					class="px-3 py-2 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
				>
					{#each folders as folder}
						<option value={folder}>
							{folder === 'all' ? 'All Folders' : folder}
						</option>
					{/each}
				</select>
				
				<Button
					variant="ghost"
					size="sm"
					onclick={loadAllFiles}
					disabled={isLoading}
					title="Refresh file list"
				>
					{#if isLoading}
						<svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
					{:else}
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
						</svg>
					{/if}
				</Button>
			</div>
		</div>
	</div>
	
	<!-- Table -->
	<div class="overflow-x-auto">
		<table class="w-full">
			<thead class="bg-slate-50 dark:bg-slate-800/50">
				<tr>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Name
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Source
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Type
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Size
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Rows
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Created
					</th>
					<th class="px-6 py-3 text-right text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Actions
					</th>
				</tr>
			</thead>
			<tbody class="bg-white dark:bg-slate-900 divide-y divide-slate-200 dark:divide-slate-700">
				{#if isLoading}
					<tr>
						<td colspan="7" class="px-6 py-12 text-center text-slate-500 dark:text-slate-400">
							<svg class="animate-spin h-8 w-8 mx-auto mb-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Loading files...
						</td>
					</tr>
				{:else if filteredFiles().length === 0}
					<tr>
						<td colspan="7" class="px-6 py-12 text-center text-slate-500 dark:text-slate-400">
							<FileText class="w-12 h-12 mx-auto mb-2 opacity-50" />
							<p class="font-medium">No data sources found</p>
							<p class="text-sm mt-1">Upload files or connect to a database to get started</p>
						</td>
					</tr>
				{:else}
					{#each filteredFiles() as file (file.filename + file.folder)}
						<tr class="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
							<td class="px-6 py-4 whitespace-nowrap">
								{#if editingFile === file.filename && !isDatabaseTable(file)}
									<div class="flex items-center gap-2">
										<input
											type="text"
											bind:value={newFilename}
											onkeydown={(e) => {
												if (e.key === 'Enter') saveRename(file);
												if (e.key === 'Escape') cancelEdit();
											}}
											class="flex-grow px-2 py-1 border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
										/>
										<button
											onclick={() => saveRename(file)}
											class="p-1 text-green-600 hover:text-green-700 dark:text-green-400"
											title="Save"
										>
											<Check class="w-4 h-4" />
										</button>
										<button
											onclick={cancelEdit}
											class="p-1 text-slate-600 hover:text-slate-700 dark:text-slate-400"
											title="Cancel"
										>
											<X class="w-4 h-4" />
										</button>
									</div>
								{:else}
									<div class="group/row relative flex items-center gap-2">
										{#if isDatabaseTable(file)}
											<Database class="w-4 h-4 text-blue-500" />
										{:else}
											<FileText class="w-4 h-4 text-slate-400" />
										{/if}
										<a
											href="/table-view?filename={encodeURIComponent(file.filename)}&folder={encodeURIComponent(file.folder)}"
											oncontextmenu={(e) => handleContextMenu(e, file)}
											class="text-sm font-medium text-blue-600 dark:text-blue-400 hover:underline"
										>
											{isDatabaseTable(file) ? file.filename.replace('.db', '') : file.filename}
										</a>
									</div>
								{/if}
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="flex items-center gap-2">
									{#if isDatabaseTable(file)}
										<span class="text-xs font-semibold px-2 py-1 rounded bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">
											{file.source_format.toUpperCase()}
										</span>
									{:else}
										<FolderOpen class="w-4 h-4 text-slate-400" />
										<span class="text-sm text-slate-900 dark:text-slate-100">{file.folder}</span>
									{/if}
								</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<span class="text-xs font-semibold px-2 py-1 rounded {
									file.file_type === 'CSV' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' :
									file.file_type === 'Parquet' ? 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400' :
									file.file_type === 'JSON' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400' :
									file.file_type === 'CACHED' || file.file_type === 'IMPORTED' ? 'bg-indigo-100 text-indigo-700 dark:bg-indigo-900/30 dark:text-indigo-400' :
									'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-400'
								}">
									{file.file_type}
								</span>
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
								{isDatabaseTable(file) ? 'N/A' : formatBytes(file.size_bytes)}
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
								{formatRowCount(file.row_count)}
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
								{formatDate(file.uploaded_at)}
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
								{#if deleteConfirm === file.filename}
									<div class="flex items-center justify-end gap-2">
										<span class="text-xs text-red-600 dark:text-red-400 mr-2">Delete?</span>
										<Button
											variant="danger"
											size="sm"
											onclick={() => confirmDelete(file)}
										>
											<Check class="w-4 h-4" />
										</Button>
										<Button
											variant="ghost"
											size="sm"
											onclick={cancelDelete}
										>
											<X class="w-4 h-4" />
										</Button>
									</div>
								{:else if editingFile !== file.filename}
									<div class="flex items-center justify-end gap-2">
										{#if !isDatabaseTable(file)}
											<Button
												variant="ghost"
												size="sm"
												onclick={() => startEdit(file)}
												title="Rename file"
											>
												<Edit2 class="w-4 h-4" />
											</Button>
										{/if}
										<Button
											variant="ghost"
											size="sm"
											onclick={() => startDelete(file.filename)}
											title={isDatabaseTable(file) ? "Drop table" : "Delete file"}
										>
											<Trash2 class="w-4 h-4 text-red-600 dark:text-red-400" />
										</Button>
									</div>
								{/if}
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>

