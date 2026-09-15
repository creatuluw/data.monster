<script lang="ts">
	import type { PageDoc } from '$lib/charts/spec-types';
	import { getChartType } from '$lib/charts/registry';
	import { availableItems } from '$lib/charts/relationships';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import { Trash2, Plus } from 'lucide-svelte';

	let {
		doc,
		ri,
		bi,
		schemas,
		items,
		relationships
	}: {
		doc: PageDoc;
		ri: number;
		bi: number;
		schemas: TableSchemas;
		items: MasterItem[];
		relationships: Relationship[];
	} = $props();

	const block = $derived(doc.rows![ri].blocks[bi]);
	const chart = $derived(block.type === 'chart' ? block.chart : null);
	const def = $derived(chart ? getChartType(chart.type) : null);
	const tableCols = $derived(
		block.type === 'chart' ? (schemas[chart!.source.table] ?? []) : block.type === 'table' ? (schemas[block.table] ?? []) : []
	);
	const usableItems = $derived(
		chart ? availableItems(chart.source.table, items, relationships) : []
	);

	const OPS = ['=', '!=', '>', '<', '>=', '<=', 'in', 'like'];
	const GRAINS = ['(none)', 'hour', 'day', 'week', 'month', 'quarter', 'year', 'day_of_week', 'month_of_year'];
	const FMTS = ['(none)', 'hours', 'usd', 'pct', 'int'];
	const itemIds = $derived(new Set(items.map((i) => i.id)));

	function removeBlock() {
		doc.rows![ri].blocks.splice(bi, 1);
		if (doc.rows![ri].blocks.length === 0) doc.rows!.splice(ri, 1);
	}
	function addDimension() {
		if (chart) chart.dimensions.push({ col: tableCols[0] ?? '' });
	}
	function addMeasure() {
		if (chart) chart.measures.push({ expr: 'count(*)', label: 'Count' });
	}
	function addAnnotation() {
		if (chart && def) {
			const mark = def.annotations[0];
			if (mark) chart.annotations = [...(chart.annotations ?? []), { mark, at: 0 } as never];
		}
	}
</script>

<div class="bg-white rounded-lg border border-zinc-200 p-4 space-y-4 text-sm">
	<div class="flex items-center justify-between">
		<h3 class="font-semibold text-zinc-900">
			{block.type === 'chart' ? `${def?.label ?? chart!.type} chart` : block.type} block
		</h3>
		<button class="text-zinc-400 hover:text-red-500" onclick={removeBlock} title="Remove block">
			<Trash2 size={14} />
		</button>
	</div>

	<!-- shared fields -->
	<div class="grid grid-cols-2 gap-2">
		<label class="space-y-1">
			<span class="text-xs text-zinc-500">Span (1–12)</span>
			<input type="number" min="1" max="12" class="w-full border border-zinc-300 rounded px-2 py-1" value={block.span ?? 12} onchange={(e) => (block.span = Number((e.target as HTMLInputElement).value))} />
		</label>
		<label class="space-y-1">
			<span class="text-xs text-zinc-500">Title</span>
			<input type="text" class="w-full border border-zinc-300 rounded px-2 py-1" value={block.type === 'text' ? '' : (block as { title?: string }).title ?? ''} onchange={(e) => ((block as { title?: string }).title = (e.target as HTMLInputElement).value || undefined)} />
		</label>
	</div>

	{#if block.type === 'text'}
		<label class="block space-y-1">
			<span class="text-xs text-zinc-500">Text</span>
			<textarea rows="4" class="w-full border border-zinc-300 rounded px-2 py-1" value={block.text} oninput={(e) => (block.text = (e.target as HTMLTextAreaElement).value)}></textarea>
		</label>
	{:else if block.type === 'table'}
		<label class="block space-y-1">
			<span class="text-xs text-zinc-500">Table</span>
			<select class="w-full border border-zinc-300 rounded px-2 py-1" value={block.table} onchange={(e) => (block.table = (e.target as HTMLSelectElement).value)}>
				{#each Object.keys(schemas) as t (t)}<option value={t}>{t}</option>{/each}
			</select>
		</label>
		<div class="grid grid-cols-2 gap-2">
			<label class="space-y-1">
				<span class="text-xs text-zinc-500">Limit</span>
				<input type="number" min="1" class="w-full border border-zinc-300 rounded px-2 py-1" value={block.limit ?? 50} onchange={(e) => (block.limit = Number((e.target as HTMLInputElement).value))} />
			</label>
		</div>
	{:else if chart}
		<label class="block space-y-1">
			<span class="text-xs text-zinc-500">Source table</span>
			<select class="w-full border border-zinc-300 rounded px-2 py-1" value={chart.source.table} onchange={(e) => (chart.source.table = (e.target as HTMLSelectElement).value)}>
				{#each Object.keys(schemas) as t (t)}<option value={t}>{t}</option>{/each}
			</select>
		</label>

		<!-- roles -->
		<div class="space-y-2">
			<div class="flex items-center justify-between">
				<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Dimensions</span>
				<button class="text-zinc-400 hover:text-zinc-900 inline-flex items-center gap-1 text-xs" onclick={addDimension}><Plus size={12} /> add</button>
			</div>
			{#each chart.dimensions as d, i (i)}
				<div class="flex gap-1 items-center">
					{#if 'ref' in d}
						<select class="flex-1 border border-zinc-300 rounded px-2 py-1" value={d.ref} onchange={(e) => (chart.dimensions[i] = { ref: (e.target as HTMLSelectElement).value })}>
							{#each usableItems.filter((it) => it.kind === 'dimension') as it (it.id)}<option value={it.id}>⭐ {it.label}</option>{/each}
						</select>
					{:else}
						<select class="flex-1 border border-zinc-300 rounded px-2 py-1" value={d.col} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; chart.dimensions[i] = itemIds.has(v) ? { ref: v } : { col: v }; }}>
							{#each tableCols as c (c)}<option value={c}>{c}</option>{/each}
							{#each usableItems.filter((it) => it.kind === 'dimension') as it (it.id)}<option value={it.id}>⭐ {it.label}</option>{/each}
						</select>
						<select class="border border-zinc-300 rounded px-1 py-1 text-xs" value={d.grain ?? '(none)'} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; chart.dimensions[i] = { col: d.col, grain: v === '(none)' ? undefined : (v as never) }; }}>
							{#each GRAINS as g (g)}<option value={g}>{g}</option>{/each}
						</select>
					{/if}
					<button class="text-zinc-300 hover:text-red-500" onclick={() => chart.dimensions.splice(i, 1)}>×</button>
				</div>
			{/each}
		</div>

		<div class="space-y-2">
			<div class="flex items-center justify-between">
				<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Measures</span>
				<button class="text-zinc-400 hover:text-zinc-900 inline-flex items-center gap-1 text-xs" onclick={addMeasure}><Plus size={12} /> add</button>
			</div>
			{#each chart.measures as m, i (i)}
				<div class="space-y-1 border border-zinc-100 rounded p-2">
					<div class="flex gap-1">
						{#if 'ref' in m}
							<select class="flex-1 border border-zinc-300 rounded px-2 py-1" value={m.ref} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; chart.measures[i] = v ? { ref: v } : { expr: 'count(*)', label: 'Count' }; }}>
								{#each usableItems.filter((it) => it.kind === 'measure') as it (it.id)}<option value={it.id}>⭐ {it.label}</option>{/each}
								<option value="">(expression)</option>
							</select>
						{:else}
							<input type="text" placeholder="expression" class="flex-1 border border-zinc-300 rounded px-2 py-1 font-mono text-xs" value={m.expr} oninput={(e) => (chart.measures[i] = { ...m, expr: (e.target as HTMLInputElement).value })} />
							<select class="border border-zinc-300 rounded px-1 py-1 text-xs" value="" title="Use master measure" onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) chart.measures[i] = { ref: v }; }}>
								<option value="" disabled hidden>⭐</option>
								{#each usableItems.filter((it) => it.kind === 'measure') as it (it.id)}<option value={it.id}>{it.label}</option>{/each}
							</select>
						{/if}
						<button class="text-zinc-300 hover:text-red-500" onclick={() => chart.measures.splice(i, 1)}>×</button>
					</div>
					<div class="grid grid-cols-2 gap-1">
						<input type="text" placeholder="label" class="border border-zinc-300 rounded px-2 py-1" value={'label' in m ? (m.label ?? '') : ''} oninput={(e) => (chart.measures[i] = { ...m, label: (e.target as HTMLInputElement).value || undefined })} />
						<select class="border border-zinc-300 rounded px-1 py-1 text-xs" value={'fmt' in m ? m.fmt ?? '(none)' : '(none)'} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; chart.measures[i] = { ...m, fmt: v === '(none)' ? undefined : v } as never; }}>
							{#each FMTS as f (f)}<option value={f}>{f}</option>{/each}
						</select>
					</div>
				</div>
			{/each}
		</div>

		<!-- schema-driven options (FR-9/Q9) -->
		{#if def}
			<div class="space-y-2">
				<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Options</span>
				{#each def.optionsSchema as field (field.name)}
					<label class="grid grid-cols-2 gap-2 items-center">
						<span class="text-xs text-zinc-500">{field.label}</span>
						{#if field.kind === 'enum'}
							<select class="border border-zinc-300 rounded px-2 py-1" value={String(chart.options?.[field.name] ?? field.default ?? '')} onchange={(e) => { chart.options = { ...chart.options, [field.name]: (e.target as HTMLSelectElement).value }; }}>
								{#each field.options ?? [] as o (o)}<option value={o}>{o}</option>{/each}
							</select>
						{:else if field.kind === 'boolean'}
							<input type="checkbox" checked={Boolean(chart.options?.[field.name] ?? field.default)} onchange={(e) => { chart.options = { ...chart.options, [field.name]: (e.target as HTMLInputElement).checked }; }} />
						{:else}
							<input type={field.kind === 'number' ? 'number' : 'text'} class="border border-zinc-300 rounded px-2 py-1" value={String(chart.options?.[field.name] ?? field.default ?? '')} onchange={(e) => { const raw = (e.target as HTMLInputElement).value; chart.options = { ...chart.options, [field.name]: field.kind === 'number' ? (raw === '' ? undefined : Number(raw)) : raw }; }} />
						{/if}
					</label>
				{/each}
			</div>
		{/if}

		<!-- annotations (per-type whitelist) -->
		{#if def && def.annotations.length}
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Annotations</span>
					<button class="text-zinc-400 hover:text-zinc-900 inline-flex items-center gap-1 text-xs" onclick={addAnnotation}><Plus size={12} /> add</button>
				</div>
				{#each chart.annotations ?? [] as a, i (i)}
					<div class="flex gap-1 items-center">
						<select class="border border-zinc-300 rounded px-1 py-1 text-xs" value={a.mark} onchange={(e) => { chart.annotations![i] = { ...a, mark: (e.target as HTMLSelectElement).value } as never; }}>
							{#each def.annotations as mk (mk)}<option value={mk}>{mk}</option>{/each}
						</select>
						{#if typeof a.at === 'number'}
							<input type="number" class="flex-1 border border-zinc-300 rounded px-2 py-1" value={a.at} onchange={(e) => { chart.annotations![i] = { ...a, at: Number((e.target as HTMLInputElement).value) } as never; }} />
						{:else}
							<input type="text" placeholder="expression" class="flex-1 border border-zinc-300 rounded px-2 py-1 font-mono text-xs" value={a.at.expr} oninput={(e) => { chart.annotations![i] = { ...a, at: { expr: (e.target as HTMLInputElement).value } } as never; }} />
						{/if}
						<button class="text-zinc-300 hover:text-red-500" onclick={() => chart.annotations!.splice(i, 1)}>×</button>
					</div>
				{/each}
			</div>
		{/if}

		<!-- tooltip -->
		<label class="block space-y-1">
			<span class="text-xs text-zinc-500">Tooltip template</span>
			<input type="text" placeholder="{'{dimension}: {measure}'}" class="w-full border border-zinc-300 rounded px-2 py-1 font-mono text-xs" value={chart.tooltip?.template ?? ''} oninput={(e) => { const v = (e.target as HTMLInputElement).value; chart.tooltip = v ? { ...chart.tooltip, template: v } : undefined; }} />
		</label>

		<div class="grid grid-cols-2 gap-2">
			<label class="space-y-1">
				<span class="text-xs text-zinc-500">Height (vh fraction)</span>
				<input type="number" min="0.1" max="1" step="0.05" class="w-full border border-zinc-300 rounded px-2 py-1" value={chart.heightVh ?? 0.3} onchange={(e) => (chart.heightVh = Number((e.target as HTMLInputElement).value))} />
			</label>
			<label class="space-y-1">
				<span class="text-xs text-zinc-500">Limit</span>
				<input type="number" min="1" class="w-full border border-zinc-300 rounded px-2 py-1" value={chart.limit ?? ''} onchange={(e) => (chart.limit = (e.target as HTMLInputElement).value === '' ? undefined : Number((e.target as HTMLInputElement).value))} />
			</label>
		</div>
	{/if}
</div>
