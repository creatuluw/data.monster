<script lang="ts">
	/** Master-item editor (FR-17): one component, used by both the Measures and Dimensions tabs. */
	import { onMount } from 'svelte';
	import { listMasterItems, saveMasterItem, deleteMasterItem } from '$lib/central-api';
	import { extractErrorMessage } from '$lib/db-operations';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import type { MasterItem } from '$lib/charts/items';
	import { Plus, Trash2, Star } from 'lucide-svelte';

	let {
		kind,
		schemas
	}: {
		kind: 'measure' | 'dimension';
		schemas: TableSchemas;
	} = $props();

	let items = $state<MasterItem[]>([]);
	let loading = $state(true);
	let error = $state('');
	let adding = $state(false);
	let draft = $state<MasterItem>({ id: '', kind, table: '', label: '', expr: '' });

	const tables = $derived(Object.keys(schemas));
	const noun = $derived(kind === 'measure' ? 'measure' : 'dimension');
	const placeholder = $derived(kind === 'measure' ? "sum(if(type = 'class', hours))" : 'region');

	function newDraft() {
		draft = { id: '', kind, table: tables[0] ?? '', label: '', expr: '' };
	}

	async function refresh() {
		loading = true;
		try {
			items = await listMasterItems(kind);
			error = '';
		} catch (err) {
			error = extractErrorMessage(err, `Failed to load ${noun}s`);
		} finally {
			loading = false;
		}
	}

	async function handleSave() {
		if (!draft.label.trim() || !draft.expr.trim() || !draft.table) return;
		const id = draft.id || `mi_${draft.table}_${draft.label.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_')}`;
		try {
			await saveMasterItem({ ...draft, id });
			adding = false;
			await refresh();
		} catch (err) {
			error = extractErrorMessage(err, `Failed to save ${noun}`);
		}
	}

	async function handleDelete(id: string) {
		try {
			await deleteMasterItem(id);
			await refresh();
		} catch (err) {
			error = extractErrorMessage(err, `Failed to delete ${noun}`);
		}
	}

	onMount(() => {
		newDraft();
		refresh();
	});
</script>

<div class="space-y-4">
	{#if error}<p class="text-sm text-red-500">{error}</p>{/if}

	{#if loading}
		<p class="text-sm text-zinc-400 py-8 text-center">Loading…</p>
	{:else if items.length === 0 && !adding}
		<div class="text-center py-10 border border-dashed border-zinc-300 rounded-lg">
			<Star size={28} class="mx-auto text-zinc-300 mb-2" />
			<p class="text-sm text-zinc-500">No master {noun}s yet.</p>
			<p class="text-xs text-zinc-400 mt-1">Define reusable {noun}s once and add them to any chart.</p>
		</div>
	{/if}

	{#if items.length}
		<div class="space-y-2">
			{#each items as item (item.id)}
				<div class="flex items-center justify-between bg-white border border-zinc-200 rounded-lg px-4 py-3 text-sm gap-3">
					<div class="min-w-0">
						<p class="font-medium text-zinc-900">{item.label} <span class="text-xs text-zinc-400 font-mono">· {item.table}</span></p>
						<p class="text-xs text-zinc-500 font-mono truncate">{item.expr}{item.fmt ? ` fmt:${item.fmt}` : ''}</p>
					</div>
					<div class="flex items-center gap-1 shrink-0">
						<button class="text-zinc-400 hover:text-zinc-900 text-xs" onclick={() => { draft = { ...item }; adding = true; }}>Edit</button>
						<button class="text-zinc-300 hover:text-red-500" onclick={() => handleDelete(item.id)} title="Delete">
							<Trash2 size={14} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}

	{#if adding}
		<div class="bg-white border border-zinc-200 rounded-lg p-4 space-y-3">
			<div class="grid grid-cols-2 gap-3">
				<label class="space-y-1">
					<span class="text-xs text-zinc-500">Label</span>
					<input type="text" class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" bind:value={draft.label} />
				</label>
				<label class="space-y-1">
					<span class="text-xs text-zinc-500">Bound table</span>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" value={draft.table} onchange={(e) => (draft.table = (e.target as HTMLSelectElement).value)}>
						{#each tables as t (t)}<option value={t}>{t}</option>{/each}
					</select>
				</label>
			</div>
			<label class="block space-y-1">
				<span class="text-xs text-zinc-500">Expression (DuckDB SQL)</span>
				<input type="text" class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm font-mono text-xs" placeholder={placeholder} bind:value={draft.expr} />
				<span class="text-[11px] text-zinc-400">{schemas[draft.table]?.length ?? 0} columns available on {draft.table}</span>
			</label>
			{#if kind === 'measure'}
				<label class="block space-y-1 w-40">
					<span class="text-xs text-zinc-500">Format</span>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" value={draft.fmt ?? ''} onchange={(e) => (draft.fmt = (e.target as HTMLSelectElement).value || undefined)}>
						<option value="">(none)</option>
						{#each ['hours', 'usd', 'pct', 'int'] as f (f)}<option value={f}>{f}</option>{/each}
					</select>
				</label>
			{/if}
			<div class="flex gap-2 justify-end">
				<button class="px-3 py-1.5 text-sm text-zinc-500 hover:text-zinc-900" onclick={() => (adding = false)}>Cancel</button>
				<button class="px-3 py-1.5 text-sm text-white rounded-md disabled:opacity-50" style="background: oklch(0.44 0.1 158)" onclick={handleSave} disabled={!draft.label.trim() || !draft.expr.trim()}>Save {noun}</button>
			</div>
		</div>
	{:else}
		<button class="px-3 py-2 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50 inline-flex items-center gap-1.5" onclick={() => { newDraft(); adding = true; }}>
			<Plus size={14} /> Add {noun}
		</button>
	{/if}
</div>
