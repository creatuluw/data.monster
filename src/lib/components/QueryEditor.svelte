<script lang="ts">
	import type { PagedQueryResult } from '$lib/db-operations';

	let {
		onrun,
		onsaveastable,
		result = null,
		loading = false,
		error = '',
		queryTime = 0,
		initialSql = '',
		suggestedTableName = '',
		pageSize = 10
	}: {
		onrun: (sql: string, page: number, pageSize: number) => void;
		onsaveastable?: (sql: string, tableName: string) => void;
		result?: PagedQueryResult | null;
		loading?: boolean;
		error?: string;
		queryTime?: number;
		initialSql?: string;
		suggestedTableName?: string;
		pageSize?: number;
	} = $props();

	let sql = $state('');
	let saveTableName = $state('');

	$effect(() => {
		if (initialSql) {
			sql = initialSql;
			if (suggestedTableName && !saveTableName) {
				saveTableName = suggestedTableName;
			}
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
			e.preventDefault();
			if (sql.trim()) onrun(sql, 1, pageSize);
		}
	}

	function handleTab(e: KeyboardEvent) {
		if (e.key === 'Tab') {
			e.preventDefault();
			const el = e.target as HTMLTextAreaElement;
			const start = el.selectionStart;
			const end = el.selectionEnd;
			sql = sql.substring(0, start) + '  ' + sql.substring(end);
			requestAnimationFrame(() => {
				el.selectionStart = el.selectionEnd = start + 2;
			});
		}
	}

	function handleIngest() {
		if (onsaveastable && saveTableName.trim()) {
			onsaveastable(sql, saveTableName);
		}
	}

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}
</script>

<div class="editor">
	<div class="editor-header">
		<h2 class="editor-title">Query editor</h2>
		<button
			onclick={() => sql.trim() && onrun(sql, 1, pageSize)}
			disabled={!sql.trim() || loading}
			class="btn btn-primary"
		>
			{loading ? 'Running...' : 'Run'}
		</button>
	</div>

	<textarea
		bind:value={sql}
		onkeydown={(e) => {
			handleKeydown(e);
			handleTab(e);
		}}
		placeholder="SELECT * FROM ..."
		rows="6"
		class="editor-textarea"
	></textarea>
	<span class="editor-hint">Ctrl+Enter to run</span>

	{#if loading}
		<div class="editor-loading">
			<svg class="spinner" viewBox="0 0 24 24" fill="none" style="color: var(--color-accent);">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
				<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			<span class="editor-hint">Running query...</span>
		</div>
	{/if}

	{#if error}
		<div class="editor-error">{error}</div>
	{/if}

	{#if !loading && result && result.columns.length > 0}
		<div class="editor-results">
			<div class="results-header">
				<h3 class="editor-title" style="font-size: var(--text-sm);">
					{result.isMutation ? 'Result' : 'Preview'}
				</h3>
				{#if queryTime > 0}
					<span class="tag tag-default">{queryTime.toFixed(0)}ms</span>
				{/if}
				<span class="tag tag-accent">
					{result.rows.length} of {result.totalRows.toLocaleString()} rows
				</span>
			</div>

			<div class="results-table-wrap">
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
			</div>

			{#if !result.isMutation && result.totalPages > 1}
				<div class="pagination">
					<button
						onclick={() => onrun(sql, result.page - 1, pageSize)}
						disabled={result.page <= 1}
						class="btn btn-ghost btn-sm"
					>
						&larr; prev
					</button>
					<span class="editor-hint">Page {result.page} of {result.totalPages}</span>
					<button
						onclick={() => onrun(sql, result.page + 1, pageSize)}
						disabled={result.page >= result.totalPages}
						class="btn btn-ghost btn-sm"
					>
						next &rarr;
					</button>
				</div>
			{/if}
		</div>
	{/if}

	{#if onsaveastable}
		<div class="ingest-row">
			<div class="field" style="flex-direction: row; align-items: center; gap: var(--space-3);">
				<label for="save-table-name" class="field-label" style="margin: 0; font-size: var(--text-xs);">Table name:</label>
				<input
					id="save-table-name"
					type="text"
					bind:value={saveTableName}
					placeholder="table_name"
					class="input"
				/>
			</div>
			<button
				onclick={handleIngest}
				disabled={!saveTableName.trim()}
				class="btn btn-primary"
			>
				Ingest as table &rarr;
			</button>
		</div>
	{/if}
</div>

<style>
	.editor {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.editor-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.editor-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.editor-textarea {
		width: 100%;
		resize: none;
		padding: var(--space-4);
		border: 1px solid var(--color-border-strong);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.editor-textarea::placeholder {
		color: var(--color-text-tertiary);
	}

	.editor-textarea:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.editor-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.editor-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-16) 0;
		gap: var(--space-3);
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.editor-error {
		padding: var(--space-3) var(--space-4);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
		font-size: var(--text-sm);
		color: oklch(0.38 0.12 22);
	}

	.results-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-2);
	}

	.results-table-wrap {
		overflow-x: auto;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
	}

	.ingest-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
	}
</style>
