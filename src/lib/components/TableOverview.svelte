<script lang="ts">
	let {
		tables = [],
		onselect
	}: {
		tables: string[];
		onselect: (table: string) => void;
	} = $props();
</script>

<div class="flex flex-col gap-6">
	<div class="flex items-center gap-3">
		<h2 class="font-display text-lg font-bold text-sand-800">Tables</h2>
		<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">
			{tables.length} table{tables.length !== 1 ? 's' : ''}
		</span>
	</div>

	{#if tables.length === 0}
		<div class="rounded-lg border border-dashed border-sand-200 bg-sand-50 px-6 py-12 text-center">
			<p class="text-sm text-sand-400">No tables yet. Connect a file to get started.</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
			{#each tables as table}
				<button
					onclick={() => onselect(table)}
					class="flex items-center gap-3 rounded-lg border border-sand-200 bg-white px-4 py-3 text-left shadow-sm transition-all hover:border-sage-300 hover:bg-sage-50 hover:shadow-md"
				>
					<div class="flex h-9 w-9 shrink-0 items-center justify-center rounded-md bg-sage-100">
						<svg class="h-4.5 w-4.5 text-sage-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
							<path stroke-linecap="round" stroke-linejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H4.875c-.621 0-1.125.504-1.125 1.125v6.75M12 19.5h8.625c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H13.125c-.621 0-1.125.504-1.125 1.125v6.75" />
						</svg>
					</div>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium text-sand-700">{table}</p>
						<p class="text-xs text-sand-400">Click to view</p>
					</div>
				</button>
			{/each}
		</div>
	{/if}
</div>
