<script lang="ts">
	import type { PagedQueryResult } from '$lib/db-helpers';

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

	// Sync with parent's initialSql changes
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
		if (value === null || value === undefined) return '—';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Editor -->
	<div>
		<div class="flex items-center justify-between">
			<h2 class="font-display text-sm font-semibold text-sand-700">Query editor</h2>
			<button
				onclick={() => sql.trim() && onrun(sql, 1, pageSize)}
				disabled={!sql.trim() || loading}
				class="rounded-md bg-sage-600 px-4 py-1.5 text-sm font-medium text-white shadow-sm transition-all hover:bg-sage-700 disabled:cursor-not-allowed disabled:opacity-40"
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
			class="mt-2 w-full resize-none rounded-lg border border-sand-200 bg-white px-4 py-3 text-sm leading-relaxed text-sand-800 shadow-sm transition-colors placeholder:text-sand-300 focus:border-sage-400 focus:outline-none focus:ring-2 focus:ring-sage-200"
		></textarea>
		<span class="text-xs text-sand-400">Ctrl+Enter to run</span>
	</div>

	<!-- Loading spinner -->
	{#if loading}
		<div class="flex items-center justify-center rounded-lg border border-sand-200 bg-white py-16">
			<svg class="h-5 w-5 animate-spin text-sage-500" viewBox="0 0 24 24" fill="none">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
				<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			<span class="ml-2 text-sm text-sand-400">Running query...</span>
		</div>
	{/if}

	<!-- Error -->
	{#if error}
		<div class="rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700">{error}</div>
	{/if}

	<!-- Results -->
	{#if !loading && result && result.columns.length > 0}
		<div>
			<div class="mb-2 flex items-center gap-2">
				<h3 class="font-display text-sm font-semibold text-sand-700">{result.isMutation ? 'Result' : 'Preview'}</h3>
				{#if queryTime > 0}
					<span class="text-xs text-sand-400">{queryTime.toFixed(0)}ms</span>
				{/if}
				{#if !result.isMutation}
					<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">
						{result.rows.length} of {result.totalRows.toLocaleString()} rows
					</span>
				{:else}
					<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">
						{result.rows.length} rows
					</span>
				{/if}
			</div>
			<div class="overflow-x-auto rounded-lg border border-sand-200 bg-white">
				<table class="w-full text-left text-sm">
					<thead>
						<tr class="border-b border-sand-200 bg-sand-50">
							{#each result.columns as col}
								<th class="whitespace-nowrap px-4 py-3 font-semibold text-sand-600">{col}</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each result.rows as row}
							<tr class="border-b border-sand-100 transition-colors hover:bg-sage-50/40">
								{#each result.columns as col}
									<td class="max-w-[300px] truncate px-4 py-2.5 text-sand-700">{formatCell(row[col])}</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<!-- Pagination -->
			{#if !result.isMutation && result.totalPages > 1}
				<div class="mt-2 flex items-center justify-center gap-3">
					<button
						onclick={() => onrun(sql, result.page - 1, pageSize)}
						disabled={result.page <= 1}
						class="rounded-md px-3 py-1 text-sm font-medium text-sand-600 transition-colors hover:bg-sand-100 disabled:cursor-not-allowed disabled:opacity-40"
					>
						← prev
					</button>
					<span class="text-xs text-sand-400">Page {result.page} of {result.totalPages}</span>
					<button
						onclick={() => onrun(sql, result.page + 1, pageSize)}
						disabled={result.page >= result.totalPages}
						class="rounded-md px-3 py-1 text-sm font-medium text-sand-600 transition-colors hover:bg-sand-100 disabled:cursor-not-allowed disabled:opacity-40"
					>
						next →
					</button>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Ingest / Save as table -->
	{#if onsaveastable}
		<div class="flex items-center gap-3 border-t border-sand-200 pt-4">
			<div class="flex items-center gap-2">
				<label for="save-table-name" class="text-sm text-sand-400">Table name:</label>
				<input
					id="save-table-name"
					type="text"
					bind:value={saveTableName}
					placeholder="table_name"
					class="rounded-md border border-sand-200 bg-white px-3 py-1.5 text-sm text-sand-800 outline-none focus:border-sage-400 focus:ring-2 focus:ring-sage-200"
				/>
			</div>
			<button
				onclick={handleIngest}
				disabled={!saveTableName.trim()}
				class="rounded-lg bg-copper-400 px-4 py-2 text-sm font-medium text-white shadow-sm transition-all hover:bg-copper-500 disabled:cursor-not-allowed disabled:opacity-40"
			>
				Ingest as table →
			</button>
		</div>
	{/if}
</div>
