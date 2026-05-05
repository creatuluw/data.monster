<script lang="ts">
	import '../app.css';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import TableList from '$lib/components/TableList.svelte';
	import { onMount } from 'svelte';

	let { children } = $props();

	onMount(() => {
		if (!app.dbReady) {
			app.init();
		}
	});

	function handleSelectTable(table: string) {
		goto(`/table/${encodeURIComponent(table)}`);
	}

	function handleConnect() {
		goto('/connect');
	}

	let selectedTable = $derived.by(() => {
		if (!$page.url.pathname.startsWith('/table/')) return '';
		return decodeURIComponent($page.url.pathname.slice('/table/'.length));
	});

	let isUiPage = $derived($page.url.pathname.startsWith('/ui'));
</script>

{#if isUiPage}
	{@render children()}
{:else}
<div class="flex h-screen flex-col">
	<header class="flex shrink-0 items-center justify-between border-b border-sand-200 bg-white px-6 py-4">
		<a href="/" class="flex items-center gap-3">
			<div class="flex h-8 w-8 items-center justify-center rounded-md bg-sage-600">
				<svg class="h-4 w-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M4.745 3A23.933 23.933 0 0 0 3 12c0 3.183.62 6.22 1.745 9M19.5 3c.967 2.78 1.5 5.817 1.5 9s-.533 6.22-1.5 9M8.25 8.885l1.444-.89a.75.75 0 0 1 1.105.402l2.402 7.206a.75.75 0 0 0 1.105.401l1.444-.889" />
				</svg>
			</div>
			<h1 class="font-display text-lg font-bold tracking-tight text-sand-800">Data Monster</h1>
		</a>
		<div class="flex items-center gap-3">
			<a href="/tables" class="cursor-pointer text-xs text-sand-400 transition-colors hover:text-sage-600">
				{app.tables.length} table{app.tables.length !== 1 ? 's' : ''}
			</a>
			{#if app.opfsStatus === 'memory'}
				<span class="rounded-full bg-copper-100 px-2 py-0.5 text-[11px] font-medium text-copper-700">In-Memory</span>
			{:else if app.opfsStatus === 'persistent'}
				<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">Persistent</span>
			{/if}
		</div>
	</header>

	{#if !app.dbReady}
		<div class="flex flex-1 items-center justify-center">
			<div class="flex items-center gap-3 text-sand-400">
				<svg class="h-5 w-5 animate-spin" viewBox="0 0 24 24" fill="none">
					<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
					<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
				</svg>
				<span class="text-sm">Initializing database...</span>
			</div>
		</div>
	{:else}
		<div class="flex flex-1 overflow-hidden">
			<main class="flex-1 overflow-y-auto px-6 py-4">
				{#if app.globalError}
					<div class="mb-4 rounded-lg bg-red-50 px-4 py-3 text-sm text-red-700">{app.globalError}</div>
				{/if}

				{@render children()}
			</main>
		</div>
	{/if}
</div>
{/if}
