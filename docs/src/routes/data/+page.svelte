<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import FileCard from '$lib/components/FileCard.svelte';
	import FileManager from '$lib/components/FileManager.svelte';
	import DatabaseConnections from '$lib/components/DatabaseConnections.svelte';
	import DropdownSelect from '$lib/components/DropdownSelect.svelte';
	import { Upload, Cable, Globe, Plus, Search, ChevronLeft, ChevronRight, DatabaseBackup, FileOutput, Table } from 'lucide-svelte';
	import { getDbContext } from '$lib/db-context';

	// Get database context from layout
	const dbContext = getDbContext();

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// DuckDB state
	let isDbInitialized = $state(false);
	let isTauriAvailable = $state(false);
	
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
	
	// Filter state - all source types enabled by default
	type SourceType = 'file' | 'database' | 'remote' | 'created';
	let activeFilters = $state<Set<SourceType>>(new Set(['file', 'database', 'remote', 'created']));
	
	// Source type filter options
	const sourceTypeOptions = ['All', 'File', 'Database', 'Remote', 'Created'];
	let selectedSourceType = $state<string>('All');
	
	// Search and pagination state
	let searchQuery = $state('');
	let currentPage = $state(1);
	const itemsPerPage = 10;
	
	function handleSourceTypeChange(event: CustomEvent<string | string[] | null>) {
		const detail = event.detail;
		// Handle both single string and array (for multiple select mode)
		const value = Array.isArray(detail) ? detail[0] : detail;
		const selectedValue = value || 'All';
		
		selectedSourceType = selectedValue;
		
		// Update active filters based on selection
		if (selectedValue === 'All') {
			activeFilters = new Set(['file', 'database', 'remote', 'created']);
		} else {
			activeFilters = new Set([selectedValue.toLowerCase() as SourceType]);
		}
		
		currentPage = 1; // Reset to first page when filter changes
	}

	async function loadRecentFiles() {
		try {
			recentFiles = await invoke<FileMetadata[]>('get_recent_files', { limit: 20 });
		} catch (error) {
			console.error('Failed to load recent files:', error);
		}
	}
	
	// Debug function to check attached tables
	async function debugAttachedTables() {
		if (!dbContext.isInitialized) {
			addSystemMessage('Database not initialized yet. Please wait...', 'warning');
			return;
		}
		
		try {
			const result = await invoke<string>('debug_list_attached_tables');
			console.log('📊 Attached Tables Debug Info:');
			console.log(result);
			addSystemMessage('Check browser console for debug info', 'info');
		} catch (error) {
			console.error('Debug error:', error);
			addSystemMessage(`Debug error: ${error}`, 'error');
		}
	}
	
	// Debug function to fix NULL timestamps
	async function debugFixTimestamps() {
		if (!dbContext.isInitialized) {
			addSystemMessage('Database not initialized yet. Please wait...', 'warning');
			return;
		}
		
		try {
			const result = await invoke<string>('debug_fix_null_timestamps');
			console.log('🔧 Fix Result:', result);
			addSystemMessage(result, 'success');
			// Reload the files after fixing
			await loadRecentFiles();
		} catch (error) {
			console.error('Fix error:', error);
			addSystemMessage(`Fix error: ${error}`, 'error');
		}
	}
	
	// Debug function to delete a table
	async function debugDeleteTable() {
		if (!dbContext.isInitialized) {
			addSystemMessage('Database not initialized yet. Please wait...', 'warning');
			return;
		}
		
		const tableName = prompt('Enter table name to delete (e.g., "apps"):');
		if (!tableName) return;
		
		if (!confirm(`Are you sure you want to delete table "${tableName}"?`)) return;
		
		try {
			const result = await invoke<string>('debug_delete_table', { tableName });
			console.log('🗑️ Delete Result:', result);
			addSystemMessage(result, 'success');
			// Reload the files after deleting
			await loadRecentFiles();
		} catch (error) {
			console.error('Delete error:', error);
			addSystemMessage(`Delete error: ${error}`, 'error');
		}
	}
	
	// Filter files based on active filters and search query
	const filteredFiles = $derived(() => {
		let files = recentFiles.filter(file => activeFilters.has(file.source_type as SourceType));
		
		// Apply search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			files = files.filter(file => 
				file.filename.toLowerCase().includes(query) ||
				file.folder.toLowerCase().includes(query) ||
				file.file_type.toLowerCase().includes(query)
			);
		}
		
		return files;
	});
	
	// Calculate total pages
	const totalPages = $derived(() => {
		return Math.ceil(filteredFiles().length / itemsPerPage);
	});
	
	// Get paginated files
	const paginatedFiles = $derived(() => {
		const start = (currentPage - 1) * itemsPerPage;
		const end = start + itemsPerPage;
		return filteredFiles().slice(start, end);
	});
	
	// Create array of items with skeleton placeholders
	const displayFiles = $derived(() => {
		const files = paginatedFiles();
		const skeletonCount = itemsPerPage - files.length;
		return {
			files,
			skeletons: Array(skeletonCount).fill(null)
		};
	});
	
	// Pagination controls
	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages()) {
			currentPage = page;
		}
	}
	
	function nextPage() {
		if (currentPage < totalPages()) {
			currentPage++;
		}
	}
	
	function previousPage() {
		if (currentPage > 1) {
			currentPage--;
		}
	}
	
	// Reset to first page when search changes
	$effect(() => {
		searchQuery;
		currentPage = 1;
	});

	onMount(() => {
		console.log('📄 DATA PAGE onMount');
		console.log('📄 dbContext:', dbContext);
		console.log('📄 dbContext.isInitializing:', dbContext.isInitializing);
		console.log('📄 dbContext.isInitialized:', dbContext.isInitialized);
		console.log('📄 dbContext.isTauriAvailable:', dbContext.isTauriAvailable);
		console.log('📄 dbContext.error:', dbContext.error);
		
		// Wait for DB initialization, then load files
		const checkAndLoad = () => {
			console.log('📄 checkAndLoad() - Checking DB state...');
			console.log('  - isInitialized:', dbContext.isInitialized);
			console.log('  - isInitializing:', dbContext.isInitializing);
			console.log('  - error:', dbContext.error);
			
			if (dbContext.isInitialized) {
				console.log('✅ DB is initialized! Loading files...');
				isTauriAvailable = true;
				isDbInitialized = true;
				loadRecentFiles();
			} else if (!dbContext.isInitializing && dbContext.error) {
				// Only show error if initialization is complete and there's an error
				console.error('❌ DB initialization failed with error');
				isTauriAvailable = dbContext.isTauriAvailable;
				addSystemMessage('Database not available', 'error');
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

		// Listen for file upload events to refresh the file list
		const handleFileUpload = () => {
			loadRecentFiles();
		};
		window.addEventListener('file-uploaded', handleFileUpload);

		// Cleanup
		return () => {
			window.removeEventListener('file-uploaded', handleFileUpload);
		};
	});
</script>

<PageLayout>
	<!-- Main Content Grid - Data Source Cards -->
	<section class="mb-12">
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<!-- Upload Card -->
			<Card
				title="Upload"
				description="Upload local files to analyze"
				href="/data/upload"
				icon={Upload}
				badgeColor="green"
			/>
			
			<!-- Connect Card -->
			<Card
				title="Connect"
				description="Connect to databases and APIs"
				href="/data/connect"
				icon={Cable}
				badgeColor="blue"
			/>
			
			<!-- Remote Card -->
			<Card
				title="Remote"
				description="Load data from remote URLs"
				href="/data/remote"
				icon={Globe}
				badgeColor="purple"
			/>
			
			<!-- Create Card -->
			<Card
				title="Create"
				description="Create new datasets manually"
				href="/data/create"
				icon={Plus}
				badge="Coming Soon"
				badgeColor="orange"
			/>
			
			<!-- Load Data Card -->
			<Card
				title="Data Loading"
				description="Refresh and reload from sources"
				href="/data/load"
				icon={DatabaseBackup}
				badgeColor="blue"
			/>
			
			<!-- Convert Card -->
			<Card
				title="Convert"
				description="Export and convert data formats"
				href="/data/convert"
				icon={FileOutput}
				badgeColor="cyan"
			/>
			
			<!-- Table View Card -->
			<Card
				title="Table View"
				description="Browse and explore your data tables"
				href="/table-view"
				icon={Table}
				badgeColor="blue"
			/>
		</div>
	</section>

	<!-- Recent Sources Section -->
	<section class="mb-12">
		<div class="mb-6">
			<div class="flex items-center justify-between">
				<h2 class="font-aspekta text-xl font-[650]">Recent Sources</h2>
				
				<div class="flex items-center gap-4">
					<!-- Search Input -->
					<div class="relative w-[500px]">
						<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
							<Search class="w-4 h-4 text-slate-400" />
						</div>
						<input
							type="text"
							bind:value={searchQuery}
							placeholder="Search files..."
							class="w-full pl-10 pr-4 py-2 text-sm border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
						/>
					</div>
					
					<!-- Source Type Filter Dropdown -->
					<div class="w-48">
						<DropdownSelect
							options={sourceTypeOptions}
							value={selectedSourceType}
							placeholder="Filter by type..."
							on:change={handleSourceTypeChange}
						/>
					</div>
				</div>
			</div>
		</div>
		
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
			{#each displayFiles().files as file}
				<FileCard
					filename={file.filename}
					folder={file.folder}
					fileType={file.file_type}
					sizeBytes={file.size_bytes}
					sourceFormat={file.source_format}
					href="/table-view?filename={encodeURIComponent(file.filename)}&folder={encodeURIComponent(file.folder)}"
				/>
			{/each}
			{#each displayFiles().skeletons as _}
				<FileCard skeleton={true} />
			{/each}
		</div>
		
		<!-- Pagination Controls -->
		{#if totalPages() > 1}
			<div class="mt-8 flex items-center justify-center gap-2">
				<button
					onclick={previousPage}
					disabled={currentPage === 1}
					class="p-2 rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
					title="Previous page"
				>
					<ChevronLeft size={16} />
				</button>
				
				<div class="flex items-center gap-1">
					{#each Array.from({ length: totalPages() }, (_, i) => i + 1) as page}
						{#if page === 1 || page === totalPages() || (page >= currentPage - 1 && page <= currentPage + 1)}
							<button
								onclick={() => goToPage(page)}
								class="px-3 py-1.5 text-sm font-medium rounded-lg transition-all {
									page === currentPage
										? 'bg-blue-600 text-white'
										: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-700'
								}"
							>
								{page}
							</button>
						{:else if page === currentPage - 2 || page === currentPage + 2}
							<span class="px-2 text-slate-400">...</span>
						{/if}
					{/each}
				</div>
				
				<button
					onclick={nextPage}
					disabled={currentPage === totalPages()}
					class="p-2 rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
					title="Next page"
				>
					<ChevronRight size={16} />
				</button>
				
				<span class="ml-4 text-sm text-slate-600 dark:text-slate-400">
					Showing {(currentPage - 1) * itemsPerPage + 1}-{Math.min(currentPage * itemsPerPage, filteredFiles().length)} of {filteredFiles().length}
				</span>
			</div>
		{/if}
	</section>

	<!-- File Manager Section -->
	<section class="mb-12">
		<FileManager
			onFileDeleted={loadRecentFiles}
			onFileRenamed={loadRecentFiles}
		/>
	</section>

	<!-- Database Connections Section (Hidden) -->
	<!-- 
		Database Connections Manager:
		- Displays active database connections (PostgreSQL, MySQL, etc.)
		- Allows users to view connection details and manage connected tables
		- Provides ability to delete/disconnect tables imported from databases
		- Triggers refresh of Recent Sources when tables are deleted
		
		To re-enable: Uncomment the section below
	-->
	<!-- 
	<section class="mb-12">
		<DatabaseConnections
			onTableDeleted={loadRecentFiles}
		/>
	</section>
	-->
</PageLayout>


