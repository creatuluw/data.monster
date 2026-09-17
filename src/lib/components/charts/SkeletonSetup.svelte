<script lang="ts">
	/**
	 * In-chart setup surface for unconfigured chart blocks: [Add dimension] /
	 * [Add measure] buttons open the RolePickerModal (searchahead + ✚ New) right
	 * on the card. Picks mutate the chart spec directly; the runtime re-gates on
	 * needsSetup.
	 */
	import { getChartType } from '$lib/charts/registry';
	import RolePickerModal from './RolePickerModal.svelte';
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

	let openKind = $state<'dimension' | 'measure' | null>(null);
</script>

<div class="flex flex-col items-center justify-center gap-3 py-6 px-4 rounded-lg border border-dashed border-zinc-300 bg-zinc-50/60">
	<div class="flex flex-col gap-1.5 w-full max-w-56" aria-hidden="true">
		<div class="h-2.5 rounded bg-zinc-200 animate-pulse w-1/3"></div>
		<div class="h-8 rounded bg-zinc-200/70 animate-pulse"></div>
		<div class="h-8 rounded bg-zinc-200/50 animate-pulse"></div>
	</div>
	<p class="text-xs text-zinc-500">Pick data to build the chart.</p>
	<div class="flex flex-wrap gap-2 justify-center">
		{#if needDim}
			<button
				class="px-3 py-1.5 rounded-md text-xs font-medium bg-zinc-900 text-white hover:bg-zinc-700 inline-flex items-center gap-1"
				onclick={(e) => { e.stopPropagation(); openKind = 'dimension'; }}
			>
				<Plus size={12} /> Add dimension
			</button>
		{/if}
		{#if needMeas}
			<button
				class="px-3 py-1.5 rounded-md text-xs font-medium bg-white border border-zinc-300 text-zinc-700 hover:bg-zinc-100 inline-flex items-center gap-1"
				onclick={(e) => { e.stopPropagation(); openKind = 'measure'; }}
			>
				<Plus size={12} /> Add measure
			</button>
		{/if}
	</div>
</div>

{#if openKind}
	<RolePickerModal
		kind={openKind}
		{chart}
		{schemas}
		{items}
		{relationships}
		{onCreateMasterItem}
		onClose={() => (openKind = null)}
	/>
{/if}
