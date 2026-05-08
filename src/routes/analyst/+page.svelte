<script lang="ts">
	import { app } from '$lib/stores/app.svelte';
	import { analyst } from '$lib/stores/analyst.svelte';
	import { goto } from '$app/navigation';
	import { runPagedQuery } from '$lib/db-operations';
	import { onMount } from 'svelte';

	interface TableInfo {
		name: string;
		rowCount: number;
		columnCount: number;
		selected: boolean;
	}

	let tables = $state<TableInfo[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');

	onMount(async () => {
		if (app.tables.length === 0) {
			loading = false;
			return;
		}

		const infos: TableInfo[] = [];
		for (const name of app.tables) {
			try {
				const data = await runPagedQuery(`SELECT * FROM "${name}"`, 1, 1);
				infos.push({
					name,
					rowCount: data.totalRows,
					columnCount: data.columns.length,
					selected: analyst.selectedTables.includes(name)
				});
			} catch {
				infos.push({ name, rowCount: 0, columnCount: 0, selected: false });
			}
		}
		tables = infos;
		loading = false;
	});

	function toggleTable(name: string) {
		tables = tables.map((t) =>
			t.name === name ? { ...t, selected: !t.selected } : t
		);
	}

	function selectAll() {
		tables = tables.map((t) => ({ ...t, selected: true }));
	}

	function deselectAll() {
		tables = tables.map((t) => ({ ...t, selected: false }));
	}

	function handleStart() {
		const selected = tables.filter((t) => t.selected).map((t) => t.name);
		if (selected.length === 0) return;
		analyst.selectedTables = selected;
		analyst.clear();
		goto('/analyst/chat');
	}

	let selectedCount = $derived(tables.filter((t) => t.selected).length);
	let filteredTables = $derived(
		searchQuery.trim()
			? tables.filter((t) => t.name.toLowerCase().includes(searchQuery.trim().toLowerCase()))
			: tables
	);

	function formatNumber(n: number): string {
		if (n >= 1_000_000) return (n / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M';
		if (n >= 1_000) return (n / 1_000).toFixed(1).replace(/\.0$/, '') + 'K';
		return n.toLocaleString();
	}
</script>

<svelte:head>
	<title>Data Analyst — Data Monster</title>
</svelte:head>

<div class="overview">
	<div class="overview-header">
		<h2 class="overview-title">Data Analyst</h2>
		<span class="tag tag-accent">{tables.length} table{tables.length !== 1 ? 's' : ''}</span>
	</div>

	{#if tables.length > 0}
		<div class="toolbar">
			<div class="toolbar-left">
				<div class="toolbar-actions">
					<button onclick={selectAll} class="toolbar-btn">Select all</button>
					<button onclick={deselectAll} class="toolbar-btn">Deselect all</button>
				</div>
				<span class="select-count">{selectedCount} of {tables.length} selected</span>
			</div>
			<div class="toolbar-search">
				<svg class="search-icon" width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
				</svg>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Filter tables..."
					class="search-input"
				/>
			</div>
		</div>
	{/if}

	<hr class="overview-divider" />

	{#if loading}
		<div class="overview-loading">
			<span class="overview-empty-text">Loading table metadata…</span>
		</div>
	{:else if tables.length === 0}
		<div class="overview-empty">
			<span class="overview-empty-text">No tables yet. Connect a data source first, then come back to start analyzing.</span>
		</div>
	{:else}
		<div class="card-grid">
			{#each filteredTables as table (table.name)}
				<div
					class="card {table.selected ? 'is-selected' : ''}"
					role="button"
					tabindex="0"
					onclick={() => toggleTable(table.name)}
					onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); toggleTable(table.name); } }}
				>
					<div class="card-header">
						<div class="card-check {table.selected ? 'is-checked' : ''}">
							{#if table.selected}
								<svg width="12" height="12" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
									<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
								</svg>
							{/if}
						</div>
						<h4 class="card-title">{table.name}</h4>
					</div>

					<hr class="card-divider" />

					<div class="card-body">
						<div class="stat">
							<span class="stat-value">{table.columnCount}</span>
							<span class="stat-label">fields</span>
						</div>
						<div class="stat">
							<span class="stat-value">{formatNumber(table.rowCount)}</span>
							<span class="stat-label">records</span>
						</div>
					</div>

					<div class="card-footer">
						<span class="tag tag-default">{table.columnCount} cols</span>
						<span class="tag tag-default">{formatNumber(table.rowCount)} rows</span>
					</div>
				</div>
			{/each}
		</div>

		<div class="overview-footer">
			<a href="/" class="btn btn-ghost">Back</a>
			<button
				onclick={handleStart}
				disabled={selectedCount === 0}
				class="btn btn-primary btn-sm"
			>
				{selectedCount === 0 ? 'Select tables to continue' : `Analyze ${selectedCount} table${selectedCount !== 1 ? 's' : ''}`}
			</button>
		</div>
	{/if}
</div>

<style>
	.overview {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		height: 100%;
		padding: var(--space-6);
	}

	.overview-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.overview-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		border: 1px solid;
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		line-height: 1.5;
		border-radius: var(--radius-xs);
	}

	.tag-default {
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
		border-color: var(--color-border);
	}

	.tag-accent {
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		border-color: oklch(0.82 0.03 41);
	}

	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		flex-wrap: wrap;
	}

	.toolbar-left {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.select-count {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text-secondary);
		letter-spacing: 0.04em;
	}

	.toolbar-actions {
		display: flex;
		gap: var(--space-1);
	}

	.toolbar-btn {
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		padding: var(--space-1) var(--space-2);
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.04em;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.toolbar-btn:hover {
		color: var(--color-text);
		border-color: var(--color-border-strong);
	}

	.toolbar-search {
		position: relative;
		display: flex;
		align-items: center;
	}

	.search-icon {
		position: absolute;
		left: var(--space-2);
		color: var(--color-text-tertiary);
		pointer-events: none;
	}

	.search-input {
		padding: var(--space-1) var(--space-2) var(--space-1) var(--space-6);
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		color: var(--color-text);
		border-radius: var(--radius-xs);
		outline: none;
		width: calc(2 * 260px + var(--space-2));
		max-width: 100%;
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.search-input:focus {
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.search-input::placeholder {
		color: var(--color-text-tertiary);
	}

	.overview-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
	}

	.overview-empty,
	.overview-loading {
		padding: var(--space-12) var(--space-6);
		text-align: center;
		border: 1px dashed var(--color-border);
		background: var(--color-surface-raised);
	}

	.overview-empty-text {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.card-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
		gap: var(--space-2);
	}

	.card {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		cursor: pointer;
		text-align: left;
		transition: border-color var(--duration-fast) ease;
		display: flex;
		flex-direction: column;
		padding: 0;
		font: inherit;
	}

	.card,
	.card *,
	.card *::before,
	.card *::after {
		box-sizing: border-box;
	}

	.card:hover {
		border-color: var(--color-border-strong);
	}

	.card.is-selected {
		border-color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.card-header {
		padding: var(--space-3);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.card-check {
		width: 18px;
		height: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1.5px solid var(--color-border-strong);
		border-radius: var(--radius-xs);
		background: var(--color-surface);
		flex-shrink: 0;
		transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.card-check.is-checked {
		background: var(--color-accent);
		border-color: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.card-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
	}

	.card-body {
		padding: var(--space-3);
		display: flex;
		gap: var(--space-6);
		flex: 1;
	}

	.stat {
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.stat-value {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.02em;
	}

	.stat-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.card-footer {
		padding: var(--space-3);
		border-top: 1px dashed var(--color-border);
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-1);
	}

	.overview-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
	}

	@media (max-width: 540px) {
		.card-grid {
			grid-template-columns: 1fr;
		}

		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.toolbar-search {
			width: 100%;
		}

		.search-input,
		.search-input:focus {
			width: 100%;
		}
	}
</style>