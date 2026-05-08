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
		refreshTableFromSource
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

					{#if source && (source.creationQuery || source.sourcePath)}
						<section class="drawer-section">
							<div class="drawer-section-header">
								<h3 class="drawer-section-title">Source</h3>
								{#if source.creationQuery}
									<button class="drawer-refresh-btn" onclick={handleRefresh} disabled={refreshing} title="Reload from source">
										<RefreshCw size={12} />
									</button>
								{/if}
							</div>
							{#if source.sourcePath}
								<div class="drawer-field">
									<label class="drawer-label">Path</label>
									<div class="drawer-source-path">{source.sourcePath}</div>
								</div>
							{/if}
							{#if source.creationQuery}
								<div class="drawer-field">
									<label class="drawer-label">Query</label>
									<pre class="drawer-source-sql">{source.creationQuery}</pre>
								</div>
							{/if}
						</section>

						<hr class="drawer-divider" />
					{/if}

					<section class="drawer-section">
						<h3 class="drawer-section-title">Columns</h3>
						<div class="drawer-columns">
							{#each meta.columns as col}
								<div class="drawer-col-row">
									<span class="drawer-col-name">{col.name}</span>
									<span class="drawer-col-type drawer-col-type-{typeColor(col.type)}">{col.type}</span>
								</div>
							{/each}
						</div>
					</section>

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
		width: 560px;
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

	.drawer-col-row:not(:last-child) {
		border-bottom: 1px solid var(--color-border);
	}

	.drawer-col-name {
		color: var(--color-text);
		font-weight: 600;
		font-family: var(--font-mono);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
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

	.drawer-col-type-str {
		background: oklch(0.93 0.02 250);
		color: oklch(0.35 0.04 250);
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
		background: oklch(0.93 0.06 60);
		color: oklch(0.30 0.08 60);
	}

	.drawer-col-type-binary {
		background: oklch(0.93 0.02 30);
		color: oklch(0.35 0.03 30);
	}

	.drawer-danger-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: 1px solid oklch(0.85 0.04 22);
		background: oklch(0.97 0.01 22);
		color: oklch(0.38 0.12 22);
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
		border: 1px solid oklch(0.85 0.04 22);
		background: oklch(0.97 0.01 22);
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