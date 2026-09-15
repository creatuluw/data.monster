<script lang="ts">
	/** Table block renderer (FR-8): engine rows, compact page variant. */
	let {
		rows,
		columns,
		title = '',
		status = 'ok',
		error = ''
	}: {
		rows: Record<string, unknown>[];
		columns: string[];
		title?: string;
		status?: 'ok' | 'loading' | 'error' | 'empty';
		error?: string;
	} = $props();

	const cols = $derived(columns.length ? columns : rows[0] ? Object.keys(rows[0]) : []);
</script>

<div class="relative bg-white rounded-lg border border-zinc-200 p-6 mb-16">
	{#if title}
		<h2 class="text-lg font-semibold text-zinc-900 tracking-tight mb-4" style="font-family: var(--font-display)">{title}</h2>
	{/if}
	{#if status === 'loading'}
		<div class="py-16 text-center text-sm text-zinc-400">Loading…</div>
	{:else if status === 'error'}
		<div class="py-16 text-center text-sm text-red-500 break-words">{error}</div>
	{:else if rows.length === 0}
		<div class="py-16 text-center text-sm text-zinc-400">No data</div>
	{:else}
		<div class="overflow-auto max-h-96 rounded-md border border-zinc-200">
			<table class="w-full text-xs">
				<thead class="bg-zinc-50 sticky top-0">
					<tr>
						{#each cols as c (c)}
							<th class="text-left font-medium text-zinc-500 px-3 py-2 whitespace-nowrap border-b border-zinc-200">{c}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each rows as row, i (i)}
						<tr class="odd:bg-white even:bg-zinc-50/50">
							{#each cols as c (c)}
								<td class="px-3 py-1.5 whitespace-nowrap text-zinc-700 font-mono">{row[c] ?? ''}</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
