<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listInternalTables,
		queryInternalTable,
		updateInternalRow,
		deleteInternalRow,
		extractErrorMessage,
		type InternalTable,
		type InternalTableData
	} from '$lib/db-operations';
	import { Database, ArrowLeft, Trash2, Edit3, ChevronLeft, ChevronRight, RefreshCw } from 'lucide-svelte';

	let tables = $state<InternalTable[]>([]);
	let selectedTable = $state<string | null>(null);
	let tableData = $state<InternalTableData | null>(null);
	let loading = $state(false);
	let error = $state('');
	let page = $state(1);
	const pageSize = 50;

	let editingRow = $state<Record<string, string> | null>(null);
	let editPkColumn = $state('');
	let editPkValue = $state('');
	let deletingPk = $state<string | null>(null);

	onMount(async () => {
		await loadTables();
	});

	async function loadTables() {
		error = '';
		try {
			tables = await listInternalTables();
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load internal tables');
		}
	}

	async function selectTable(name: string) {
		selectedTable = name;
		page = 1;
		editingRow = null;
		deletingPk = null;
		await loadData();
	}

	async function loadData() {
		if (!selectedTable) return;
		loading = true;
		error = '';
		try {
			tableData = await queryInternalTable(selectedTable, page, pageSize);
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load table data');
			tableData = null;
		}
		loading = false;
	}

	async function goToPage(p: number) {
		page = p;
		await loadData();
	}

	function startEdit(row: Record<string, unknown>) {
		const pkCol = getPkColumn();
		if (!pkCol) return;
		const editValues: Record<string, string> = {};
		for (const [k, v] of Object.entries(row)) {
			editValues[k] = v === null || v === undefined ? '' : String(v);
		}
		editPkColumn = pkCol;
		editPkValue = editValues[pkCol];
		editingRow = editValues;
		deletingPk = null;
	}

	async function saveEdit() {
		if (!editingRow || !selectedTable) return;
		error = '';
		try {
			const updates = { ...editingRow };
			delete updates[editPkColumn];
			await updateInternalRow(selectedTable, editPkColumn, editPkValue, updates);
			editingRow = null;
			await loadData();
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to update row');
		}
	}

	function cancelEdit() {
		editingRow = null;
	}

	function confirmDelete(row: Record<string, unknown>) {
		const pkCol = getPkColumn();
		if (!pkCol) return;
		deletingPk = String(row[pkCol] ?? '');
		editingRow = null;
	}

	async function executeDelete(pkValue: string) {
		if (!selectedTable) return;
		error = '';
		try {
			const pkCol = getPkColumn();
			await deleteInternalRow(selectedTable, pkCol!, pkValue);
			deletingPk = null;
			await loadData();
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to delete row');
		}
	}

	function getPkColumn(): string | null {
		if (!tableData || tableData.columns.length === 0) return null;
		return tableData.columns[0];
	}

	function formatValue(val: unknown): string {
		if (val === null || val === undefined) return '—';
		return String(val);
	}

	function truncate(s: string, max: number = 80): string {
		return s.length > max ? s.slice(0, max) + '…' : s;
	}

	let totalPages = $derived(
		tableData ? Math.max(1, Math.ceil(tableData.totalRows / pageSize)) : 1
	);
</script>

<svelte:head>
	<title>Internal DB — Data Monster</title>
</svelte:head>

<div class="idb-page">
	<div class="idb-header">
		<a href="/settings" class="btn btn-ghost btn-sm">
			<ArrowLeft size={14} />
			Settings
		</a>
		<Database size={18} />
		<h1 class="idb-title">Internal Database</h1>
		<span class="idb-subtitle">d8a_monster_* tables</span>
	</div>

	{#if error}
		<div class="idb-error">
			<span>{error}</span>
			<button class="modal-error-close" onclick={() => error = ''}>✕</button>
		</div>
	{/if}

	<div class="idb-body">
		<aside class="idb-sidebar">
			<div class="idb-sidebar-title">Tables</div>
			{#each tables as table}
				<button
					class="idb-table-btn"
					class:idb-table-btn--active={selectedTable === table.name}
					onclick={() => selectTable(table.name)}
				>
					<span class="idb-table-name">{table.name.replace('d8a_monster_', '')}</span>
					<span class="idb-table-count">{table.rowCount}</span>
				</button>
			{/each}
		</aside>

		<div class="idb-content">
			{#if !selectedTable}
				<div class="idb-empty">Select a table to browse its data</div>
			{:else if loading}
				<div class="idb-empty">Loading...</div>
			{:else if !tableData}
				<div class="idb-empty">No data</div>
			{:else}
				<div class="idb-toolbar">
					<span class="idb-toolbar-table">{selectedTable}</span>
					<span class="idb-toolbar-rows">{tableData.totalRows} rows</span>
					<button class="btn btn-ghost btn-sm" onclick={loadData} title="Refresh">
						<RefreshCw size={12} />
					</button>
				</div>

				<div class="idb-grid-wrap">
					<table class="idb-grid">
						<thead>
							<tr>
								{#each tableData.columns as col}
									<th>{col}</th>
								{/each}
								<th class="idb-grid-actions">Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each tableData.rows as row, ri}
								{@const pkCol = getPkColumn()}
								{@const pkVal = pkCol ? String(row[pkCol] ?? '') : ''}
								{#if editingRow && editPkValue === pkVal}
									<tr class="idb-row-editing">
										{#each tableData.columns as col}
											<td>
												{#if col === pkCol}
													<span class="idb-cell-readonly">{editingRow[col]}</span>
												{:else}
													<input
														type="text"
														class="idb-cell-input"
														bind:value={editingRow[col]}
													/>
												{/if}
											</td>
										{/each}
										<td class="idb-grid-actions">
											<button class="btn btn-primary btn-sm" onclick={saveEdit}>Save</button>
											<button class="btn btn-secondary btn-sm" onclick={cancelEdit}>Cancel</button>
										</td>
									</tr>
								{:else if deletingPk === pkVal}
									<tr class="idb-row-confirm">
										<td colspan={tableData.columns.length + 1}>
											<span>Delete this row?</span>
											<button class="btn btn-sm btn-danger" onclick={() => executeDelete(pkVal)}>Delete</button>
											<button class="btn btn-sm btn-secondary" onclick={() => deletingPk = null}>Cancel</button>
										</td>
									</tr>
								{:else}
									<tr>
										{#each tableData.columns as col}
											<td title={formatValue(row[col])}>{truncate(formatValue(row[col]))}</td>
										{/each}
										<td class="idb-grid-actions">
											<button
												class="btn btn-ghost btn-sm"
												title="Edit"
												onclick={() => startEdit(row)}
											>
												<Edit3 size={12} />
											</button>
											<button
												class="btn btn-ghost btn-sm"
												title="Delete"
												onclick={() => confirmDelete(row)}
											>
												<Trash2 size={12} style="color: var(--color-danger);" />
											</button>
										</td>
									</tr>
								{/if}
							{/each}
							{#if tableData.rows.length === 0}
								<tr>
									<td colspan={tableData.columns.length + 1} class="idb-empty">No rows</td>
								</tr>
							{/if}
						</tbody>
					</table>
				</div>

				{#if totalPages > 1}
					<div class="idb-pagination">
						<button
							class="btn btn-ghost btn-sm"
							disabled={page <= 1}
							onclick={() => goToPage(page - 1)}
						>
							<ChevronLeft size={14} />
						</button>
						<span class="idb-page-info">Page {page} of {totalPages}</span>
						<button
							class="btn btn-ghost btn-sm"
							disabled={page >= totalPages}
							onclick={() => goToPage(page + 1)}
						>
							<ChevronRight size={14} />
						</button>
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>

<style>
	.idb-page {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: var(--space-6);
		height: 100%;
	}

	.idb-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		color: var(--color-text);
	}

	.idb-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.idb-subtitle {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
	}

	.idb-error {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		color: var(--color-error);
		background: var(--color-error-bg, rgba(220, 38, 38, 0.08));
		border-radius: var(--radius-md, 6px);
		font-size: var(--text-sm, 0.875rem);
	}

	.modal-error-close {
		background: none;
		border: none;
		color: var(--color-error);
		cursor: pointer;
		padding: 0 0 0 var(--space-2);
		font-size: var(--text-sm, 0.875rem);
		opacity: 0.7;
	}

	.modal-error-close:hover {
		opacity: 1;
	}

	.idb-body {
		display: flex;
		gap: var(--space-4);
		flex: 1;
		min-height: 0;
		overflow: hidden;
	}

	.idb-sidebar {
		width: 220px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		background: var(--color-surface);
		padding: var(--space-3);
		overflow-y: auto;
	}

	.idb-sidebar-title {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		padding: 0 var(--space-2) var(--space-2);
		border-bottom: 1px solid var(--color-border);
		margin-bottom: var(--space-1);
	}

	.idb-table-btn {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		border: none;
		background: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		font-size: var(--text-sm);
		color: var(--color-text);
		text-align: left;
		width: 100%;
	}

	.idb-table-btn:hover {
		background: var(--color-bg-hover);
	}

	.idb-table-btn--active {
		background: var(--color-bg-active, rgba(99, 102, 241, 0.1));
		font-weight: 600;
	}

	.idb-table-name {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.idb-table-count {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
	}

	.idb-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
		overflow: hidden;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		background: var(--color-surface);
	}

	.idb-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		flex: 1;
		color: var(--color-text-tertiary);
		font-size: var(--text-sm);
		padding: var(--space-8);
	}

	.idb-toolbar {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-4);
		border-bottom: 1px solid var(--color-border);
	}

	.idb-toolbar-table {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
	}

	.idb-toolbar-rows {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.idb-grid-wrap {
		flex: 1;
		overflow: auto;
	}

	.idb-grid {
		width: 100%;
		border-collapse: collapse;
		font-size: var(--text-xs);
		font-family: var(--font-mono);
	}

	.idb-grid th {
		position: sticky;
		top: 0;
		background: var(--color-bg, #0f0f0f);
		padding: var(--space-2) var(--space-3);
		text-align: left;
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		border-bottom: 1px solid var(--color-border);
		white-space: nowrap;
		z-index: 1;
	}

	.idb-grid td {
		padding: var(--space-1) var(--space-3);
		border-bottom: 1px solid var(--color-border);
		white-space: nowrap;
		max-width: 300px;
		overflow: hidden;
		text-overflow: ellipsis;
		color: var(--color-text);
	}

	.idb-grid tr:hover td {
		background: var(--color-bg-hover);
	}

	.idb-grid-actions {
		position: sticky;
		right: 0;
		background: var(--color-surface);
		text-align: right !important;
		white-space: nowrap;
		display: flex;
		gap: var(--space-1);
		align-items: center;
		z-index: 1;
	}

	.idb-grid th.idb-grid-actions {
		background: var(--color-bg, #0f0f0f);
	}

	.idb-grid td.idb-grid-actions {
		background: var(--color-surface);
	}

	.idb-grid tr:hover td.idb-grid-actions {
		background: var(--color-bg-hover);
	}

	.idb-row-editing td {
		background: var(--color-bg-active, rgba(99, 102, 241, 0.06));
	}

	.idb-row-confirm td {
		padding: var(--space-2) var(--space-3);
		display: flex;
		align-items: center;
		gap: var(--space-3);
		color: var(--color-danger);
		font-size: var(--text-sm);
		background: rgba(220, 38, 38, 0.05);
	}

	.idb-cell-input {
		width: 100%;
		padding: 2px var(--space-1);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-bg);
		color: var(--color-text);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		outline: none;
	}

	.idb-cell-input:focus {
		border-color: var(--color-primary);
	}

	.idb-cell-readonly {
		color: var(--color-text-tertiary);
		opacity: 0.6;
	}

	.idb-pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		padding: var(--space-2);
		border-top: 1px solid var(--color-border);
	}

	.idb-page-info {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}
</style>
