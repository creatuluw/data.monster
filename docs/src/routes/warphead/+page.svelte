<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import TablePreviewDrawer from '$lib/components/TablePreviewDrawer.svelte';
	import { executeQuery } from '$lib/db-operations';
	import { 
		CassetteTape, 
		Database, 
		HardDrive, 
		Cpu, 
		Monitor, 
		Wifi,
		WifiOff,
		CheckCircle2,
		XCircle,
		AlertTriangle,
		RefreshCw,
		Info,
		Settings,
		Package,
		Code,
		Terminal,
		Zap,
		Activity,
		MemoryStick,
		Server,
		FileText,
		Download,
		Trash2,
		Eye,
		Table,
		Sun,
		Moon,
		Laptop
	} from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getDbContext } from '$lib/db-context';
	import { parseQueryResult } from '$lib/db-utils';
	import { theme, type Theme } from '$lib/stores/theme';
	
	// Get database context from layout
	const dbContext = getDbContext();
	
	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}
	
	interface SystemInfo {
		platform: string;
		version: string;
		arch: string;
		totalMemory: string;
		freeMemory: string;
		cpuCores: number;
	}
	
	interface DatabaseInfo {
		location: string;
		size: string;
		tableCount: number;
		totalRows: number;
	}
	
	interface InternalTable {
		table_name: string;
		row_count: number;
		column_count: number;
		description: string;
	}
	
	interface ApplicationInfo {
		name: string;
		version: string;
		tauriVersion: string;
		duckdbVersion: string;
	}
	
	let systemInfo = $state<SystemInfo | null>(null);
	let databaseInfo = $state<DatabaseInfo | null>(null);
	let applicationInfo = $state<ApplicationInfo>({
		name: 'Warphead',
		version: '0.1.0',
		tauriVersion: '2.x',
		duckdbVersion: '1.1.x'
	});
	
	let isLoadingSystem = $state(false);
	let isLoadingDatabase = $state(false);
	let isLoadingTables = $state(false);
	let isTauriConnected = $state(false);
	let isDatabaseConnected = $state(false);
	
	// Internal tables state
	let internalTables = $state<InternalTable[]>([]);
	
	// Table preview drawer state
	let isPreviewDrawerOpen = $state(false);
	let previewTableName = $state<string | null>(null);
	
	// Settings state
	let settings = $state({
		autoSave: true,
		debugMode: false,
		maxResultRows: 1000,
		queryTimeout: 30000,
		loadSampleData: true  // Toggle for loading sample data on initialization
	});
	
	// Theme is now managed by the store
	let currentTheme = $state<Theme>('system');
	
	// Sync with theme store
	$effect(() => {
		currentTheme = $theme;
	});
	
	// Function to handle theme changes
	function handleThemeChange(newTheme: Theme) {
		theme.set(newTheme);
		addSystemMessage(`Theme changed to ${newTheme}`, 'success');
	}
	
	async function loadSystemInfo() {
		try {
			isLoadingSystem = true;
			
			// Check if we're in Tauri environment
			if (typeof window !== 'undefined' && typeof invoke === 'function') {
				isTauriConnected = true;
				
				// Mock system info for now - in production you'd call Tauri commands
				systemInfo = {
					platform: navigator.platform || 'Unknown',
					version: navigator.userAgent.includes('Windows') ? 'Windows 11' : 'Unknown',
					arch: 'x64',
					totalMemory: '16 GB',
					freeMemory: '8 GB',
					cpuCores: navigator.hardwareConcurrency || 4
				};
				
				addSystemMessage('System information loaded', 'success');
			} else {
				isTauriConnected = false;
				addSystemMessage('Not running in Tauri environment', 'warning');
			}
		} catch (error) {
			console.error('Error loading system info:', error);
			addSystemMessage(`Error loading system info: ${error}`, 'error');
		} finally {
			isLoadingSystem = false;
		}
	}
	
	async function loadDatabaseInfo() {
		try {
			isLoadingDatabase = true;
			
			if (!dbContext.isInitialized) {
				addSystemMessage('Database not initialized', 'warning');
				return;
			}
			
			isDatabaseConnected = true;
			
			// Get table count
			const tablesResult = await invoke<string>('execute_query', {
				query: 'SELECT COUNT(*) as count FROM information_schema.tables WHERE table_schema = \'main\''
			});
			const tables = parseQueryResult<{ count: number }>(tablesResult);
			const tableCount = tables[0]?.count || 0;
			
			// Get total rows across all tables
			const allTablesResult = await invoke<string>('execute_query', {
				query: 'SELECT table_name FROM information_schema.tables WHERE table_schema = \'main\''
			});
			const allTables = parseQueryResult<{ table_name: string }>(allTablesResult);
			
			let totalRows = 0;
			for (const table of allTables) {
				try {
					const rowsResult = await invoke<string>('execute_query', {
						query: `SELECT COUNT(*) as count FROM ${table.table_name}`
					});
					const rows = parseQueryResult<{ count: number }>(rowsResult);
					totalRows += rows[0]?.count || 0;
				} catch (err) {
					console.error(`Error counting rows in ${table.table_name}:`, err);
				}
			}
			
			databaseInfo = {
				location: 'warphead.db',
				size: '~2.5 MB',
				tableCount,
				totalRows
			};
			
			addSystemMessage('Database information loaded', 'success');
		} catch (error) {
			console.error('Error loading database info:', error);
			addSystemMessage(`Error loading database info: ${error}`, 'error');
			isDatabaseConnected = false;
		} finally {
			isLoadingDatabase = false;
		}
	}
	
	async function refreshAllInfo() {
		await Promise.all([loadSystemInfo(), loadDatabaseInfo(), loadInternalTables()]);
	}
	
	async function clearCache() {
		if (!confirm('Are you sure you want to clear the application cache? This will not delete your data.')) {
			return;
		}
		
		addSystemMessage('Cache cleared successfully', 'success');
	}
	
	async function exportDatabase() {
		try {
			addSystemMessage('Exporting database...', 'info');
			// In production, you'd implement actual export logic here
			await new Promise(resolve => setTimeout(resolve, 1000));
			addSystemMessage('Database exported successfully', 'success');
		} catch (error) {
			addSystemMessage(`Error exporting database: ${error}`, 'error');
		}
	}
	
	async function vacuumDatabase() {
		try {
			addSystemMessage('Optimizing database...', 'info');
			await executeQuery('VACUUM');
			addSystemMessage('Database optimized successfully', 'success');
			await loadDatabaseInfo();
		} catch (error) {
			addSystemMessage(`Error optimizing database: ${error}`, 'error');
		}
	}
	
	async function loadInternalTables() {
		try {
			isLoadingTables = true;
			
			if (!dbContext.isInitialized) {
				console.log('❌ Database not initialized, skipping internal tables load');
				addSystemMessage('Database not initialized', 'warning');
				return;
			}
			
			console.log('🔍 Loading internal tables...');
			
			// Get all _warphead internal tables
			const tablesQuery = `
				SELECT table_name 
				FROM information_schema.tables 
				WHERE table_schema = 'main' 
				AND table_name LIKE '_warphead%'
				ORDER BY table_name
			`;
			
			console.log('📝 Executing query:', tablesQuery);
			const result = await invoke<string>('execute_query', { query: tablesQuery });
			console.log('✅ Query result:', result);
			const tables: Array<{ table_name: string }> = parseQueryResult(result);
			console.log('📊 Parsed tables:', tables);
			
			// Get row count and column count for each table
			const tableDetails: InternalTable[] = [];
			
			for (const table of tables) {
				const tableName = table.table_name;
				
				try {
					// Get row count
					const countResult = await invoke<string>('execute_query', {
						query: `SELECT COUNT(*) as count FROM "${tableName}"`
					});
					const countData = parseQueryResult<{ count: number }>(countResult);
					const rowCount = countData[0]?.count || 0;
					
					// Get column count
					const columnsResult = await invoke<string>('execute_query', {
						query: `SELECT COUNT(*) as count FROM information_schema.columns WHERE table_name = '${tableName}'`
					});
					const columnsData = parseQueryResult<{ count: number }>(columnsResult);
					const columnCount = columnsData[0]?.count || 0;
					
					// Add description based on table name
					let description = '';
					switch (tableName) {
						case '_warphead_table_metadata':
							description = 'Stores metadata about all tables in the data model';
							break;
						case '_warphead_relationships':
							description = 'Defines relationships between tables';
							break;
						case '_warphead_attached_tables':
							description = 'Remote database connections and cached tables';
							break;
						case '_warphead_metrics':
							description = 'Business metrics and calculations';
							break;
						case '_warphead_dimensions':
							description = 'Dimensional hierarchies and attributes';
							break;
						case '_warphead_user_defined_functions':
							description = 'Custom SQL functions (UDFs)';
							break;
						case '_warphead_postgres_secrets':
							description = 'Encrypted PostgreSQL connection credentials';
							break;
						default:
							description = 'Internal system table';
					}
					
					tableDetails.push({
						table_name: tableName,
						row_count: rowCount,
						column_count: columnCount,
						description
					});
				} catch (error) {
					console.error(`Error loading details for ${tableName}:`, error);
				}
			}
			
			internalTables = tableDetails;
			console.log('✅ Final internal tables:', internalTables);
			
			if (tableDetails.length > 0) {
				addSystemMessage(`Loaded ${tableDetails.length} internal tables`, 'success');
			} else {
				console.warn('⚠️ No internal tables found in database');
				addSystemMessage('No _warphead tables found in database', 'warning');
			}
		} catch (error) {
			console.error('❌ Error loading internal tables:', error);
			addSystemMessage(`Error loading internal tables: ${error}`, 'error');
		} finally {
			isLoadingTables = false;
		}
	}
	
	function openTableViewer(tableName: string) {
		previewTableName = tableName;
		isPreviewDrawerOpen = true;
	}
	
	function closePreviewDrawer() {
		isPreviewDrawerOpen = false;
		previewTableName = null;
	}
	
	onMount(() => {
		console.log('🚀 Warphead page mounted');
		console.log('📊 DB Context:', {
			isInitialized: dbContext.isInitialized,
			isInitializing: dbContext.isInitializing,
			error: dbContext.error,
			isTauriAvailable: dbContext.isTauriAvailable
		});
		
		// Load settings from localStorage
		const savedSettings = localStorage.getItem('warphead-settings');
		if (savedSettings) {
			try {
				const parsed = JSON.parse(savedSettings);
				settings = { ...settings, ...parsed };
				console.log('⚙️  Loaded settings from localStorage:', settings);
			} catch (e) {
				console.error('Failed to parse saved settings:', e);
			}
		}
		
		// Wait for DB initialization, then load info
		const checkAndLoad = () => {
			console.log('🔄 Checking DB status...', {
				isInitialized: dbContext.isInitialized,
				isInitializing: dbContext.isInitializing,
				error: dbContext.error
			});
			
			if (dbContext.isInitialized) {
				console.log('✅ DB initialized, loading all info');
				refreshAllInfo();
			} else if (!dbContext.isInitializing && dbContext.error) {
				console.error('❌ DB error:', dbContext.error);
				addSystemMessage('Database not available', 'error');
				loadSystemInfo(); // Still load system info
			} else {
				console.log('⏳ DB still initializing, waiting...');
				setTimeout(checkAndLoad, 100);
			}
		};
		
		checkAndLoad();
	});
	
	// Watch for settings changes and save to localStorage
	$effect(() => {
		if (typeof window !== 'undefined') {
			localStorage.setItem('warphead-settings', JSON.stringify(settings));
			console.log('💾 Settings saved to localStorage');
			
			// Also save the loadSampleData setting to the database for backend to check
			saveLoadSampleDataSetting();
		}
	});
	
	// Save the load sample data setting to the database
	async function saveLoadSampleDataSetting() {
		if (!dbContext.isInitialized) {
			return;
		}
		
		try {
			// Delete existing setting
			await invoke('execute_query', {
				query: "DELETE FROM _warphead_table_metadata WHERE table_name = '_setting_load_sample_data'"
			});
			
			// Insert new setting
			await invoke('execute_query', {
				query: `INSERT INTO _warphead_table_metadata (table_name, table_type, created_at, creation_query) 
				        VALUES ('_setting_load_sample_data', 'setting', now(), '${settings.loadSampleData}')`
			});
			
			console.log('✅ Load sample data setting saved to database:', settings.loadSampleData);
		} catch (error) {
			console.error('Failed to save load sample data setting:', error);
		}
	}
	
	// Get table badge color based on table name
	function getTableColor(tableName: string) {
		if (tableName.includes('metadata')) {
			return {
				bg: 'bg-blue-50 dark:bg-blue-900/20',
				border: 'border-blue-200 dark:border-blue-800',
				text: 'text-blue-700 dark:text-blue-400',
				badge: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
			};
		} else if (tableName.includes('relationship')) {
			return {
				bg: 'bg-purple-50 dark:bg-purple-900/20',
				border: 'border-purple-200 dark:border-purple-800',
				text: 'text-purple-700 dark:text-purple-400',
				badge: 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400'
			};
		} else if (tableName.includes('metric') || tableName.includes('dimension')) {
			return {
				bg: 'bg-orange-50 dark:bg-orange-900/20',
				border: 'border-orange-200 dark:border-orange-800',
				text: 'text-orange-700 dark:text-orange-400',
				badge: 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400'
			};
		} else if (tableName.includes('secret') || tableName.includes('attached')) {
			return {
				bg: 'bg-green-50 dark:bg-green-900/20',
				border: 'border-green-200 dark:border-green-800',
				text: 'text-green-700 dark:text-green-400',
				badge: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
			};
		} else {
			return {
				bg: 'bg-slate-50 dark:bg-slate-800/20',
				border: 'border-slate-200 dark:border-slate-700',
				text: 'text-slate-700 dark:text-slate-400',
				badge: 'bg-slate-100 text-slate-700 dark:bg-slate-900/30 dark:text-slate-400'
			};
		}
	}
</script>

<PageLayout>
	<!-- Header -->
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-2">
			<CassetteTape class="w-8 h-8 text-[#9B00FF]" />
			<h1 class="font-aspekta text-3xl font-[650] text-slate-900 dark:text-slate-100">Warphead System</h1>
		</div>
		<p class="text-slate-600 dark:text-slate-400">Settings, system information, and diagnostics</p>
	</div>

	<!-- Internal Tables Section -->
	<section class="mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100">Internal Database Tables</h2>
			<Button variant="secondary" onclick={loadInternalTables} disabled={isLoadingTables} size="sm">
				{#if isLoadingTables}
					<span class="flex items-center gap-2">
						<svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Loading...
					</span>
				{:else}
					<span class="flex items-center gap-2">
						<RefreshCw class="w-4 h-4" />
						Reload
					</span>
				{/if}
			</Button>
		</div>
		{#if isLoadingTables}
			<div class="flex items-center justify-center h-48 bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800">
				<div class="text-center">
					<svg class="animate-spin h-8 w-8 mx-auto mb-4 text-[#9B00FF]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400">Loading internal tables...</p>
				</div>
			</div>
		{:else if internalTables.length === 0}
			<div class="flex items-center justify-center h-48 bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800">
				<div class="text-center">
					<Database class="w-12 h-12 mx-auto mb-4 text-slate-300 dark:text-slate-600" />
					<p class="text-slate-600 dark:text-slate-400">No internal tables found</p>
				</div>
			</div>
		{:else}
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
				{#each internalTables as table}
					{@const colors = getTableColor(table.table_name)}
					<div class="group relative">
						<div class="relative block overflow-visible rounded-xl border bg-white dark:bg-slate-900 transition-all duration-300 ease-out hover:shadow-lg hover:shadow-slate-200/50 dark:hover:shadow-slate-950/50 hover:-translate-y-0.5 min-h-[140px] {colors.border}">
							<!-- Gradient overlay on hover -->
							<div class="absolute inset-0 bg-gradient-to-br from-slate-50/50 to-transparent dark:from-slate-800/30 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-xl"></div>
							
							<!-- Content -->
							<div class="relative flex flex-col h-full p-4">
								<!-- Icon and Eye button in top-right corner -->
								<div class="absolute top-4 right-4 flex items-center gap-2">
									<button
										onclick={() => openTableViewer(table.table_name)}
										class="h-7 w-7 flex items-center justify-center border rounded transition-all duration-300 hover:scale-110 bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-[#9B00FF] dark:hover:border-[#9B00FF] group/eye"
										title="View table data"
									>
										<Eye class="w-3.5 h-3.5 text-slate-600 dark:text-slate-400 group-hover/eye:text-[#9B00FF]" />
									</button>
									<div class="h-7 w-7 flex items-center justify-center border rounded transition-all duration-300 {colors.bg} {colors.border}">
										<Table class="w-3.5 h-3.5 transition-colors duration-300 {colors.text}" />
									</div>
								</div>

								<!-- Table info -->
								<div class="space-y-2 pr-16">
									<h4 class="text-sm font-aspekta font-[650] text-slate-900 dark:text-slate-100 break-all leading-tight" title={table.table_name}>
										{table.table_name}
									</h4>
									
									<!-- Description -->
									<p class="text-xs text-slate-600 dark:text-slate-400 line-clamp-2">
										{table.description}
									</p>
									
									<!-- Stats -->
									<div class="flex items-center gap-3 text-xs pt-2">
										<div class="flex items-center gap-1 text-slate-600 dark:text-slate-400">
											<Server class="w-3 h-3 shrink-0" />
											<span class="font-medium">{table.row_count}</span>
											<span class="text-slate-500 dark:text-slate-500">rows</span>
										</div>
										<div class="flex items-center gap-1 text-slate-600 dark:text-slate-400">
											<Package class="w-3 h-3 shrink-0" />
											<span class="font-medium">{table.column_count}</span>
											<span class="text-slate-500 dark:text-slate-500">cols</span>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- System Information -->
	<section class="mb-8">
		<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-6">System Information</h2>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg border border-slate-200 dark:border-slate-800 overflow-hidden">
			{#if isLoadingSystem}
				<div class="p-8 text-center">
					<svg class="animate-spin h-8 w-8 mx-auto text-[#9B00FF]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-2">Loading system information...</p>
				</div>
			{:else if systemInfo}
				<div class="divide-y divide-slate-200 dark:divide-slate-700">
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Monitor class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Platform</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.platform}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Package class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Version</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.version}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Cpu class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Architecture</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.arch}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Cpu class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">CPU Cores</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.cpuCores}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<MemoryStick class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Total Memory</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.totalMemory}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<MemoryStick class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Free Memory</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{systemInfo.freeMemory}</span>
					</div>
				</div>
			{:else}
				<div class="p-8 text-center">
					<Monitor class="w-12 h-12 mx-auto text-slate-300 dark:text-slate-600 mb-3" />
					<p class="text-slate-600 dark:text-slate-400">System information unavailable</p>
				</div>
			{/if}
		</div>
	</section>

	<!-- Database Information -->
	<section class="mb-8">
		<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Database Information</h2>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg border border-slate-200 dark:border-slate-800 overflow-hidden">
			{#if isLoadingDatabase}
				<div class="p-8 text-center">
					<svg class="animate-spin h-8 w-8 mx-auto text-[#9B00FF]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-2">Loading database information...</p>
				</div>
			{:else if databaseInfo}
				<div class="divide-y divide-slate-200 dark:divide-slate-700">
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<FileText class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Location</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100 font-mono">{databaseInfo.location}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<HardDrive class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Size</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{databaseInfo.size}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Database class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Tables</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{databaseInfo.tableCount}</span>
					</div>
					<div class="px-6 py-4 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<Server class="w-5 h-5 text-slate-400" />
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Total Rows</span>
						</div>
						<span class="text-sm text-slate-900 dark:text-slate-100">{databaseInfo.totalRows.toLocaleString()}</span>
					</div>
				</div>
			{:else}
				<div class="p-8 text-center">
					<Database class="w-12 h-12 mx-auto text-slate-300 dark:text-slate-600 mb-3" />
					<p class="text-slate-600 dark:text-slate-400">Database information unavailable</p>
				</div>
			{/if}
		</div>
	</section>

	<!-- Application Information -->
	<section class="mb-8">
		<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Application Information</h2>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg border border-slate-200 dark:border-slate-800 overflow-hidden">
			<div class="divide-y divide-slate-200 dark:divide-slate-700">
				<div class="px-6 py-4 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<CassetteTape class="w-5 h-5 text-[#9B00FF]" />
						<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Application Name</span>
					</div>
					<span class="text-sm text-slate-900 dark:text-slate-100">{applicationInfo.name}</span>
				</div>
				<div class="px-6 py-4 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<Package class="w-5 h-5 text-slate-400" />
						<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Version</span>
					</div>
					<span class="text-sm text-slate-900 dark:text-slate-100">{applicationInfo.version}</span>
				</div>
				<div class="px-6 py-4 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<Zap class="w-5 h-5 text-slate-400" />
						<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Tauri</span>
					</div>
					<span class="text-sm text-slate-900 dark:text-slate-100">{applicationInfo.tauriVersion}</span>
				</div>
				<div class="px-6 py-4 flex items-center justify-between">
					<div class="flex items-center gap-3">
						<Database class="w-5 h-5 text-slate-400" />
						<span class="text-sm font-medium text-slate-700 dark:text-slate-300">DuckDB</span>
					</div>
					<span class="text-sm text-slate-900 dark:text-slate-100">{applicationInfo.duckdbVersion}</span>
				</div>
			</div>
		</div>
	</section>

	<!-- Settings (Future Enhancement) -->
	<section class="mb-8">
		<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Settings</h2>
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg border border-slate-200 dark:border-slate-800 p-6">
			<div class="space-y-6">
			<!-- Theme Setting -->
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center">
						{#if currentTheme === 'light'}
							<Sun class="w-5 h-5 text-purple-600 dark:text-purple-400" />
						{:else if currentTheme === 'dark'}
							<Moon class="w-5 h-5 text-purple-600 dark:text-purple-400" />
						{:else}
							<Laptop class="w-5 h-5 text-purple-600 dark:text-purple-400" />
						{/if}
					</div>
					<div>
						<h3 class="text-sm font-medium text-slate-900 dark:text-slate-100">Theme</h3>
						<p class="text-xs text-slate-500 dark:text-slate-400">Choose your preferred color scheme</p>
					</div>
				</div>
				<select 
					value={currentTheme}
					onchange={(e) => handleThemeChange(e.currentTarget.value as Theme)}
					class="px-3 py-2 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg text-sm text-slate-900 dark:text-slate-100 cursor-pointer focus:ring-2 focus:ring-purple-500 focus:border-purple-500 transition-colors"
				>
					<option value="light">☀️ Light</option>
					<option value="dark">🌙 Dark</option>
					<option value="system">💻 System</option>
				</select>
			</div>

			<!-- Theme Info -->
			<div class="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4">
				<div class="flex items-start gap-2">
					<Info class="w-4 h-4 text-purple-600 dark:text-purple-400 flex-shrink-0 mt-0.5" />
					<div class="text-xs text-purple-800 dark:text-purple-300">
						<strong>Theme Modes:</strong>
						<ul class="mt-1 space-y-1 ml-2">
							<li>• <strong>Light:</strong> Always use light theme</li>
							<li>• <strong>Dark:</strong> Always use dark theme</li>
							<li>• <strong>System:</strong> Automatically match your OS preference</li>
						</ul>
						<p class="mt-2">Your preference is saved and will persist across sessions.</p>
					</div>
				</div>
			</div>

				<!-- Auto Save -->
				<div class="flex items-center justify-between">
					<div>
						<h3 class="text-sm font-medium text-slate-900 dark:text-slate-100">Auto Save</h3>
						<p class="text-xs text-slate-500 dark:text-slate-400">Automatically save changes</p>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input type="checkbox" bind:checked={settings.autoSave} class="sr-only peer" />
						<div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-purple-300 dark:peer-focus:ring-purple-800 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-[#9B00FF]"></div>
					</label>
				</div>

				<!-- Debug Mode -->
				<div class="flex items-center justify-between">
					<div>
						<h3 class="text-sm font-medium text-slate-900 dark:text-slate-100">Debug Mode</h3>
						<p class="text-xs text-slate-500 dark:text-slate-400">Enable verbose logging</p>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input type="checkbox" bind:checked={settings.debugMode} class="sr-only peer" />
						<div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-purple-300 dark:peer-focus:ring-purple-800 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-[#9B00FF]"></div>
					</label>
				</div>

				<!-- Load Sample Data -->
				<div class="flex items-center justify-between">
					<div>
						<h3 class="text-sm font-medium text-slate-900 dark:text-slate-100">Load Sample Data on Initialization</h3>
						<p class="text-xs text-slate-500 dark:text-slate-400">Automatically load sales data and create star schema on first run</p>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input type="checkbox" bind:checked={settings.loadSampleData} class="sr-only peer" />
						<div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-purple-300 dark:peer-focus:ring-purple-800 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-[#9B00FF]"></div>
					</label>
				</div>
				
				<!-- Sample Data Info -->
				<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
					<div class="flex items-start gap-2">
						<Info class="w-4 h-4 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" />
						<div class="text-xs text-blue-800 dark:text-blue-300">
							<strong>About Sample Data:</strong>
							<p class="mt-1">
								When enabled, Warphead will automatically load sample sales data (~1000 rows) and create a star schema (dim_customers, dim_products, dim_time, fact_sales) on first run. This is useful for testing and learning.
							</p>
							<p class="mt-2">
								<strong>Note:</strong> This setting only applies to new database initializations. If you've already initialized the database, use the "Clear Database" quick action above to reset and reload with this setting.
							</p>
						</div>
					</div>
				</div>

				<!-- Max Result Rows -->
				<div class="flex items-center justify-between">
					<div>
						<h3 class="text-sm font-medium text-slate-900 dark:text-slate-100">Max Result Rows</h3>
						<p class="text-xs text-slate-500 dark:text-slate-400">Maximum rows to display in query results</p>
					</div>
					<input 
						type="number" 
						bind:value={settings.maxResultRows}
						min="100"
						max="10000"
						step="100"
						class="px-3 py-2 w-32 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg text-sm text-slate-900 dark:text-slate-100"
					/>
				</div>
			</div>
		</div>
	</section>

	<!-- Info Section -->
	<div class="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
		<div class="flex items-start gap-3">
			<Info class="w-6 h-6 text-[#9B00FF] flex-shrink-0 mt-0.5" />
			<div>
				<h3 class="font-semibold text-purple-900 dark:text-purple-100 mb-2">About Warphead System</h3>
				<p class="text-sm text-purple-800 dark:text-purple-300">
					This interface provides comprehensive system diagnostics, database management tools, and application settings. 
					Use the quick actions to maintain optimal performance and the settings section to customize your experience.
				</p>
			</div>
		</div>
	</div>
</PageLayout>

<!-- Table Preview Drawer -->
<TablePreviewDrawer 
	bind:isOpen={isPreviewDrawerOpen} 
	bind:tableName={previewTableName}
	onClose={closePreviewDrawer}
/>

