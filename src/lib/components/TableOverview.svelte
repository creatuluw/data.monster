<script lang="ts">
	import { onMount } from 'svelte';
	import { SquarePen, X, Plus, Tag, FolderOpen, Trash2 } from 'lucide-svelte';
	import { getAllTableMeta, type TableMeta } from '$lib/db-operations';
	import {
		getTableLabels,
		saveTableLabels,
		getAllTags as getAllTagsOp,
		getAllGroups as getAllGroupsOp,
		renameTable,
		type TableLabels
	} from '$lib/db-operations';

	let {
		tables = [],
		onselect,
		onrename,
		ondelete
	}: {
		tables: string[];
		onselect: (table: string) => void;
		onrename?: (oldName: string, newName: string) => void;
		ondelete?: (tableName: string) => Promise<void>;
	} = $props();

	let tableMetas = $state<TableMeta[]>([]);
	let loading = $state(true);
	let labelsMap = $state<Record<string, TableLabels>>({});
	let allTags = $state<string[]>([]);
	let allGroups = $state<string[]>([]);

	let filterTag = $state<string | null>(null);
	let filterGroup = $state<string | null>(null);

	let editingTable = $state<string | null>(null);
	let editTags = $state<string[]>([]);
	let editGroup = $state<string>('');
	let editName = $state<string>('');
	let newTagInput = $state('');

	let confirmDeleteTable = $state<string | null>(null);
	let deletingTable = $state<string | null>(null);

	onMount(async () => {
		try {
			tableMetas = await getAllTableMeta();
		} catch {
			tableMetas = [];
		}
		refreshLabels();
		loading = false;
	});

	async function refreshLabels() {
		const map: Record<string, TableLabels> = {};
		for (const t of tables) {
			try {
				map[t] = await getTableLabels(t);
			} catch {
				map[t] = { tableName: t, tags: [], group: null };
			}
		}
		labelsMap = map;
		try {
			allTags = await getAllTagsOp();
			allGroups = await getAllGroupsOp();
		} catch {
			allTags = [];
			allGroups = [];
		}
	}

	let filteredMetas = $derived(
		tableMetas.filter((meta) => {
			const labels = labelsMap[meta.name];
			if (!labels) return true;
			if (filterTag && !labels.tags.includes(filterTag)) return false;
			if (filterGroup && labels.group !== filterGroup) return false;
			return true;
		})
	);

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

	function startEdit(tableName: string) {
		const labels = labelsMap[tableName] ?? { tableName, tags: [], group: null };
		editingTable = tableName;
		editName = tableName;
		editTags = [...labels.tags];
		editGroup = labels.group ?? '';
		newTagInput = '';
	}

	async function saveEdit() {
		if (!editingTable) return;
		const trimmedName = editName.trim();
		const oldName = editingTable;
		if (!trimmedName) {
			editingTable = null;
			return;
		}
		try {
			if (trimmedName !== oldName) {
				await renameTable(oldName, trimmedName);
				onrename?.(oldName, trimmedName);
			}
			const tagsStr = editTags.filter((t) => t.trim() !== '').join(',');
			const group = editGroup.trim() || null;
			await saveTableLabels(trimmedName, tagsStr, group);
		} catch {
			// fallback
		}
		editingTable = null;
		if (trimmedName !== oldName) {
			try {
				tableMetas = await getAllTableMeta();
			} catch {
				// ignore
			}
		}
		await refreshLabels();
	}

	function cancelEdit() {
		editingTable = null;
	}

	async function handleDelete(tableName: string) {
		if (!ondelete) return;
		deletingTable = tableName;
		try {
			await ondelete(tableName);
			tableMetas = tableMetas.filter((m) => m.name !== tableName);
			delete labelsMap[tableName];
			labelsMap = { ...labelsMap };
		} catch {
			// error handled by parent
		} finally {
			deletingTable = null;
			confirmDeleteTable = null;
		}
	}

	function addTag() {
		const tag = newTagInput.trim();
		if (tag && !editTags.includes(tag)) {
			editTags = [...editTags, tag];
		}
		newTagInput = '';
	}

	function removeTag(tag: string) {
		editTags = editTags.filter((t) => t !== tag);
	}

	function handleTagKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			addTag();
		}
	}

	function handleCardKeydown(e: KeyboardEvent, tableName: string) {
		if (editingTable === tableName) return;
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			onselect(tableName);
		}
	}

	function toggleFilterTag(tag: string) {
		filterTag = filterTag === tag ? null : tag;
	}

	function toggleFilterGroup(group: string) {
		filterGroup = filterGroup === group ? null : group;
	}
</script>

<div class="overview">
	<div class="overview-header">
		<h2 class="overview-title">Tables</h2>
		<span class="tag tag-accent">{tables.length} table{tables.length !== 1 ? 's' : ''}</span>
	</div>

	{#if allTags.length > 0 || allGroups.length > 0}
		<div class="filter-bar">
			{#if allTags.length > 0}
				<div class="filter-group">
					<Tag size={12} class="filter-icon" />
					{#each allTags as tag}
						<button
							class="filter-chip"
							class:filter-chip-active={filterTag === tag}
							onclick={() => toggleFilterTag(tag)}
						>
							{tag}
						</button>
					{/each}
				</div>
			{/if}
			{#if allGroups.length > 0}
				<div class="filter-group">
					<FolderOpen size={12} class="filter-icon" />
					{#each allGroups as group}
						<button
							class="filter-chip"
							class:filter-chip-active={filterGroup === group}
							onclick={() => toggleFilterGroup(group)}
						>
							{group}
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<hr class="overview-divider" />

	{#if tables.length === 0}
		<div class="overview-empty">
			<span class="overview-empty-text">No tables yet. Connect a file to get started.</span>
		</div>
	{:else if loading}
		<div class="overview-loading">
			<span class="overview-empty-text">Loading metadata…</span>
		</div>
	{:else}
		<div class="card-grid">
			{#each filteredMetas as meta (meta.name)}
				{@const labels = labelsMap[meta.name]}
				{@const isEditing = editingTable === meta.name}
				<div
					class="card"
					class:card-editing={isEditing}
					role="button"
					tabindex="0"
					onkeydown={(e) => handleCardKeydown(e, meta.name)}
				>
					<div class="card-header">
						<h4 class="card-title">{meta.name}</h4>
						<div class="card-header-actions">
							{#if confirmDeleteTable === meta.name}
								<span class="confirm-delete">
									<span class="confirm-delete-text">Delete?</span>
									<button
										class="confirm-delete-btn confirm-delete-yes"
										onclick={(e) => { e.stopPropagation(); handleDelete(meta.name); }}
										disabled={deletingTable === meta.name}
									>Yes</button>
									<button
										class="confirm-delete-btn confirm-delete-no"
										onclick={(e) => { e.stopPropagation(); confirmDeleteTable = null; }}
										disabled={deletingTable === meta.name}
									>No</button>
								</span>
							{:else}
								<button
									class="card-edit-btn card-delete-btn"
									onclick={(e) => { e.stopPropagation(); confirmDeleteTable = meta.name; }}
									title="Delete table"
									disabled={deletingTable !== null}
								>
									<Trash2 size={14} />
								</button>
								<button
									class="card-edit-btn"
									onclick={(e) => { e.stopPropagation(); startEdit(meta.name); }}
									title="Edit tags & group"
									disabled={deletingTable !== null}
								>
									<SquarePen size={14} />
								</button>
							{/if}
						</div>
					</div>

					{#if isEditing}
						<div class="card-edit-area" onclick={(e) => e.stopPropagation()}>
							<div class="edit-field">
								<label class="edit-label">Name</label>
								<input
									type="text"
									class="input edit-input"
									bind:value={editName}
								/>
							</div>
							<div class="edit-field">
								<label class="edit-label">Group</label>
								<input
									type="text"
									class="input edit-input"
									placeholder="e.g. Sales, Finance..."
									bind:value={editGroup}
								/>
							</div>
							<div class="edit-field">
								<label class="edit-label">Tags</label>
								<div class="edit-tags">
									{#each editTags as tag}
										<span class="edit-tag-chip">
											{tag}
											<button class="edit-tag-remove" onclick={() => removeTag(tag)}>
												<X size={10} />
											</button>
										</span>
									{/each}
									<div class="edit-tag-input-row">
										<input
											type="text"
											class="input edit-tag-input"
											placeholder="Add tag…"
											bind:value={newTagInput}
											onkeydown={handleTagKeydown}
										/>
										<button class="edit-tag-add-btn" onclick={addTag}>
											<Plus size={12} />
										</button>
									</div>
								</div>
							</div>
							<div class="edit-actions">
								<button class="btn btn-secondary btn-sm" onclick={cancelEdit}>Cancel</button>
								<button class="btn btn-primary btn-sm" onclick={saveEdit}>Done</button>
							</div>
						</div>
					{:else}
						<hr class="card-divider" />

						<div class="card-body">
							<div class="stat">
								<span class="stat-value">{meta.columnCount}</span>
								<span class="stat-label">fields</span>
							</div>
							<div class="stat">
								<span class="stat-value">{formatNumber(meta.rowCount)}</span>
								<span class="stat-label">records</span>
							</div>
						</div>

						<div class="card-footer">
							{#if labels?.group}
								<span class="tag tag-group">{labels.group}</span>
							{/if}
							{#each (labels?.tags ?? []) as tag}
								<span class="tag tag-label">{tag}</span>
							{/each}
							<span class="tag tag-default">{inferSourceType(meta.name)}</span>
							<span class="tag tag-default">{meta.columnCount} cols</span>
							<span class="card-action" onclick={() => onselect(meta.name)}>Open →</span>
						</div>
					{/if}
				</div>
			{/each}
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

	.filter-bar {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-3);
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}

	.filter-icon {
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.filter-chip {
		padding: 2px var(--space-2);
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text-secondary);
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.filter-chip:hover {
		border-color: var(--color-accent-light);
		color: var(--color-accent-dark);
	}

	.filter-chip-active {
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		border-color: oklch(0.82 0.03 41);
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
		transition: color var(--duration-fast) ease;
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



	.card-editing {
		cursor: default;
	}

	.card-header {
		padding: var(--space-3);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.card-icon {
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.card-edit-btn {
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
		flex-shrink: 0;
	}

	.card-edit-btn:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.card-edit-btn-saving {
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		padding: 2px var(--space-2);
		color: var(--color-accent-dark);
		background: var(--color-accent-muted);
		border: 1px solid oklch(0.82 0.03 41);
		border-radius: var(--radius-xs);
		cursor: pointer;
	}

	.card-header-actions {
		display: flex;
		align-items: center;
		gap: 2px;
		flex-shrink: 0;
	}

	.card-delete-btn {
		color: var(--color-text-tertiary);
	}

	.card-delete-btn:hover {
		color: var(--color-danger);
		background: color-mix(in oklch, var(--color-danger) 10%, transparent);
	}

	.confirm-delete {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.confirm-delete-text {
		font-size: 9px;
		font-weight: 600;
		color: var(--color-danger);
		letter-spacing: 0.02em;
	}

	.confirm-delete-btn {
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
		cursor: pointer;
		border: 1px solid;
		transition: all var(--duration-fast) ease;
	}

	.confirm-delete-yes {
		background: var(--color-danger);
		color: white;
		border-color: var(--color-danger);
	}

	.confirm-delete-yes:hover {
		opacity: 0.85;
	}

	.confirm-delete-no {
		background: var(--color-surface);
		color: var(--color-text-secondary);
		border-color: var(--color-border);
	}

	.confirm-delete-no:hover {
		border-color: var(--color-text-tertiary);
	}

	.card-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
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

	.tag-label {
		background: var(--color-sage-50);
		color: var(--color-sage-700);
		border-color: var(--color-sage-200);
	}

	.tag-group {
		background: var(--color-copper-50);
		color: var(--color-copper-600);
		border-color: var(--color-copper-200);
	}

	.card-action {
		margin-left: auto;
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
		cursor: pointer;
		background: none;
		border: none;
		padding: 0;
		transition: color var(--duration-fast) ease;
	}

	.card-action:hover {
		color: var(--color-accent);
	}



	.card-edit-area {
		padding: var(--space-3);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		flex: 1;
	}

	.edit-field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.edit-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.edit-input {
		font-size: var(--text-xs);
		padding: var(--space-1) var(--space-2);
	}

	.edit-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
		align-items: center;
	}

	.edit-tag-chip {
		display: inline-flex;
		align-items: center;
		gap: 2px;
		padding: 2px var(--space-2);
		background: var(--color-sage-50);
		border: 1px solid var(--color-sage-200);
		border-radius: var(--radius-xs);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-sage-700);
	}

	.edit-tag-remove {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		color: var(--color-sage-400);
		cursor: pointer;
		padding: 0;
		line-height: 1;
	}

	.edit-tag-remove:hover {
		color: var(--color-danger);
	}

	.edit-tag-input-row {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.edit-tag-input {
		font-size: 9px;
		padding: 2px var(--space-2);
		width: 90px;
	}

	.edit-tag-add-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 2px;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		color: var(--color-text-secondary);
		cursor: pointer;
		border-radius: var(--radius-xs);
	}

	.edit-tag-add-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.edit-actions {
		display: flex;
		gap: var(--space-2);
		justify-content: flex-end;
	}
</style>