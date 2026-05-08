<script lang="ts">
	let {
		tables = [],
		selected = '',
		onselect,
		onconnect
	}: {
		tables: string[];
		selected?: string;
		onselect: (table: string) => void;
		onconnect: () => void;
	} = $props();
</script>

<div class="table-list">
	<button onclick={onconnect} class="connect-btn">
		<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
			<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
		</svg>
		Connect
	</button>

	<span class="list-label">Data</span>

	{#if tables.length === 0}
		<div class="empty-state">
			<span class="empty-text">No tables yet.</span>
			<span class="empty-hint">Load a file to get started.</span>
		</div>
	{:else}
		<nav class="list-nav">
			{#each tables as table}
				<button
					onclick={() => onselect(table)}
					class="list-item {selected === table ? 'is-selected' : ''}"
				>
					<span class="list-icon">&#x25A0;</span>
					<span class="list-name">{table}</span>
				</button>
			{/each}
		</nav>
	{/if}

	<span class="list-count">{tables.length} table{tables.length !== 1 ? 's' : ''}</span>
</div>

<style>
	.table-list {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.connect-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-2) var(--space-3);
		margin-bottom: var(--space-4);
		border: 1px dashed var(--color-border);
		background: var(--color-surface-raised);
		border-radius: var(--radius-xs);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-secondary);
		cursor: pointer;
		transition: border-color var(--duration-fast) ease, color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.connect-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-accent-dark);
		background: var(--color-accent-muted);
	}

	.list-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-3);
	}

	.empty-state {
		padding: var(--space-6) var(--space-4);
		text-align: center;
		border: 1px dashed var(--color-border);
		background: var(--color-surface-raised);
	}

	.empty-text {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.empty-hint {
		font-size: 9px;
		color: var(--color-text-tertiary);
	}

	.list-nav {
		flex: 1;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: none;
		background: transparent;
		text-align: left;
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		cursor: pointer;
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
		border-radius: var(--radius-xs);
	}

	.list-item:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.list-item.is-selected {
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		font-weight: 600;
	}

	.list-icon {
		font-size: 7px;
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.list-item:not(.is-selected) .list-icon {
		color: var(--color-text-tertiary);
	}

	.list-name {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.list-count {
		margin-top: var(--space-4);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}
</style>
