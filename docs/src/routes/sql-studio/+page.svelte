<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount, getContext } from 'svelte';
	import Button from '$lib/components/Button.svelte';
	import { Play, X, Plus, FileText, Table as TableIcon, RefreshCw, ExternalLink, Save, BookmarkPlus, Tag as TagIcon, Pencil } from 'lucide-svelte';
	import { parseQueryResultWithColumns } from '$lib/db-utils';
	import { page } from '$app/stores';
	import { toasts } from '$lib/stores/toast';

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

	// Data Models Types
	interface DataModelMetadata {
		table_name: string;
		table_type: string;
		created_at: string;
	}

	// Saved Queries Types
	interface SavedQuery {
		slug: string;
		query_name: string;
		query_sql: string;
		description: string | null;
		tags: string | null;
		created_at: string;
		updated_at: string;
	}

	// Loaded Query Tracking
	interface LoadedQueryInfo {
		slug: string;
		query_name: string;
		original_sql: string;
		description: string | null;
		tags: string | null;
	}

	// Tab management
	interface QueryTab {
		id: string;
		name: string;
		query: string;
		results: any[];
		columns: string[];
		executionTime: number | null;
		error: string | null;
		isExecuting: boolean;
		progressRowsLoaded: number;
		showProgress: boolean;
		progressMessage: string;
		progressStage: 'loading' | 'processing' | 'serializing' | 'parsing' | 'rendering';
		progressPercent: number;
		currentPage: number;
		rowsPerPage: number;
		loadedQuery: LoadedQueryInfo | null; // Track if this tab has a loaded saved query
	}

	let tabs = $state<QueryTab[]>([{
		id: '1',
		name: 'new-query.sql',
		query: '-- Start writing your SQL query here\nSELECT * FROM information_schema.tables LIMIT 10;',
		results: [],
		columns: [],
		executionTime: null,
		error: null,
		isExecuting: false,
		progressRowsLoaded: 0,
		showProgress: false,
		progressMessage: '',
		progressStage: 'loading',
		progressPercent: 0,
		currentPage: 1,
		rowsPerPage: 100,
		loadedQuery: null
	}]);
	let activeTabId = $state('1');
	let nextTabId = 2;

	// Sidebar state
	let dataModels = $state<DataModelMetadata[]>([]);
	let isLoadingModels = $state(false);
	let selectedTableName = $state<string | null>(null);
	let originalQuery = $state<string>(''); // Store original query for comparison

	// Resizable panes
	let sidebarWidth = $state(250);
	let editorHeightPercent = $state(40); // 40% for editor, 60% for results
	let isDraggingSidebar = $state(false);
	let isDraggingEditor = $state(false);

	// Saved Queries state
	let savedQueries = $state<SavedQuery[]>([]);
	let showSavedQueriesModal = $state(false);
	let showSaveQueryModal = $state(false);
	let isLoadingSavedQueries = $state(false);
	let queryName = $state('');
	let queryDescription = $state('');
	let queryTags = $state('');
	let isSavingQuery = $state(false);
	let searchQueryTerm = $state('');
	let selectedQueryTag = $state('');
	let deletingQuerySlug = $state<string | null>(null);
	let isUpdatingQuery = $state(false);

	// Derived values for saved queries
	const allQueryTags = $derived(() => {
		const tagsSet = new Set<string>();
		savedQueries.forEach(q => {
			if (q.tags) {
				q.tags.split(',').forEach(tag => tagsSet.add(tag.trim()));
			}
		});
		return Array.from(tagsSet).sort();
	});

	const filteredQueries = $derived(() => {
		return savedQueries.filter(q => {
			const matchesSearch = searchQueryTerm === '' || 
				q.query_name.toLowerCase().includes(searchQueryTerm.toLowerCase()) ||
				(q.description && q.description.toLowerCase().includes(searchQueryTerm.toLowerCase()));
			
			const matchesTag = selectedQueryTag === '' ||
				(q.tags && q.tags.split(',').some(t => t.trim() === selectedQueryTag));
			
			return matchesSearch && matchesTag;
		});
	});

	// Derived active tab
	const activeTab = $derived(() => tabs.find(t => t.id === activeTabId) || tabs[0]);
	
	// Pagination helper for active tab
	const paginatedResults = $derived(() => {
		const tab = activeTab();
		if (!tab || !tab.results.length) return [];
		
		const startIndex = (tab.currentPage - 1) * tab.rowsPerPage;
		const endIndex = startIndex + tab.rowsPerPage;
		return tab.results.slice(startIndex, endIndex);
	});
	
	const totalPages = $derived(() => {
		const tab = activeTab();
		if (!tab || !tab.results.length) return 0;
		return Math.ceil(tab.results.length / tab.rowsPerPage);
	});
	
	// Check if query has been modified from original
	const isQueryModified = $derived(() => {
		const tab = activeTab();
		return selectedTableName && tab && originalQuery && tab.query !== originalQuery;
	});

	// Check if loaded saved query has been modified
	const isSavedQueryModified = $derived(() => {
		const tab = activeTab();
		return tab && tab.loadedQuery && tab.query !== tab.loadedQuery.original_sql;
	});

	function createNewTab() {
		const newTab: QueryTab = {
			id: String(nextTabId++),
			name: `new-query-${nextTabId - 1}.sql`,
			query: '-- New query\nSELECT * FROM ',
			results: [],
			columns: [],
			executionTime: null,
			error: null,
			isExecuting: false,
			progressRowsLoaded: 0,
			showProgress: false,
			progressMessage: '',
			progressStage: 'loading',
			progressPercent: 0,
			currentPage: 1,
			rowsPerPage: 100,
			loadedQuery: null
		};
		tabs = [...tabs, newTab];
		activeTabId = newTab.id;
		
		// Clear selection when creating new tab
		selectedTableName = null;
		originalQuery = '';
	}

	function closeTab(tabId: string) {
		if (tabs.length === 1) {
			addSystemMessage('Cannot close the last tab', 'warning');
			return;
		}
		const index = tabs.findIndex(t => t.id === tabId);
		tabs = tabs.filter(t => t.id !== tabId);
		if (activeTabId === tabId) {
			activeTabId = tabs[Math.max(0, index - 1)].id;
		}
	}

	async function executeQuery() {
		const tab = activeTab();
		if (!tab || !tab.query.trim()) {
			addSystemMessage('Please enter a SQL query', 'warning');
			return;
		}

		const tabIndex = tabs.findIndex(t => t.id === tab.id);
		tabs[tabIndex].isExecuting = true;
		tabs[tabIndex].error = null;
		tabs[tabIndex].progressRowsLoaded = 0;
		tabs[tabIndex].showProgress = false;
		tabs[tabIndex].progressMessage = '';
		tabs[tabIndex].progressStage = 'loading';
		tabs[tabIndex].progressPercent = 0;
		tabs[tabIndex].currentPage = 1; // Reset to first page

		try {
			const startTime = performance.now();
			
			const result = await invoke<string>('execute_query', { query: tab.query.trim() });
			
			// Show parsing message if we have large dataset
			if (tabs[tabIndex].progressRowsLoaded > 10000) {
				tabs[tabIndex].progressStage = 'parsing';
				tabs[tabIndex].progressMessage = 'Parsing results...';
				tabs[tabIndex].progressPercent = 80;
			}
			
			const { columns, data } = parseQueryResultWithColumns(result);
			
			// Show rendering message
			if (data.length > 10000) {
				tabs[tabIndex].progressStage = 'rendering';
				tabs[tabIndex].progressMessage = `Preparing ${data.length.toLocaleString()} rows for display...`;
				tabs[tabIndex].progressPercent = 90;
				// Give UI a chance to update
				await new Promise(resolve => setTimeout(resolve, 50));
			}
			
			const endTime = performance.now();
			const executionTime = endTime - startTime;

			tabs[tabIndex].columns = columns;
			tabs[tabIndex].results = data;
			tabs[tabIndex].executionTime = executionTime;
			tabs[tabIndex].showProgress = false;
			tabs[tabIndex].progressMessage = '';
			tabs[tabIndex].progressPercent = 100;
			
			if (data.length > 0) {
				const pageInfo = data.length > 100 ? ` (page 1 of ${Math.ceil(data.length / 100)})` : '';
				addSystemMessage(`Query executed successfully (${data.length} rows in ${executionTime.toFixed(2)}ms)${pageInfo}`, 'success');
			} else {
				addSystemMessage('Query returned no results', 'warning');
			}
		} catch (error) {
			tabs[tabIndex].error = `${error}`;
			tabs[tabIndex].columns = [];
			tabs[tabIndex].results = [];
			tabs[tabIndex].showProgress = false;
			tabs[tabIndex].progressMessage = '';
			tabs[tabIndex].progressPercent = 0;
			addSystemMessage(`Query error: ${error}`, 'error');
		} finally {
			tabs[tabIndex].isExecuting = false;
		}
	}
	
	function goToPage(page: number) {
		const tab = activeTab();
		if (!tab) return;
		
		const tabIndex = tabs.findIndex(t => t.id === tab.id);
		if (tabIndex !== -1) {
			tabs[tabIndex].currentPage = Math.max(1, Math.min(page, totalPages()));
		}
	}
	
	function changeRowsPerPage(rowsPerPage: number) {
		const tab = activeTab();
		if (!tab) return;
		
		const tabIndex = tabs.findIndex(t => t.id === tab.id);
		if (tabIndex !== -1) {
			tabs[tabIndex].rowsPerPage = rowsPerPage;
			tabs[tabIndex].currentPage = 1; // Reset to first page
		}
	}

	// Saved Queries Functions
	async function loadSavedQueries() {
		try {
			isLoadingSavedQueries = true;
			const result = await invoke<string>('list_saved_queries');
			savedQueries = JSON.parse(result);
		} catch (error) {
			console.error('Error loading saved queries:', error);
			toasts.add(`Error loading saved queries: ${error}`, 'error');
		} finally {
			isLoadingSavedQueries = false;
		}
	}

	function openSaveQueryModal() {
		const tab = activeTab();
		if (!tab || !tab.query.trim()) {
			toasts.add('No query to save', 'warning');
			return;
		}
		queryName = '';
		queryDescription = '';
		queryTags = '';
		showSaveQueryModal = true;
	}

	async function saveCurrentQuery() {
		const tab = activeTab();
		if (!tab || !queryName.trim()) {
			toasts.add('Please enter a query name', 'warning');
			return;
		}

		try {
			isSavingQuery = true;
			const slug = queryName.toLowerCase().replace(/[^a-z0-9]+/g, '_');
			
			await invoke('save_query', {
				slug,
				queryName: queryName.trim(),
				querySql: tab.query,
				description: queryDescription.trim() || null,
				tags: queryTags.trim() || null
			});

			toasts.add(`Query "${queryName}" saved successfully`, 'success');
			showSaveQueryModal = false;
			await loadSavedQueries();
		} catch (error) {
			toasts.add(`Error saving query: ${error}`, 'error');
		} finally {
			isSavingQuery = false;
		}
	}

	function loadQueryFromSaved(query: SavedQuery) {
		const tab = activeTab();
		if (!tab) return;

		const tabIndex = tabs.findIndex(t => t.id === tab.id);
		if (tabIndex !== -1) {
			tabs[tabIndex].query = query.query_sql;
			tabs[tabIndex].name = `${query.query_name}.sql`;
			// Track the loaded query so we can detect modifications
			tabs[tabIndex].loadedQuery = {
				slug: query.slug,
				query_name: query.query_name,
				original_sql: query.query_sql,
				description: query.description,
				tags: query.tags
			};
			// Clear table selection state since we loaded a saved query
			selectedTableName = null;
			originalQuery = '';
		}

		showSavedQueriesModal = false;
		toasts.add(`Loaded query: ${query.query_name}`, 'success');
	}

	async function deleteSavedQuery(slug: string) {
		try {
			await invoke('delete_saved_query', { slug });
			toasts.add('Query deleted successfully', 'success');
			await loadSavedQueries();
			deletingQuerySlug = null;
		} catch (error) {
			toasts.add(`Error deleting query: ${error}`, 'error');
		}
	}

	async function updateLoadedQuery() {
		const tab = activeTab();
		if (!tab || !tab.loadedQuery) {
			toasts.add('No loaded query to update', 'warning');
			return;
		}

		try {
			isUpdatingQuery = true;
			
			await invoke('update_saved_query', {
				slug: tab.loadedQuery.slug,
				queryName: tab.loadedQuery.query_name,
				querySql: tab.query,
				description: tab.loadedQuery.description,
				tags: tab.loadedQuery.tags
			});

			// Update the loadedQuery to reflect the new saved state
			const tabIndex = tabs.findIndex(t => t.id === tab.id);
			if (tabIndex !== -1) {
				tabs[tabIndex].loadedQuery = {
					...tab.loadedQuery,
					original_sql: tab.query
				};
			}

			toasts.add(`Query "${tab.loadedQuery.query_name}" updated successfully`, 'success');
			await loadSavedQueries(); // Refresh the saved queries list
		} catch (error) {
			toasts.add(`Error updating query: ${error}`, 'error');
		} finally {
			isUpdatingQuery = false;
		}
	}

	async function loadDataModels() {
		try {
			isLoadingModels = true;
			console.log('🔍 SQL Studio: Loading ingested tables...');
			
			// Get only ingested tables from metadata
			// This matches the Data Loads page - only shows tables that are marked as ingested
			const query = `SELECT 
				tm.table_name, 
				tm.table_type,
				tm.created_at,
				tm.is_ingested
			FROM _warphead_table_metadata tm
			WHERE tm.is_ingested = true
			ORDER BY tm.table_name ASC`;
			
			console.log('🔍 SQL Studio: Executing query:', query);
			const result = await invoke<string>('execute_query', { query });
			console.log('🔍 SQL Studio: Raw result:', result);
			
			// Parse using the same utility function used for queries
			const { data: tables } = parseQueryResultWithColumns(result);
			console.log('🔍 SQL Studio: Parsed tables:', tables);
			
			// Transform to DataModelMetadata format
			dataModels = tables.map((t: any) => ({
				table_name: t.table_name,
				table_type: t.table_type || 'table',
				created_at: t.created_at || null
			}));
			
			console.log('📊 Loaded ingested tables for SQL Studio:', dataModels);
			addSystemMessage(`Loaded ${dataModels.length} ingested tables`, 'success');
		} catch (error) {
			console.error('❌ Error loading tables:', error);
			addSystemMessage(`Error loading tables: ${error}`, 'error');
			dataModels = [];
		} finally {
			isLoadingModels = false;
		}
	}

	async function loadTableQuery(tableName: string) {
		try {
			// Set the selected table
			selectedTableName = tableName;
			
			// Get columns for the table
			const columnsQuery = `SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '${tableName}' ORDER BY ordinal_position`;
			const columnsResult = await invoke<string>('execute_query', { query: columnsQuery });
			const columns: Array<{ column_name: string; data_type: string }> = JSON.parse(columnsResult);

		let query: string;
		if (columns.length > 0) {
			const columnList = columns.map(col => `  "${col.column_name}"`).join(',\n');
			query = `-- Query from table: ${tableName}\n-- ${columns.length} columns available\n\nSELECT\n${columnList}\nFROM ${tableName}\nWHERE 1=1\nLIMIT 100;`;
		} else {
			query = `-- Query from table: ${tableName}\n\nSELECT *\nFROM ${tableName}\nLIMIT 100;`;
		}

			// Store the original query for comparison
			originalQuery = query;

			const tab = activeTab();
			if (tab) {
				const tabIndex = tabs.findIndex(t => t.id === tab.id);
				tabs[tabIndex].query = query;
				tabs[tabIndex].name = `${tableName}.sql`;
			}

			addSystemMessage(`Loaded query for table: ${tableName}`, 'success');
			
			// Automatically execute the query
			await executeQuery();
		} catch (error) {
			console.error('Failed to load table query:', error);
			// Fallback query
			const query = `SELECT * FROM ${tableName} LIMIT 100;`;
			originalQuery = query;
			selectedTableName = tableName;
			
			const tab = activeTab();
			if (tab) {
				const tabIndex = tabs.findIndex(t => t.id === tab.id);
				tabs[tabIndex].query = query;
				tabs[tabIndex].name = `${tableName}.sql`;
			}
			addSystemMessage(`Loaded simple query for table: ${tableName}`, 'info');
			
			// Try to execute fallback query
			await executeQuery();
		}
	}

	// Sidebar resize handlers
	function startSidebarResize(e: MouseEvent) {
		isDraggingSidebar = true;
		e.preventDefault();
	}

	function handleMouseMove(e: MouseEvent) {
		if (isDraggingSidebar) {
			sidebarWidth = Math.max(200, Math.min(500, e.clientX));
		} else if (isDraggingEditor) {
			const container = document.getElementById('sql-studio-container');
			if (container) {
				const rect = container.getBoundingClientRect();
				const newHeightPixels = e.clientY - rect.top;
				// Convert to percentage
				const newHeightPercent = (newHeightPixels / rect.height) * 100;
				editorHeightPercent = Math.max(20, Math.min(80, newHeightPercent));
			}
		}
	}

	function stopResize() {
		isDraggingSidebar = false;
		isDraggingEditor = false;
	}

	// Editor resize handlers
	function startEditorResize(e: MouseEvent) {
		isDraggingEditor = true;
		e.preventDefault();
	}

	// Listen for data model updates to refresh the sidebar
	function handleDataModelUpdate(event: CustomEvent) {
		console.log('🔄 Data model update event received in SQL Studio:', event.detail);
		loadDataModels(); // Reload the sidebar list
	}

	onMount(() => {
		// Initialize the app
		const init = async () => {
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

			addSystemMessage('Connected to database', 'success');
			
			// Load ingested tables
			try {
				await loadDataModels();
			} catch (error) {
				addSystemMessage('Database connection issue. Some features may not work.', 'warning');
				console.error('DuckDB data loading error:', error);
			}

			// Check for query parameter in URL
			const queryParam = $page.url.searchParams.get('query');
			if (queryParam) {
				const tab = activeTab();
				if (tab) {
					const tabIndex = tabs.findIndex(t => t.id === tab.id);
					tabs[tabIndex].query = queryParam;
					tabs[tabIndex].name = 'query.sql';
					// Auto-execute the query
					await executeQuery();
				}
			}
		};

		init();

		// Listen for query progress events
		let progressUnlisten: (() => void) | undefined;
		const setupProgressListener = async () => {
			try {
				progressUnlisten = await listen<{ rowsLoaded: number; message: string }>('query-progress', (event) => {
					const tab = activeTab();
					if (tab && tab.isExecuting) {
						const tabIndex = tabs.findIndex(t => t.id === tab.id);
						if (tabIndex !== -1) {
							tabs[tabIndex].progressRowsLoaded = event.payload.rowsLoaded;
							tabs[tabIndex].progressMessage = event.payload.message;
							
							// Determine stage and progress percentage based on message
							if (event.payload.message.includes('Serializing')) {
								tabs[tabIndex].progressStage = 'serializing';
								tabs[tabIndex].progressPercent = 70;
							} else if (event.payload.message.includes('Processing')) {
								tabs[tabIndex].progressStage = 'processing';
								tabs[tabIndex].progressPercent = 60;
							} else {
								// Loading stage - calculate percentage based on row count
								// Assume 0-60% for loading phase
								tabs[tabIndex].progressStage = 'loading';
								// Use logarithmic scale for better UX (fast initial progress, slows down)
								const rows = event.payload.rowsLoaded;
								if (rows > 0) {
									// Progress from 10% to 60% during loading
									tabs[tabIndex].progressPercent = Math.min(60, 10 + (Math.log(rows) / Math.log(100000)) * 50);
								}
							}
							
							// Show progress bar if we have more than 10000 rows
							if (event.payload.rowsLoaded > 10000) {
								tabs[tabIndex].showProgress = true;
							}
						}
					}
				});
			} catch (error) {
				console.error('Failed to setup progress listener:', error);
			}
		};
		setupProgressListener();

		// Window event listeners for resize
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', stopResize);

		// Listen for data model updates
		window.addEventListener('data-model-updated', handleDataModelUpdate as EventListener);

		return () => {
			if (progressUnlisten) {
				progressUnlisten();
			}
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', stopResize);
			window.removeEventListener('data-model-updated', handleDataModelUpdate as EventListener);
		};
	});
</script>

<div id="sql-studio-container" class="flex h-full w-full bg-slate-50 dark:bg-slate-950">
	<!-- Left Sidebar: Data Sources -->
	<div class="bg-white dark:bg-slate-900 border-r border-slate-200 dark:border-slate-800 flex flex-col" style="width: {sidebarWidth}px;">
		<div class="px-3 py-2 border-b border-slate-200 dark:border-slate-800">
			<div class="flex items-center justify-between">
				<h2 class="text-xs font-semibold text-slate-900 dark:text-slate-100">Data Sources</h2>
				<button
					onclick={loadDataModels}
					class="p-0.5 rounded hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-400"
					title="Refresh tables"
				>
					<RefreshCw class="w-3.5 h-3.5" />
				</button>
			</div>
		</div>
		
		<div class="flex-1 overflow-y-auto">
			<!-- Ingested Tables Section -->
			<div class="py-1">
				<div class="flex items-center justify-between px-3 py-1.5">
					<div class="text-[10px] font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider">
						Tables
					</div>
					<a
						href="/models/data-loads"
						class="p-0.5 rounded hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-400 dark:text-slate-500 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
						title="Go to Data Loads page"
					>
						<ExternalLink class="w-3 h-3" />
					</a>
				</div>
				{#if isLoadingModels}
					<div class="px-3 py-2 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">Loading...</div>
					</div>
				{:else if dataModels.length === 0}
					<div class="px-3 py-2 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">No tables</div>
						<div class="text-[10px] text-slate-400 dark:text-slate-500 mt-0.5">Load from Data Loads</div>
					</div>
				{:else}
					{#each dataModels as model}
						<button
							type="button"
							onclick={() => loadTableQuery(model.table_name)}
							class="w-full flex items-center gap-1.5 px-3 py-1 text-left text-xs rounded-none border-l-2 {selectedTableName === model.table_name ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-transparent hover:bg-slate-50 dark:hover:bg-slate-800/50'}"
						>
							<TableIcon class="w-3.5 h-3.5 flex-shrink-0 {selectedTableName === model.table_name ? 'text-blue-600 dark:text-blue-400' : 'text-slate-400 dark:text-slate-500'}" />
							<span class="flex-1 truncate {selectedTableName === model.table_name ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-700 dark:text-slate-300'}">{model.table_name}</span>
						</button>
					{/each}
				{/if}
			</div>
		</div>
	</div>

	<!-- Sidebar Resize Handle -->
	<div
		role="separator"
		aria-label="Resize sidebar"
		class="w-1 bg-slate-200 dark:bg-slate-800 hover:bg-blue-500 dark:hover:bg-blue-500 cursor-col-resize transition-colors"
		onmousedown={startSidebarResize}
	></div>

	<!-- Main Content Area -->
	<div class="flex-1 flex flex-col min-w-0">
		<!-- Top Bar with Run Query Button -->
		<div class="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 px-4 py-2 flex items-center justify-between">
			<div class="flex items-center gap-2">
				<div class="flex items-center gap-1 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded text-xs">
					<span class="w-2 h-2 rounded-full bg-green-500"></span>
					<span class="text-slate-600 dark:text-slate-400">live</span>
				</div>
				<div class="text-xs text-slate-500 dark:text-slate-400">
					{#if activeTab()?.executionTime}
						{activeTab()?.executionTime?.toFixed(0)}ms
					{:else}
						0.00s
					{/if}
				</div>
			</div>
			<div class="flex items-center gap-2">
				<Button
					variant="secondary"
					size="xs"
					onclick={() => { showSavedQueriesModal = true; loadSavedQueries(); }}
					disabled={!dbContext.isTauriAvailable}
				>
					<BookmarkPlus class="w-3.5 h-3.5 mr-1.5" />
					Saved Queries
				</Button>
				{#if isSavedQueryModified()}
					<Button
						variant="secondary"
						size="xs"
						onclick={updateLoadedQuery}
						disabled={!dbContext.isTauriAvailable || isUpdatingQuery}
						class="bg-amber-50 dark:bg-amber-900/20 border-amber-300 dark:border-amber-700 text-amber-700 dark:text-amber-300 hover:bg-amber-100 dark:hover:bg-amber-900/30"
					>
						{#if isUpdatingQuery}
							<svg class="animate-spin h-3.5 w-3.5 mr-1.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Updating...
						{:else}
							<RefreshCw class="w-3.5 h-3.5 mr-1.5" />
							Update Query
						{/if}
					</Button>
				{:else}
					<Button
						variant="secondary"
						size="xs"
						onclick={openSaveQueryModal}
						disabled={!dbContext.isTauriAvailable}
					>
						<Save class="w-3.5 h-3.5 mr-1.5" />
						Save Query
					</Button>
				{/if}
				<Button
					variant="primary"
					size="xs"
					onclick={executeQuery}
					disabled={activeTab()?.isExecuting || !dbContext.isTauriAvailable}
				>
					{#if activeTab()?.isExecuting}
						<svg class="animate-spin h-3.5 w-3.5 mr-1.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Running...
					{:else}
						<Play class="w-3.5 h-3.5 mr-1.5" />
						Run Query
					{/if}
				</Button>
			</div>
		</div>

		<!-- Editor Tabs -->
		<div class="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 flex items-center">
			<div class="flex items-center flex-1 overflow-x-auto">
				{#each tabs as tab}
					<div
						class="group flex items-center gap-2 px-4 py-2 text-sm border-r border-slate-200 dark:border-slate-800 {tab.id === activeTabId ? 'bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100' : 'text-slate-600 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800'}"
					>
						<button
							type="button"
							onclick={() => activeTabId = tab.id}
							class="flex items-center gap-2"
						>
							<FileText class="w-4 h-4" />
							<span class="flex items-center gap-1">
								{tab.name}
								{#if tab.loadedQuery && tab.query !== tab.loadedQuery.original_sql}
									<span class="w-1.5 h-1.5 rounded-full bg-amber-500" title="Query modified"></span>
								{/if}
							</span>
						</button>
						{#if tabs.length > 1}
							<button
								type="button"
								onclick={() => closeTab(tab.id)}
								class="opacity-0 group-hover:opacity-100 hover:bg-slate-200 dark:hover:bg-slate-700 rounded p-0.5"
								title="Close tab"
							>
								<X class="w-3 h-3" />
							</button>
						{/if}
					</div>
				{/each}
			</div>
			<button
				type="button"
				onclick={createNewTab}
				class="px-3 py-2 hover:bg-slate-100 dark:hover:bg-slate-800 border-l border-slate-200 dark:border-slate-800"
				title="New query"
			>
				<Plus class="w-4 h-4 text-slate-600 dark:text-slate-400" />
			</button>
		</div>

		<!-- Editor Area -->
		<div class="bg-slate-50 dark:bg-slate-950 flex-shrink-0" style="height: {editorHeightPercent}%;">
			<textarea
				value={activeTab()?.query || ''}
				oninput={(e) => {
					const tab = activeTab();
					if (tab) {
						const tabIndex = tabs.findIndex(t => t.id === tab.id);
						if (tabIndex !== -1) {
							tabs[tabIndex].query = (e.target as HTMLTextAreaElement).value;
						}
					}
				}}
				onkeydown={(e) => {
					if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
						e.preventDefault();
						executeQuery();
					}
			}}
			placeholder="-- Write your SQL query here\nSELECT * FROM table_name LIMIT 100;"
			class="w-full h-full px-4 py-3 font-mono text-sm bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 border-0 focus:outline-none focus:ring-0 resize-none"
			disabled={!dbContext.isTauriAvailable}
			spellcheck="false"
		></textarea>
		</div>

		<!-- Editor Resize Handle -->
		<div
			role="separator"
			aria-label="Resize editor"
			class="h-1 bg-slate-200 dark:bg-slate-800 hover:bg-blue-500 dark:hover:bg-blue-500 cursor-row-resize transition-colors"
			onmousedown={startEditorResize}
		></div>

		<!-- Results Area -->
		<div class="flex-1 bg-white dark:bg-slate-900 flex flex-col min-h-0">
			<!-- Results Tabs -->
			<div class="border-b border-slate-200 dark:border-slate-800 flex items-center justify-between px-4">
				<button class="px-3 py-2 text-sm font-medium border-b-2 border-blue-500 text-blue-600 dark:text-blue-400">
					Results
					{#if activeTab()?.results.length > 0}
						<span class="ml-2 text-xs text-slate-500 dark:text-slate-400">
							{activeTab()?.results.length} rows
						</span>
					{/if}
				</button>
			</div>

			<!-- Results Table -->
			<div class="flex-1 overflow-auto">
				{#if activeTab()?.showProgress && activeTab()?.isExecuting}
					<div class="p-4">
						<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
							<div class="flex items-center justify-between mb-3">
								<div class="flex-1">
									<p class="text-sm text-blue-700 dark:text-blue-400 font-medium">
										{activeTab()?.progressMessage || 'Loading large dataset...'}
									</p>
									<p class="text-xs text-blue-600 dark:text-blue-300 mt-1">
										{activeTab()?.progressRowsLoaded.toLocaleString()} rows • {activeTab()?.progressPercent.toFixed(0)}% complete
									</p>
								</div>
							</div>
							
							<!-- Main Progress Bar -->
							<div class="relative w-full h-3 bg-blue-100 dark:bg-blue-900/50 rounded-full overflow-hidden mb-3">
								<div 
									class="absolute top-0 left-0 h-full bg-gradient-to-r from-blue-500 to-blue-600 dark:from-blue-400 dark:to-blue-500 transition-all duration-300 ease-out"
									style="width: {activeTab()?.progressPercent || 0}%"
								></div>
								<!-- Shimmer effect on top -->
								<div class="absolute top-0 left-0 h-full w-1/3 bg-gradient-to-r from-transparent via-white/30 to-transparent animate-[shimmer_1.5s_infinite]"></div>
							</div>
							
							<!-- Stage Indicators -->
							<div class="flex items-center justify-between text-xs mb-2">
								<div class="flex items-center gap-1 {activeTab()?.progressStage === 'loading' ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-400 dark:text-slate-500'}">
									<span class="w-1.5 h-1.5 rounded-full {activeTab()?.progressStage === 'loading' ? 'bg-blue-600 dark:bg-blue-400 animate-pulse' : activeTab()?.progressPercent > 0 ? 'bg-green-500' : 'bg-slate-300 dark:bg-slate-600'}"></span>
									<span>Loading</span>
								</div>
								<div class="flex items-center gap-1 {activeTab()?.progressStage === 'processing' ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-400 dark:text-slate-500'}">
									<span class="w-1.5 h-1.5 rounded-full {activeTab()?.progressStage === 'processing' ? 'bg-blue-600 dark:bg-blue-400 animate-pulse' : activeTab()?.progressPercent >= 60 ? 'bg-green-500' : 'bg-slate-300 dark:bg-slate-600'}"></span>
									<span>Processing</span>
								</div>
								<div class="flex items-center gap-1 {activeTab()?.progressStage === 'serializing' ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-400 dark:text-slate-500'}">
									<span class="w-1.5 h-1.5 rounded-full {activeTab()?.progressStage === 'serializing' ? 'bg-blue-600 dark:bg-blue-400 animate-pulse' : activeTab()?.progressPercent >= 70 ? 'bg-green-500' : 'bg-slate-300 dark:bg-slate-600'}"></span>
									<span>Serializing</span>
								</div>
								<div class="flex items-center gap-1 {activeTab()?.progressStage === 'parsing' ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-400 dark:text-slate-500'}">
									<span class="w-1.5 h-1.5 rounded-full {activeTab()?.progressStage === 'parsing' ? 'bg-blue-600 dark:bg-blue-400 animate-pulse' : activeTab()?.progressPercent >= 80 ? 'bg-green-500' : 'bg-slate-300 dark:bg-slate-600'}"></span>
									<span>Parsing</span>
								</div>
								<div class="flex items-center gap-1 {activeTab()?.progressStage === 'rendering' ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-400 dark:text-slate-500'}">
									<span class="w-1.5 h-1.5 rounded-full {activeTab()?.progressStage === 'rendering' ? 'bg-blue-600 dark:bg-blue-400 animate-pulse' : activeTab()?.progressPercent >= 90 ? 'bg-green-500' : 'bg-slate-300 dark:bg-slate-600'}"></span>
									<span>Rendering</span>
								</div>
							</div>
							
							<p class="text-xs text-blue-600 dark:text-blue-400 mt-2">
								{#if activeTab()?.progressStage === 'serializing' || activeTab()?.progressStage === 'processing'}
									Preparing data for display...
								{:else if activeTab()?.progressStage === 'parsing'}
									Parsing response data...
								{:else if activeTab()?.progressStage === 'rendering'}
									Rendering table in browser...
								{:else}
									This may take a moment for queries with more than 10,000 records
								{/if}
							</p>
						</div>
					</div>
				{:else if activeTab()?.error}
					<div class="p-4">
						<div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
							<p class="text-sm text-red-700 dark:text-red-400 font-mono">{activeTab()?.error}</p>
						</div>
					</div>
				{:else if activeTab()?.results.length === 0}
					<div class="flex items-center justify-center h-full text-slate-500 dark:text-slate-400 text-sm">
						No query results
					</div>
				{:else}
					<table class="w-full text-sm">
						<thead class="sticky top-0 bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
							<tr>
								{#each activeTab()?.columns || [] as column}
									<th class="px-4 py-2 text-left font-medium text-slate-900 dark:text-slate-100 whitespace-nowrap">
										{column}
									</th>
								{/each}
							</tr>
						</thead>
						<tbody>
							{#each paginatedResults() as row, idx}
								<tr class="border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
									{#each activeTab()?.columns || [] as column}
										<td class="px-4 py-2 text-slate-700 dark:text-slate-300 whitespace-nowrap font-mono text-xs">
											{row[column] ?? 'NULL'}
										</td>
									{/each}
								</tr>
							{/each}
						</tbody>
					</table>
				{/if}
			</div>

			<!-- Results Footer -->
			{#if activeTab()?.results.length > 0}
				<div class="border-t border-slate-200 dark:border-slate-800 px-4 py-3 flex items-center justify-between text-xs">
					<div class="flex items-center gap-4">
					<!-- Rows per page selector -->
					<div class="flex items-center gap-2 text-slate-600 dark:text-slate-400">
						<span>Rows per page:</span>
						<select
							value={activeTab()?.rowsPerPage || 100}
							onchange={(e) => changeRowsPerPage(Number((e.target as HTMLSelectElement).value))}
							class="px-2 py-1 text-xs border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
						>
							<option value={100}>100</option>
							<option value={500}>500</option>
							<option value={1000}>1000</option>
							<option value={2500}>2500</option>
							<option value={5000}>5000</option>
						</select>
					</div>
						
						<!-- Total rows info -->
						<div class="text-slate-600 dark:text-slate-400">
							{#if totalPages() > 1}
								Showing {((activeTab()?.currentPage || 1) - 1) * (activeTab()?.rowsPerPage || 100) + 1} to {Math.min((activeTab()?.currentPage || 1) * (activeTab()?.rowsPerPage || 100), activeTab()?.results.length || 0)} of {activeTab()?.results.length || 0} rows
							{:else}
								{activeTab()?.results.length || 0} rows
							{/if}
						</div>
					</div>
					
					<!-- Pagination controls -->
					{#if totalPages() > 1}
						<div class="flex items-center gap-2">
							<button
								onclick={() => goToPage(1)}
								disabled={activeTab()?.currentPage === 1}
								class="px-2 py-1 rounded hover:bg-slate-100 dark:hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed text-slate-700 dark:text-slate-300"
								title="First page"
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7"/>
								</svg>
							</button>
							
							<button
								onclick={() => goToPage((activeTab()?.currentPage || 1) - 1)}
								disabled={activeTab()?.currentPage === 1}
								class="px-2 py-1 rounded hover:bg-slate-100 dark:hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed text-slate-700 dark:text-slate-300"
								title="Previous page"
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
								</svg>
							</button>
							
							<span class="text-slate-600 dark:text-slate-400 font-medium px-2">
								Page {activeTab()?.currentPage} of {totalPages()}
							</span>
							
							<button
								onclick={() => goToPage((activeTab()?.currentPage || 1) + 1)}
								disabled={activeTab()?.currentPage === totalPages()}
								class="px-2 py-1 rounded hover:bg-slate-100 dark:hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed text-slate-700 dark:text-slate-300"
								title="Next page"
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
								</svg>
							</button>
							
							<button
								onclick={() => goToPage(totalPages())}
								disabled={activeTab()?.currentPage === totalPages()}
								class="px-2 py-1 rounded hover:bg-slate-100 dark:hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed text-slate-700 dark:text-slate-300"
								title="Last page"
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 5l7 7-7 7M5 5l7 7-7 7"/>
								</svg>
							</button>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Saved Queries Modal -->
{#if showSavedQueriesModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between sticky top-0 bg-white dark:bg-slate-900">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Saved Queries</h2>
				<button onclick={() => showSavedQueriesModal = false} class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300">
					<X class="w-5 h-5" />
				</button>
			</div>

			{#if isLoadingSavedQueries}
				<div class="flex items-center justify-center p-12">
					<div class="animate-spin h-8 w-8 border-4 border-blue-500 border-t-transparent rounded-full"></div>
				</div>
			{:else if savedQueries.length === 0}
				<div class="p-12 text-center">
					<BookmarkPlus class="w-16 h-16 text-slate-300 dark:text-slate-600 mx-auto mb-4" />
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">No saved queries yet</h3>
					<p class="text-slate-600 dark:text-slate-400 mb-6">Save your frequently used queries for quick access</p>
					<Button variant="primary" onclick={() => { showSavedQueriesModal = false; openSaveQueryModal(); }}>
						<Save class="w-4 h-4 mr-2" />
						Save Current Query
					</Button>
				</div>
			{:else}
				<!-- Search and Filter -->
				<div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center gap-4">
					<input
						type="text"
						bind:value={searchQueryTerm}
						placeholder="Search queries..."
						class="flex-1 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
					{#if allQueryTags().length > 0}
						<select
							bind:value={selectedQueryTag}
							class="px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
						>
							<option value="">All Tags</option>
							{#each allQueryTags() as tag}
								<option value={tag}>{tag}</option>
							{/each}
						</select>
					{/if}
					<span class="text-sm text-slate-500 dark:text-slate-400">
						{filteredQueries().length} quer{filteredQueries().length === 1 ? 'y' : 'ies'}
					</span>
				</div>

				<!-- Queries Grid -->
				<div class="p-6 grid grid-cols-1 lg:grid-cols-2 gap-4">
					{#each filteredQueries() as query}
						<div class="bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg p-4 hover:shadow-md transition-shadow">
							<!-- Header -->
							<div class="flex items-start justify-between mb-3">
								<div class="flex-1 min-w-0">
									<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100 truncate mb-1">
										{query.query_name}
									</h3>
									{#if query.description}
										<p class="text-xs text-slate-600 dark:text-slate-400 line-clamp-2">
											{query.description}
										</p>
									{/if}
								</div>
								<div class="flex items-center gap-1 ml-2">
									<button
										onclick={() => loadQueryFromSaved(query)}
										class="p-1.5 rounded hover:bg-slate-200 dark:hover:bg-slate-700 text-blue-600 dark:text-blue-400"
										title="Load query"
									>
										<FileText class="w-4 h-4" />
									</button>
									<button
										onclick={() => deletingQuerySlug = query.slug}
										class="p-1.5 rounded hover:bg-slate-200 dark:hover:bg-slate-700 text-red-600 dark:text-red-400"
										title="Delete query"
									>
										<X class="w-4 h-4" />
									</button>
								</div>
							</div>

							<!-- SQL Preview -->
							<div class="mb-3 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded p-2">
								<code class="text-xs font-mono text-slate-700 dark:text-slate-300 line-clamp-3 whitespace-pre-wrap">
									{query.query_sql}
								</code>
							</div>

							<!-- Tags -->
							{#if query.tags}
								<div class="flex items-center gap-1 flex-wrap mb-2">
									{#each query.tags.split(',').map(t => t.trim()).filter(t => t) as tag}
										<span class="inline-flex items-center gap-1 text-xs px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-300">
											<TagIcon class="w-2.5 h-2.5" />
											{tag}
										</span>
									{/each}
								</div>
							{/if}

							<!-- Metadata -->
							<div class="text-xs text-slate-400 dark:text-slate-500">
								Updated {new Date(query.updated_at).toLocaleDateString()}
							</div>

							<!-- Delete Confirmation -->
							{#if deletingQuerySlug === query.slug}
								<div class="mt-3 pt-3 border-t border-slate-200 dark:border-slate-700 flex items-center gap-2">
									<span class="text-sm text-slate-700 dark:text-slate-300 flex-1">Delete this query?</span>
									<Button size="sm" variant="primary" onclick={() => deleteSavedQuery(query.slug)} class="bg-red-500 hover:bg-red-600">
										Delete
									</Button>
									<Button size="sm" variant="secondary" onclick={() => deletingQuerySlug = null}>
										Cancel
									</Button>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- Save Query Modal -->
{#if showSaveQueryModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-xl w-full">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Save Query</h2>
			</div>

			<div class="p-6 space-y-4">
				<!-- Query Name -->
				<div>
					<label for="query-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Query Name *
					</label>
					<input
						id="query-name"
						type="text"
						bind:value={queryName}
						placeholder="My Important Query"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
				</div>

				<!-- Description -->
				<div>
					<label for="query-description" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Description
					</label>
					<textarea
						id="query-description"
						bind:value={queryDescription}
						placeholder="What does this query do?"
						rows="3"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
					></textarea>
				</div>

				<!-- Tags -->
				<div>
					<label for="query-tags" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Tags
					</label>
					<input
						id="query-tags"
						type="text"
						bind:value={queryTags}
						placeholder="reporting, sales, monthly (comma-separated)"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">Separate tags with commas</p>
				</div>

				<!-- SQL Preview -->
				<div>
					<div class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						SQL Query
					</div>
					<div class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-md p-3 max-h-32 overflow-auto">
						<code class="text-xs font-mono text-slate-700 dark:text-slate-300 whitespace-pre-wrap">
							{activeTab()?.query || ''}
						</code>
					</div>
				</div>
			</div>

			<div class="p-6 border-t border-slate-200 dark:border-slate-800 flex items-center justify-end gap-3">
				<Button variant="secondary" onclick={() => showSaveQueryModal = false} disabled={isSavingQuery}>
					Cancel
				</Button>
				<Button variant="primary" onclick={saveCurrentQuery} disabled={isSavingQuery || !queryName.trim()}>
					{#if isSavingQuery}
						<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Saving...
					{:else}
						<Save class="w-4 h-4 mr-2" />
						Save Query
					{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
