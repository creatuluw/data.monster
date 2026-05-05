<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Tooltip from '$lib/components/Tooltip.svelte';
	import RecordViewDrawer from '$lib/components/RecordViewDrawer.svelte';
	import { Database, Play, Save, Trash2, FileText, Table as TableIcon, Network, Maximize2, Minimize2, XCircle } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { parseQueryResult, parseQueryResultWithColumns } from '$lib/db-utils';
	import { 
		executeQuery as executeQueryDB, 
		createTableFromQuery, 
		getTableCreationQuery, 
		listTables,
		detectTableRelationships
	} from '$lib/db-operations';
	import { 
		formatSQLQuery, 
		formatSelectQuery, 
		formatQueryWithHeader,
		extractSelectFromCreateTable 
	} from '$lib/utils/sql-formatter';
	
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
	
	interface RelationshipInfo {
		from_table: string;
		from_column: string;
		to_table: string;
		to_column: string;
	}
	
	// Sidebar state
	let dataModels = $state<DataModelMetadata[]>([]);
	let availableFiles = $state<FileMetadata[]>([]);
	let isLoadingModels = $state(false);
	let isLoadingFiles = $state(false);
	let isTauriAvailable = $state(false);
	let selectedTableName = $state<string | null>(null);
	let originalQuery = $state<string>(''); // Store original query for comparison
	let fullQuery = $state<string>(''); // Store full query without limit for execution
	
	// Folder tree state
	interface FolderTree {
		[folderName: string]: FileMetadata[];
	}
	let folderTree = $derived(() => {
		const tree: FolderTree = {};
		for (const file of availableFiles) {
			if (!tree[file.folder]) {
				tree[file.folder] = [];
			}
			tree[file.folder].push(file);
		}
		return tree;
	});
	let expandedFolders = $state<Set<string>>(new Set(['main']));
	
	function toggleFolder(folderName: string) {
		if (expandedFolders.has(folderName)) {
			expandedFolders.delete(folderName);
		} else {
			expandedFolders.add(folderName);
		}
		expandedFolders = new Set(expandedFolders);
	}
	
	// Resizable panes
	let sidebarWidth = $state(250);
	let isDraggingSidebar = $state(false);
	
	// Query state
	let sqlQuery = $state(`-- Example: Query data from tables loaded via /data route
-- Reference tables by name (derived from filename without extension)

SELECT 
  *
FROM sample_data
LIMIT 100`);
	let tableName = $state('');
	let queryResults = $state<any[]>([]);
	let queryColumns = $state<string[]>([]);
	let isExecuting = $state(false);
	let isSaving = $state(false);
	let queryError = $state('');
	let executionTime = $state<number | null>(null);
	let detectedRelationships = $state<RelationshipInfo[]>([]);
	let activeTab = $state<'results' | 'relationships' | 'meta'>('results');
	let tableStats = $state<{
		rowCount: number;
		columnCount: number;
		columns: Array<{ name: string; type: string; nullCount: number; uniqueCount: number }>;
	} | null>(null);
	
	// Record drawer state
	let isRecordDrawerOpen = $state(false);
	let selectedRecord = $state<any | null>(null);
	let selectedRecordIndex = $state(0);
	
	// Fullscreen SQL editor state
	let isFullscreenSQL = $state(false);
	
	// Long-running query modal state
	let showLoadingModal = $state(false);
	let queryStartTime = $state<number | null>(null);
	let loadingModalTimer: ReturnType<typeof setTimeout> | null = null;
	let queryAborted = $state(false);
	let currentQueryId = $state(0); // Track query execution ID
	let elapsedTime = $state(0);
	let elapsedTimeInterval: ReturnType<typeof setInterval> | null = null;
	
	// Check if query has been modified from original
	const isQueryModified = $derived(() => {
		return selectedTableName && originalQuery && sqlQuery !== originalQuery;
	});
	
	function openRecordDrawer(record: any, index: number) {
		selectedRecord = record;
		selectedRecordIndex = index;
		isRecordDrawerOpen = true;
	}
	
	function closeRecordDrawer() {
		isRecordDrawerOpen = false;
		selectedRecord = null;
		selectedRecordIndex = 0;
	}
	
	function toggleFullscreenSQL() {
		isFullscreenSQL = !isFullscreenSQL;
	}
	
	async function loadRelationshipsForTable(tableNameParam: string) {
		try {
			// Load relationships where this table is either the source or target
			const query = `
				SELECT * FROM _warphead_relationships 
				WHERE from_table = '${tableNameParam}' OR to_table = '${tableNameParam}'
				ORDER BY from_table, from_column
			`;
			const result = await invoke<string>('execute_query', { query });
			const relationships: RelationshipInfo[] = parseQueryResult(result);
			detectedRelationships = relationships;
			
			if (relationships.length > 0) {
				console.log(`📊 Loaded ${relationships.length} relationship(s) for table: ${tableNameParam}`, relationships);
			} else {
				console.log(`ℹ️ No relationships found for table: ${tableNameParam}`);
			}
		} catch (error) {
			console.error('Error loading relationships:', error);
			detectedRelationships = [];
		}
	}
	
	async function calculateTableStats() {
		if (!queryColumns.length || !queryResults.length) {
			tableStats = null;
			return;
		}

		try {
			const columns = await Promise.all(queryColumns.map(async (colName) => {
				const values = queryResults.map(row => row[colName]);
				const nullCount = values.filter(v => v === null || v === undefined).length;
				const uniqueValues = new Set(values.filter(v => v !== null && v !== undefined));
				
				// Try to infer type from values
				let type = 'unknown';
				const sampleValue = values.find(v => v !== null && v !== undefined);
				if (sampleValue !== undefined) {
					if (typeof sampleValue === 'number') {
						type = Number.isInteger(sampleValue) ? 'INTEGER' : 'DOUBLE';
					} else if (typeof sampleValue === 'boolean') {
						type = 'BOOLEAN';
					} else if (typeof sampleValue === 'string') {
						// Check if it's a date
						if (!isNaN(Date.parse(sampleValue)) && sampleValue.match(/^\d{4}-\d{2}-\d{2}/)) {
							type = 'DATE/TIMESTAMP';
						} else {
							type = 'VARCHAR';
						}
					}
				}

				return {
					name: colName,
					type,
					nullCount,
					uniqueCount: uniqueValues.size
				};
			}));

			tableStats = {
				rowCount: queryResults.length,
				columnCount: queryColumns.length,
				columns
			};
		} catch (error) {
			console.error('Error calculating stats:', error);
			tableStats = null;
		}
	}

	async function executeQuery() {
		// ALWAYS use the query from the editor (sqlQuery)
		// This ensures LIMIT 100 queries are respected
		const queryToExecute = sqlQuery;
		
		if (!queryToExecute.trim()) {
			addSystemMessage('Please enter a SQL query', 'warning');
			return;
		}
		
		// Increment query ID to track this specific execution
		currentQueryId++;
		const thisQueryId = currentQueryId;
		
		try {
			isExecuting = true;
			queryAborted = false;
			queryError = '';
			queryResults = [];
			queryColumns = [];
			tableStats = null;
			queryStartTime = performance.now();
			elapsedTime = 0;
			
			// Set up timer to show loading modal after 5 seconds
			loadingModalTimer = setTimeout(() => {
				if (!queryAborted && thisQueryId === currentQueryId) {
					showLoadingModal = true;
				}
			}, 3000); // Show at 3 seconds to give earlier warning
			
			// Update elapsed time every 100ms when modal is shown
			elapsedTimeInterval = setInterval(() => {
				if (queryStartTime && showLoadingModal && !queryAborted) {
					elapsedTime = (performance.now() - queryStartTime) / 1000;
				}
			}, 100);
			
			const startTime = performance.now();
			const result = await executeQueryDB(queryToExecute.trim());
			
			// Check if this query was aborted or superseded by another query
			if (queryAborted || thisQueryId !== currentQueryId) {
				return; // Ignore results from aborted/old queries
			}
			
			const { columns, data } = parseQueryResultWithColumns(result);
			const endTime = performance.now();
			executionTime = endTime - startTime;
			
			queryColumns = columns;
			queryResults = data;
			
			// Calculate stats for the Stats tab
			await calculateTableStats();
			
			if (data.length > 0) {
				addSystemMessage(`Query returned ${data.length} rows in ${executionTime.toFixed(0)}ms`, 'success');
			} else {
				addSystemMessage('Query returned no results', 'warning');
			}
		} catch (error) {
			// Check if this query was aborted
			if (queryAborted || thisQueryId !== currentQueryId) {
				return; // Ignore errors from aborted/old queries
			}
			
			queryError = `${error}`;
			addSystemMessage(`Query error: ${error}`, 'error');
		} finally {
			// Only clean up if this is still the current query
			if (thisQueryId === currentQueryId) {
				isExecuting = false;
				showLoadingModal = false;
				queryStartTime = null;
				elapsedTime = 0;
				if (loadingModalTimer) {
					clearTimeout(loadingModalTimer);
					loadingModalTimer = null;
				}
				if (elapsedTimeInterval) {
					clearInterval(elapsedTimeInterval);
					elapsedTimeInterval = null;
				}
			}
		}
	}
	
	// Helper function to add LIMIT 100 to a query if it doesn't have a LIMIT clause
	function addLimitToQuery(query: string, limit: number = 100): string {
		const trimmedQuery = query.trim();
		// Check if query already has a LIMIT clause (case-insensitive)
		if (/\bLIMIT\s+\d+/i.test(trimmedQuery)) {
			return trimmedQuery;
		}
		// Add LIMIT to the end
		return `${trimmedQuery}\nLIMIT ${limit}`;
	}
	
	function abortQuery() {
		// Call the backend to actually cancel the query in DuckDB
		invoke('cancel_query')
			.then(() => {
				console.log('Query cancellation sent to backend');
			})
			.catch((error) => {
				console.error('Error cancelling query:', error);
			});
		
		// Immediately mark as aborted
		queryAborted = true;
		
		// Increment query ID to invalidate current query
		currentQueryId++;
		
		// Immediately update UI state
		isExecuting = false;
		showLoadingModal = false;
		queryStartTime = null;
		elapsedTime = 0;
		
		// Clean up timers
		if (loadingModalTimer) {
			clearTimeout(loadingModalTimer);
			loadingModalTimer = null;
		}
		if (elapsedTimeInterval) {
			clearInterval(elapsedTimeInterval);
			elapsedTimeInterval = null;
		}
		
		addSystemMessage('Query cancelled successfully', 'success');
	}
	
	async function saveTableToDataModel() {
		console.log('🔘 Add to Data Model button clicked!');
		console.log('📊 Current state:', {
			tableName: tableName,
			tableNameTrimmed: tableName.trim(),
			queryResultsLength: queryResults.length,
			isSaving
		});
		
		if (!tableName.trim()) {
			addSystemMessage('Please enter a table name', 'warning');
			return;
		}
		
		if (!queryResults.length) {
			addSystemMessage('Please execute a query first to preview results', 'warning');
			return;
		}
		
		try {
			isSaving = true;
			const newTableName = tableName.trim();
			addSystemMessage(`Saving table '${newTableName}'...`, 'info');
			console.log('💾 Saving table:', newTableName);
			
			// Create the table
			const result = await createTableFromQuery(newTableName, sqlQuery);
			console.log('✅ Table created:', result);
			addSystemMessage(result, 'success');
			
			// Detect relationships
			const relationshipsJson = await detectTableRelationships(newTableName);
			const relationships: RelationshipInfo[] = JSON.parse(relationshipsJson);
			
			if (relationships.length > 0) {
				detectedRelationships = relationships;
				addSystemMessage(`Detected ${relationships.length} relationship(s) based on matching field names`, 'success');
			} else {
				detectedRelationships = [];
				addSystemMessage('No relationships detected', 'info');
			}
			
			console.log('🔄 Reloading data models...');
			// Reload data models
			await loadDataModels();
			console.log('✅ Data models reloaded, current list:', dataModels);
			
			// Automatically load the newly created table
			await loadTableQuery(newTableName);
			
			// Clear the table name input
			tableName = '';
			
			// Notify data model canvas to refresh
			console.log('📢 Dispatching data-model-updated event');
			window.dispatchEvent(new CustomEvent('data-model-updated', {
				detail: { action: 'created', tableName: newTableName }
			}));
		} catch (error) {
			console.error('❌ Error saving table:', error);
			addSystemMessage(`Error saving table: ${error}`, 'error');
		} finally {
			isSaving = false;
		}
	}
	
	async function updateSelectedTable() {
		if (!selectedTableName) {
			addSystemMessage('No table selected to update', 'warning');
			return;
		}

		if (!queryResults.length) {
			addSystemMessage('Please execute the query first to verify it works', 'warning');
			return;
		}

		try {
			isSaving = true;
			const tableToUpdate = selectedTableName;
			const tempTableName = `_temp_update_${tableToUpdate}_${Date.now()}`;
			
			addSystemMessage(`Updating table '${tableToUpdate}'...`, 'info');
			
			// STEP 1: Create temp table with the new query
			try {
				await createTableFromQuery(tempTableName, sqlQuery);
				addSystemMessage(`Created temporary table for validation...`, 'info');
			} catch (error) {
				throw new Error(`Failed to create new table with updated query: ${error}`);
			}
			
			// STEP 2: Clean up old table metadata
			await executeQueryDB(`DELETE FROM _warphead_table_metadata WHERE table_name = '${tableToUpdate}'`);
			
			await executeQueryDB(`DELETE FROM datamodels WHERE table_name = '${tableToUpdate}'`);
			
			// STEP 3: Drop the old table
			await executeQueryDB(`DROP TABLE IF EXISTS ${tableToUpdate}`);
			
			// STEP 4: Rename temp table to final name
			await executeQueryDB(`ALTER TABLE ${tempTableName} RENAME TO ${tableToUpdate}`);
			
			// STEP 5: Clean up temp table metadata and register the final table
			await executeQueryDB(`DELETE FROM _warphead_table_metadata WHERE table_name = '${tempTableName}'`);
			
			await executeQueryDB(`DELETE FROM datamodels WHERE table_name = '${tempTableName}'`);
			
			// Register the updated table in datamodels
			await executeQueryDB(`INSERT INTO datamodels (table_name) VALUES ('${tableToUpdate}') ON CONFLICT DO NOTHING`);
			
			// Register table in metadata as 'model' type with the updated query
			// Escape single quotes in SQL query for insertion
			const escapedQuery = sqlQuery.replace(/'/g, "''");
			await executeQueryDB(`INSERT INTO _warphead_table_metadata (table_name, table_type, creation_query) 
			        VALUES ('${tableToUpdate}', 'model', '${escapedQuery}') 
			        ON CONFLICT (table_name) DO UPDATE SET 
			          table_type = EXCLUDED.table_type, 
			          creation_query = EXCLUDED.creation_query`);
			
			addSystemMessage(`Successfully updated table: ${tableToUpdate}`, 'success');
			
			// Update the original query to the new one
			originalQuery = sqlQuery;
			
			// Detect relationships
			const relationshipsJson = await invoke<string>('detect_table_relationships', {
				tableName: tableToUpdate
			});
			const relationships: RelationshipInfo[] = JSON.parse(relationshipsJson);
			
			if (relationships.length > 0) {
				detectedRelationships = relationships;
				addSystemMessage(`Detected ${relationships.length} relationship(s)`, 'success');
			} else {
				detectedRelationships = [];
			}
			
			// Reload data models to refresh the list
			await loadDataModels();
			
			// Keep the table selected and re-execute to show updated results
			await executeQuery();
			
			// Notify data model canvas to refresh
			window.dispatchEvent(new CustomEvent('data-model-updated', {
				detail: { action: 'updated', tableName: tableToUpdate }
			}));
		} catch (error) {
			addSystemMessage(`Error updating table: ${error}`, 'error');
			// Try to clean up temp table if it exists
			const tempTableName = `_temp_update_${selectedTableName}_`;
			try {
				await invoke<string>('execute_query', { 
					query: `DROP TABLE IF EXISTS ${tempTableName}*` 
				});
			} catch (cleanupError) {
				// Ignore cleanup errors
			}
		} finally {
			isSaving = false;
		}
	}
	
	async function loadDataModels() {
		try {
			isLoadingModels = true;
			// Load only 'model' type tables (derived tables) - source tables shown in SOURCE FILES section
			const query = 'SELECT table_name, table_type, created_at FROM _warphead_table_metadata WHERE table_type = \'model\' ORDER BY created_at DESC';
			const result = await invoke<string>('execute_query', { query });
			dataModels = parseQueryResult(result);
			console.log('📊 Loaded data models:', dataModels);
		} catch (error) {
			console.error('Error loading data models:', error);
			dataModels = [];
		} finally {
			isLoadingModels = false;
		}
	}
	
	async function loadAvailableFiles() {
		try {
			isLoadingFiles = true;
			availableFiles = await invoke<FileMetadata[]>('get_recent_files', { limit: 100 });
		} catch (error) {
			console.error('Error loading files:', error);
		} finally {
			isLoadingFiles = false;
		}
	}
	
	async function loadTableQuery(tableNameParam: string) {
		try {
			// Set the selected table
			selectedTableName = tableNameParam;
			
			// Load relationships for this table
			await loadRelationshipsForTable(tableNameParam);
			
			// Try to get the original creation query first
			const creationQuery = await invoke<string | null>('get_table_creation_query', { 
				tableName: tableNameParam 
			});
			
		if (creationQuery) {
			// We have the original query! Extract the SELECT part
			let selectQuery = extractSelectFromCreateTable(creationQuery);
			
			// Format the query without the header
			const formattedQuery = formatSQLQuery(selectQuery);
			
			// Store the full query without limit
			fullQuery = formattedQuery;
			originalQuery = formattedQuery;
			
			// Display the limited version in the editor
			sqlQuery = addLimitToQuery(formattedQuery, 100);
			
			addSystemMessage(`Loaded query for table: ${tableNameParam} (showing first 100 rows)`, 'success');
			
			// Automatically execute the LIMITED query
			await executeQuery();
		} else {
				// Fallback: No creation query stored (legacy table), use column-based query
				await loadTableQueryFallback(tableNameParam);
			}
		} catch (error) {
			console.error('Failed to load table query:', error);
			// Fallback on error
			await loadTableQueryFallback(tableNameParam);
		}
	}
	
	async function loadTableQueryFallback(tableNameParam: string) {
		try {
			// Get columns for the table using DESCRIBE (works for all tables)
			const columnsQuery = `DESCRIBE ${tableNameParam}`;
			const columnsResult = await invoke<string>('execute_query', { query: columnsQuery });
			const describeRows = JSON.parse(columnsResult);
			
		let query: string;
		if (describeRows.length > 0) {
			// Extract column info from DESCRIBE result
			const columns = describeRows.map((row: any) => ({
				name: row.column_name,
				type: row.column_type
			}));
			
			// Use the formatter to create a clean query without header
			query = formatSelectQuery(tableNameParam, columns, false);
		} else {
			// Ultimate fallback
			query = formatSQLQuery(`SELECT * FROM ${tableNameParam}`);
		}
			
			// Store the full query without limit
			fullQuery = query;
			originalQuery = query;
			
			// Display the limited version in the editor
			sqlQuery = addLimitToQuery(query, 100);
			
			addSystemMessage(`Loaded query for table: ${tableNameParam} (showing first 100 rows)`, 'info');
			
			// Automatically execute the LIMITED query
			await executeQuery();
		} catch (error) {
			console.error('Failed to load table query (fallback):', error);
			// Ultimate fallback query
			const query = formatSQLQuery(`SELECT * FROM ${tableNameParam}`);
			fullQuery = query;
			originalQuery = query;
			sqlQuery = addLimitToQuery(query, 100);
			selectedTableName = tableNameParam;
			
			addSystemMessage(`Loaded simple query for table: ${tableNameParam} (showing first 100 rows)`, 'info');
			
			// Try to execute fallback query
			await executeQuery();
		}
	}
	
	async function loadFileQuery(file: FileMetadata) {
		try {
			// Clear table selection since we're loading a file
			selectedTableName = null;
			originalQuery = '';
			fullQuery = '';
			
			// Check if this is a database file (.db extension or in database folder)
			const isDatabaseFile = file.folder === 'database' || file.filename.endsWith('.db');
			
			if (isDatabaseFile) {
				// Database file - treat as a table that's already loaded
				const tableName = file.filename.replace(/\.db$/, '').replace(/[^a-zA-Z0-9_]/g, '_');
				addSystemMessage(`Loading database table: ${tableName}...`, 'info');
				
				try {
					// Get columns for the table using DESCRIBE (works for all tables including imported ones)
					const columnsQuery = `DESCRIBE ${tableName}`;
					const columnsResult = await invoke<string>('execute_query', { query: columnsQuery });
					const describeRows = parseQueryResult(columnsResult);
					
					console.log('DESCRIBE result for', tableName, ':', describeRows);
					
					let query: string;
					if (describeRows.length > 0) {
						// Extract column info from DESCRIBE result
						const columns = describeRows.map((row: any) => ({
							name: row.column_name,
							type: row.column_type
						}));
						
						// Use the formatter to create a clean query
						query = formatSelectQuery(tableName, columns, false);
					} else {
						console.warn('DESCRIBE returned no rows for table:', tableName);
						query = formatSQLQuery(`SELECT * FROM ${tableName}`);
					}
					
					// Store the full query without limit
					fullQuery = query;
					
					// Display the limited version in the editor
					sqlQuery = addLimitToQuery(query, 100);
					
					addSystemMessage(`Loaded query for database table: ${tableName} (showing first 100 rows)`, 'success');
				} catch (error) {
					console.error('Error getting columns with DESCRIBE for', tableName, ':', error);
					// Fallback to SELECT *
					const query = formatSQLQuery(`SELECT * FROM ${tableName}`);
					fullQuery = query;
					sqlQuery = addLimitToQuery(query, 100);
					addSystemMessage(`Loaded simple query for table: ${tableName} (showing first 100 rows)`, 'info');
				}
				
				// Automatically execute the LIMITED query
				await executeQuery();
			} else {
				// Regular file - load it into DuckDB first
				addSystemMessage(`Loading ${file.filename}...`, 'info');
				await invoke<string>('load_file_by_name', {
					filename: file.filename,
					folder: file.folder
				});
				
				// Get columns
				const columnsJson = await invoke<string>('get_file_columns', {
					folder: file.folder,
					filename: file.filename
				});
				const columns: Array<{ name: string; type: string }> = JSON.parse(columnsJson);
				
				const tableName = file.filename.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');
				
				// Use the formatter to create a clean query
				const query = formatSelectQuery(tableName, columns, false);
				
				// Store the full query without limit
				fullQuery = query;
				
				// Display the limited version in the editor
				sqlQuery = addLimitToQuery(query, 100);
				
				addSystemMessage(`Loaded query for file: ${file.filename} (showing first 100 rows)`, 'success');
				
				// Automatically execute the LIMITED query
				await executeQuery();
			}
		} catch (error) {
			addSystemMessage(`Error loading file: ${error}`, 'error');
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
		}
	}
	
	function stopResize() {
		isDraggingSidebar = false;
	}
	
	// Listen for data model updates to refresh the sidebar
	function handleDataModelUpdate(event: CustomEvent) {
		console.log('🔄 Data model update event received in tables page:', event.detail);
		loadDataModels(); // Reload the sidebar list
	}

	onMount(() => {
		// Initialize the app
		const init = async () => {
			try {
				await invoke('initialize_duckdb');
				isTauriAvailable = true;
				addSystemMessage('Connected to database', 'success');
				
				// Load data sources
				await Promise.all([
					loadDataModels(),
					loadAvailableFiles()
				]);
				
				// Handle URL parameters for edit/derive actions from data model
				const editTable = $page.url.searchParams.get('edit');
				const deriveTable = $page.url.searchParams.get('derive');
				
				if (editTable) {
					addSystemMessage(`Loading table for editing: ${editTable}`, 'info');
					await loadTableQuery(editTable);
					tableName = editTable;
				} else if (deriveTable) {
					addSystemMessage(`Creating derived table from: ${deriveTable}`, 'info');
					await loadTableQuery(deriveTable);
					// Clear table name so user can enter a new one
					tableName = '';
					// Update query to suggest derived table pattern
					sqlQuery = sqlQuery.replace(/LIMIT \d+/, `-- Create a derived table\n-- Modify the query below\nLIMIT 100`);
				}
			} catch (error) {
				if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
					isTauriAvailable = false;
					addSystemMessage('Tauri API not available. Please run: npm run tauri:dev', 'error');
				} else {
					isTauriAvailable = true;
					addSystemMessage('Database connection issue. Some features may not work.', 'warning');
					console.error('DuckDB initialization error:', error);
				}
			}
		};
		
		init();
		
		// Window event listeners for resize
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', stopResize);
		
		// Listen for data model updates
		window.addEventListener('data-model-updated', handleDataModelUpdate as EventListener);
		
		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', stopResize);
			window.removeEventListener('data-model-updated', handleDataModelUpdate as EventListener);
		};
	});
</script>

<div id="table-builder-container" class="flex h-full w-full bg-slate-50 dark:bg-slate-950">
	<!-- Left Sidebar: Data Sources -->
	<div class="bg-white dark:bg-slate-900 border-r border-slate-200 dark:border-slate-800 flex flex-col" style="width: {sidebarWidth}px;">
		<div class="h-10 px-4 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between flex-shrink-0">
			<h2 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Data Sources</h2>
			<Tooltip text="Go to data model" position="bottom" size="sm">
				<a 
					href="/models/datamodel"
					class="h-6 w-6 flex items-center justify-center border rounded-lg transition-all duration-300 hover:scale-110 bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-800 hover:bg-blue-100 dark:hover:bg-blue-900/30"
				>
					<Network class="w-3 h-3 text-blue-600 dark:text-blue-400" />
				</a>
			</Tooltip>
		</div>
		
		<div class="flex-1 overflow-y-auto">
			<!-- Tables Section -->
			<div class="p-2">
				<div class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider px-2 py-2">
					Tables in Data Model
				</div>
				{#if isLoadingModels}
					<div class="px-2 py-4 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">Loading...</div>
					</div>
				{:else if dataModels.length === 0}
					<div class="px-2 py-4 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">No tables yet</div>
					</div>
				{:else}
					{#each dataModels as model}
						<button
							type="button"
							onclick={() => loadTableQuery(model.table_name)}
							class="w-full flex items-center gap-2 px-2 py-1.5 text-left text-sm rounded group {selectedTableName === model.table_name ? 'bg-blue-100 dark:bg-blue-900/30' : 'hover:bg-slate-100 dark:hover:bg-slate-800'}"
						>
							<TableIcon class="w-4 h-4 {selectedTableName === model.table_name ? 'text-blue-600 dark:text-blue-400' : model.table_type === 'source' ? 'text-green-500 dark:text-green-400' : 'text-blue-500 dark:text-blue-400'}" />
							<span class="flex-1 truncate {selectedTableName === model.table_name ? 'text-blue-700 dark:text-blue-300 font-medium' : 'text-slate-700 dark:text-slate-300'}">{model.table_name}</span>
							<span class="text-xs text-slate-400 dark:text-slate-500">{model.table_type}</span>
						</button>
					{/each}
				{/if}
			</div>
			
			<!-- Folder Tree Section -->
			<div class="p-2 mt-2 border-t border-slate-200 dark:border-slate-800">
				<div class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider px-2 py-2">
					Source Files
				</div>
				{#if isLoadingFiles}
					<div class="px-2 py-4 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">Loading...</div>
					</div>
				{:else if Object.keys(folderTree()).length === 0}
					<div class="px-2 py-4 text-center">
						<div class="text-xs text-slate-500 dark:text-slate-400">No files yet</div>
					</div>
				{:else}
					{#each Object.entries(folderTree()) as [folderName, files]}
						<div class="mb-1">
							<!-- Folder Header -->
							<button
								type="button"
								onclick={() => toggleFolder(folderName)}
								class="w-full flex items-center gap-2 px-2 py-1.5 text-left text-sm hover:bg-slate-100 dark:hover:bg-slate-800 rounded"
							>
								<svg class="w-3 h-3 text-slate-400 dark:text-slate-500 transition-transform {expandedFolders.has(folderName) ? 'rotate-90' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
								</svg>
								<Database class="w-4 h-4 text-amber-500 dark:text-amber-400" />
								<span class="flex-1 truncate font-medium text-slate-700 dark:text-slate-300">{folderName}</span>
								<span class="text-xs text-slate-400 dark:text-slate-500">{files.length}</span>
							</button>
							
							<!-- Files in Folder -->
							{#if expandedFolders.has(folderName)}
								<div class="ml-4 mt-1 space-y-0.5">
									{#each files as file}
										<button
											type="button"
											onclick={() => loadFileQuery(file)}
											class="w-full flex items-center gap-2 px-2 py-1 text-left text-xs hover:bg-slate-100 dark:hover:bg-slate-800 rounded group"
										>
											<FileText class="w-3.5 h-3.5 text-slate-400 dark:text-slate-500" />
											<span class="flex-1 truncate text-slate-600 dark:text-slate-400">{file.filename}</span>
										</button>
									{/each}
								</div>
							{/if}
						</div>
					{/each}
			{/if}
			</div>
		</div>
	</div>
	
	<!-- Sidebar Resize Handle -->
	<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		role="separator"
		aria-label="Resize sidebar"
		tabindex="0"
		class="w-1 bg-slate-200 dark:bg-slate-800 hover:bg-blue-500 dark:hover:bg-blue-500 cursor-col-resize transition-colors"
		onmousedown={startSidebarResize}
	></div>
	
	<!-- Main Content Area -->
	<div class="flex-1 flex flex-col min-w-0">
		<!-- Top Bar with Run Query Button -->
		<div class="h-10 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 px-4 flex items-center justify-between flex-shrink-0">
			<div class="flex items-center gap-2">
				<div class="flex items-center gap-1 px-2 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs">
					<span class="w-2 h-2 rounded-full bg-green-500"></span>
					<span class="text-slate-600 dark:text-slate-400">live</span>
				</div>
				<div class="text-xs text-slate-500 dark:text-slate-400">
					{#if executionTime}
						{executionTime.toFixed(0)}ms
					{:else}
						0ms
					{/if}
				</div>
			</div>
			<button
				onclick={executeQuery}
				disabled={isExecuting || !isTauriAvailable}
				class="inline-flex items-center gap-1.5 px-2.5 py-0.5 text-xs font-medium text-white bg-green-600 hover:bg-green-700 disabled:bg-slate-400 disabled:cursor-not-allowed rounded transition-colors"
			>
					{#if isExecuting}
					<svg class="animate-spin h-3 w-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
					Running...
					{:else}
					<Play class="w-3 h-3" />
					Run Query
					{/if}
				</button>
			</div>

		<!-- Split View: 40% SQL Editor / 60% Results OR Fullscreen SQL Editor -->
		{#if isFullscreenSQL}
			<!-- Fullscreen SQL Editor -->
			<div class="fixed inset-0 z-50 bg-slate-50 dark:bg-slate-950 flex flex-col">
				<!-- Fullscreen Header -->
				<div class="h-12 bg-slate-100 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 px-4 flex items-center justify-between flex-shrink-0">
					<div class="flex items-center gap-3">
						{#if selectedTableName}
							<TableIcon class="w-4 h-4 text-blue-500 dark:text-blue-400" />
							<span class="text-sm font-semibold text-slate-700 dark:text-slate-300">{selectedTableName}</span>
							{#if originalQuery && originalQuery.includes('Original creation query')}
								<span class="text-xs px-2 py-0.5 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded-full border border-green-300 dark:border-green-700">
									Original Query
								</span>
							{:else}
								<span class="text-xs px-2 py-0.5 bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300 rounded-full border border-amber-300 dark:border-amber-700">
									Generated Query
								</span>
							{/if}
							{#if isQueryModified()}
								<span class="text-xs px-2 py-0.5 bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300 rounded-full border border-amber-300 dark:border-amber-700">
									● Modified
								</span>
							{/if}
						{:else}
							<span class="text-sm font-semibold text-slate-700 dark:text-slate-300">SQL Editor - Fullscreen</span>
						{/if}
					</div>
					<div class="flex items-center gap-2">
						<button
							onclick={executeQuery}
							disabled={isExecuting || !isTauriAvailable}
							class="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-white bg-green-600 hover:bg-green-700 disabled:bg-slate-400 disabled:cursor-not-allowed rounded transition-colors"
						>
							{#if isExecuting}
								<svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
								</svg>
								Running...
							{:else}
								<Play class="w-4 h-4" />
								Run Query
							{/if}
						</button>
						<Tooltip text="Exit fullscreen" position="bottom" size="sm">
							<button
								onclick={toggleFullscreenSQL}
								class="h-8 w-8 flex items-center justify-center rounded hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
								type="button"
							>
								<Minimize2 class="w-4 h-4 text-slate-600 dark:text-slate-400" />
							</button>
						</Tooltip>
					</div>
				</div>
				<!-- Fullscreen Editor -->
				<textarea
					value={sqlQuery}
					oninput={(e) => {
						sqlQuery = (e.target as HTMLTextAreaElement).value;
					}}
					onkeydown={(e) => {
						if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
							e.preventDefault();
							executeQuery();
						}
						if (e.key === 'Escape') {
							e.preventDefault();
							toggleFullscreenSQL();
						}
					}}
					placeholder="-- Write your SQL query here&#10;-- Press Ctrl+Enter to run&#10;-- Press Escape to exit fullscreen&#10;&#10;SELECT * FROM table_name LIMIT 100;"
					class="flex-1 px-6 py-4 font-mono text-base bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 border-0 focus:outline-none focus:ring-0 resize-none"
					disabled={!isTauriAvailable}
					spellcheck="false"
				></textarea>
			</div>
		{:else}
		<!-- Split View: 40% SQL Editor / 60% Results -->
		<div class="flex-1 flex flex-col min-h-0" style="display: grid; grid-template-rows: 40% 60%;">
			<!-- SQL Editor (40%) -->
			<div class="bg-slate-50 dark:bg-slate-950 flex flex-col min-h-0 relative">
			<!-- Info banner when showing limited results -->
			{#if fullQuery && sqlQuery.includes('LIMIT')}
				<div class="absolute top-0 left-0 right-0 z-10 bg-blue-50 dark:bg-blue-900/30 border-b border-blue-200 dark:border-blue-700 px-4 py-2 flex items-center justify-between text-xs">
					<div class="flex items-center gap-2">
						<svg class="w-4 h-4 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
						</svg>
						<span class="text-blue-700 dark:text-blue-300 font-medium">
							Preview mode: Remove the LIMIT clause to query all data.
						</span>
					</div>
				</div>
			{/if}
				<!-- Maximize button positioned at top right of textarea -->
				<button
					onclick={toggleFullscreenSQL}
					class="absolute {fullQuery && sqlQuery.includes('LIMIT') ? 'top-11' : 'top-2'} right-2 z-10 h-6 w-6 flex items-center justify-center rounded bg-slate-100/80 hover:bg-slate-200 dark:bg-slate-800/80 dark:hover:bg-slate-700 transition-colors shadow-sm border border-slate-300 dark:border-slate-600"
					type="button"
					title="Toggle fullscreen (Esc to exit)"
				>
					{#if isFullscreenSQL}
						<Minimize2 class="w-3 h-3 text-slate-600 dark:text-slate-400" />
					{:else}
						<Maximize2 class="w-3 h-3 text-slate-600 dark:text-slate-400" />
					{/if}
				</button>
				<textarea
					value={sqlQuery}
					oninput={(e) => {
						sqlQuery = (e.target as HTMLTextAreaElement).value;
					}}
					onkeydown={(e) => {
						if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
							e.preventDefault();
							executeQuery();
						}
					}}
					placeholder="-- Write your SQL query here&#10;SELECT * FROM table_name LIMIT 100;"
					class="w-full h-full px-4 {fullQuery && sqlQuery.includes('LIMIT') ? 'pt-12' : 'pt-3'} pb-3 font-mono text-sm bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 border-0 focus:outline-none focus:ring-0 resize-none"
					disabled={!isTauriAvailable}
					spellcheck="false"
				></textarea>
			</div>
		
		<!-- Results Area -->
		<div class="flex-1 bg-white dark:bg-slate-900 flex flex-col min-h-0 overflow-hidden">
			<!-- Thin hairline divider -->
			<div class="h-px bg-slate-200 dark:bg-slate-700"></div>
			
			<!-- Results Header with Tabs -->
			<div class="border-b border-slate-200 dark:border-slate-800 flex items-center justify-between px-4 py-2 flex-shrink-0">
				<div class="flex items-center gap-2">
					<button 
						onclick={() => activeTab = 'results'}
						class="px-3 py-1 text-sm font-medium border-b-2 {activeTab === 'results' ? 'border-blue-500 text-blue-600 dark:text-blue-400' : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'}"
					>
						Results
					</button>
					<button 
						onclick={() => activeTab = 'relationships'}
						class="px-3 py-1 text-sm font-medium border-b-2 {activeTab === 'relationships' ? 'border-blue-500 text-blue-600 dark:text-blue-400' : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'}"
					>
						Relationships
					</button>
					<button 
						onclick={() => activeTab = 'meta'}
						class="px-3 py-1 text-sm font-medium border-b-2 {activeTab === 'meta' ? 'border-blue-500 text-blue-600 dark:text-blue-400' : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'}"
					>
						Meta
					</button>
			{#if activeTab === 'results' && queryResults.length > 0}
						<span class="text-xs text-slate-500 dark:text-slate-400">
							{queryResults.length} rows
						</span>
					{/if}
					{#if activeTab === 'relationships' && detectedRelationships.length > 0}
						<span class="text-xs text-slate-500 dark:text-slate-400">
							{detectedRelationships.length} relationships
						</span>
					{/if}
				</div>

				<!-- Save to Data Model Section -->
				{#if queryResults.length > 0}
					<div class="flex items-center gap-2">
						{#if isQueryModified()}
							<!-- Update button for modified queries -->
							<span class="text-xs text-amber-600 dark:text-amber-400 font-medium">Modified</span>
							<Button
								variant="primary"
								size="sm"
								onclick={updateSelectedTable}
								disabled={isSaving}
							>
								{#if isSaving}
									<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
										<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
										<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
									</svg>
									Updating...
								{:else}
									<Database class="w-4 h-4 mr-2" />
									Update {selectedTableName}
								{/if}
							</Button>
							<span class="text-xs text-slate-500 dark:text-slate-400">or</span>
						{/if}
						
						<span class="text-xs text-slate-600 dark:text-slate-400">Save as:</span>
							<input
								type="text"
								bind:value={tableName}
								placeholder="table_name"
								class="px-3 py-1.5 text-sm border border-slate-300 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 min-w-[160px]"
								disabled={isSaving}
								onkeydown={(e) => {
									if (e.key === 'Enter' && tableName.trim()) {
										e.preventDefault();
										saveTableToDataModel();
									}
								}}
							/>
						{#if !tableName.trim()}
							<Tooltip text="Enter a table name first" position="bottom" size="sm">
								<Button
									variant="primary"
									size="sm"
									onclick={saveTableToDataModel}
									disabled={isSaving || !tableName.trim()}
								>
									{#if isSaving}
										<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
												<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
												<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
											</svg>
											Saving...
									{:else}
										<Database class="w-4 h-4 mr-2" />
										Add to Data Model
									{/if}
								</Button>
							</Tooltip>
						{:else}
							<Button
								variant="primary"
								size="sm"
								onclick={saveTableToDataModel}
								disabled={isSaving || !tableName.trim()}
							>
								{#if isSaving}
									<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
											<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
											<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
										</svg>
										Saving...
								{:else}
									<Database class="w-4 h-4 mr-2" />
									Add to Data Model
								{/if}
							</Button>
						{/if}
				</div>
			{/if}
		</div>
			
			<!-- Tab Content -->
			<div class="flex-1 min-h-0 flex flex-col">
				{#if queryError}
					<div class="p-4">
						<div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
							<p class="text-sm text-red-700 dark:text-red-400 font-mono">{queryError}</p>
						</div>
					</div>
				{:else if activeTab === 'results'}
					<!-- Results Tab -->
					{#if queryResults.length === 0 && !isExecuting}
						<div class="flex items-center justify-center h-full text-slate-500 dark:text-slate-400 text-sm">
							No query results
						</div>
					{:else if queryResults.length > 0}
						<div class="flex-1 overflow-auto">
							<table class="w-full text-sm">
								<thead class="sticky top-0 bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
									<tr>
										{#each queryColumns as column}
											<th class="px-4 py-2 text-left font-medium text-slate-900 dark:text-slate-100 whitespace-nowrap">
												{column}
											</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each queryResults as row, idx}
										<tr 
											class="border-b border-slate-100 dark:border-slate-800 hover:bg-purple-50/50 dark:hover:bg-purple-950/20 cursor-pointer transition-colors"
											onclick={() => openRecordDrawer(row, idx)}
											role="button"
											tabindex="0"
											onkeydown={(e) => e.key === 'Enter' && openRecordDrawer(row, idx)}
										>
											{#each queryColumns as column}
												<td class="px-4 py-2 text-slate-700 dark:text-slate-300 whitespace-nowrap font-mono text-xs">
													{row[column] ?? 'NULL'}
												</td>
											{/each}
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{/if}
				{:else if activeTab === 'relationships'}
					<!-- Relationships Tab -->
					<div class="flex-1 overflow-y-auto overflow-x-hidden">
						{#if detectedRelationships.length > 0}
							<div class="p-4">
								<div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
									<h4 class="text-sm font-semibold text-green-900 dark:text-green-100 mb-3">
										✓ Detected {detectedRelationships.length} Relationship(s)
									</h4>
									<div class="space-y-2">
										{#each detectedRelationships as rel}
											<div class="bg-white dark:bg-green-950/30 border border-green-200 dark:border-green-800 rounded p-3">
												<p class="text-xs text-green-800 dark:text-green-300 font-mono">
													{rel.from_table}.{rel.from_column} → {rel.to_table}.{rel.to_column}
												</p>
											</div>
										{/each}
									</div>
								</div>
							</div>
						{:else}
							<div class="flex items-center justify-center h-full text-slate-500 dark:text-slate-400 text-sm">
								<div class="text-center">
									<Network class="w-12 h-12 mx-auto mb-2 opacity-50" />
									<p>No relationships detected</p>
									<p class="text-xs mt-1">Save the table to automatically detect relationships</p>
								</div>
							</div>
						{/if}
					</div>
				{:else if activeTab === 'meta'}
					<!-- Meta Tab -->
					<div class="flex-1 overflow-y-auto overflow-x-hidden">
						{#if tableStats}
							<div class="p-4">
								<!-- Overview Stats -->
								<div class="grid grid-cols-2 gap-4 mb-6">
								<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
									<div class="text-xs text-blue-600 dark:text-blue-400 font-medium mb-1">Total Rows</div>
									<div class="text-2xl font-bold text-blue-900 dark:text-blue-100">{tableStats.rowCount.toLocaleString()}</div>
								</div>
								<div class="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4">
									<div class="text-xs text-purple-600 dark:text-purple-400 font-medium mb-1">Total Columns</div>
									<div class="text-2xl font-bold text-purple-900 dark:text-purple-100">{tableStats.columnCount}</div>
								</div>
							</div>

							<!-- Column Details -->
							<div class="bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg overflow-hidden">
								<div class="px-4 py-3 bg-slate-100 dark:bg-slate-750 border-b border-slate-200 dark:border-slate-700">
									<h4 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Column Details</h4>
								</div>
								<div class="overflow-x-auto">
									<table class="w-full text-sm">
										<thead class="bg-slate-50 dark:bg-slate-800">
											<tr>
												<th class="px-4 py-2 text-left font-medium text-slate-700 dark:text-slate-300">Column Name</th>
												<th class="px-4 py-2 text-left font-medium text-slate-700 dark:text-slate-300">Type</th>
												<th class="px-4 py-2 text-left font-medium text-slate-700 dark:text-slate-300">Unique Values</th>
												<th class="px-4 py-2 text-left font-medium text-slate-700 dark:text-slate-300">Null Count</th>
												<th class="px-4 py-2 text-left font-medium text-slate-700 dark:text-slate-300">Completeness</th>
											</tr>
										</thead>
										<tbody>
											{#each tableStats.columns as col}
												{@const completeness = ((tableStats.rowCount - col.nullCount) / tableStats.rowCount * 100).toFixed(1)}
												<tr class="border-t border-slate-200 dark:border-slate-700">
													<td class="px-4 py-3 font-mono text-xs text-slate-900 dark:text-slate-100">{col.name}</td>
													<td class="px-4 py-3">
														<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300">
															{col.type}
														</span>
													</td>
													<td class="px-4 py-3 text-slate-700 dark:text-slate-300">{col.uniqueCount.toLocaleString()}</td>
													<td class="px-4 py-3 text-slate-700 dark:text-slate-300">{col.nullCount.toLocaleString()}</td>
													<td class="px-4 py-3">
														<div class="flex items-center gap-2">
															<div class="flex-1 h-2 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden">
																<div 
																	class="h-full {parseFloat(completeness) === 100 ? 'bg-green-500' : parseFloat(completeness) > 90 ? 'bg-blue-500' : parseFloat(completeness) > 50 ? 'bg-amber-500' : 'bg-red-500'}"
																	style="width: {completeness}%"
																></div>
															</div>
															<span class="text-xs text-slate-600 dark:text-slate-400 w-12 text-right">{completeness}%</span>
														</div>
													</td>
												</tr>
											{/each}
										</tbody>
									</table>
								</div>
							</div>
						</div>
						{:else}
							<div class="flex items-center justify-center h-full text-slate-500 dark:text-slate-400 text-sm">
								<div class="text-center">
									<TableIcon class="w-12 h-12 mx-auto mb-2 opacity-50" />
									<p>No metadata available</p>
									<p class="text-xs mt-1">Execute a query to see table statistics</p>
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
		</div>
		{/if}
	</div>
</div>

<!-- Record View Drawer -->
<RecordViewDrawer 
	bind:isOpen={isRecordDrawerOpen}
	bind:record={selectedRecord}
	columns={queryColumns}
	recordIndex={selectedRecordIndex}
	onClose={closeRecordDrawer}
/>

<!-- Long-Running Query Loading Modal -->
{#if showLoadingModal}
	<div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm animate-fade-in">
		<div class="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl border-2 border-blue-500 dark:border-blue-600 p-8 max-w-md w-full mx-4 animate-scale-in">
			<!-- Animated Spinner with Pulse -->
			<div class="flex justify-center mb-6">
				<div class="relative animate-pulse-slow">
					<div class="w-16 h-16 border-4 border-blue-200 dark:border-blue-900 rounded-full"></div>
					<div class="w-16 h-16 border-4 border-blue-600 dark:border-blue-400 border-t-transparent rounded-full animate-spin absolute top-0 left-0"></div>
				</div>
			</div>
			
			<!-- Title with Info Icon -->
			<div class="flex items-center justify-center gap-2 mb-2">
				<svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
				</svg>
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 text-center">
					Long-Running Query
				</h3>
			</div>
			
			<!-- Description -->
			<div class="text-sm text-slate-600 dark:text-slate-400 text-center mb-4 space-y-2">
				<p class="font-medium">This query is taking longer than expected.</p>
				<p class="text-xs">You can continue waiting or cancel the query execution.</p>
			</div>
			
			<!-- Elapsed Time -->
			{#if queryStartTime}
				<div class="text-center mb-6">
					<div class="inline-flex items-center gap-2 px-4 py-2 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
						<svg class="w-4 h-4 text-blue-600 dark:text-blue-400 animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
						</svg>
						<span class="text-sm font-bold text-blue-700 dark:text-blue-300">
							{elapsedTime.toFixed(1)}s elapsed
						</span>
					</div>
				</div>
			{/if}
			
			<!-- Recommendation -->
			<div class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-3 mb-4">
				<p class="text-xs text-amber-800 dark:text-amber-300 text-center">
					💡 <strong>Tip:</strong> Consider adding LIMIT clauses or filters to reduce query execution time.
				</p>
			</div>
			
			<!-- Actions -->
			<div class="flex flex-col gap-2">
				<button
					onclick={abortQuery}
					class="w-full inline-flex items-center justify-center gap-2 px-4 py-3 text-sm font-semibold text-white bg-red-600 hover:bg-red-700 active:bg-red-800 rounded-lg transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] shadow-lg"
				>
					<XCircle class="w-4 h-4" />
					Cancel Query
				</button>
				<button
					onclick={() => showLoadingModal = false}
					class="w-full inline-flex items-center justify-center gap-2 px-4 py-3 text-sm font-medium text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 border border-slate-300 dark:border-slate-600 rounded-lg transition-all duration-200"
				>
					Continue Waiting
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	
	@keyframes scale-in {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}
	
	@keyframes pulse-slow {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.6;
		}
	}
	
	.animate-fade-in {
		animation: fade-in 0.2s ease-out;
	}
	
	.animate-scale-in {
		animation: scale-in 0.3s ease-out;
	}
	
	.animate-pulse-slow {
		animation: pulse-slow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
</style>
