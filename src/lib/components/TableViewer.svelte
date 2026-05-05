<script lang="ts">
	import type { QueryResult } from '$lib/db-helpers';

	let {
		result,
		tableName = '',
		page = 1,
		totalPages = 1,
		onpagechange,
		onquerytable,
		oneditsource,
		ondroptable,
		hasRecipe = false
	}: {
		result: QueryResult & { totalRows: number };
		tableName?: string;
		page?: number;
		totalPages?: number;
		onpagechange?: (page: number) => void;
		onquerytable?: () => void;
		oneditsource?: () => void;
		ondroptable?: () => void;
		hasRecipe?: boolean;
	} = $props();

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '—';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Header -->
	<div class="flex items-center gap-3">
		<h2 class="font-display text-lg font-bold text-sand-800">{tableName}</h2>
		<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">
			{result.totalRows.toLocaleString()} rows
		</span>
	</div>

	<!-- Data table -->
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
		<div class="border-t border-sand-200 bg-sand-50 px-4 py-2 text-xs text-sand-400">
			{result.rows.length} of {result.totalRows.toLocaleString()} rows
		</div>
	</div>

	<!-- Pagination -->
	{#if totalPages > 1}
		<div class="flex items-center justify-center gap-3 text-sm text-sand-600">
			<button
				onclick={() => onpagechange?.(page - 1)}
				disabled={page <= 1}
				class="rounded-md px-3 py-1 text-sm font-medium text-sand-600 transition-colors hover:bg-sand-100 disabled:cursor-not-allowed disabled:opacity-40"
			>
				← prev
			</button>
			<span class="text-xs text-sand-400">Page {page} of {totalPages}</span>
			<button
				onclick={() => onpagechange?.(page + 1)}
				disabled={page >= totalPages}
				class="rounded-md px-3 py-1 text-sm font-medium text-sand-600 transition-colors hover:bg-sand-100 disabled:cursor-not-allowed disabled:opacity-40"
			>
				next →
			</button>
		</div>
	{/if}

	<!-- Actions -->
	<div class="flex items-center gap-3 border-t border-sand-200 pt-4">
		{#if onquerytable}
			<button
				onclick={onquerytable}
				class="rounded-md bg-sage-600 px-4 py-1.5 text-sm font-medium text-white shadow-sm transition-all hover:bg-sage-700"
			>
				Query this table
			</button>
		{/if}
		{#if hasRecipe && oneditsource}
			<button
				onclick={oneditsource}
				class="rounded-md px-4 py-1.5 text-sm font-medium text-sand-600 transition-colors hover:bg-sand-100"
			>
				Edit source
			</button>
		{/if}
		{#if ondroptable}
			<button
				onclick={ondroptable}
				class="rounded-md px-4 py-1.5 text-sm font-medium text-red-600 transition-colors hover:bg-red-50"
			>
				Drop table
			</button>
		{/if}
	</div>
</div>
