<script lang="ts">
	/** Relationship editor (FR-16): list/create/delete table relationships. */
	import { onMount } from 'svelte';
	import { listRelationships, saveRelationship, deleteRelationship, type RelationshipInput } from '$lib/central-api';
	import { extractErrorMessage } from '$lib/db-operations';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import { Plus, Trash2, Link2 } from 'lucide-svelte';

	let {
		schemas
	}: {
		schemas: TableSchemas;
	} = $props();

	let relationships = $state<{ id: string; fromTable: string; fromColumn: string; toTable: string; toColumn: string }[]>([]);
	let loading = $state(true);
	let error = $state('');
	let adding = $state(false);
	let draft = $state<RelationshipInput>({ fromTable: '', fromColumn: '', toTable: '', toColumn: '' });

	const tables = $derived(Object.keys(schemas));

	async function refresh() {
		loading = true;
		try {
			relationships = await listRelationships();
			error = '';
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to load relationships');
		} finally {
			loading = false;
		}
	}

	function pickDefaults() {
		draft = {
			fromTable: tables[0] ?? '',
			fromColumn: (schemas[tables[0] ?? ''] ?? [])[0] ?? '',
			toTable: tables[1] ?? tables[0] ?? '',
			toColumn: (schemas[tables[1] ?? ''] ?? [])[0] ?? ''
		};
	}

	async function handleAdd() {
		if (!draft.fromTable || !draft.fromColumn || !draft.toTable || !draft.toColumn) return;
		try {
			await saveRelationship(draft);
			adding = false;
			await refresh();
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to save relationship');
		}
	}

	async function handleDelete(id: string) {
		try {
			await deleteRelationship(id);
			await refresh();
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to delete relationship');
		}
	}

	onMount(() => {
		pickDefaults();
		refresh();
	});
</script>

<div class="space-y-4">
	{#if error}<p class="text-sm text-red-500">{error}</p>{/if}

	{#if loading}
		<p class="text-sm text-zinc-400 py-8 text-center">Loading…</p>
	{:else if relationships.length === 0 && !adding}
		<div class="text-center py-10 border border-dashed border-zinc-300 rounded-lg">
			<Link2 size={28} class="mx-auto text-zinc-300 mb-2" />
			<p class="text-sm text-zinc-500">No relationships defined.</p>
			<p class="text-xs text-zinc-400 mt-1">Connect tables on shared columns to enable master items and auto-JOIN across tables.</p>
		</div>
	{/if}

	{#if relationships.length}
		<div class="space-y-2">
			{#each relationships as rel (rel.id)}
				<div class="flex items-center justify-between bg-white border border-zinc-200 rounded-lg px-4 py-3 text-sm">
					<div class="flex items-center gap-2 font-mono text-xs text-zinc-700">
						<span>{rel.fromTable}<span class="text-zinc-400">.</span><span class="text-zinc-500">{rel.fromColumn}</span></span>
						<span class="text-zinc-300">→</span>
						<span>{rel.toTable}<span class="text-zinc-400">.</span><span class="text-zinc-500">{rel.toColumn}</span></span>
					</div>
					<button class="text-zinc-300 hover:text-red-500" onclick={() => handleDelete(rel.id)} title="Delete relationship">
						<Trash2 size={14} />
					</button>
				</div>
			{/each}
		</div>
	{/if}

	{#if adding}
		<div class="bg-white border border-zinc-200 rounded-lg p-4 space-y-3">
			<div class="grid grid-cols-2 gap-3">
				<label class="space-y-1">
					<span class="text-xs text-zinc-500">From table</span>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" value={draft.fromTable} onchange={(e) => { draft.fromTable = (e.target as HTMLSelectElement).value; draft.fromColumn = (schemas[draft.fromTable] ?? [])[0] ?? ''; }}>
						{#each tables as t (t)}<option value={t}>{t}</option>{/each}
					</select>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm font-mono text-xs" value={draft.fromColumn} onchange={(e) => (draft.fromColumn = (e.target as HTMLSelectElement).value)}>
						{#each schemas[draft.fromTable] ?? [] as c (c)}<option value={c}>{c}</option>{/each}
					</select>
				</label>
				<label class="space-y-1">
					<span class="text-xs text-zinc-500">To table</span>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" value={draft.toTable} onchange={(e) => { draft.toTable = (e.target as HTMLSelectElement).value; draft.toColumn = (schemas[draft.toTable] ?? [])[0] ?? ''; }}>
						{#each tables as t (t)}<option value={t}>{t}</option>{/each}
					</select>
					<select class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm font-mono text-xs" value={draft.toColumn} onchange={(e) => (draft.toColumn = (e.target as HTMLSelectElement).value)}>
						{#each schemas[draft.toTable] ?? [] as c (c)}<option value={c}>{c}</option>{/each}
					</select>
				</label>
			</div>
			<div class="flex gap-2 justify-end">
				<button class="px-3 py-1.5 text-sm text-zinc-500 hover:text-zinc-900" onclick={() => (adding = false)}>Cancel</button>
				<button class="px-3 py-1.5 text-sm text-white rounded-md" style="background: oklch(0.44 0.1 158)" onclick={handleAdd}>Save relationship</button>
			</div>
		</div>
	{:else}
		<button class="px-3 py-2 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50 inline-flex items-center gap-1.5" onclick={() => { pickDefaults(); adding = true; }}>
			<Plus size={14} /> Add relationship
		</button>
	{/if}
</div>
