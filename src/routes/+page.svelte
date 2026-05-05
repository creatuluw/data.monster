<script lang="ts">
	import { app } from '$lib/stores/app.svelte';
</script>

<svelte:head>
	<title>Data Monster</title>
</svelte:head>

<div class="flex flex-1 flex-col items-center justify-center gap-6">
	<div class="flex h-16 w-16 items-center justify-center rounded-xl bg-sage-600">
		<svg class="h-8 w-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
			<path stroke-linecap="round" stroke-linejoin="round" d="M4.745 3A23.933 23.933 0 0 0 3 12c0 3.183.62 6.22 1.745 9M19.5 3c.967 2.78 1.5 5.817 1.5 9s-.533 6.22-1.5 9M8.25 8.885l1.444-.89a.75.75 0 0 1 1.105.402l2.402 7.206a.75.75 0 0 0 1.105.401l1.444-.889" />
		</svg>
	</div>
	<h2 class="font-display text-2xl font-bold tracking-tight text-sand-800">Data Monster</h2>
	<p class="text-sm text-sand-400">Connect data, query, explore.</p>
	<div class="flex items-center gap-0">
		<a
			href="/connect"
			class="flex items-center justify-center gap-2 rounded-l-lg border border-r-0 border-sand-200 bg-sage-600 px-5 py-2.5 text-sm font-medium text-white shadow-sm transition-all hover:bg-sage-700"
		>
			<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
			</svg>
			Connect
		</a>
		{#if app.tables.length > 0}
			<a
				href="/query"
				class="flex items-center justify-center gap-2 border border-sand-200 bg-white px-5 py-2.5 text-sm font-medium text-sand-700 shadow-sm transition-all hover:bg-sand-50"
			>
				<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5" />
				</svg>
				Query
			</a>
			<a
				href="/tables"
				class="flex items-center justify-center gap-2 border border-sand-200 bg-white px-5 py-2.5 text-sm font-medium text-sand-700 shadow-sm transition-all hover:bg-sand-50"
			>
				<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H4.875c-.621 0-1.125.504-1.125 1.125v6.75M12 19.5h8.625c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H13.125c-.621 0-1.125.504-1.125 1.125v6.75" />
				</svg>
				Tables ({app.tables.length})
			</a>
			<a
				href="/analyst"
				class="flex items-center justify-center gap-2 rounded-r-lg border border-sage-200 bg-sage-50 px-5 py-2.5 text-sm font-medium text-sage-700 shadow-sm transition-all hover:bg-sage-100 hover:shadow-md"
			>
				<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5" />
				</svg>
				Data Analyst
			</a>
		{:else}
			<span class="flex items-center justify-center gap-2 rounded-r-lg border border-sand-200 bg-sand-50 px-5 py-2.5 text-sm font-medium text-sand-300">
				Connect data to unlock Query, Tables & Analyst
			</span>
		{/if}
	</div>
</div>
