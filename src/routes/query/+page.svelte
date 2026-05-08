<script lang="ts">
	import { RefreshCw, Share, Table, Bookmark, Copy, Upload, Play, FileText, XCircle, PlusCircle, Database, ArrowLeft, Trash2, Tag } from 'lucide-svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { runPagedQuery, createTableFromQuery, getTableMeta, getAllTableMeta, type PagedQueryResult, type TableMeta, listSavedQueries, saveQuery, updateSavedQuery, deleteSavedQuery as deleteSavedQueryOp, type SavedQuery as SavedQueryOp } from '$lib/db-operations';
	import { onMount } from 'svelte';

	interface SavedQuery {
		id: string;
		name: string;
		sql: string;
		description: string;
		tags: string;
		createdAt: string;
		updatedAt: string;
	}

	interface QueryTab {
		id: string;
		name: string;
		query: string;
		result: PagedQueryResult | null;
		error: string;
		loading: boolean;
		queryTime: number;
		loadedQuery: SavedQuery | null;
	}

	let tabs = $state<QueryTab[]>([{
		id: '1',
		name: 'query-1.sql',
		query: 'SELECT * FROM information_schema.tables LIMIT 10;',
		result: null,
		error: '',
		loading: false,
		queryTime: 0,
		loadedQuery: null
	}]);
	let activeTabId = $state('1');
	let nextTabId = 2;

	let sidebarWidth = $state(240);
	let editorHeightPercent = $state(38);
	let isDraggingSidebar = $state(false);
	let isDraggingEditor = $state(false);

	let selectedTableName = $state<string | null>(null);
	let tableMetas = $state<TableMeta[]>([]);
	let isLoadingMetas = $state(false);

	let savedQueries = $state<SavedQuery[]>([]);
	let showSavedQueriesModal = $state(false);
	let showSaveQueryModal = $state(false);
	let showIngestModal = $state(false);
	let queryName = $state('');
	let queryDescription = $state('');
	let queryTags = $state('');
	let ingestTableName = $state('');
	let searchQueryTerm = $state('');
	let selectedQueryTag = $state('');
	let deletingQuerySlug = $state<string | null>(null);
	let initialSql = $state('');
	let showPreviewLimit = $state(true);

	const activeTab = $derived(tabs.find(t => t.id === activeTabId) || tabs[0]);

	const isSavedQueryModified = $derived(
		activeTab.loadedQuery && activeTab.query !== activeTab.loadedQuery.sql
	);

	const displayQuery = $derived(() => {
		if (!showPreviewLimit) return activeTab.query;
		const trimmed = activeTab.query.replace(/;+\s*$/, '').trim();
		const lower = trimmed.toLowerCase();
		const isMutation =
			lower.startsWith('insert') ||
			lower.startsWith('update') ||
			lower.startsWith('delete') ||
			lower.startsWith('alter') ||
			lower.startsWith('drop') ||
			lower.startsWith('create');
		
		if (isMutation || lower.includes('limit')) {
			return activeTab.query;
		}
		
		return trimmed;
	});

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
				q.name.toLowerCase().includes(searchQueryTerm.toLowerCase()) ||
				(q.description && q.description.toLowerCase().includes(searchQueryTerm.toLowerCase()));
			const matchesTag = selectedQueryTag === '' ||
				(q.tags && q.tags.split(',').some(t => t.trim() === selectedQueryTag));
			return matchesSearch && matchesTag;
		});
	});

	function createNewTab() {
		const newTab: QueryTab = {
			id: String(nextTabId++),
			name: `query-${nextTabId - 1}.sql`,
			query: 'SELECT * FROM ',
			result: null,
			error: '',
			loading: false,
			queryTime: 0,
			loadedQuery: null
		};
		tabs = [...tabs, newTab];
		activeTabId = newTab.id;
		selectedTableName = null;
	}

	function closeTab(tabId: string) {
		if (tabs.length === 1) return;
		const index = tabs.findIndex(t => t.id === tabId);
		tabs = tabs.filter(t => t.id !== tabId);
		if (activeTabId === tabId) {
			activeTabId = tabs[Math.max(0, index - 1)].id;
		}
	}

	function updateTab(patch: Partial<QueryTab>) {
		const idx = tabs.findIndex(t => t.id === activeTabId);
		if (idx !== -1) {
			tabs[idx] = { ...tabs[idx], ...patch };
		}
	}

	async function handleRunQuery() {
		const sql = activeTab.query.trim();
		if (!sql) return;

		let sqlToRun = sql;
		if (showPreviewLimit) {
			const trimmed = sql.replace(/;+\s*$/, '').trim();
			const lower = trimmed.toLowerCase();
			const isMutation =
				lower.startsWith('insert') ||
				lower.startsWith('update') ||
				lower.startsWith('delete') ||
				lower.startsWith('alter') ||
				lower.startsWith('drop') ||
				lower.startsWith('create');
			
			if (!isMutation && !lower.includes('limit')) {
				sqlToRun = `${trimmed} LIMIT 100`;
			}
		}

		updateTab({ loading: true, error: '' });
		const t0 = performance.now();
		try {
			const result = await runPagedQuery(sqlToRun, 1, 100);
			const queryTime = performance.now() - t0;
			updateTab({ result, queryTime, loading: false });
			await app.refreshTables();
		} catch (e) {
			updateTab({
				error: e instanceof Error ? e.message : 'Query failed',
				queryTime: performance.now() - t0,
				loading: false
			});
		}
	}

	async function handlePageChange(page: number) {
		const sql = activeTab.query.trim();
		if (!sql) return;
		updateTab({ loading: true, error: '' });
		try {
			const result = await runPagedQuery(sql, page, 100);
			updateTab({ result, loading: false });
		} catch (e) {
			updateTab({ error: e instanceof Error ? e.message : 'Query failed', loading: false });
		}
	}

	async function loadTableMetas() {
		isLoadingMetas = true;
		try {
			if (app.tables.length > 0) {
				tableMetas = await getAllTableMeta();
			} else {
				tableMetas = [];
			}
		} catch {
			tableMetas = [];
		} finally {
			isLoadingMetas = false;
		}
	}

	async function loadTableQuery(tableName: string) {
		selectedTableName = tableName;
		try {
			const meta = await getTableMeta(tableName);
			const columnList = meta.columns.map(c => `  "${c.name}"`).join(',\n');
			const query = `-- ${meta.columnCount} columns, ${meta.rowCount.toLocaleString()} rows\nSELECT\n${columnList}\nFROM "${tableName}"\nLIMIT 100;`;
			updateTab({ query, name: `${tableName}.sql` });
		} catch {
			const query = `SELECT * FROM "${tableName}" LIMIT 100;`;
			updateTab({ query, name: `${tableName}.sql` });
		}
		handleRunQuery();
	}

	async function loadSavedQueries() {
		try {
			const queries = await listSavedQueries();
			savedQueries = queries.map(q => ({
				id: q.slug,
				name: q.name,
				sql: q.sql,
				description: q.description ?? '',
				tags: q.tags ?? '',
				createdAt: q.createdAt ?? '',
				updatedAt: q.updatedAt ?? ''
			}));
		} catch {
			savedQueries = [];
		}
	}

	async function saveCurrentQuery() {
		if (!queryName.trim()) return;
		try {
			if (activeTab.loadedQuery) {
				await updateSavedQuery(
					activeTab.loadedQuery.id,
					queryName.trim(),
					activeTab.query,
					queryDescription.trim() || undefined,
					queryTags.trim() || undefined
				);
			} else {
				await saveQuery(
					queryName.trim(),
					activeTab.query,
					queryDescription.trim() || undefined,
					queryTags.trim() || undefined
				);
			}
			await loadSavedQueries();
			showSaveQueryModal = false;
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to save query';
		}
	}

	function openSaveQueryModal() {
		if (!activeTab.query.trim()) return;
		queryName = '';
		queryDescription = '';
		queryTags = '';
		if (activeTab.loadedQuery) {
			queryName = activeTab.loadedQuery.name;
			queryDescription = activeTab.loadedQuery.description;
			queryTags = activeTab.loadedQuery.tags;
		}
		showSaveQueryModal = true;
	}

	function loadQueryFromSaved(query: SavedQuery) {
		updateTab({
			query: query.sql,
			name: `${query.name}.sql`,
			loadedQuery: { ...query },
			result: null,
			error: ''
		});
		selectedTableName = null;
		showSavedQueriesModal = false;
	}

	async function deleteSavedQuery(slug: string) {
		try {
			await deleteSavedQueryOp(slug);
			await loadSavedQueries();
		} catch {
			// ignore
		}
		deletingQuerySlug = null;
		if (activeTab.loadedQuery?.id === slug) {
			updateTab({ loadedQuery: null });
		}
	}

	function openIngestModal() {
		ingestTableName = '';
		showIngestModal = true;
	}

	async function handleIngest() {
		if (!ingestTableName.trim()) return;
		app.globalError = '';
		try {
			await createTableFromQuery(ingestTableName, activeTab.query);
			await app.refreshTables();
			showIngestModal = false;
			goto('/data');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to save table';
		}
	}

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}

	let hScrollTrack: HTMLElement | undefined = $state();
	let tableWrapEl: HTMLElement | undefined = $state();

	let syncing = false;

	function syncHScroll() {
		if (!hScrollTrack || !tableWrapEl) return;
		const table = tableWrapEl.querySelector('table');
		if (!table) return;
		const spacer = hScrollTrack.firstElementChild as HTMLElement | null;
		if (spacer) spacer.style.width = table.scrollWidth + 'px';
	}

	function onHScroll() {
		if (syncing || !hScrollTrack || !tableWrapEl) return;
		syncing = true;
		tableWrapEl.scrollLeft = hScrollTrack.scrollLeft;
		syncing = false;
	}

	function onTableWrapScroll() {
		if (syncing || !hScrollTrack || !tableWrapEl) return;
		syncing = true;
		hScrollTrack.scrollLeft = tableWrapEl.scrollLeft;
		syncing = false;
	}

	function startSidebarResize(e: MouseEvent) {
		isDraggingSidebar = true;
		e.preventDefault();
	}

	function startEditorResize(e: MouseEvent) {
		isDraggingEditor = true;
		e.preventDefault();
	}

	function handleMouseMove(e: MouseEvent) {
		if (isDraggingSidebar) {
			sidebarWidth = Math.max(180, Math.min(400, e.clientX));
		} else if (isDraggingEditor) {
			const container = document.getElementById('sql-studio-container');
			if (container) {
				const rect = container.getBoundingClientRect();
				const newHeightPixels = e.clientY - rect.top;
				const newHeightPercent = (newHeightPixels / rect.height) * 100;
				editorHeightPercent = Math.max(20, Math.min(70, newHeightPercent));
			}
		}
	}

	function stopResize() {
		isDraggingSidebar = false;
		isDraggingEditor = false;
	}

	onMount(() => {
		loadSavedQueries();
		loadTableMetas();

		if (app.pendingSql || app.pendingPreviewData) {
			const previewData = app.pendingPreviewData;
			
			if (previewData) {
				initialSql = app.pendingSqlForPreview;
				showPreviewLimit = true;
			} else {
				initialSql = app.pendingSql;
			}
			
			const shouldAutoRun = app.pendingAutoRun;
			app.pendingSql = '';
			app.pendingAutoRun = false;
			app.pendingPreviewData = null;
			
			updateTab({ query: initialSql });
			if (shouldAutoRun && initialSql) {
				requestAnimationFrame(() => handleRunQuery());
			}
		}

		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', stopResize);

		$effect(() => {
			if (activeTab.result && activeTab.result.columns.length > 0) {
				void activeTab.result;
				requestAnimationFrame(() => syncHScroll());
			}
		});

		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', stopResize);
		};
	});
</script>

<svelte:head>
	<title>Query — Data Monster</title>
</svelte:head>

<div id="sql-studio-container" class="sql-studio">
	<!-- Left Sidebar: Data Sources -->
	<div class="sidebar" style="width: {sidebarWidth}px;">
		<div class="sidebar-header">
			<h2 class="sidebar-title">Data sources</h2>
			<button onclick={loadTableMetas} class="sidebar-refresh" title="Refresh data">
				<RefreshCw size={14} />
			</button>
		</div>

		<div class="sidebar-body">
			<div class="sidebar-section-label">
				<span>Data</span>
				<a href="/data" class="sidebar-external-link" title="View all data">
					<Share size={12} />
				</a>
			</div>

			{#if isLoadingMetas}
				<div class="sidebar-empty">Loading...</div>
			{:else if tableMetas.length === 0}
				<div class="sidebar-empty">
					<span>No tables</span>
					<span class="sidebar-empty-hint">Ingest data to query</span>
				</div>
			{:else}
				{#each tableMetas as meta}
					<button
						type="button"
						onclick={() => loadTableQuery(meta.name)}
						class="sidebar-table-item {selectedTableName === meta.name ? 'sidebar-table-item--active' : ''}"
					>
						<Table
							size={14}
							class="sidebar-table-icon {selectedTableName === meta.name ? 'sidebar-table-icon--active' : ''}"
						/>
						<span class="sidebar-table-name">{meta.name}</span>
						<span class="sidebar-table-count">{meta.rowCount.toLocaleString()}</span>
					</button>
				{/each}
			{/if}
		</div>
	</div>

	<!-- Sidebar Resize Handle -->
	<div
		role="separator"
		aria-label="Resize sidebar"
		class="resize-handle resize-handle--vertical"
		onmousedown={startSidebarResize}
	></div>

	<!-- Main Content Area -->
	<div class="main-area">
		<!-- Top Bar -->
		<div class="top-bar">
			<div class="top-bar-left">
				<div class="status-pill">
					<span class="status-dot"></span>
					<span>live</span>
				</div>
				{#if activeTab.queryTime > 0}
					<span class="exec-time">{activeTab.queryTime.toFixed(0)}ms</span>
				{:else}
					<span class="exec-time">0.00s</span>
				{/if}
			</div>
			<div class="top-bar-center">
				<label class="preview-limit-toggle">
					<input type="checkbox" checked={showPreviewLimit} onchange={() => { showPreviewLimit = !showPreviewLimit; handleRunQuery(); }} />
					<span>Preview limit (100 rows)</span>
				</label>
			</div>
			<div class="top-bar-right">
				<button
					class="btn btn-secondary btn-sm"
					onclick={() => { showSavedQueriesModal = true; loadSavedQueries(); }}
				>
					<Bookmark size={14} />
					Saved
				</button>

				{#if isSavedQueryModified}
					<button class="btn btn-secondary btn-sm btn-update" onclick={openSaveQueryModal}>
						<RefreshCw size={14} />
						Update
					</button>
				{:else}
					<button class="btn btn-secondary btn-sm" onclick={openSaveQueryModal}>
						<Copy size={14} />
						Save
					</button>
				{/if}

				{#if activeTab.result && !activeTab.result.isMutation}
					<button class="btn btn-secondary btn-sm" onclick={openIngestModal}>
						<Upload size={14} />
						Ingest
					</button>
				{/if}

				<button
					class="btn btn-primary btn-sm"
					onclick={handleRunQuery}
					disabled={activeTab.loading || !activeTab.query.trim()}
				>
					{#if activeTab.loading}
						<svg class="spinner spinner--sm" viewBox="0 0 24 24" fill="none">
							<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
							<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
						</svg>
						Running
					{:else}
						<Play size={14} />
						Run
					{/if}
				</button>
			</div>
		</div>

		<!-- Editor Tabs -->
		<div class="tab-bar">
			<div class="tab-bar-scroll">
				{#each tabs as tab}
					<div class="tab-item {tab.id === activeTabId ? 'tab-item--active' : ''}">
						<button
							type="button"
							onclick={() => activeTabId = tab.id}
							class="tab-button"
						>
							<FileText size={14} />
							<span>{tab.name}</span>
							{#if tab.loadedQuery && tab.query !== tab.loadedQuery.sql}
								<span class="tab-modified-dot" title="Modified"></span>
							{/if}
						</button>
						{#if tabs.length > 1}
							<button
								type="button"
								onclick={() => closeTab(tab.id)}
								class="tab-close"
								title="Close tab"
							>
								<XCircle size={12} />
							</button>
						{/if}
					</div>
				{/each}
			</div>
			<button type="button" onclick={createNewTab} class="tab-new" title="New query">
				<PlusCircle size={16} />
			</button>
		</div>

		<!-- Editor + Results Split -->
		<div class="editor-results-split">
			<!-- Editor Area -->
			<div class="editor-pane" style="height: {editorHeightPercent}%;">
				<textarea
					value={displayQuery()}
					oninput={(e) => updateTab({ query: (e.target as HTMLTextAreaElement).value })}
					onkeydown={(e) => {
						if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
							e.preventDefault();
							handleRunQuery();
						}
						if (e.key === 'Tab') {
							e.preventDefault();
							const el = e.target as HTMLTextAreaElement;
							const start = el.selectionStart;
							const end = el.selectionEnd;
							const current = activeTab.query;
							updateTab({ query: current.substring(0, start) + '  ' + current.substring(end) });
							requestAnimationFrame(() => {
								el.selectionStart = el.selectionEnd = start + 2;
							});
						}
					}}
					placeholder="-- Write your SQL query here&#10;SELECT * FROM table_name LIMIT 100;"
					class="editor-textarea"
					spellcheck="false"
				></textarea>
				<span class="editor-hint">Ctrl+Enter to run</span>
			</div>

			<!-- Editor Resize Handle -->
			<div
				role="separator"
				aria-label="Resize editor"
				class="resize-handle resize-handle--horizontal"
				onmousedown={startEditorResize}
			></div>

			<!-- Results Area -->
			<div class="results-pane">
				<!-- Results Header Tab -->
				<div class="results-header">
					<span class="results-tab">
						Results
						{#if activeTab.result && activeTab.result.rows.length > 0}
							<span class="tag tag-accent">
								{activeTab.result.totalRows.toLocaleString()} rows
							</span>
						{/if}
					</span>
				</div>

				<!-- Results Content -->
				<div class="results-body">
					{#if activeTab.loading}
						<div class="results-loading">
							<svg class="spinner" viewBox="0 0 24 24" fill="none" style="color: var(--color-accent);">
								<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
								<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
							</svg>
							<span>Running query...</span>
						</div>
					{:else if activeTab.error}
						<div class="results-error">{activeTab.error}</div>
					{:else if !activeTab.result || activeTab.result.columns.length === 0}
						<div class="results-empty">
							<Database size={32} />
							<span>No query results</span>
							<span class="results-empty-hint">Write a query and press Run</span>
						</div>
					{:else}
						<div class="results-table-wrap" bind:this={tableWrapEl} onscroll={onTableWrapScroll}>
							<table class="data-table">
								<thead>
									<tr>
										{#each activeTab.result.columns as col}
											<th>{col}</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each activeTab.result.rows as row}
										<tr>
											{#each activeTab.result.columns as col}
												<td>{formatCell(row[col])}</td>
											{/each}
										</tr>
									{/each}
								</tbody>
							</table>
						</div>

						<!-- Pagination -->
						{#if !activeTab.result.isMutation && activeTab.result.totalPages > 1}
							<div class="pagination">
								<button
									onclick={() => handlePageChange(activeTab.result!.page - 1)}
									disabled={activeTab.result!.page <= 1}
									class="btn btn-ghost btn-sm"
								>
									<ArrowLeft size={14} />
									prev
								</button>
								<span class="pagination-info">
									Page {activeTab.result.page} of {activeTab.result.totalPages}
									<span class="tag tag-default" style="margin-left: var(--space-2);">
										{activeTab.result.rows.length} of {activeTab.result.totalRows.toLocaleString()}
									</span>
								</span>
								<button
									onclick={() => handlePageChange(activeTab.result!.page + 1)}
									disabled={activeTab.result!.page >= activeTab.result!.totalPages}
									class="btn btn-ghost btn-sm"
								>
									next
									<Share size={14} style="transform: scaleX(-1);" />
								</button>
							</div>
						{/if}

						{#if activeTab.result.isMutation}
							<div class="mutation-result">
								<span class="tag tag-success">Query executed</span>
								{#if activeTab.result.rows.length > 0}
									<span class="tag tag-default">{activeTab.result.rows.length} rows affected</span>
								{/if}
							</div>
						{/if}
					{/if}
				</div>

				<!-- Sticky Horizontal Scrollbar -->
				{#if activeTab.result && activeTab.result.columns.length > 0}
					<div class="h-scroll-track" bind:this={hScrollTrack} onscroll={onHScroll}>
						<div class="h-scroll-spacer"></div>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<!-- Saved Queries Modal -->
{#if showSavedQueriesModal}
	<div class="modal-overlay" onclick={() => showSavedQueriesModal = false}>
		<div class="modal modal--lg" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2 class="modal-title">Saved queries</h2>
				<button onclick={() => showSavedQueriesModal = false} class="modal-close">
					<XCircle size={20} />
				</button>
			</div>

			{#if savedQueries.length === 0}
				<div class="modal-empty">
					<Bookmark size={48} />
					<h3 class="modal-empty-title">No saved queries</h3>
					<p class="modal-empty-text">Save your frequently used queries for quick access</p>
					<button
						class="btn btn-primary"
						onclick={() => { showSavedQueriesModal = false; openSaveQueryModal(); }}
					>
						<Copy size={14} />
						Save current query
					</button>
				</div>
			{:else}
				<div class="saved-search">
					<input
						type="text"
						bind:value={searchQueryTerm}
						placeholder="Search queries..."
						class="input"
					/>
					{#if allQueryTags().length > 0}
						<select bind:value={selectedQueryTag} class="input">
							<option value="">All tags</option>
							{#each allQueryTags() as tag}
								<option value={tag}>{tag}</option>
							{/each}
						</select>
					{/if}
					<span class="saved-count">{filteredQueries().length} quer{filteredQueries().length === 1 ? 'y' : 'ies'}</span>
				</div>

				<div class="saved-grid">
					{#each filteredQueries() as query}
						<div class="saved-card">
							<div class="saved-card-head">
								<div class="saved-card-title-row">
									<h3 class="saved-card-name">{query.name}</h3>
									<div class="saved-card-actions">
										<button
											onclick={() => loadQueryFromSaved(query)}
											class="btn btn-ghost btn-sm"
											title="Load query"
										>
											<FileText size={14} />
										</button>
										<button
											onclick={() => deletingQuerySlug = query.id}
											class="btn btn-ghost btn-sm"
											title="Delete"
										>
											<Trash2 size={14} style="color: var(--color-danger);" />
										</button>
									</div>
								</div>
								{#if query.description}
									<p class="saved-card-desc">{query.description}</p>
								{/if}
							</div>

							<div class="saved-card-sql">
								<code>{query.sql}</code>
							</div>

							{#if query.tags}
								<div class="saved-card-tags">
									{#each query.tags.split(',').map(t => t.trim()).filter(t => t) as tag}
										<span class="tag tag-default">
											<Tag size={10} />
											{tag}
										</span>
									{/each}
								</div>
							{/if}

							<div class="saved-card-meta">
								{new Date(query.updatedAt).toLocaleDateString()}
							</div>

							{#if deletingQuerySlug === query.id}
								<div class="saved-card-confirm">
									<span>Delete this query?</span>
									<button class="btn btn-sm btn-danger" onclick={() => deleteSavedQuery(query.id)}>Delete</button>
									<button class="btn btn-sm btn-secondary" onclick={() => deletingQuerySlug = null}>Cancel</button>
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
	<div class="modal-overlay" onclick={() => showSaveQueryModal = false}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2 class="modal-title">Save query</h2>
			</div>

			<div class="modal-body">
				<div class="field">
					<label for="save-query-name" class="field-label">Name *</label>
					<input id="save-query-name" type="text" bind:value={queryName} placeholder="Monthly sales report" class="input" />
				</div>

				<div class="field">
					<label for="save-query-desc" class="field-label">Description</label>
					<textarea id="save-query-desc" bind:value={queryDescription} placeholder="What does this query do?" rows="3" class="input" style="resize: none;"></textarea>
				</div>

				<div class="field">
					<label for="save-query-tags" class="field-label">Tags</label>
					<input id="save-query-tags" type="text" bind:value={queryTags} placeholder="reporting, sales, monthly" class="input" />
					<span class="field-hint">Separate tags with commas</span>
				</div>

				<div class="field">
					<span class="field-label">SQL preview</span>
					<div class="sql-preview">
						<code>{activeTab.query || ''}</code>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn btn-secondary" onclick={() => showSaveQueryModal = false}>Cancel</button>
				<button class="btn btn-primary" onclick={saveCurrentQuery} disabled={!queryName.trim()}>
					<Copy size={14} />
					Save query
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Ingest as Table Modal -->
{#if showIngestModal}
	<div class="modal-overlay" onclick={() => showIngestModal = false}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2 class="modal-title">Ingest as table</h2>
			</div>

			<div class="modal-body">
				<div class="field">
					<label for="ingest-table-name" class="field-label">Table name</label>
					<input id="ingest-table-name" type="text" bind:value={ingestTableName} placeholder="new_table" class="input input-mono" />
				</div>

				<div class="field">
					<span class="field-label">SQL to save</span>
					<div class="sql-preview">
						<code>{activeTab.query || ''}</code>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn btn-secondary" onclick={() => showIngestModal = false}>Cancel</button>
				<button class="btn btn-primary" onclick={handleIngest} disabled={!ingestTableName.trim()}>
					<Upload size={14} />
					Ingest
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	:global(.app-body) {
		min-height: 0;
	}

	:global(.app-main) {
		overflow: hidden !important;
		padding: 0 !important;
		display: flex !important;
		flex-direction: column !important;
		min-height: 0;
	}

	.sql-studio {
		display: flex;
		flex: 1 1 0%;
		width: 100%;
		min-height: 0;
		margin: 0;
		background: var(--color-surface);
		overflow: hidden;
	}

	/* ── Sidebar ── */

	.sidebar {
		background: var(--color-surface-raised);
		border-right: 1px solid var(--color-border);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
	}

	.sidebar-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		flex-shrink: 0;
	}

	.sidebar-title {
		font-family: var(--font-display);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
		letter-spacing: 0.02em;
	}

	.sidebar-refresh {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2px;
		border: none;
		background: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.sidebar-refresh:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.sidebar-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-1) 0;
	}

	.sidebar-section-label {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.sidebar-external-link {
		color: var(--color-text-tertiary);
		transition: color var(--duration-fast) ease;
	}

	.sidebar-external-link:hover {
		color: var(--color-accent);
	}

	.sidebar-empty {
		padding: var(--space-3);
		text-align: center;
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.sidebar-empty-hint {
		font-size: 9px;
		color: var(--color-text-tertiary);
		opacity: 0.7;
	}

	.sidebar-table-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-1) var(--space-3);
		padding-left: calc(var(--space-3) + 2px);
		border: none;
		background: none;
		cursor: pointer;
		text-align: left;
		border-left: 2px solid transparent;
		transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.sidebar-table-item:hover {
		background: var(--color-surface-sunken);
	}

	.sidebar-table-item--active {
		background: var(--color-accent-muted);
		border-left-color: var(--color-accent);
	}

	.sidebar-table-icon {
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.sidebar-table-icon--active {
		color: var(--color-accent-dark);
	}

	.sidebar-table-name {
		flex: 1;
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.sidebar-table-item--active .sidebar-table-name {
		color: var(--color-accent-dark);
		font-weight: 500;
	}

	.sidebar-table-count {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	/* ── Resize Handles ── */

	.resize-handle {
		flex-shrink: 0;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle--vertical {
		width: 3px;
		cursor: col-resize;
		background: var(--color-border);
	}

	.resize-handle--vertical:hover {
		background: var(--color-accent);
	}

	.resize-handle--horizontal {
		height: 3px;
		cursor: row-resize;
		background: var(--color-border);
	}

	.resize-handle--horizontal:hover {
		background: var(--color-accent);
	}

	/* ── Main Area ── */

	.main-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	/* ── Top Bar ── */

	.top-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		background: var(--color-surface);
		flex-shrink: 0;
	}

	.top-bar-left,
	.top-bar-right,
	.top-bar-center {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.top-bar {
		justify-content: space-between;
	}

	.status-pill {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		background: var(--color-surface-sunken);
		border-radius: var(--radius-xs);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--color-success);
	}

	.exec-time {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.preview-limit-toggle {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		cursor: pointer;
		user-select: none;
	}

	.preview-limit-toggle input[type="checkbox"] {
		accent-color: var(--color-accent);
		cursor: pointer;
	}

	.btn-update {
		background: var(--color-warning);
		color: oklch(0.25 0.05 80);
		border-color: oklch(0.7 0.06 80);
	}

	.btn-update:hover:not(:disabled) {
		background: oklch(0.68 0.12 80);
	}

	.tab-bar {
		display: flex;
		align-items: center;
		background: var(--color-surface);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.tab-bar-scroll {
		display: flex;
		flex: 1;
		overflow-x: auto;
	}

	.tab-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border-right: 1px solid var(--color-border);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.tab-item:hover {
		background: var(--color-surface-raised);
	}

	.tab-item--active {
		background: var(--color-surface-raised);
		color: var(--color-text);
	}

	.tab-button {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		border: none;
		background: none;
		cursor: pointer;
		font: inherit;
		color: inherit;
		padding: 0;
	}

	.tab-modified-dot {
		width: 5px;
		height: 5px;
		border-radius: 50%;
		background: var(--color-warning);
	}

	.tab-close {
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		cursor: pointer;
		padding: 2px;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		opacity: 0;
		transition: opacity var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.tab-item:hover .tab-close {
		opacity: 1;
	}

	.tab-close:hover {
		color: var(--color-danger);
	}

	.tab-new {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-2) var(--space-3);
		border: none;
		background: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		border-left: 1px solid var(--color-border);
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.tab-new:hover {
		background: var(--color-surface-raised);
		color: var(--color-text);
	}

	/* ── Editor / Results Split ── */

	.editor-results-split {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	.editor-pane {
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
		position: relative;
		background: var(--color-surface);
	}

	.editor-textarea {
		flex: 1;
		width: 100%;
		resize: none;
		padding: var(--space-4);
		border: none;
		background: var(--color-surface);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text);
		outline: none;
	}

	.editor-textarea::placeholder {
		color: var(--color-text-tertiary);
	}

	.editor-hint {
		position: absolute;
		bottom: var(--space-2);
		right: var(--space-3);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
		opacity: 0.6;
	}

	/* ── Results Pane ── */

	.results-pane {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
		background: var(--color-surface);
	}

	.results-header {
		display: flex;
		align-items: center;
		padding: 0 var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.results-tab {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) 0;
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
		border-bottom: 2px solid var(--color-accent);
		margin-bottom: -1px;
	}

	.results-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		min-height: 0;
	}

	.results-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		padding: var(--space-16) 0;
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.results-error {
		padding: var(--space-4);
		margin: var(--space-3);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: oklch(0.38 0.12 22);
	}

	.results-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		height: 100%;
		color: var(--color-text-tertiary);
		padding: var(--space-8) 0;
	}

	.results-empty-hint {
		font-size: var(--text-xs);
		opacity: 0.7;
	}

	.results-table-wrap {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
		min-height: 0;
	}

	.h-scroll-track {
		flex-shrink: 0;
		overflow-x: auto;
		overflow-y: hidden;
		height: 12px;
		border-top: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.h-scroll-spacer {
		height: 1px;
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		padding: var(--space-3);
		border-top: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.pagination-info {
		display: flex;
		align-items: center;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}

	.mutation-result {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border-top: 1px solid var(--color-border);
	}

	/* ── Modals ── */

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.5);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 4rem 1rem;
		z-index: 60;
	}

	.modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		box-shadow: var(--shadow-lg);
		width: 100%;
		max-width: 36rem;
		max-height: 80vh;
		overflow-y: auto;
	}

	.modal--lg {
		max-width: 48rem;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-6);
		border-bottom: 1px solid var(--color-border);
		position: sticky;
		top: 0;
		background: var(--color-surface);
		z-index: 1;
	}

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		color: var(--color-text);
	}

	.modal-close {
		display: flex;
		align-items: center;
		border: none;
		background: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		transition: color var(--duration-fast) ease;
	}

	.modal-close:hover {
		color: var(--color-text);
	}

	.modal-body {
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px solid var(--color-border);
	}

	.modal-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: var(--space-16) var(--space-6);
		gap: var(--space-4);
		color: var(--color-text-tertiary);
		text-align: center;
	}

	.modal-empty-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		color: var(--color-text);
	}

	.modal-empty-text {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		max-width: 30ch;
	}

	/* ── Saved Queries ── */

	.saved-search {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-bottom: 1px solid var(--color-border);
	}

	.saved-count {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		white-space: nowrap;
	}

	.saved-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
		padding: var(--space-6);
	}

	.saved-card {
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.saved-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.saved-card-head {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.saved-card-title-row {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.saved-card-name {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.saved-card-actions {
		display: flex;
		gap: 2px;
		flex-shrink: 0;
	}

	.saved-card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.saved-card-sql {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		padding: var(--space-2);
		overflow: hidden;
	}

	.saved-card-sql code {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-secondary);
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
		white-space: pre-wrap;
	}

	.saved-card-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
	}

	.saved-card-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
	}

	.saved-card-confirm {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-top: var(--space-3);
		border-top: 1px solid var(--color-border);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
	}

	/* ── SQL Preview ── */

	.sql-preview {
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		padding: var(--space-3);
		max-height: 8rem;
		overflow: auto;
	}

	.sql-preview code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		white-space: pre-wrap;
	}

	/* ── Spinner variant ── */

	.spinner--sm {
		width: 14px;
		height: 14px;
	}
</style>