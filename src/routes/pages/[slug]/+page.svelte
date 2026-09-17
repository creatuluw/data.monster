<script lang="ts">
	import { onMount } from 'svelte';
	import { page as pageState } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAllTableMeta, extractErrorMessage, type QueryResult } from '$lib/db-operations';
	import { getPage, savePage, listMasterItems, listRelationships, saveMasterItem } from '$lib/central-api';
	import { validatePageDoc } from '$lib/charts/validate';
import { normalizePageDoc, rowColumns } from '$lib/charts/spec-types';
	import { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import PageGrid from '$lib/components/charts/PageGrid.svelte';
	import BlockInspector from '$lib/components/charts/BlockInspector.svelte';
	import DrawerTabs from '$lib/components/charts/DrawerTabs.svelte';
	import ChartConfigDrawer from '$lib/components/charts/ChartConfigDrawer.svelte';
	import type { PageDoc } from '$lib/charts/spec-types';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import { invoke } from '@tauri-apps/api/core';
	import { Plus, Code, LayoutGrid, Save, Trash2, FileCog, BarChart3, Table, Type, Search } from 'lucide-svelte';
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getChartType } from '$lib/charts/registry';
	import { getLibraryComponents } from '$lib/library/registry';

	setupChartRegistry();

	// library components drive the + Component menu inside columns — registered = available
	const libraryCharts = getLibraryComponents();
	// registry also registers table/text built-ins — dedupe by key, keep first
	const components = [
		...libraryCharts.map((c) => ({
			key: c.def.type,
			label: c.def.label,
			kind: (c.blockKind ?? 'chart') as 'chart' | 'table' | 'text'
		})),
		{ key: 'table', label: 'Table', kind: 'table' as const },
		{ key: 'text', label: 'Text', kind: 'text' as const }
	].filter((c, i, all) => all.findIndex((x) => x.key === c.key) === i);

	const slug = $derived(pageState.params.slug ?? '');

	let doc = $state<PageDoc>({ slug: '', title: '', rows: [] });
	let mode = $state<'design' | 'code' | 'page'>('design');
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
	let rowConfig = $state<number | null>(null);
	let rowTab = $state<'settings' | 'danger'>('settings');
	let colTab = $state<'settings' | 'danger'>('settings');
	let blockTab = $state<'settings' | 'danger'>('settings');
	let colConfig = $state<{ ri: number; ci: number } | null>(null);
	/** + Component picker: which column a picked component lands in */
	let pickerFor = $state<{ ri: number; ci: number } | null>(null);
	let pickerQuery = $state('');

	const filteredComponents = $derived(
		components.filter((c) => {
			const q = pickerQuery.trim().toLowerCase();
			return !q || c.label.toLowerCase().includes(q) || c.key.toLowerCase().includes(q);
		})
	);
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

	/** in-chart ✚ create: save + refresh the library, resolve to the new item id */
	async function createMasterItem(kind: 'dimension' | 'measure', table: string, label: string, expr: string): Promise<string> {
		const id = `mi_${table}_${label.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_')}`;
		await saveMasterItem({ id, kind, table, label, expr });
		await refreshItems();
		return id;
	}

	async function refreshItems() {
		const [its, rels] = await Promise.all([
			listMasterItems().catch(() => items),
			listRelationships().catch(() => relationships)
		]);
		items = its;
		relationships = rels;
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
				doc = normalizePageDoc(await getPage(slug));
			} catch {
				doc = { slug, title: slug.replaceAll('-', ' ').replace(/\b\w/g, (c) => c.toUpperCase()), rows: [{ columns: [{ span: 12, blocks: [] }] }] };
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
				doc = normalizePageDoc(parsed);
			}
		} catch (err) {
			codeErrors = [{ path: '', message: err instanceof Error ? err.message : 'Invalid JSON' }];
		}
	}

	function switchMode(m: 'design' | 'code' | 'page') {
		if (m === 'code') codeText = JSON.stringify(doc, null, '\t');
		else if (mode === 'code') applyCode();
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
		doc.rows = [...(doc.rows ?? []), { columns: [{ span: 12, blocks: [] }], height: 180 }];
		handleSave(); // spawn = persisted instantly
	}

	function removeRow(ri: number) {
		doc.rows!.splice(ri, 1);
	}

	/** add a component (chart type key | 'table' | 'text') to a column; opens its config */
	function addComponent(ri: number, ci: number, key: string) {
		const table = Object.keys(schemas)[0] ?? '';
		const cols = schemas[table] ?? [];
		const block =
			key === 'table'
				? { type: 'table', table, limit: 50 }
				: key === 'text'
					? { type: 'text', text: 'Text…' }
					: (() => {
						// empty skeleton: table preselected, roles empty — user configures via the inspector
						return {
							type: 'chart',
							chart: {
								type: key,
								source: { table },
								dimensions: [],
								measures: []
							}
						};
					})();
		// every component gets its own column: occupied target → spawn a fresh column for it
		const columns = doc.rows![ri].columns?.length ? doc.rows![ri].columns! : (doc.rows![ri].columns = [{ span: 12, blocks: [] }]);
		const target = columns[ci].blocks.length > 0 ? addColumn(ri) : ci;
		const blocks = doc.rows![ri].columns![target].blocks;
		blocks.push(block as never);
		// a spawned component needs vertical room — grow an explicit row/column height to the spawn minimum
		const SPAWN_MIN_PX = 320;
		const col = doc.rows![ri].columns![target];
		const eff = col.height ?? doc.rows![ri].height;
		if (eff !== undefined && eff < SPAWN_MIN_PX) {
			if (col.height !== undefined) col.height = SPAWN_MIN_PX;
			else doc.rows![ri].height = SPAWN_MIN_PX;
		}
		// no auto-open: the block stays a skeleton on canvas until the user configures it
	}

	/** split the row wider: re-divide 12 columns equally across n+1 columns; returns the new column's index */
	function addColumn(ri: number): number {
		const cols = doc.rows![ri].columns ?? (doc.rows![ri].columns = []);
		const span = Math.max(1, Math.floor(12 / (cols.length + 1)));
		cols.forEach((c) => (c.span = span));
		cols.push({ span, blocks: [] });
		return cols.length - 1;
	}

	function removeColumn(ri: number, ci: number) {
		const cols = doc.rows![ri].columns!;
		if (cols.length > 1) cols.splice(ci, 1);
	}

	function configureBlock(id: string) {
		configId = id;
		blockTab = 'settings';
	}


	const configBlock = $derived.by(() => {
		if (configId === null) return null;
		const [ri, ci, bi] = configId.split('-').map((p) => Number(p.replace(/\D/g, '')));
		const block = doc.rows?.[ri]?.columns?.[ci]?.blocks[bi];
		return block ? { ri, ci, bi, block } : null;
	});
</script>

<svelte:head><title>{doc.title || slug} — data.monster</title></svelte:head>

<div class="page-shell" style="padding: var(--space-6);">
	<div class="flex items-center gap-3 mb-6">
		<input
			type="text"
			class="page-title bg-transparent border-none outline-none focus:ring-0 flex-1"
			value={doc.title}
			oninput={(e) => (doc.title = (e.target as HTMLInputElement).value)}
		/>
		<div class="flex items-center gap-1 bg-zinc-100 rounded-lg p-1">
			<button class:active={mode === 'design'} class="px-3 py-1.5 rounded-md text-sm inline-flex items-center gap-1.5 {mode === 'design' ? 'bg-white shadow-sm font-medium' : 'text-zinc-500'}" onclick={() => switchMode('design')}><LayoutGrid size={14} /> Design</button>
			<button class:active={mode === 'code'} class="px-3 py-1.5 rounded-md text-sm inline-flex items-center gap-1.5 {mode === 'code' ? 'bg-white shadow-sm font-medium' : 'text-zinc-500'}" onclick={() => switchMode('code')}><Code size={14} /> Code</button>
			<button class="px-3 py-1.5 rounded-md text-sm inline-flex items-center gap-1.5 {mode === 'page' ? 'bg-white shadow-sm font-medium' : 'text-zinc-500'}" onclick={() => switchMode('page')}><FileCog size={14} /> Page</button>
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
			width="45vw"
			overlay={false}
			onClosed={() => (configId = null)}
		>
			<DrawerTabs active={blockTab} onchange={(t) => (blockTab = t)} />
			{#if blockTab === 'settings'}
				<BlockInspector {doc} ri={configBlock.ri} ci={configBlock.ci} bi={configBlock.bi} {schemas} {items} {relationships} onItemsChanged={refreshItems} />
			{:else}
				<div class="border border-red-200 bg-red-50 rounded-lg p-4 space-y-3">
					<div>
						<p class="text-sm font-medium text-red-700">Delete this component</p>
						<p class="text-xs text-red-500/80 mt-0.5">Removes the block from the column. This cannot be undone (until you hit Save).</p>
					</div>
					<button class="px-3 py-1.5 rounded-lg text-sm font-medium text-white inline-flex items-center gap-1.5 bg-red-600 hover:bg-red-700" onclick={() => { doc.rows![configBlock.ri].columns![configBlock.ci].blocks.splice(configBlock.bi, 1); configId = null; }}><Trash2 size={13} /> Delete component</button>
				</div>
			{/if}
		</ChartConfigDrawer>
	{:else if mode === 'page'}
		<!-- page settings: only non-visual configuration lives here -->
		<div class="max-w-xl space-y-4">
			<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-3">
				<label class="block space-y-1">
					<span class="text-xs text-zinc-500">Page title</span>
					<input type="text" class="w-full border border-zinc-300 rounded px-2 py-1.5 text-sm" bind:value={doc.title} />
				</label>
				<label class="block space-y-1">
					<span class="text-xs text-zinc-500">Slug (read-only)</span>
					<input type="text" class="w-full border border-zinc-200 rounded px-2 py-1.5 text-sm text-zinc-400 font-mono bg-zinc-50" value={doc.slug} disabled />
				</label>
			</div>
			<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-2">
				<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Rows</span>
				{#if doc.rows?.length}
					{#each doc.rows as _, ri (ri)}
						<div class="flex items-center justify-between text-sm">
							<span class="text-zinc-600">Row {ri + 1} · {rowColumns(doc.rows![ri]).length} columns · {rowColumns(doc.rows![ri]).reduce((n, c) => n + c.blocks.length, 0)} blocks</span>
						</div>
					{/each}
				{:else}
					<p class="text-sm text-zinc-400">No rows yet — add one in Design mode.</p>
				{/if}
			</div>
		</div>
	{:else}
		<!-- canvas: rows are the only page-level primitive — components are added inside columns -->
		<div class="space-y-4">
			{#if runtime}
				<PageGrid
					{doc}
					{runtime}
					{schemas}
					{items}
					{relationships}
					onConfigure={(id) => configureBlock(id)}
					onConfigureRow={(ri) => { rowTab = 'settings'; rowConfig = ri; }}
					onConfigureColumn={(ri, ci) => { colTab = 'settings'; colConfig = { ri, ci }; }}
					onCreateMasterItem={createMasterItem}

					onAdd={(ri, ci) => { pickerQuery = ''; pickerFor = { ri, ci }; }}
				onAddRow={addRow}
				/>
			{/if}

		</div>

		<!-- row settings drawer: tabbed — settings | danger zone (row delete lives only there) -->
		{#if rowConfig !== null && doc.rows?.[rowConfig]}
			{@const row = doc.rows[rowConfig]}
			<ChartConfigDrawer open={true} title={`Row ${rowConfig + 1}`} width="45vw" onClosed={() => (rowConfig = null)}>
				<DrawerTabs active={rowTab} onchange={(t) => (rowTab = t)} />

				{#if rowTab === 'settings'}
					<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-3 text-sm">
						<div class="flex items-center justify-between">
							<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Columns</span>
						</div>
						{#each row.columns ?? [] as col, ci (ci)}
							<div class="flex items-center gap-2">
								<span class="text-xs text-zinc-500 w-14">Column {ci + 1}</span>
								<input type="number" min="1" max="12" class="w-20 border border-zinc-300 rounded px-2 py-1" value={col.span ?? 12} onchange={(e) => (col.span = Number((e.target as HTMLInputElement).value))} />
								<span class="text-xs text-zinc-400">/ 12 width</span>
								<button class="ml-auto text-zinc-300 hover:text-red-500 disabled:opacity-30 disabled:hover:text-zinc-300" disabled={(row.columns?.length ?? 0) <= 1} onclick={() => removeColumn(rowConfig!, ci)} title="Remove column"><Trash2 size={13} /></button>
							</div>
						{/each}
						<button class="text-xs text-zinc-500 hover:text-zinc-900 inline-flex items-center gap-1" onclick={() => addColumn(rowConfig!)}><Plus size={12} /> Split into another column</button>
					</div>
					<label class="block space-y-1">
						<span class="text-xs text-zinc-500">Row height (px, optional — components fill it unless they set their own)</span>
						<input type="number" min="40" class="w-full border border-zinc-300 rounded px-2 py-1" placeholder="auto" value={row.height ?? ''} onchange={(e) => { const v = (e.target as HTMLInputElement).value; row.height = v ? Number(v) : undefined; }} />
					</label>
				{:else}
					<div class="border border-red-200 bg-red-50 rounded-lg p-4 space-y-3">
						<div>
							<p class="text-sm font-medium text-red-700">Delete this row</p>
							<p class="text-xs text-red-500/80 mt-0.5">Removes the row and every component inside its columns. This cannot be undone (until you hit Save).</p>
						</div>
						<button class="px-3 py-1.5 rounded-lg text-sm font-medium text-white inline-flex items-center gap-1.5 bg-red-600 hover:bg-red-700" onclick={() => { removeRow(rowConfig!); rowConfig = null; }}><Trash2 size={13} /> Delete row</button>
					</div>
				{/if}
			</ChartConfigDrawer>
		{/if}

		<!-- component picker drawer: search + cards, adds into the target column -->
		{#if pickerFor}
			<ChartConfigDrawer open={true} title="Add component" width="50vw" onClosed={() => (pickerFor = null)}>
				<div class="picker-search">
					<Search size={14} />
					<input type="text" placeholder="Search components…" bind:value={pickerQuery} />
				</div>
				<div class="picker-grid">
					{#each filteredComponents as c (c.key)}
						<button
							class="picker-card"
							onclick={() => { addComponent(pickerFor!.ri, pickerFor!.ci, c.key); pickerFor = null; }}
						>
							<span class="picker-card-ico">
								{#if c.kind === 'chart'}<BarChart3 size={20} />{:else if c.kind === 'table'}<Table size={20} />{:else}<Type size={20} />{/if}
							</span>
							<span class="picker-card-label">{c.label}</span>
							<span class="picker-card-kind">{c.kind}</span>
						</button>
					{/each}
				</div>
				{#if filteredComponents.length === 0}
					<p class="text-sm text-zinc-400">No components match &ldquo;{pickerQuery}&rdquo;.</p>
				{/if}
			</ChartConfigDrawer>
		{/if}

		<!-- column settings drawer: width + remove -->
		{#if colConfig && doc.rows?.[colConfig.ri]?.columns?.[colConfig.ci]}
			{@const col = doc.rows[colConfig.ri].columns![colConfig.ci]}
			<ChartConfigDrawer open={true} title={`Column ${colConfig.ci + 1} settings`} width="45vw" onClosed={() => (colConfig = null)}>
				<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-3 text-sm">
					<label class="block space-y-1">
						<span class="text-xs text-zinc-500">Width (1–12 of the row)</span>
						<input type="number" min="1" max="12" class="w-full border border-zinc-300 rounded px-2 py-1" value={col.span ?? 12} onchange={(e) => (col.span = Number((e.target as HTMLInputElement).value))} />
					</label>
					<label class="block space-y-1">
						<span class="text-xs text-zinc-500">Height (px, optional — overrides the row height)</span>
						<input type="number" min="40" class="w-full border border-zinc-300 rounded px-2 py-1" placeholder="row height / auto" value={col.height ?? ''} onchange={(e) => { const v = (e.target as HTMLInputElement).value; col.height = v ? Number(v) : undefined; }} />
					</label>
					<button class="text-zinc-400 hover:text-red-500 text-xs inline-flex items-center gap-1 disabled:opacity-30 disabled:hover:text-zinc-400" disabled={(doc.rows![colConfig.ri].columns?.length ?? 0) <= 1} onclick={() => { removeColumn(colConfig!.ri, colConfig!.ci); colConfig = null; }}><Trash2 size={12} /> Remove column</button>
				</div>
			</ChartConfigDrawer>
		{/if}
	{/if}
</div>

<style>
	/* component picker drawer content */
	.picker-search {
		position: sticky;
		top: calc(-1 * var(--space-4));
		z-index: 5;
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm, 8px);
		background: var(--color-surface);
		color: var(--color-text-tertiary);
	}
	.picker-search input {
		flex: 1;
		border: none;
		background: transparent;
		outline: none;
		font-size: 14px;
		color: var(--color-text);
	}
	.picker-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: var(--space-3);
	}
	.picker-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: var(--space-2);
		padding: var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md, 10px);
		background: var(--color-surface);
		text-align: left;
		cursor: pointer;
		transition:
			border-color var(--duration-fast, 150ms) ease,
			box-shadow var(--duration-fast, 150ms) ease;
	}
	.picker-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}
	.picker-card-ico {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: var(--radius-sm, 8px);
		background: var(--color-accent-muted);
		color: var(--color-accent);
	}
	.picker-card-label {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text);
	}
	.picker-card-kind {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}
</style>
