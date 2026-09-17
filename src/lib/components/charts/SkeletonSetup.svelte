<script lang="ts">
	/**
	 * In-chart setup surface for unconfigured chart blocks: inline grouped pickers
	 * (⭐ master items | source-table fields | linked-table fields | ✚ create master
	 * item) so the common path never needs the config drawer. Mutates the chart
	 * spec directly (same as the drawer does) — the runtime re-gates on needsSetup.
	 */
	import { availableItems, linkedTables } from '$lib/charts/relationships';
	import { getChartType } from '$lib/charts/registry';
	import { needsSetup } from '$lib/charts/registry';
	import {
		dimensionFromPick,
		measureFromPick
	} from '$lib/charts/pickers';
	import type { ChartBlockSpec } from '$lib/charts/spec-types';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import { Plus } from 'lucide-svelte';

	let {
		chart,
		schemas,
		items,
		relationships,
		onCreateMasterItem
	}: {
		chart: ChartBlockSpec;
		schemas: TableSchemas;
		items: MasterItem[];
		relationships: Relationship[];
		/** host saves the master item + refreshes the library; resolves to the new item id */
		onCreateMasterItem?: (kind: 'dimension' | 'measure', table: string, label: string, expr: string) => Promise<string>;
	} = $props();

	const def = $derived(getChartType(chart.type));
	const needDim = $derived(
		(def?.roles.find((r) => r.kind === 'dimension')?.min ?? 0) > chart.dimensions.length
	);
	const needMeas = $derived(
		(def?.roles.find((r) => r.kind === 'measure')?.min ?? 0) > chart.measures.length
	);
	const table = $derived(chart.source.table);
	const tableCols = $derived(schemas[table] ?? []);
	const linked = $derived(linkedTables(table, relationships).filter((t) => schemas[t]?.length));
	const usable = $derived(availableItems(table, items, relationships));

	let creating = $state<{ kind: 'dimension' | 'measure'; label: string; expr: string } | null>(null);
	let formError = $state('');
	let saving = $state(false);

	function onDimPick(e: Event) {
		const v = (e.target as HTMLSelectElement).value;
		if (!v) return;
		if (v === '__new') {
			creating = { kind: 'dimension', label: '', expr: '' };
			formError = '';
		} else chart.dimensions.push(dimensionFromPick(v, table));
		(e.target as HTMLSelectElement).value = '';
	}

	function onMeasPick(e: Event) {
		const v = (e.target as HTMLSelectElement).value;
		if (!v) return;
		if (v === '__new') {
			creating = { kind: 'measure', label: '', expr: '' };
			formError = '';
		} else chart.measures.push(measureFromPick(v, table));
		(e.target as HTMLSelectElement).value = '';
	}

	async function save() {
		if (!creating || !onCreateMasterItem || !creating.label.trim() || !creating.expr.trim()) return;
		saving = true;
		try {
			const id = await onCreateMasterItem(creating.kind, table, creating.label.trim(), creating.expr.trim());
			if (creating.kind === 'dimension') chart.dimensions.push({ ref: id });
			else chart.measures.push({ ref: id });
			creating = null;
			formError = '';
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}
</script>

<div class="flex flex-col items-center justify-center gap-3 py-6 px-4 rounded-lg border border-dashed border-zinc-300 bg-zinc-50/60">
	<div class="flex flex-col gap-1.5 w-full max-w-56" aria-hidden="true">
		<div class="h-2.5 rounded bg-zinc-200 animate-pulse w-1/3"></div>
		<div class="h-8 rounded bg-zinc-200/70 animate-pulse"></div>
		<div class="h-8 rounded bg-zinc-200/50 animate-pulse"></div>
	</div>
	<p class="text-xs text-zinc-500">Pick data to build the chart — right here.</p>
	<div class="flex flex-wrap gap-2 justify-center">
		{#if needDim}
			<select
				class="px-3 py-1.5 rounded-md text-xs font-medium bg-zinc-900 text-white border-0"
				aria-label="Add dimension"
				onchange={onDimPick}
			>
				<option value="" disabled selected><Plus size={12} /> dimension</option>
				{#each usable.filter((it) => it.kind === 'dimension') as it (it.id)}
					<option value={`ref:${it.id}`}>⭐ {it.label}</option>
				{/each}
				{#if tableCols.length}
					<optgroup label={table}>
						{#each tableCols as c (c)}<option value={`col:${table}:${c}`}>{c}</option>{/each}
					</optgroup>
				{/if}
				{#each linked as t (t)}
					<optgroup label={`${t} ⤳ linked`}>
						{#each schemas[t] ?? [] as c (c)}<option value={`col:${t}:${c}`}>{c}</option>{/each}
					</optgroup>
				{/each}
				<option value="__new">✚ Create master dimension…</option>
			</select>
		{/if}
		{#if needMeas}
			<select
				class="px-3 py-1.5 rounded-md text-xs font-medium bg-white border border-zinc-300 text-zinc-700"
				aria-label="Add measure"
				onchange={onMeasPick}
			>
				<option value="" disabled selected>＋ measure</option>
				{#each usable.filter((it) => it.kind === 'measure') as it (it.id)}
					<option value={`ref:${it.id}`}>⭐ {it.label}</option>
				{/each}
				{#if tableCols.length}
					<optgroup label={table}>
						{#each tableCols as c (c)}<option value={`field:${table}:${c}`}>sum({c})</option>{/each}
					</optgroup>
				{/if}
				{#each linked as t (t)}
					<optgroup label={`${t} ⤳ linked`}>
						{#each schemas[t] ?? [] as c (c)}<option value={`field:${t}:${c}`}>sum({c})</option>{/each}
					</optgroup>
				{/each}
				<option value="__new">✚ Create master measure…</option>
			</select>
		{/if}
	</div>
	{#if creating}
		<div class="w-full max-w-md space-y-1 border border-dashed border-zinc-300 rounded p-2 bg-white">
			<span class="text-xs text-zinc-500">New master {creating.kind} on {table}</span>
			<div class="flex gap-1">
				<input type="text" placeholder="label" class="flex-1 border border-zinc-300 rounded px-2 py-1 text-sm" bind:value={creating.label} />
				<input type="text" placeholder="expression (e.g. region / sum(amount))" class="flex-1 border border-zinc-300 rounded px-2 py-1 font-mono text-xs" bind:value={creating.expr} />
			</div>
			{#if formError}<p class="text-xs text-red-500">{formError}</p>{/if}
			<div class="flex gap-1 justify-end text-xs">
				<button class="px-2 py-1 rounded text-zinc-500 hover:text-zinc-900" onclick={() => (creating = null)}>Cancel</button>
				<button class="px-2 py-1 rounded bg-zinc-900 text-white disabled:opacity-40" disabled={saving} onclick={save}>
					{saving ? 'Saving…' : 'Save to library'}
				</button>
			</div>
		</div>
	{/if}
</div>
