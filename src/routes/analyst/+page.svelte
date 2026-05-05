<script lang="ts">
	import { app } from '$lib/stores/app.svelte';
	import { analyst } from '$lib/stores/analyst.svelte';
	import { goto } from '$app/navigation';
	import { getTableData } from '$lib/db-helpers';
	import { onMount } from 'svelte';

	interface TableInfo {
		name: string;
		rowCount: number;
		columnCount: number;
		selected: boolean;
	}

	let tables = $state<TableInfo[]>([]);
	let loading = $state(true);

	onMount(async () => {
		if (app.tables.length === 0) {
			loading = false;
			return;
		}

		const infos: TableInfo[] = [];
		for (const name of app.tables) {
			try {
				const data = await getTableData(name, 1, 1);
				infos.push({
					name,
					rowCount: data.totalRows,
					columnCount: data.columns.length,
					selected: analyst.selectedTables.includes(name)
				});
			} catch {
				infos.push({ name, rowCount: 0, columnCount: 0, selected: false });
			}
		}
		tables = infos;
		loading = false;
	});

	function toggleTable(name: string) {
		tables = tables.map((t) =>
			t.name === name ? { ...t, selected: !t.selected } : t
		);
	}

	function selectAll() {
		tables = tables.map((t) => ({ ...t, selected: true }));
	}

	function deselectAll() {
		tables = tables.map((t) => ({ ...t, selected: false }));
	}

	function handleStart() {
		const selected = tables.filter((t) => t.selected).map((t) => t.name);
		if (selected.length === 0) return;
		analyst.selectedTables = selected;
		analyst.clear();
		goto('/analyst/chat');
	}

	let selectedCount = $derived(tables.filter((t) => t.selected).length);
</script>

<svelte:head>
	<title>Data Analyst — Data Monster</title>
</svelte:head>

<div class="flex flex-1 flex-col items-center justify-center gap-6 px-4">
	<div class="flex h-16 w-16 items-center justify-center rounded-xl bg-sage-600">
		<svg
			class="h-8 w-8 text-white"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
			stroke-width="2"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"
			/>
		</svg>
	</div>

	<div class="text-center">
		<h2 class="font-display text-2xl font-bold tracking-tight text-sand-800">
			Flue Data Analyst
		</h2>
		<p class="mt-1 text-sm text-sand-400">
			Select tables to load into the agent's context, then start analyzing.
		</p>
	</div>

	{#if loading}
		<div class="flex items-center gap-3 text-sand-400">
			<svg class="h-5 w-5 animate-spin" viewBox="0 0 24 24" fill="none">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
				<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			<span class="text-sm">Loading tables...</span>
		</div>
	{:else if tables.length === 0}
		<div class="rounded-lg border border-dashed border-sand-200 bg-sand-50 px-6 py-8 text-center">
			<p class="text-sm text-sand-400">No tables available. Connect data first.</p>
			<a
				href="/connect"
				class="mt-3 inline-block rounded-lg bg-sage-600 px-5 py-2 text-sm font-medium text-white shadow-sm transition-all hover:bg-sage-700"
			>
				Connect Data
			</a>
		</div>
	{:else}
		<div class="w-full max-w-lg">
			<div class="mb-3 flex items-center justify-between">
				<span class="text-xs font-medium text-sand-500"
					>{selectedCount} of {tables.length} selected</span
				>
				<div class="flex gap-2">
					<button
						onclick={selectAll}
						class="text-xs font-medium text-sage-600 transition-colors hover:text-sage-700"
						>Select all</button
					>
					<button
						onclick={deselectAll}
						class="text-xs font-medium text-sand-400 transition-colors hover:text-sand-500"
						>Deselect all</button
					>
				</div>
			</div>

			<div class="flex flex-col gap-2">
				{#each tables as table (table.name)}
					<button
						onclick={() => toggleTable(table.name)}
						class="flex items-center gap-3 rounded-lg border px-4 py-3 text-left shadow-sm transition-all {table.selected
							? 'border-sage-400 bg-sage-50 shadow-md ring-1 ring-sage-200'
							: 'border-sand-200 bg-white hover:border-sand-300 hover:shadow-md'}"
					>
						<div
							class="flex h-5 w-5 shrink-0 items-center justify-center rounded border-2 transition-colors {table.selected
								? 'border-sage-600 bg-sage-600'
								: 'border-sand-300 bg-white'}"
						>
							{#if table.selected}
								<svg class="h-3 w-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
									<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
								</svg>
							{/if}
						</div>
						<div class="min-w-0 flex-1">
							<p class="truncate text-sm font-medium text-sand-700">{table.name}</p>
							<p class="text-xs text-sand-400"
								>{table.columnCount} columns · {table.rowCount.toLocaleString()} rows</p
							>
						</div>
					</button>
				{/each}
			</div>

			<div class="mt-6 flex justify-center gap-3">
				<a
					href="/"
					class="rounded-lg border border-sand-200 bg-white px-5 py-2.5 text-sm font-medium text-sand-700 shadow-sm transition-all hover:bg-sand-50"
				>
					Back
				</a>
				<button
					onclick={handleStart}
					disabled={selectedCount === 0}
					class="rounded-lg bg-sage-600 px-5 py-2.5 text-sm font-medium text-white shadow-sm transition-all hover:bg-sage-700 disabled:cursor-not-allowed disabled:opacity-40"
				>
					Start Analysis ({selectedCount})
				</button>
			</div>
		</div>
	{/if}
</div>
