<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { listPages, savePage, deletePage, type PageMeta } from '$lib/central-api';
	import { extractErrorMessage } from '$lib/db-operations';
	import { FileText, Plus, Trash2, LayoutTemplate, X } from 'lucide-svelte';

	let pages = $state<PageMeta[]>([]);
	let loading = $state(true);
	let error = $state('');

	// new-page modal
	let modalOpen = $state(false);
	let newTitle = $state('');
	let creating = $state(false);
	let confirmDelete = $state<string | null>(null);

	function slugify(name: string): string {
		return name
			.toLowerCase()
			.trim()
			.replaceAll(/[^a-z0-9]+/g, '-')
			.replaceAll(/^-+|-+$/g, '');
	}

	const newSlug = $derived(slugify(newTitle));

	async function refresh() {
		loading = true;
		try {
			pages = await listPages();
			error = '';
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to load pages');
		} finally {
			loading = false;
		}
	}

	function openModal() {
		newTitle = '';
		modalOpen = true;
	}

	async function handleCreate() {
		const name = newTitle.trim();
		if (!name) return;
		const slug = slugify(name);
		creating = true;
		try {
			await savePage({ slug, title: name, rows: [] });
			goto(`/page/${slug}`);
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to create page');
			creating = false;
		}
	}

	async function handleDelete(slug: string) {
		try {
			await deletePage(slug);
			confirmDelete = null;
			await refresh();
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to delete page');
		}
	}

	function handleModalKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') modalOpen = false;
		if (e.key === 'Enter') handleCreate();
	}

	onMount(refresh);
</script>

<svelte:head><title>Pages — data.monster</title></svelte:head>

<svelte:window onkeydown={(e) => { if (e.key === 'Escape' && modalOpen) modalOpen = false; }} />

<div class="full-bleed" style="padding: var(--space-6);">
	<div class="flex items-end justify-between mb-6">
		<div>
			<h1 class="text-2xl font-semibold text-zinc-900 tracking-tight" style="font-family: var(--font-display)">Report pages</h1>
			<p class="text-sm text-zinc-500 mt-1">Build insight pages from your data — charts, tables and text on one grid.</p>
		</div>
		<button
			class="px-4 py-2 rounded-lg text-sm font-medium text-white inline-flex items-center gap-2"
			style="background: oklch(0.44 0.1 158)"
			onclick={openModal}
		>
			<Plus size={14} /> New page
		</button>
	</div>

	{#if error}<p class="text-sm text-red-500 mb-4">{error}</p>{/if}

	{#if loading}
		<p class="text-sm text-zinc-400 py-16 text-center">Loading…</p>
	{:else if pages.length === 0}
		<div class="text-center py-20 border border-dashed border-zinc-300 rounded-xl">
			<LayoutTemplate size={36} class="mx-auto text-zinc-300 mb-3" />
			<h2 class="text-lg font-medium text-zinc-700">No pages yet</h2>
			<p class="text-sm text-zinc-400 mt-1">Create your first report page.</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
			{#each pages as p (p.slug)}
				<div class="group relative bg-white border border-zinc-200 rounded-xl p-5 hover:border-zinc-400 hover:shadow-sm transition-colors cursor-pointer flex flex-col min-h-32" onclick={() => goto(`/page/${p.slug}`)} onkeydown={(e) => e.key === 'Enter' && goto(`/page/${p.slug}`)} role="button" tabindex="0">
					<FileText size={18} class="text-zinc-400 mb-3" />
					<p class="font-medium text-zinc-900 leading-snug">{p.title}</p>
					<p class="text-xs text-zinc-400 font-mono mt-1">/{p.slug}</p>
					<div class="mt-auto pt-4 flex items-center justify-between">
						<span class="text-xs text-zinc-400">{p.updatedAt ? new Date(p.updatedAt).toLocaleDateString() : ''}</span>
						{#if confirmDelete === p.slug}
							<span class="flex items-center gap-2 text-xs">
								<span class="text-red-500">Delete?</span>
								<button class="text-red-500 font-medium hover:underline" onclick={(e) => { e.stopPropagation(); handleDelete(p.slug); }}>Yes</button>
								<button class="text-zinc-400 hover:underline" onclick={(e) => { e.stopPropagation(); confirmDelete = null; }}>No</button>
							</span>
						{:else}
							<button class="text-zinc-300 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity" onclick={(e) => { e.stopPropagation(); confirmDelete = p.slug; }} title="Delete page">
								<Trash2 size={14} />
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

{#if modalOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-50 bg-black/40 flex items-center justify-center p-6" onclick={() => (modalOpen = false)} onkeydown={handleModalKeydown}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6 space-y-4" onclick={(e) => e.stopPropagation()} onkeydown={handleModalKeydown}>
			<div class="flex items-center justify-between">
				<h2 class="text-lg font-semibold text-zinc-900" style="font-family: var(--font-display)">New page</h2>
				<button class="text-zinc-400 hover:text-zinc-900" onclick={() => (modalOpen = false)} title="Close"><X size={16} /></button>
			</div>
			<label class="block space-y-1">
				<span class="text-xs text-zinc-500">Title</span>
				<input type="text" autofocus class="w-full border border-zinc-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-zinc-400" placeholder="e.g. Monthly utilization" bind:value={newTitle} onkeydown={handleModalKeydown} />
				{#if newTitle.trim()}
					<span class="text-xs text-zinc-400 font-mono">/page/{newSlug || '…'}</span>
				{/if}
			</label>
			<div class="flex justify-end gap-2 pt-2">
				<button class="px-3 py-2 text-sm text-zinc-500 hover:text-zinc-900" onclick={() => (modalOpen = false)}>Cancel</button>
				<button
					class="px-4 py-2 rounded-lg text-sm font-medium text-white disabled:opacity-50 inline-flex items-center gap-2"
					style="background: oklch(0.44 0.1 158)"
					onclick={handleCreate}
					disabled={!newTitle.trim() || creating}
				>
					<Plus size={14} /> {creating ? 'Creating…' : 'Create page'}
				</button>
			</div>
		</div>
	</div>
{/if}
