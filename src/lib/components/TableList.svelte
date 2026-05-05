<script lang="ts">
	import type { QueryResult } from '$lib/db-helpers';

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

<div class="flex h-full flex-col">
	<!-- Connect button -->
	<button
		onclick={onconnect}
		class="mb-4 flex w-full items-center justify-center gap-2 rounded-lg border border-dashed border-sand-200 bg-sand-50 px-3 py-2 text-sm font-medium text-sand-500 transition-colors hover:border-sage-300 hover:bg-sage-50 hover:text-sage-700"
	>
		<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
			<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
		</svg>
		Connect
	</button>

	<!-- Table list -->
	<h2 class="mb-3 text-[11px] font-semibold uppercase tracking-wider text-sand-400">Tables</h2>

	{#if tables.length === 0}
		<div class="rounded-lg border border-dashed border-sand-200 bg-sand-50 px-4 py-6 text-center">
			<p class="text-sm text-sand-400">No tables yet.</p>
			<p class="text-xs text-sand-300">Load a file to get started.</p>
		</div>
	{:else}
		<nav class="flex flex-1 flex-col gap-0.5 overflow-y-auto">
			{#each tables as table}
				<button
					onclick={() => onselect(table)}
					class="flex items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors {selected === table
						? 'bg-sage-100 font-medium text-sage-800'
						: 'text-sand-600 hover:bg-sand-100'}"
				>
					<svg class="h-4 w-4 shrink-0 {selected === table ? 'text-sage-600' : 'text-sand-300'}" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
						<path stroke-linecap="round" stroke-linejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H4.875c-.621 0-1.125.504-1.125 1.125v6.75M12 19.5h8.625c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H13.125c-.621 0-1.125.504-1.125 1.125v6.75" />
					</svg>
					<span class="truncate">{table}</span>
				</button>
			{/each}
		</nav>
	{/if}

	<span class="mt-4 text-xs text-sand-400">{tables.length} table{tables.length !== 1 ? 's' : ''}</span>
</div>
