<script lang="ts">
	import '../app.css';
	import FileConnector from '$lib/components/FileConnector.svelte';
	import PreviewPane from '$lib/components/PreviewPane.svelte';
	import QueryEditor from '$lib/components/QueryEditor.svelte';
	import TableViewer from '$lib/components/TableViewer.svelte';
	import TableList from '$lib/components/TableList.svelte';
	import TableOverview from '$lib/components/TableOverview.svelte';
	import { getDb, isOpfsSupported } from '$lib/duckdb';
	import {
		previewFile,
		previewUrl,
		ingestFileAsTable,
		runQuery,
		listTables,
		getTableData,
		dropTable,
		saveQueryAsTable,
		type PreviewData,
		type PagedQueryResult,
		type QueryResult,
	type ColumnOverride
		} from '$lib/db-helpers';
	import { getRecipe, saveRecipe, deleteRecipe } from '$lib/recipes';

	// ── View state ──
	type View = 'connect' | 'preview' | 'query' | 'table' | 'tables';
	let activeView = $state<View>('connect');
	let tables = $state<string[]>([]);
	let selectedTable = $state('');
	let dbReady = $state(false);
	let opfsStatus = $state<'checking' | 'persistent' | 'memory'>('checking');
	let globalError = $state('');

	// ── Connect state ──
	let connectLoading = $state(false);

	// ── Preview state ──
	let previewData = $state<PreviewData | null>(null);
	let previewFileName = $state('');
	let previewSourceType = $state<'file' | 'url'>('file');
	let previewSourceName = $state('');
	let columnOverrides = $state<ColumnOverride[]>([]);
	let ingestTableName = $state('');

	// ── Query state ──
	let queryResult = $state<PagedQueryResult | null>(null);
	let queryLoading = $state(false);
	let queryError = $state('');
	let queryTime = $state(0);
	let queryEditorSql = $state('');

	// ── Table viewer state ──
	let tableData = $state<(QueryResult & { totalRows: number }) | null>(null);
	let tablePage = $state(1);
	let tableTotalPages = $state(1);
	const PAGE_SIZE = 100;

	// ── Init ──
	async function init() {
		try {
			await getDb();
			opfsStatus = isOpfsSupported() ? 'persistent' : 'memory';

			tables = await listTables();

			dbReady = true;
			// If tables exist, default to query view; otherwise connect
			if (tables.length > 0) {
				activeView = 'query';
			} else {
				activeView = 'connect';
			}
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to initialize database';
		}
	}

	// ── Navigation helpers ──
	function switchToConnect() {
		activeView = 'connect';
	}

	function switchToTable(tableName: string) {
		selectedTable = tableName;
		tablePage = 1;
		activeView = 'table';
		loadTableData(tableName, 1);
	}

	function switchToQuery(sql?: string, autoRun = false) {
		if (sql) queryEditorSql = sql;
		queryResult = null;
		queryError = '';
		queryTime = 0;
		activeView = 'query';
		// Auto-run after view renders
		if (autoRun && sql) {
			requestAnimationFrame(() => handleRunQuery(sql, 1, 10));
		}
	}

	// ── Connect handlers ──
	async function handleFileConnect(file: File) {
		connectLoading = true;
		globalError = '';
		try {
			previewData = await previewFile(file);
			previewFileName = file.name;
			previewSourceType = 'file';
			previewSourceName = file.name;
			ingestTableName = file.name.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

			// Initialize column overrides from detected types
			columnOverrides = previewData.detectedTypes.map((col) => ({
				originalName: col.name,
				newName: col.name,
				detectedType: col.type,
				overrideType: null,
				enabled: true
			}));

			activeView = 'preview';
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to load file';
		} finally {
			connectLoading = false;
		}
	}

	async function handleUrlConnect(url: string) {
		connectLoading = true;
		globalError = '';
		try {
			previewData = await previewUrl(url);
			previewSourceType = 'url';
			previewSourceName = url;
			const fileName = url.split('/').pop()?.split('?')[0] ?? 'remote_data';
			ingestTableName = fileName.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

			columnOverrides = previewData.detectedTypes.map((col) => ({
				originalName: col.name,
				newName: col.name,
				detectedType: col.type,
				overrideType: null,
				enabled: true
			}));

			activeView = 'preview';
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to load URL';
		} finally {
			connectLoading = false;
		}
	}

	// ── Preview → Query (with pre-populated SELECT) ──
	function handlePreviewNext() {
		if (!previewData) return;

		// Build the column list — only enabled columns, CAST only where type changed
		const enabledColumns = columnOverrides.filter((c) => c.enabled);
		const columns = enabledColumns.map((c) => {
			const typeChanged = c.overrideType !== null && c.overrideType !== c.detectedType;
			const name = typeChanged
				? `"${c.originalName}"::${c.overrideType}`
				: `"${c.originalName}"`;
			return `${name} AS "${c.newName}"`;
		}).join(',\n       ');

		// Build the reader expression
		const ext = previewFileName.split('.').pop()?.toLowerCase() ?? 'csv';
		const reader =
			ext === 'parquet'
				? `read_parquet('${previewFileName}')`
				: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
					? `read_json_auto('${previewFileName}')`
					: `read_csv_auto('${previewFileName}')`;

		const defaultTableName = previewSourceName.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');
		ingestTableName = defaultTableName;

		const sql = `SELECT\n       ${columns}\nFROM ${reader}`;

		// Switch to query view — auto-run will trigger
		switchToQuery(sql, true);
	}

	// ── Query handlers ──
	async function handleRunQuery(sql: string, page: number = 1, pageSize: number = 10) {
		queryLoading = true;
		queryError = '';
		const t0 = performance.now();
		try {
			const result = await runQuery(sql, page, pageSize);
			queryTime = performance.now() - t0;
			queryResult = result;
			// Refresh table list in case of CREATE/DROP/ALTER
			tables = await listTables();
		} catch (e) {
			queryError = e instanceof Error ? e.message : 'Query failed';
			queryTime = performance.now() - t0;
		} finally {
			queryLoading = false;
		}
	}

	async function handleSaveAsTable(sql: string, tableName: string) {
		if (!tableName.trim()) return;
		globalError = '';
		try {
			await saveQueryAsTable(sql, tableName);
			// Save recipe
			saveRecipe({
				tableName,
				createdAt: new Date().toISOString(),
				source: { type: previewSourceType, name: previewSourceName },
				columnOverrides,
				query: sql
			});

			tables = await listTables();
			selectedTable = tableName;
			activeView = 'tables';
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to save table';
		}
	}

	// ── Table viewer handlers ──
	async function loadTableData(tableName: string, page: number) {
		try {
			tableData = await getTableData(tableName, page, PAGE_SIZE);
			tablePage = page;
			tableTotalPages = Math.max(1, Math.ceil(tableData.totalRows / PAGE_SIZE));
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to load table';
		}
	}

	function handleQueryTable() {
		switchToQuery(`SELECT * FROM "${selectedTable}"\nLIMIT 100`);
	}

	async function handleDropTable() {
		if (!selectedTable) return;
		globalError = '';
		try {
			await dropTable(selectedTable);
			deleteRecipe(selectedTable);
			tables = await listTables();
			selectedTable = '';
			activeView = tables.length > 0 ? 'query' : 'connect';
		} catch (e) {
			globalError = e instanceof Error ? e.message : 'Failed to drop table';
		}
	}

	function handleEditSource() {
		const recipe = getRecipe(selectedTable);
		if (!recipe) return;
		// TODO: implement walk-back flow — for now, switch to query with recipe SQL
		if (recipe.query) {
			switchToQuery(recipe.query);
		} else {
			switchToConnect();
		}
	}

	function handlePageChange(page: number) {
		loadTableData(selectedTable, page);
	}

	// ── Boot ──
	init();
</script>

<svelte:head>
	<title>Data Monster</title>
</svelte:head>

<div class="flex h-screen flex-col">
	<!-- Header -->
	<header class="flex shrink-0 items-center justify-between border-b border-sand-200 bg-white px-6 py-4">
		<div class="flex items-center gap-3">
			<div class="flex h-8 w-8 items-center justify-center rounded-md bg-sage-600">
				<svg class="h-4 w-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M4.745 3A23.933 23.933 0 0 0 3 12c0 3.183.62 6.22 1.745 9M19.5 3c.967 2.78 1.5 5.817 1.5 9s-.533 6.22-1.5 9M8.25 8.885l1.444-.89a.75.75 0 0 1 1.105.402l2.402 7.206a.75.75 0 0 0 1.105.401l1.444-.889" />
				</svg>
			</div>
			<h1 class="font-display text-lg font-bold tracking-tight text-sand-800">Data Monster</h1>
		</div>
		<div class="flex items-center gap-3">
			<span class="cursor-pointer text-xs text-sand-400 transition-colors hover:text-sage-600" onclick={() => activeView = 'tables'}>{tables.length} table{tables.length !== 1 ? 's' : ''}</span>
			{#if opfsStatus === 'memory'}
				<span class="rounded-full bg-copper-100 px-2 py-0.5 text-[11px] font-medium text-copper-700">In-Memory</span>
			{:else if opfsStatus === 'persistent'}
				<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">Persistent</span>
			{/if}
		</div>
	</header>

	<!-- Body: sidebar + main -->
	{#if !dbReady}
		<div class="flex flex-1 items-center justify-center">
			<div class="flex items-center gap-3 text-sand-400">
				<svg class="h-5 w-5 animate-spin" viewBox="0 0 24 24" fill="none">
					<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
					<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
				</svg>
				<span class="text-sm">Initializing database...</span>
			</div>
		</div>
	{:else}
		<div class="flex flex-1 overflow-hidden">
			<!-- Sidebar -->
			<aside class="w-56 shrink-0 border-r border-sand-200 bg-sand-50 p-4">
				<TableList {tables} selected={selectedTable} onselect={switchToTable} onconnect={switchToConnect} />
			</aside>

			<!-- Main view -->
			<main class="flex-1 overflow-y-auto px-6 py-4">
				<!-- Global error -->
				{#if globalError}
					<div class="mb-4 rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700">{globalError}</div>
				{/if}

				{#if activeView === 'tables'}
					<TableOverview {tables} onselect={switchToTable} />
				{:else if activeView === 'connect'}
					<FileConnector onfile={handleFileConnect} onurl={handleUrlConnect} loading={connectLoading} />

				{:else if activeView === 'preview' && previewData}
					<PreviewPane
						data={previewData}
						bind:columnOverrides
						onnext={handlePreviewNext}
					/>

				{:else if activeView === 'query'}
					<QueryEditor
						onrun={handleRunQuery}
						onsaveastable={handleSaveAsTable}
						result={queryResult}
						loading={queryLoading}
						error={queryError}
						queryTime={queryTime}
						initialSql={queryEditorSql}
						suggestedTableName={ingestTableName}
					/>

				{:else if activeView === 'table' && tableData}
					<TableViewer
						result={tableData}
						tableName={selectedTable}
						page={tablePage}
						totalPages={tableTotalPages}
						onpagechange={handlePageChange}
						onquerytable={handleQueryTable}
						oneditsource={handleEditSource}
						ondroptable={handleDropTable}
						hasRecipe={!!getRecipe(selectedTable)}
					/>
				{/if}
			</main>
		</div>
	{/if}
</div>
