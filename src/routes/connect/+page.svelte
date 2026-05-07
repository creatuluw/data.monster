<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import {
		loadCsvFile,
		loadParquetFile,
		loadJsonFile,
		connectPostgres,
		listPostgresTables,
		ingestPostgresTables
	} from '$lib/db-operations';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { Upload, Link, Database, ChevronRight, Check, LoaderCircle } from 'lucide-svelte';

	let loading = $state(false);
	let urlInput = $state('');
	let urlLoading = $state(false);

	let pgUrl = $state('');
	let pgConnecting = $state(false);
	let pgConnected = $state(false);
	let pgSchemas = $state<string[]>([]);
	let pgSelectedSchema = $state('');
	let pgTables = $state<string[]>([]);
	let pgTablesLoading = $state(false);
	let pgSelectedTables = $state<Set<string>>(new Set());
	let pgIngesting = $state(false);
	let pgIngestedTables = $state<string[]>([]);

	async function handleFilePick() {
		loading = true;
		app.globalError = '';
		try {
			const selected = await open({
				multiple: false,
				filters: [
					{
						name: 'Data files',
						extensions: ['csv', 'parquet', 'json', 'jsonl', 'ndjson', 'tsv', 'txt']
					}
				]
			});

			if (!selected) {
				loading = false;
				return;
			}

			const filePath = selected as string;
			const ext = filePath.split('.').pop()?.toLowerCase() ?? 'csv';
			const fileName = filePath.split(/[/\\]/).pop() ?? 'data';
			const tableName = fileName.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

			if (ext === 'parquet') {
				await loadParquetFile(filePath, tableName);
			} else if (ext === 'json' || ext === 'jsonl' || ext === 'ndjson') {
				await loadJsonFile(filePath, tableName);
			} else {
				await loadCsvFile(filePath, tableName);
			}

			await app.refreshTables();
			goto('/query');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to load file';
		} finally {
			loading = false;
		}
	}

	async function handleUrl() {
		if (!urlInput.trim()) return;
		urlLoading = true;
		app.globalError = '';
		try {
			await app.connectUrl(urlInput.trim());
			await app.refreshTables();
			goto('/query');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to load URL';
		} finally {
			urlLoading = false;
		}
	}

	async function handlePgConnect() {
		if (!pgUrl.trim()) return;
		pgConnecting = true;
		pgConnected = false;
		pgTables = [];
		pgSelectedTables = new Set();
		pgIngestedTables = [];
		app.globalError = '';
		try {
			pgSchemas = await connectPostgres(pgUrl.trim());
			pgConnected = true;
			if (pgSchemas.length === 1) {
				pgSelectedSchema = pgSchemas[0];
				await loadPgTables();
			}
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to connect';
		} finally {
			pgConnecting = false;
		}
	}

	async function loadPgTables() {
		if (!pgSelectedSchema) return;
		pgTablesLoading = true;
		pgTables = [];
		pgSelectedTables = new Set();
		pgIngestedTables = [];
		try {
			pgTables = await listPostgresTables(pgSelectedSchema);
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to list tables';
		} finally {
			pgTablesLoading = false;
		}
	}

	function toggleTable(name: string) {
		const next = new Set(pgSelectedTables);
		if (next.has(name)) {
			next.delete(name);
		} else {
			next.add(name);
		}
		pgSelectedTables = next;
	}

	function toggleAll() {
		if (pgSelectedTables.size === pgTables.length) {
			pgSelectedTables = new Set();
		} else {
			pgSelectedTables = new Set(pgTables);
		}
	}

	async function handleIngest() {
		if (pgSelectedTables.size === 0) return;
		pgIngesting = true;
		app.globalError = '';
		try {
			const names = Array.from(pgSelectedTables);
			pgIngestedTables = await ingestPostgresTables(pgSelectedSchema, names);
			await app.refreshTables();
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to ingest tables';
		} finally {
			pgIngesting = false;
		}
	}
</script>

<svelte:head>
	<title>Connect — Data Monster</title>
</svelte:head>

<div class="connect-page">
	<div class="hero">
		<span class="brand-mark-lg"></span>
		<h1 class="hero-title">Connect data</h1>
		<p class="hero-desc">Load a file, paste a URL, or connect a database.</p>

		<div class="connect-options">
			<div class="connect-section">
				<button class="btn btn-primary" onclick={handleFilePick} disabled={loading}>
					{#if loading}
						<svg class="spinner spinner--sm" viewBox="0 0 24 24" fill="none">
							<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
							<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
						</svg>
						Loading...
					{:else}
						<Upload size={16} />
						Open file
					{/if}
				</button>
				<span class="connect-hint">CSV, Parquet, JSON, JSONL</span>
			</div>

			<div class="connect-divider">or</div>

			<div class="connect-section">
				<div class="url-row">
					<input
						type="text"
						bind:value={urlInput}
						placeholder="https://example.com/data.csv"
						class="input"
						onkeydown={(e) => { if (e.key === 'Enter') handleUrl(); }}
					/>
					<button class="btn btn-secondary" onclick={handleUrl} disabled={urlLoading || !urlInput.trim()}>
						<Link size={14} />
						Load
					</button>
				</div>
				<span class="connect-hint">Paste a URL to a CSV, Parquet, or JSON file</span>
			</div>

			<div class="connect-divider">or</div>

			<div class="connect-section">
				<div class="url-row">
					<input
						type="text"
						bind:value={pgUrl}
						placeholder="postgresql://user:pass@host:5432/dbname"
						class="input input-mono"
						disabled={pgConnecting || pgConnected}
						onkeydown={(e) => { if (e.key === 'Enter' && !pgConnected) handlePgConnect(); }}
					/>
					{#if pgConnected}
						<button class="btn btn-secondary" onclick={() => { pgConnected = false; pgTables = []; pgSelectedTables = new Set(); pgIngestedTables = []; }}>
							Reset
						</button>
					{:else}
						<button class="btn btn-secondary" onclick={handlePgConnect} disabled={pgConnecting || !pgUrl.trim()}>
							{#if pgConnecting}
								<LoaderCircle size={14} class="spinner-icon" />
								Connecting
							{:else}
								<Database size={14} />
								Connect
							{/if}
						</button>
					{/if}
				</div>
				<span class="connect-hint">Connect to a PostgreSQL database</span>

				{#if pgConnected}
					<div class="pg-browser">
						{#if pgSchemas.length > 1}
							<div class="pg-schema-row">
								<label class="pg-label">Schema</label>
								<select class="input" bind:value={pgSelectedSchema} onchange={loadPgTables}>
									<option value="" disabled>Select schema</option>
									{#each pgSchemas as s}
										<option value={s}>{s}</option>
									{/each}
								</select>
							</div>
						{/if}

						{#if pgTablesLoading}
							<div class="pg-loading">
								<LoaderCircle size={14} class="spinner-icon" />
								Loading tables...
							</div>
						{:else if pgTables.length > 0}
							<div class="pg-tables-header">
								<span class="pg-label">{pgTables.length} tables</span>
								<button class="btn btn-ghost btn-sm" onclick={toggleAll}>
									{pgSelectedTables.size === pgTables.length ? 'Deselect all' : 'Select all'}
								</button>
							</div>
							<div class="pg-table-list">
								{#each pgTables as table}
									{@const isIngested = pgIngestedTables.includes(table)}
									{@const isSelected = pgSelectedTables.has(table)}
									<label class="pg-table-item" class:pg-table-item--selected={isSelected} class:pg-table-item--ingested={isIngested}>
										<input
											type="checkbox"
											checked={isSelected || isIngested}
											disabled={isIngested}
											onchange={() => toggleTable(table)}
										/>
										<span class="pg-table-name">{table}</span>
										{#if isIngested}
											<Check size={12} class="pg-check" />
										{/if}
									</label>
								{/each}
							</div>
							<div class="pg-actions">
								<button
									class="btn btn-primary"
									onclick={handleIngest}
									disabled={pgIngesting || pgSelectedTables.size === 0}
								>
									{#if pgIngesting}
										<LoaderCircle size={14} class="spinner-icon" />
										Ingesting {pgSelectedTables.size} tables...
									{:else}
										<ChevronRight size={14} />
										Ingest {pgSelectedTables.size || ''} {pgSelectedTables.size === 1 ? 'table' : 'tables'}
									{/if}
								</button>
								{#if pgIngestedTables.length > 0}
									<button class="btn btn-secondary" onclick={() => goto('/tables')}>
										View tables
									</button>
								{/if}
							</div>
						{:else if pgSelectedSchema}
							<span class="connect-hint">No tables found in this schema.</span>
						{/if}
					</div>
				{/if}
			</div>
		</div>

		{#if app.globalError}
			<div class="connect-error">{app.globalError}</div>
		{/if}
	</div>
</div>

<style>
	.connect-page {
		flex: 1;
	}

	.hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		padding-top: 10vh;
	}

	.brand-mark-lg {
		width: 64px;
		height: 64px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-md);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: var(--color-accent-muted);
	}

	.brand-mark-lg::after {
		content: "";
		width: 24px;
		height: 24px;
		background: var(--color-accent);
		border-radius: var(--radius-xs);
	}

	.hero-title {
		margin-top: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-2xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	.hero-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.connect-options {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		width: 100%;
		max-width: 28rem;
		margin-top: var(--space-4);
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		padding: var(--space-8);
	}

	.connect-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.connect-hint {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.connect-divider {
		text-align: center;
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		position: relative;
	}

	.connect-divider::before,
	.connect-divider::after {
		content: "";
		position: absolute;
		top: 50%;
		width: calc(50% - 1rem);
		height: 1px;
		background: var(--color-border);
	}

	.connect-divider::before { left: 0; }
	.connect-divider::after { right: 0; }

	.url-row {
		display: flex;
		gap: var(--space-2);
	}

	.url-row .input {
		flex: 1;
	}

	.connect-error {
		padding: var(--space-2) var(--space-3);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
		font-size: var(--text-xs);
		color: oklch(0.38 0.12 22);
		width: 100%;
		max-width: 28rem;
		text-align: center;
	}

	.spinner--sm {
		width: 14px;
		height: 14px;
	}

	.spinner-icon {
		animation: spin 1s linear infinite;
	}

	.pg-browser {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		margin-top: var(--space-3);
		padding-top: var(--space-3);
		border-top: 1px dashed var(--color-border);
	}

	.pg-schema-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.pg-schema-row .input {
		flex: 1;
	}

	.pg-label {
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-secondary);
		white-space: nowrap;
	}

	.pg-loading {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.pg-tables-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.pg-table-list {
		display: flex;
		flex-direction: column;
		max-height: 240px;
		overflow-y: auto;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
	}

	.pg-table-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		font-size: var(--text-xs);
		font-family: var(--font-mono);
		color: var(--color-text-secondary);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
		border-bottom: 1px solid var(--color-border);
	}

	.pg-table-item:last-child {
		border-bottom: none;
	}

	.pg-table-item:hover {
		background: var(--color-surface-sunken);
	}

	.pg-table-item--selected {
		background: var(--color-accent-muted);
	}

	.pg-table-item--ingested {
		color: oklch(0.4 0.1 160);
	}

	.pg-table-item input[type="checkbox"] {
		accent-color: var(--color-accent);
	}

	.pg-table-name {
		flex: 1;
	}

	.pg-check {
		color: oklch(0.55 0.14 160);
	}

	.pg-actions {
		display: flex;
		gap: var(--space-2);
		align-items: center;
	}
</style>
