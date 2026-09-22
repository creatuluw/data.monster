<script lang="ts">
	/**
	 * Role picker modal (new-page-modal pattern): searchable list over ⭐ master
	 * items + source-table fields + linked-table fields, plus a + New form that
	 * creates a master item on the spot. Applying pushes into the chart spec.
	 */
	import { availableItems, linkedTables } from '$lib/charts/relationships';
	import { dimensionFromPick, measureFromPick, roleLabels } from '$lib/charts/pickers';
	import type { ChartBlockSpec } from '$lib/charts/spec-types';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import { Plus, Search, X, SquareArrowOutUpRight } from 'lucide-svelte';
	import TextInput from './controls/TextInput.svelte';
	import Btn from './controls/Btn.svelte';
	import ExprEditor from './ExprEditor.svelte';

	let {
		kind,
		chart,
		schemas,
		items,
		relationships,
		onCreateMasterItem,
		onExternalCreate,
		onClose
	}: {
		kind: 'dimension' | 'measure';
		chart: ChartBlockSpec;
		schemas: TableSchemas;
		items: MasterItem[];
		relationships: Relationship[];
		onCreateMasterItem?: (kind: 'dimension' | 'measure', table: string, label: string, expr: string) => Promise<string>;
		/** open the /data master-item section for full-panel creation — host navigates + returns */
		onExternalCreate?: (kind: 'dimension' | 'measure', table: string) => void;
		onClose: () => void;
	} = $props();

	const table = $derived(chart.source.table);
	const noun = $derived(kind === 'dimension' ? 'dimension' : 'measure');

	type Entry = { value: string; label: string; meta: string; group: string };
	const entries = $derived.by(() => {
		const out: Entry[] = [];
		for (const it of availableItems(table, items, relationships).filter((i) => i.kind === kind)) {
			out.push({ value: `ref:${it.id}`, label: `⭐ ${it.label}`, meta: it.table, group: 'Master items' });
		}
		const field = (t: string, c: string) =>
			kind === 'dimension'
				? { value: `col:${t}:${c}`, label: c, meta: t, group: t === table ? table : `${t} ⤳ linked` }
				: { value: `field:${t}:${c}`, label: `sum(${c})`, meta: t, group: t === table ? table : `${t} ⤳ linked` };
		for (const c of schemas[table] ?? []) out.push(field(table, c));
		for (const t of linkedTables(table, relationships)) {
			if (!schemas[t]?.length) continue;
			for (const c of schemas[t]) out.push(field(t, c));
		}
		return out;
	});

	let query = $state('');
	const filtered = $derived(
		entries.filter((e) => {
			if (!query.trim()) return true;
			const q = query.toLowerCase();
			return e.label.toLowerCase().includes(q) || e.meta.toLowerCase().includes(q);
		})
	);
	const grouped = $derived.by(() => {
		const map = new Map<string, Entry[]>();
		for (const e of filtered) map.set(e.group, [...(map.get(e.group) ?? []), e]);
		return [...map.entries()];
	});

	function apply(value: string) {
		if (kind === 'dimension') chart.dimensions.push(dimensionFromPick(value, table));
		else chart.measures.push(measureFromPick(value, table));
		onClose();
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onClose();
		else if (e.key === 'Enter' && filtered[0]) apply(filtered[0].value);
	}

	let creating = $state(false);
	let newLabel = $state('');
	let newExpr = $state('');
	let formError = $state('');
	let saving = $state(false);

	async function create() {
		if (!onCreateMasterItem || !newLabel.trim() || !newExpr.trim()) return;
		saving = true;
		try {
			const id = await onCreateMasterItem(kind, table, newLabel.trim(), newExpr.trim());
			if (kind === 'dimension') chart.dimensions.push({ ref: id });
			else chart.measures.push({ ref: id });
			onClose();
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to save';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:window onkeydown={onKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="fixed inset-0 z-50 bg-black/40 flex items-center justify-center p-6"
	onclick={onClose}
	onkeydown={onKeydown}
	role="presentation"
>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="bg-white rounded-xl shadow-xl w-full max-w-md p-6 space-y-4"
		onclick={(e) => e.stopPropagation()}
		onkeydown={onKeydown}
		role="presentation"
	>
		<div class="flex items-center justify-between">
			<h2 class="text-lg font-semibold text-zinc-900" style="font-family: var(--font-display)">Add {noun}</h2>
			<button class="text-zinc-400 hover:text-zinc-900" onclick={onClose} title="Close"><X size={16} /></button>
		</div>
		<div class="flex flex-wrap gap-1.5">
			{#if roleLabels(chart, 'dimension', items).length === 0 && roleLabels(chart, 'measure', items).length === 0}
				<span class="text-xs text-zinc-400">Nothing added yet</span>
			{:else}
				{#each roleLabels(chart, 'dimension', items) as d (d.label + d.meta)}
				<span class="px-2 py-0.5 rounded-full text-[11px] font-medium bg-zinc-900/90 text-white" title="dimension">{d.label}</span>
				{/each}
			{#each roleLabels(chart, 'measure', items) as m (m.label + m.meta)}
				<span class="px-2 py-0.5 rounded-full text-[11px] font-medium bg-white border border-zinc-300 text-zinc-700" title="measure">{m.label}</span>
				{/each}
			{/if}
		</div>

		{#if !creating}
			<label class="block space-y-1">
				<span class="text-xs" style="color: var(--color-text-secondary)">Search {noun}s — master items, fields of {table}, linked tables</span>
				<div class="flex items-center gap-2" style="border: 1px solid var(--color-border-strong); border-radius: var(--radius-sm); padding: 0 var(--space-2); height: 30px; background: var(--color-surface)">
					<Search size={14} style="color: var(--color-text-tertiary)" />
					<input type="text" autofocus class="flex-1 outline-none text-sm" style="background: none; border: none; color: var(--color-text)" placeholder="Type to filter…" bind:value={query} />
				</div>
			</label>
			<div class="max-h-64 overflow-auto rounded-lg border border-zinc-200 divide-y divide-zinc-100">
				{#each grouped as [group, list] (group)}
					<div class="py-1">
						<p class="px-3 pt-1.5 pb-0.5 text-[11px] font-medium text-zinc-400 uppercase tracking-wide">{group}</p>
						{#each list as e (e.value)}
							<button
								class="w-full text-left px-3 py-1.5 text-sm text-zinc-700 hover:bg-zinc-100 flex items-center justify-between gap-3"
								onclick={() => apply(e.value)}
							>
								<span>{e.label}</span>
								<span class="text-xs text-zinc-400 font-mono">{e.meta}</span>
							</button>
						{/each}
					</div>
				{/each}
				{#if grouped.length === 0}
					<p class="px-3 py-4 text-sm text-zinc-400 text-center">No matches.</p>
				{/if}
			</div>
			<div class="flex flex-wrap justify-end gap-2 pt-2">
				{#if onExternalCreate}
					<button
						class="inline-flex items-center gap-1 text-xs px-2.5 rounded-md border border-dashed border-zinc-300 text-zinc-500 hover:text-zinc-900 hover:border-zinc-400"
						style="height: 30px;"
						title="Open the measures & dimensions editor in /data"
						onclick={() => onExternalCreate?.(kind, table)}
					>
						<SquareArrowOutUpRight size={12} /> Create in /data
					</button>
				{/if}
				<Btn variant="primary" onclick={() => { creating = true; formError = ''; }}>
					<Plus size={14} /> New master {noun}
				</Btn>
			</div>
		{:else}
			<label class="block space-y-1">
				<span class="text-xs" style="color: var(--color-text-secondary)">Label</span>
				<div class="w-full"><TextInput placeholder={kind === 'dimension' ? 'e.g. Region' : 'e.g. Revenue'} bind:value={newLabel} /></div>
			</label>
			<div class="block space-y-1">
				<span class="text-xs" style="color: var(--color-text-secondary)">Expression (DuckDB)</span>
				<ExprEditor bind:value={newExpr} {kind} table={table} columns={schemas[table] ?? []} masterItems={availableItems(table, items, relationships).filter((i) => i.kind === kind)} placeholder={kind === 'dimension' ? 'e.g. region' : 'e.g. sum(amount)'} />
			</div>
			{#if formError}<p class="text-sm" style="color: var(--color-danger)">{formError}</p>{/if}
			<div class="flex justify-end gap-2 pt-2">
				<Btn variant="ghost" onclick={() => (creating = false)}>Back</Btn>
				<Btn variant="primary" disabled={!newLabel.trim() || !newExpr.trim() || saving} onclick={create}>
					<Plus size={14} /> {saving ? 'Saving…' : 'Save to library'}
				</Btn>
			</div>
		{/if}
	</div>
</div>
