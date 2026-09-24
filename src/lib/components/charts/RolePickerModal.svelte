<script lang="ts">
	/**
	 * Role picker modal (new-page-modal pattern): searchable list over ⭐ master
	 * items + source-table fields + linked-table fields, plus a + New form that
	 * creates a master item on the spot. Applying pushes into the chart spec.
	 *
	 * Path B pilot: bits-ui owns behavior (Dialog = focus trap/Escape/overlay,
	 * Combobox = search input + listbox keyboard nav), styling is utility-first
	 * over the app token layer.
	 */
	import { Dialog, Combobox } from 'bits-ui';
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

	let searchValue = $state('');
	const filtered = $derived(
		entries.filter((e) => {
			if (!searchValue.trim()) return true;
			const q = searchValue.toLowerCase();
			return e.label.toLowerCase().includes(q) || e.meta.toLowerCase().includes(q);
		})
	);
	const grouped = $derived.by(() => {
		const map = new Map<string, Entry[]>();
		for (const e of filtered) map.set(e.group, [...(map.get(e.group) ?? []), e]);
		return [...map.entries()];
	});

	let inputRef = $state<HTMLInputElement | null>(null);

	function apply(value: string) {
		if (kind === 'dimension') chart.dimensions.push(dimensionFromPick(value, table));
		else chart.measures.push(measureFromPick(value, table));
		onClose();
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

<Dialog.Root open onOpenChange={(o) => !o && onClose()}>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-black/40" />
		<Dialog.Content
			class="fixed left-1/2 top-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 space-y-4 rounded-xl bg-surface p-6 shadow-xl outline-none"
			onOpenAutoFocus={(e) => {
				// focus the search input, not the first focusable (the X button)
				e.preventDefault();
				inputRef?.focus();
			}}
		>
			<div class="flex items-center justify-between">
				<Dialog.Title class="text-lg font-semibold text-text" style="font-family: var(--font-display)">Add {noun}</Dialog.Title>
				<Dialog.Close class="text-text-tertiary hover:text-text" title="Close"><X size={16} /></Dialog.Close>
			</div>
			<div class="flex flex-wrap gap-1.5">
				{#if roleLabels(chart, 'dimension', items).length === 0 && roleLabels(chart, 'measure', items).length === 0}
					<span class="text-xs text-text-tertiary">Nothing added yet</span>
				{:else}
					{#each roleLabels(chart, 'dimension', items) as d (d.label + d.meta)}
						<span class="rounded-full bg-text px-2 py-0.5 text-[11px] font-medium text-surface" title="dimension">{d.label}</span>
					{/each}
					{#each roleLabels(chart, 'measure', items) as m (m.label + m.meta)}
						<span class="rounded-full border border-border-strong px-2 py-0.5 text-[11px] font-medium text-text-secondary" title="measure">{m.label}</span>
					{/each}
				{/if}
			</div>

			{#if !creating}
				<!-- the pick list is always visible (not a dropdown) — hold the combobox open -->
					<Combobox.Root type="single" bind:open={() => true, () => {}} onValueChange={(v) => v && apply(v)}>
					<div class="relative">
						<Search size={14} class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-text-tertiary" />
						<Combobox.Input
							bind:ref={inputRef}
							class="flex h-[30px] w-full items-center rounded-sm border border-border-strong bg-surface pl-8 pr-3 text-sm text-text outline-none placeholder:text-text-tertiary focus:border-accent"
							placeholder="Type to filter…"
							aria-label="Search {noun}s — master items, fields of {table}, linked tables"
							oninput={(e) => (searchValue = e.currentTarget.value)}
						/>
					</div>
					<Combobox.ContentStatic
						class="max-h-64 overflow-y-auto rounded-lg border border-border bg-surface"
					>
						{#each grouped as [group, list] (group)}
							<Combobox.Group>
								<Combobox.GroupHeading class="px-3 pt-1.5 pb-0.5 text-[11px] font-medium uppercase tracking-wide text-text-tertiary">{group}</Combobox.GroupHeading>
								{#each list as e (e.value)}
									<Combobox.Item
										value={e.value}
										label={e.label}
										class="flex w-full cursor-pointer items-center justify-between gap-3 px-3 py-1.5 text-left text-sm text-text-secondary data-highlighted:bg-surface-sunken data-highlighted:text-text"
									>
										{#snippet children({ selected })}
											<span>{e.label}</span>
											<span class="font-mono text-xs text-text-tertiary">{e.meta}{selected ? ' ✓' : ''}</span>
										{/snippet}
									</Combobox.Item>
								{/each}
							</Combobox.Group>
						{/each}
						{#if grouped.length === 0}
							<p class="px-3 py-4 text-center text-sm text-text-tertiary">No matches.</p>
						{/if}
					</Combobox.ContentStatic>
				</Combobox.Root>
				<div class="flex flex-wrap justify-end gap-2 pt-2">
					{#if onExternalCreate}
						<button
							class="inline-flex h-[30px] items-center gap-1 rounded-md border border-dashed border-border px-2.5 text-xs text-text-tertiary hover:border-border-strong hover:text-text"
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
					<span class="text-xs text-text-secondary">Label</span>
					<div class="w-full"><TextInput placeholder={kind === 'dimension' ? 'e.g. Region' : 'e.g. Revenue'} bind:value={newLabel} /></div>
				</label>
				<div class="block space-y-1">
					<span class="text-xs text-text-secondary">Expression (DuckDB)</span>
					<ExprEditor bind:value={newExpr} {kind} table={table} columns={schemas[table] ?? []} masterItems={availableItems(table, items, relationships).filter((i) => i.kind === kind)} placeholder={kind === 'dimension' ? 'e.g. region' : 'e.g. sum(amount)'} />
				</div>
				{#if formError}<p class="text-sm text-danger">{formError}</p>{/if}
				<div class="flex justify-end gap-2 pt-2">
					<Btn variant="ghost" onclick={() => (creating = false)}>Back</Btn>
					<Btn variant="primary" disabled={!newLabel.trim() || !newExpr.trim() || saving} onclick={create}>
						<Plus size={14} /> {saving ? 'Saving…' : 'Save to library'}
					</Btn>
				</div>
			{/if}
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
