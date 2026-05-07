<script lang="ts">
	let {
		result,
		tableName = '',
		page = 1,
		totalPages = 1,
		onpagechange,
		onquerytable,
		ondroptable
	}: {
		result: { columns: string[]; rows: Record<string, unknown>[]; totalRows: number };
		tableName?: string;
		page?: number;
		totalPages?: number;
		onpagechange?: (page: number) => void;
		onquerytable?: () => void;
		ondroptable?: () => void;
	} = $props();

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}
</script>

<div class="viewer">
	<div class="viewer-header">
		<h2 class="viewer-title">{tableName}</h2>
		<span class="tag tag-accent">{result.totalRows.toLocaleString()} rows</span>
	</div>

	<div class="viewer-table-wrap">
		<table class="data-table">
			<thead>
				<tr>
					{#each result.columns as col}
						<th>{col}</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each result.rows as row}
					<tr>
						{#each result.columns as col}
							<td>{formatCell(row[col])}</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
		<div class="viewer-footer-bar">
			{result.rows.length} of {result.totalRows.toLocaleString()} rows
		</div>
	</div>

	{#if totalPages > 1}
		<div class="pagination">
			<button
				onclick={() => onpagechange?.(page - 1)}
				disabled={page <= 1}
				class="btn btn-ghost btn-sm"
			>
				&larr; prev
			</button>
			<span class="page-info">Page {page} of {totalPages}</span>
			<button
				onclick={() => onpagechange?.(page + 1)}
				disabled={page >= totalPages}
				class="btn btn-ghost btn-sm"
			>
				next &rarr;
			</button>
		</div>
	{/if}

	<div class="viewer-actions">
		{#if onquerytable}
			<button onclick={onquerytable} class="btn btn-primary">Query this table</button>
		{/if}
		{#if ondroptable}
			<button onclick={ondroptable} class="btn btn-danger btn-sm">Drop table</button>
		{/if}
	</div>
</div>

<style>
	.viewer {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: var(--space-6);
	}

	.viewer-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.viewer-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.viewer-table-wrap {
		overflow-x: auto;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.viewer-footer-bar {
		padding: var(--space-2) var(--space-4);
		background: var(--color-surface-raised);
		border-top: 1px solid var(--color-border);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
	}

	.page-info {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.viewer-actions {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
	}
</style>
