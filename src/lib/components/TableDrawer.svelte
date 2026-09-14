<script lang="ts">
	import { X, Trash2, RefreshCw } from 'lucide-svelte';
	import {
		getTableMeta,
		type TableMeta,
		getTableLabels,
		saveTableLabels,
		renameTable,
		type TableLabels,
		getTableSource,
		type TableSource,
		refreshTableFromSource,
		executeQuery
	} from '$lib/db-operations';
	import TagInput from '$lib/components/TagInput.svelte';

	let {
		tableName = '',
		onclose,
		onrename,
		ondelete,
		onrefresh
	}: {
		tableName: string;
		onclose: () => void;
		onrename?: (oldName: string, newName: string) => void;
		ondelete?: (tableName: string) => Promise<void>;
		onrefresh?: () => Promise<void>;
	} = $props();

	let meta = $state<TableMeta | null>(null);
	let labels = $state<TableLabels>({ tableName: tableName, tags: [], group: null });
	let loading = $state(true);
	let saving = $state(false);
	let editName = $state('');
	let editTags = $state<string[]>([]);
	let editGroupTags = $state<string[]>([]);
	let confirmDelete = $state(false);
	let deleting = $state(false);
	let refreshing = $state(false);
	let drawerOpen = $state(false);
	let source = $state<TableSource | null>(null);
	let editColumn = $state<{ name: string; type: string } | null>(null);
	let editColumnType = $state('');

	const availableTypes = [
		'VARCHAR',
		'INTEGER',
		'BIGINT',
		'DOUBLE',
		'FLOAT',
		'DECIMAL',
		'BOOLEAN',
		'DATE',
		'TIMESTAMP',
		'TIME',
		'BLOB',
		'UUID',
		'JSON'
	];

	async function loadTable(name: string) {
		loading = true;
		drawerOpen = true;
		editName = name;
		editTags = [];
		editGroupTags = [];
		confirmDelete = false;
		deleting = false;
		try {
			const [m, l, s] = await Promise.all([getTableMeta(name), getTableLabels(name), getTableSource(name)]);
			meta = m;
			labels = l;
			source = s;
			editTags = [...l.tags];
			editGroupTags = l.group ? [l.group] : [];
		} catch {
			meta = null;
			labels = { tableName: name, tags: [], group: null };
			source = null;
			editTags = [];
			editGroupTags = [];
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		if (tableName) {
			loadTable(tableName);
		}
	});

	function handleClose() {
		drawerOpen = false;
		setTimeout(() => onclose(), 200);
	}

	async function handleSave() {
		saving = true;
		try {
			const trimmedName = editName.trim();
			if (!trimmedName) return;
			if (trimmedName !== tableName) {
				await renameTable(tableName, trimmedName);
				onrename?.(tableName, trimmedName);
			}
			const tagsStr = editTags.filter((t) => t.trim() !== '').join(',');
			const group = editGroupTags.length > 0 ? editGroupTags[0].trim() || null : null;
			await saveTableLabels(trimmedName, tagsStr, group);
			labels = { tableName: trimmedName, tags: [...editTags], group };
		} catch {
		} finally {
			saving = false;
		}
	}

	async function handleDelete() {
		if (!ondelete) return;
		deleting = true;
		try {
			await ondelete(tableName);
			drawerOpen = false;
			setTimeout(() => onclose(), 200);
		} catch {
		} finally {
			deleting = false;
			confirmDelete = false;
		}
	}

	async function handleRefresh() {
		refreshing = true;
		try {
			await refreshTableFromSource(tableName);
			const m = await getTableMeta(tableName);
			meta = m;
			await onrefresh?.();
		} catch {
		} finally {
			refreshing = false;
		}
	}

	async function handleTypeChange(colName: string, newType: string) {
		try {
			await executeQuery(`ALTER TABLE "${tableName}" ALTER "${colName}" TYPE ${newType}`);
			const m = await getTableMeta(tableName);
			meta = m;
		} catch {
		} finally {
			editColumn = null;
			editColumnType = '';
		}
	}

	function openTypeModal(colName: string, colType: string) {
		editColumn = { name: colName, type: colType };
		editColumnType = colType;
	}

	function closeTypeModal() {
		editColumn = null;
		editColumnType = '';
	}

	function handleModalKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			closeTypeModal();
		}
		if (e.key === 'Enter') {
			const select = (e.target as HTMLElement).closest('.drawer-modal-body')?.querySelector('select');
			if (select && document.activeElement === select) return;
			if (editColumn && editColumnType.trim()) {
				handleTypeChange(editColumn.name, editColumnType.trim().toUpperCase());
			}
		}
	}

	function formatNumber(n: number): string {
		if (n >= 1_000_000) return (n / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M';
		if (n >= 1_000) return (n / 1_000).toFixed(1).replace(/\.0$/, '') + 'K';
		return n.toLocaleString();
	}

	function inferSourceType(name: string): string {
		if (/\.parquet$/i.test(name) || /_parquet$/i.test(name)) return 'Parquet';
		if (/\.json$/i.test(name) || /\.jsonl$/i.test(name) || /\.ndjson$/i.test(name)) return 'JSON';
		return 'CSV';
	}

	function typeColor(type: string): string {
		const t = type.toLowerCase();
		if (t.includes('int')) return 'int';
		if (t.includes('float') || t.includes('double') || t.includes('decimal') || t.includes('numeric') || t.includes('real')) return 'float';
		if (t.includes('bool')) return 'bool';
		if (t.includes('date') || t.includes('time') || t.includes('timestamp')) return 'time';
		if (t.includes('blob') || t.includes('byte') || t.includes('binary')) return 'binary';
		return 'str';
	}
</script>

{#if tableName}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-overlay" class:drawer-overlay-visible={drawerOpen} onclick={handleClose} onkeydown={() => {}}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="drawer" class:drawer-open={drawerOpen} onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
			<div class="drawer-header">
				<h2 class="drawer-title">Table settings</h2>
				<button class="drawer-close" onclick={handleClose} title="Close">
					<X size={16} />
				</button>
			</div>

			{#if loading}
				<div class="drawer-body">
					<div class="drawer-loading">
						<span>Loading…</span>
					</div>
				</div>
			{:else if meta}
				<div class="drawer-body">
					<section class="drawer-section">
						<div class="drawer-field">
							<label class="drawer-label">Name</label>
							<input
								type="text"
								class="input drawer-input"
								bind:value={editName}
							/>
						</div>
						<div class="drawer-field">
							<label class="drawer-label">Group</label>
							<TagInput bind:tags={editGroupTags} placeholder="Add group and press enter…" />
						</div>
						<div class="drawer-field">
							<label class="drawer-label">Tags</label>
							<TagInput bind:tags={editTags} placeholder="Add tag and press enter…" />
						</div>
						<div class="drawer-save-row">
							<button class="btn btn-primary btn-sm" onclick={handleSave} disabled={saving}>
								{saving ? 'Saving…' : 'Save changes'}
							</button>
						</div>
					</section>

					<hr class="drawer-divider" />

					<section class="drawer-section">
						<h3 class="drawer-section-title">Overview</h3>
						<div class="drawer-stats">
							<div class="drawer-stat">
								<span class="drawer-stat-value">{formatNumber(meta.rowCount)}</span>
								<span class="drawer-stat-label">rows</span>
							</div>
							<div class="drawer-stat">
								<span class="drawer-stat-value">{meta.columnCount}</span>
								<span class="drawer-stat-label">columns</span>
							</div>
							<div class="drawer-stat">
								<span class="drawer-stat-value">{inferSourceType(meta.name)}</span>
								<span class="drawer-stat-label">source</span>
							</div>
						</div>
					</section>

					<hr class="drawer-divider" />

					{#if source && (source.sourcePath || source.originalSource)}
						<section class="drawer-section">
							<div class="drawer-section-header">
								<h3 class="drawer-section-title">Source</h3>
							</div>
							{#if source.sourceType}
								<div class="drawer-field">
									<label class="drawer-label">Type</label>
									<div class="drawer-source-path">
										{#if source.sourceType === 'url'}
											URL
										{:else if source.sourceType === 'postgres'}
											PostgreSQL
										{:else}
											File
										{/if}
									</div>
								</div>
							{/if}
							{#if source.originalSource}
								<div class="drawer-field">
									<label class="drawer-label">Original source</label>
									<div class="drawer-source-path" title={source.originalSource}>{source.originalSource}</div>
								</div>
							{/if}
							{#if source.sourcePath}
								<div class="drawer-field">
									<label class="drawer-label">Workspace path</label>
									<div class="drawer-source-path">{source.sourcePath}</div>
								</div>
							{/if}
						</section>

						<hr class="drawer-divider" />
					{/if}

					<section class="drawer-section">
						<h3 class="drawer-section-title">Columns</h3>
						<div class="drawer-columns">
							{#each meta.columns as col}
								<div class="drawer-col-group">
									<button
										class="drawer-col-row drawer-col-row-clickable"
										title="Change type of {col.name}"
										onclick={() => openTypeModal(col.name, col.type)}
									>
										<span class="drawer-col-name">{col.name}</span>
										<span class="drawer-col-type drawer-col-type-{typeColor(col.type)}">{col.type}</span>
									</button>
								</div>
							{/each}
						</div>
					</section>

					{#if source?.creationQuery}
						<hr class="drawer-divider" />

						<section class="drawer-section">
							<div class="drawer-section-header">
								<h3 class="drawer-section-title">Query</h3>
								<button class="drawer-refresh-btn" onclick={handleRefresh} disabled={refreshing} title="Reload from source">
									<RefreshCw size={12} />
								</button>
							</div>
							<pre class="drawer-source-sql">{source.creationQuery}</pre>
						</section>
					{/if}

					<hr class="drawer-divider" />

					<section class="drawer-section">
						<h3 class="drawer-section-title">Danger zone</h3>
						{#if confirmDelete}
							<div class="drawer-danger-confirm">
								<p class="drawer-danger-text">Delete <strong>{tableName}</strong>? This cannot be undone.</p>
								<div class="drawer-danger-actions">
									<button class="btn btn-secondary btn-sm" onclick={() => (confirmDelete = false)} disabled={deleting}>Cancel</button>
									<button class="btn btn-danger btn-sm" onclick={handleDelete} disabled={deleting}>
										{deleting ? 'Deleting…' : 'Delete permanently'}
									</button>
								</div>
							</div>
						{:else}
							<button class="drawer-danger-btn" onclick={() => (confirmDelete = true)}>
								<Trash2 size={13} />
								Delete table
							</button>
						{/if}
					</section>
				</div>
			{/if}
		</div>
	</div>

	<!-- svelte-ignore a11y_no_static_element_interactions -->
	{#if editColumn}
		<div class="drawer-modal-overlay" onclick={closeTypeModal} onkeydown={handleModalKeydown} role="dialog" aria-modal="true">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="drawer-modal" onclick={(e) => e.stopPropagation()} onkeydown={handleModalKeydown}>
				<div class="drawer-modal-header">
					<h3 class="drawer-modal-title">Change type</h3>
					<button class="drawer-modal-close" onclick={closeTypeModal} aria-label="Close">&times;</button>
				</div>
				<div class="drawer-modal-body">
					<div class="drawer-modal-field">
						<span class="drawer-modal-field-name">{editColumn.name}</span>
					</div>
					<select
						class="drawer-modal-select"
						bind:value={editColumnType}
						autofocus
					>
						{#each availableTypes as t}
							<option value={t} selected={t === editColumn.type}>{t}</option>
						{/each}
					</select>
				</div>
				<div class="drawer-modal-footer">
					<button class="btn btn-ghost btn-sm" onclick={closeTypeModal}>Cancel</button>
					<button class="btn btn-primary btn-sm" onclick={() => handleTypeChange(editColumn.name, editColumnType)}>
						Apply
					</button>
				</div>
			</div>
		</div>
	{/if}
{/if}

<style>
	.drawer-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0 0 0 / 0.3);
		z-index: 200;
		opacity: 0;
		transition: opacity var(--duration-base) ease;
		pointer-events: none;
	}

	.drawer-overlay-visible {
		opacity: 1;
		pointer-events: auto;
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 40vw;
		max-width: 100vw;
		background: var(--color-surface);
		border-left: 1px solid var(--color-border);
		z-index: 201;
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform var(--duration-base) var(--ease-out-expo);
		box-shadow: var(--shadow-lg);
	}

	.drawer-open {
		transform: translateX(0);
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.01em;
	}

	.drawer-close {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: none;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.drawer-close:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: 0;
	}

	.drawer-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-12) 0;
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.drawer-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.drawer-section-title {
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.drawer-field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.drawer-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.drawer-input {
		font-size: var(--text-sm);
		padding: var(--space-2) var(--space-3);
	}

	.drawer-save-row {
		display: flex;
		justify-content: flex-end;
	}

	.drawer-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
		margin: var(--space-4) 0;
	}

	.drawer-stats {
		display: flex;
		gap: var(--space-6);
	}

	.drawer-stat {
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.drawer-stat-value {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.02em;
	}

	.drawer-stat-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.drawer-columns {
		display: flex;
		flex-direction: column;
		gap: 1px;
		max-height: 240px;
		overflow-y: auto;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface-raised);
	}

	.drawer-col-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-1) var(--space-2);
		font-size: var(--text-xs);
	}

	.drawer-col-row-clickable {
		border: none;
		background: none;
		width: 100%;
		cursor: pointer;
		font-family: inherit;
		transition: background var(--duration-fast) ease;
	}

	.drawer-col-row-clickable:hover {
		background: var(--color-surface-sunken);
	}

	.drawer-col-name {
		color: var(--color-text);
		font-weight: 600;
		font-family: var(--font-mono);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		text-align: left;
	}

	.drawer-col-type {
		font-size: 9px;
		font-weight: 600;
		padding: 1px var(--space-1);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		flex-shrink: 0;
		margin-left: var(--space-2);
	}

	.drawer-modal-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		z-index: 300;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drawer-modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-lg);
		width: 90%;
		max-width: 360px;
		animation: drawerModalIn 0.15s var(--ease-out-expo);
	}

	@keyframes drawerModalIn {
		from { transform: translateY(8px) scale(0.98); opacity: 0; }
		to { transform: translateY(0) scale(1); opacity: 1; }
	}

	.drawer-modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
	}

	.drawer-modal-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.drawer-modal-close {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		color: var(--color-text-tertiary);
		font-size: 14px;
		line-height: 1;
	}

	.drawer-modal-close:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.drawer-modal-body {
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.drawer-modal-field {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.drawer-modal-field-name {
		word-break: break-all;
	}

	.drawer-modal-select {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		cursor: pointer;
		outline: none;
		width: 100%;
	}

	.drawer-modal-select:focus {
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.drawer-modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		border-top: 1px dashed var(--color-border);
	}

	.drawer-col-type-str {
		background: oklch(0.94 0.025 160);
		color: oklch(0.36 0.035 165);
	}

	.drawer-col-type-int {
		background: oklch(0.93 0.06 155);
		color: oklch(0.30 0.08 155);
	}

	.drawer-col-type-float {
		background: oklch(0.93 0.06 200);
		color: oklch(0.30 0.08 200);
	}

	.drawer-col-type-bool {
		background: oklch(0.93 0.04 310);
		color: oklch(0.35 0.08 310);
	}

	.drawer-col-type-time {
		background: oklch(0.93 0.035 95);
		color: oklch(0.32 0.045 82);
	}

	.drawer-col-type-binary {
		background: oklch(0.93 0.02 30);
		color: oklch(0.35 0.03 30);
	}

	.drawer-col-group {
		display: flex;
		flex-direction: column;
	}

	.drawer-col-group:not(:last-child) {
		border-bottom: 1px solid var(--color-border);
	}

	.drawer-col-fields {
		display: flex;
		flex-direction: column;
		padding: 0 0 var(--space-1) var(--space-4);
	}

	.drawer-col-field {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: 1px var(--space-2);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.drawer-col-field:hover {
		background: var(--color-surface-sunken);
	}

	.drawer-col-field input[type="checkbox"] {
		width: 12px;
		height: 12px;
		margin: 0;
		cursor: pointer;
		accent-color: var(--color-accent);
		flex-shrink: 0;
	}

	.drawer-col-field-name {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-secondary);
	}

	.drawer-col-field-suffix {
		font-family: var(--font-body);
		font-size: 8px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.drawer-danger-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: 1px solid oklch(0.86 0.035 25);
		background: oklch(0.97 0.008 25);
		color: oklch(0.38 0.12 25);
		border-radius: var(--radius-xs);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.drawer-danger-btn:hover {
		background: var(--color-danger);
		border-color: var(--color-danger);
		color: white;
	}

	.drawer-danger-confirm {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-3);
		border: 1px solid oklch(0.86 0.035 25);
		background: oklch(0.97 0.008 25);
		border-radius: var(--radius-sm);
	}

	.drawer-danger-text {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
	}

	.drawer-danger-actions {
		display: flex;
		gap: var(--space-2);
		justify-content: flex-end;
	}

	.drawer-section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.drawer-refresh-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: none;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.drawer-refresh-btn:hover:not(:disabled) {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.drawer-refresh-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.drawer-source-path {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		padding: var(--space-2) var(--space-3);
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		overflow-x: auto;
		white-space: nowrap;
	}

	.drawer-source-sql {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		padding: var(--space-2) var(--space-3);
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-word;
		max-height: 200px;
		overflow-y: auto;
		margin: 0;
		line-height: 1.5;
	}

	:global(.btn-danger) {
		background: var(--color-danger);
		color: white;
		border-color: var(--color-danger);
	}

	:global(.btn-danger:hover) {
		opacity: 0.85;
	}
</style>