<script lang="ts">
	import { onMount } from 'svelte';
	import { page as pageState } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAllTableMeta, extractErrorMessage, type QueryResult } from '$lib/db-operations';
	import { getPage, savePage, listMasterItems, listRelationships } from '$lib/central-api';
	import { validatePageDoc } from '$lib/charts/validate';
	import { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import PageGrid from '$lib/components/charts/PageGrid.svelte';
	import BlockInspector from '$lib/components/charts/BlockInspector.svelte';
	import ChartConfigDrawer from '$lib/components/charts/ChartConfigDrawer.svelte';
	import type { PageDoc } from '$lib/charts/spec-types';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import { invoke } from '@tauri-apps/api/core';
	import { Plus, Code, LayoutGrid, Save, ArrowLeft, Trash2, Bolt } from 'lucide-svelte';

	setupChartRegistry();

	const slug = $derived(pageState.params.slug ?? '');

	let doc = $state<PageDoc>({ slug: '', title: '', rows: [] });
	let mode = $state<'design' | 'code'>('design');
	let codeText = $state('');
	let codeErrors = $state<{ path: string; message: string }[]>([]);
	let saveError = $state('');
	let saving = $state(false);
	let saved = $state(false);
	let loading = $state(true);
	let loadError = $state('');

	let schemas = $state<TableSchemas>({});
	let items = $state<MasterItem[]>([]);
	let relationships = $state<Relationship[]>([]);

	let configId = $state<string | null>(null);
	let runtime = $state<ReturnType<typeof createPageRuntime> | null>(null);

	// live re-render: deep-track the doc (inspector mutates in place), debounce
	// the runtime recreation so typing doesn't re-query per keystroke
	$effect(() => {
		JSON.stringify(doc); // deep dependency on the whole document
		const t = setTimeout(() => {
			if (!doc.rows) return;
			const rt = createPageRuntime(doc, {
				schemas,
				items,
				relationships,
				runQuery,
				palette: undefined
			});
			runtime = rt;
			rt.init();
		}, 250);
		return () => clearTimeout(t);
	});

	async function runQuery(sql: string): Promise<Record<string, unknown>[]> {
		const result = await invoke<QueryResult>('execute_query', { sql });
		if (!('data' in result) || !('columns' in result)) return [];
		// columnar → row objects
		return result.data.map((row) => Object.fromEntries(result.columns.map((c, i) => [c, row[i]])));
	}

	onMount(async () => {
		try {
			const [metas, its, rels] = await Promise.all([
				getAllTableMeta(),
				listMasterItems().catch(() => [] as MasterItem[]),
				listRelationships().catch(() => [] as Relationship[])
			]);
			schemas = Object.fromEntries(metas.map((m) => [m.name, m.columns.map((c) => c.name)]));
			items = its;
			relationships = rels;
			try {
				doc = await getPage(slug);
			} catch {
				doc = { slug, title: slug.replaceAll('-', ' ').replace(/\b\w/g, (c) => c.toUpperCase()), rows: [] };
			}
			codeText = JSON.stringify(doc, null, '\t');
		} catch (err) {
			loadError = extractErrorMessage(err, 'Failed to load workspace data');
		} finally {
			loading = false;
		}
	});

	function applyCode() {
		try {
			const parsed = JSON.parse(codeText) as PageDoc;
			codeErrors = validatePageDoc(parsed);
			if (codeErrors.length === 0) {
				doc = parsed;
			}
		} catch (err) {
			codeErrors = [{ path: '', message: err instanceof Error ? err.message : 'Invalid JSON' }];
		}
	}

	function switchMode(m: 'design' | 'code') {
		if (m === 'code') codeText = JSON.stringify(doc, null, '\t');
		else applyCode();
		mode = m;
	}

	async function handleSave() {
		const errors = validatePageDoc(doc);
		codeErrors = errors;
		if (errors.length) return;
		saving = true;
		saveError = '';
		saved = false;
		try {
			await savePage(doc);
			saved = true;
			setTimeout(() => (saved = false), 2000);
		} catch (err) {
			saveError = extractErrorMessage(err, 'Failed to save page');
		} finally {
			saving = false;
		}
	}

	function addRow() {
		doc.rows = [...(doc.rows ?? []), { blocks: [] }];

	}

	function removeRow(ri: number) {
		doc.rows!.splice(ri, 1);

	}

	function addBlock(kind: 'chart' | 'table' | 'text', chartType?: string) {
		const span = 6;
		const block =
			kind === 'chart'
				? {
						type: 'chart',
						span,
						chart: {
							type: chartType ?? 'bar',
							source: { table: Object.keys(schemas)[0] ?? '' },
							dimensions: [{ col: (schemas[Object.keys(schemas)[0] ?? ''] ?? [])[0] ?? '' }],
							measures: [{ expr: 'count(*)', label: 'Count' }]
						}
					}
				: kind === 'table'
					? { type: 'table', span: 12, table: Object.keys(schemas)[0] ?? '', limit: 50 }
					: { type: 'text', span, text: 'Text…' };
		if (!doc.rows?.length) doc.rows = [{ blocks: [] }];
		doc.rows[doc.rows.length - 1].blocks.push(block as never);
		configId = `r${doc.rows.length - 1}-b${doc.rows[doc.rows.length - 1].blocks.length - 1}`;

	}

	function configureBlock(id: string) {
		configId = id;
	}

	const configBlock = $derived.by(() => {
		if (configId === null) return null;
		const [ri, bi] = configId.replace('r', '').split('-b').map(Number);
		const block = doc.rows?.[ri]?.blocks[bi];
		return block ? { ri, bi, block } : null;
	});
</script>

<svelte:head><title>{doc.title || slug} — data.monster</title></svelte:head>

<div class="page-shell" style="padding: var(--space-6);">
	<div class="flex items-center gap-3 mb-6">
		<a href="/pages" class="text-zinc-400 hover:text-zinc-900" title="Back to pages"><ArrowLeft size={18} /></a>
		<input
			type="text"
			class="text-xl font-semibold bg-transparent border-none outline-none focus:ring-0 flex-1"
			style="font-family: var(--font-display)"
			value={doc.title}
			oninput={(e) => (doc.title = (e.target as HTMLInputElement).value)}
		/>
		<div class="flex items-center gap-1 bg-zinc-100 rounded-lg p-1">
			<button class:active={mode === 'design'} class="px-3 py-1.5 rounded-md text-sm inline-flex items-center gap-1.5 {mode === 'design' ? 'bg-white shadow-sm font-medium' : 'text-zinc-500'}" onclick={() => switchMode('design')}><LayoutGrid size={14} /> Design</button>
			<button class:active={mode === 'code'} class="px-3 py-1.5 rounded-md text-sm inline-flex items-center gap-1.5 {mode === 'code' ? 'bg-white shadow-sm font-medium' : 'text-zinc-500'}" onclick={() => switchMode('code')}><Code size={14} /> Code</button>
		</div>
		<button
			class="px-4 py-2 rounded-lg text-sm font-medium text-white inline-flex items-center gap-2 disabled:opacity-50"
			style="background: oklch(0.44 0.1 158)"
			onclick={handleSave}
			disabled={saving || codeErrors.length > 0}
		>
			<Save size={14} /> {saved ? 'Saved' : saving ? 'Saving…' : 'Save'}
		</button>
	</div>

	{#if saveError}<p class="text-sm text-red-500 mb-4">{saveError}</p>{/if}
	{#if loadError}<p class="text-sm text-red-500 mb-4">{loadError}</p>{/if}

	{#if loading}
		<p class="text-sm text-zinc-400 py-16 text-center">Loading…</p>
	{:else if mode === 'code'}
		<div class="space-y-3">
			<textarea
				class="w-full h-[70vh] font-mono text-xs border border-zinc-200 rounded-lg p-4 focus:outline-none focus:ring-1 focus:ring-zinc-300"
				spellcheck="false"
				bind:value={codeText}
				onblur={applyCode}
			></textarea>
			{#if codeErrors.length}
				<div class="text-sm text-red-500 space-y-1">
					{#each codeErrors as e (e.path + e.message)}
						<p><span class="font-mono text-xs">{e.path || 'document'}</span> — {e.message}</p>
					{/each}
				</div>
			{:else}
				<p class="text-xs text-zinc-400">Valid — switching to Design applies the document.</p>
			{/if}
		</div>
	{:else if configBlock && runtime}
		<!-- focused config view: configured chart left, 50vw drawer right, other blocks hidden -->
		<div class="w-1/2 pr-8">
			<PageGrid {doc} {runtime} configureId={configId} />
		</div>
		<ChartConfigDrawer
			open={true}
			title={configBlock.block.type === 'chart' ? (configBlock.block.chart.title ?? 'Chart configuration') : `${configBlock.block.type} configuration`}
			width="50vw"
			overlay={false}
			onClosed={() => (configId = null)}
		>
			<BlockInspector {doc} ri={configBlock.ri} bi={configBlock.bi} {schemas} {items} {relationships} onremove={() => (configId = null)} />
		</ChartConfigDrawer>
	{:else}
		<div class="flex gap-6 items-start">
			<!-- canvas -->
			<div class="flex-1 min-w-0 space-y-4">
				{#if runtime}
					<PageGrid {doc} {runtime} onConfigure={(id) => configureBlock(id)} />
				{/if}

				<div class="flex items-center gap-2 pt-2">
					<button class="px-3 py-1.5 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50 inline-flex items-center gap-1.5" onclick={addRow}><Plus size={14} /> Row</button>
					<button class="px-3 py-1.5 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50" onclick={() => addBlock('chart', 'bar')}>+ Bar chart</button>
					<button class="px-3 py-1.5 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50" onclick={() => addBlock('chart', 'heatmap')}>+ Heatmap</button>
					<button class="px-3 py-1.5 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50" onclick={() => addBlock('table')}>+ Table</button>
					<button class="px-3 py-1.5 border border-zinc-300 rounded-lg text-sm text-zinc-600 hover:bg-zinc-50" onclick={() => addBlock('text')}>+ Text</button>
				</div>
			</div>

			<!-- inspector -->
			<aside class="w-80 shrink-0 sticky top-8 space-y-3">
				<div class="bg-white rounded-lg border border-zinc-200 p-4 text-sm text-zinc-400">
					Click the <Bolt size={13} class="inline-block -mt-0.5" /> on a block to configure it — or add one below.
				</div>
				{#if doc.rows?.length}
					<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-2">
						<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Rows</span>
						{#each doc.rows as _, ri (ri)}
							<div class="flex items-center justify-between text-sm">
								<span class="text-zinc-600">Row {ri + 1} · {doc.rows![ri].blocks.length} blocks</span>
								<button class="text-zinc-300 hover:text-red-500" onclick={() => removeRow(ri)} title="Remove row"><Trash2 size={13} /></button>
							</div>
						{/each}
					</div>
				{/if}
			</aside>
		</div>
	{/if}
</div>
