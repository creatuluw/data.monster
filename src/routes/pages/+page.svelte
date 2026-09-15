<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { listPages, savePage, deletePage, type PageMeta } from '$lib/central-api';
	import { extractErrorMessage } from '$lib/db-operations';
	import { FileText, Plus, Trash2, LayoutTemplate } from 'lucide-svelte';

	let pages = $state<PageMeta[]>([]);
	let loading = $state(true);
	let error = $state('');
	let newName = $state('');
	let creating = $state(false);
	let confirmDelete = $state<string | null>(null);

	function slugify(name: string): string {
		return name
			.toLowerCase()
			.trim()
			.replaceAll(/[^a-z0-9]+/g, '-')
			.replaceAll(/^-+|-+$/g, '');
	}

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

	async function handleCreate() {
		const name = newName.trim();
		if (!name) return;
		const slug = slugify(name);
		creating = true;
		try {
			await savePage({ slug, title: name, rows: [] });
			goto(`/page/${slug}`);
		} catch (err) {
			error = extractErrorMessage(err, 'Failed to create page');
		} finally {
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

	onMount(refresh);
</script>

<svelte:head><title>Pages — data.monster</title></svelte:head>

<div class="max-w-5xl mx-auto px-6 py-10">
	<div class="flex items-end justify-between mb-8">
		<div>
			<h1 class="text-2xl font-semibold text-zinc-900 tracking-tight" style="font-family: var(--font-display)">Report pages</h1>
			<p class="text-sm text-zinc-500 mt-1">Build insight pages from your data — charts, tables and text on one grid.</p>
		</div>
		<div class="flex items-center gap-2">
			<input
				type="text"
				placeholder="New page name…"
				class="border border-zinc-300 rounded-lg px-3 py-2 text-sm w-56 focus:outline-none focus:ring-1 focus:ring-zinc-400"
				bind:value={newName}
				onkeydown={(e) => e.key === 'Enter' && handleCreate()}
			/>
			<button
				class="px-4 py-2 rounded-lg text-sm font-medium text-white inline-flex items-center gap-2 disabled:opacity-50"
				style="background: oklch(0.44 0.1 158)"
				onclick={handleCreate}
				disabled={creating || !newName.trim()}
			>
				<Plus size={14} /> New page
			</button>
		</div>
	</div>

	{#if error}<p class="text-sm text-red-500 mb-4">{error}</p>{/if}

	{#if loading}
		<p class="text-sm text-zinc-400 py-16 text-center">Loading…</p>
	{:else if pages.length === 0}
		<div class="text-center py-20 border border-dashed border-zinc-300 rounded-xl">
			<LayoutTemplate size={36} class="mx-auto text-zinc-300 mb-3" />
			<h2 class="text-lg font-medium text-zinc-700">No pages yet</h2>
			<p class="text-sm text-zinc-400 mt-1">Create your first report page above.</p>
		</div>
	{:else}
		<div class="grid gap-3">
			{#each pages as p (p.slug)}
				<div class="flex items-center justify-between bg-white border border-zinc-200 rounded-lg px-5 py-4 hover:border-zinc-300 transition-colors">
					<button class="flex items-center gap-3 text-left flex-1" onclick={() => goto(`/page/${p.slug}`)}>
						<FileText size={18} class="text-zinc-400 shrink-0" />
						<div>
							<p class="font-medium text-zinc-900">{p.title}</p>
							<p class="text-xs text-zinc-400 font-mono">/{p.slug}</p>
						</div>
					</button>
					<span class="text-xs text-zinc-400 mr-4">{p.updatedAt ? new Date(p.updatedAt).toLocaleString() : ''}</span>
					{#if confirmDelete === p.slug}
						<span class="flex items-center gap-2 text-xs">
							<span class="text-red-500">Delete?</span>
							<button class="text-red-500 font-medium hover:underline" onclick={() => handleDelete(p.slug)}>Yes</button>
							<button class="text-zinc-400 hover:underline" onclick={() => (confirmDelete = null)}>No</button>
						</span>
					{:else}
						<button class="text-zinc-300 hover:text-red-500" onclick={() => (confirmDelete = p.slug)} title="Delete page">
							<Trash2 size={15} />
						</button>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
