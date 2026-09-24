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

<div class="relative bg-surface rounded-lg border border-border p-6">
	{#if title}
		<h2 class="text-lg font-semibold text-text tracking-tight mb-4" style="font-family: var(--font-display)">{title}</h2>
	{/if}
	{#if status === 'loading'}
		<div class="py-16 text-center text-sm text-text-tertiary">Loading…</div>
	{:else if status === 'error'}
		<div class="py-16 text-center text-sm text-red-500 break-words">{error}</div>
	{:else if rows.length === 0}
		<div class="py-16 text-center text-sm text-text-tertiary">No data</div>
	{:else}
		<div class="overflow-auto max-h-96 rounded-md border border-border">
			<table class="w-full text-xs">
				<thead class="bg-surface-sunken sticky top-0">
					<tr>
						{#each cols as c (c)}
							<th class="text-left font-medium text-text-tertiary px-3 py-2 whitespace-nowrap border-b border-border">{c}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each rows as row, i (i)}
						<tr class="odd:bg-surface even:bg-surface-sunken/50">
							{#each cols as c (c)}
								<td class="px-3 py-1.5 whitespace-nowrap text-text-secondary font-mono">{row[c] ?? ''}</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
