<script lang="ts">
	import type { PageDoc } from '$lib/charts/spec-types';
	import type { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import { chartRenderers, setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getChartType } from '$lib/charts/registry';
	import TableRenderer from './renderers/TableRenderer.svelte';
	import ChartCard from './ChartCard.svelte';

	setupChartRegistry();

	let {
		doc,
		runtime,
		selectedId = null,
		onSelect
	}: {
		doc: PageDoc;
		runtime: ReturnType<typeof createPageRuntime>;
		selectedId?: string | null;
		onSelect?: (id: string) => void;
	} = $props();

	function stateFor(ri: number, bi: number) {
		return runtime.states[`r${ri}-b${bi}`];
	}
</script>

<div class="space-y-4">
	{#each doc.rows ?? [] as row, ri}
		<div class="grid grid-cols-12 gap-4 items-start">
			{#each row.blocks as block, bi (ri + '-' + bi)}
				{@const id = `r${ri}-b${bi}`}
				{@const state = stateFor(ri, bi)}
				{@const span = block.span ?? 12}
				<div
					data-block={id}
					style={`grid-column: span ${span} / span ${span};${selectedId === id ? ' box-shadow: 0 0 0 2px oklch(0.44 0.1 158); border-radius: 8px;' : ''}`}
					onclick={(e) => { e.stopPropagation(); onSelect?.(id); }}
				>>>
					{#if block.type === 'text'}
						<div class="bg-white rounded-lg border border-zinc-200 p-6 mb-16 text-sm text-zinc-700 whitespace-pre-wrap">
							{block.text}
						</div>
					{:else if block.type === 'table'}
						<TableRenderer
							rows={state?.rows ?? []}
							columns={block.columns ?? []}
							title={block.title ?? block.table}
							status={state?.error ? 'error' : state?.loading ? 'loading' : 'ok'}
							error={state?.error ?? ''}
						/>
					{:else if block.type === 'chart'}
						{@const def = getChartType(block.chart.type)}
						{@const Renderer = chartRenderers[block.chart.type]}
						{@const sel = runtime.selection && runtime.selection.blockId === id ? runtime.selection : null}
						{#if Renderer && state && !state.missing.length && !state.loading && !state.error && state.dimensionAliases.length > 0}
							<Renderer
								rows={state.rows}
								status={state.loading ? 'loading' : state.error ? 'error' : undefined}
								error={state.error}
								dimensionAliases={state.dimensionAliases}
								measureAliases={state.measureAliases}
								options={state.options}
								annotations={state.annotations}
								title={block.chart.title ?? def?.label ?? block.chart.type}
								subtitle={block.chart.subtitle ?? ''}
								tooltip={block.chart.tooltip}
								selected={sel ? { dimension: sel.dimension, value: sel.value } : null}
								onSelect={(s) => runtime.select(id, s)}
								colorScale={runtime.colorScale}
								fmts={runtime.fmtsFor(state)}
								heightVh={block.chart.heightVh ?? 0.3}
							/>
						{:else}
							<!-- unresolved/missing master item or unloaded state -->
							<ChartCard
								title={block.chart.title ?? block.chart.type}
								subtitle={block.chart.subtitle ?? ''}
								status={state?.missing.length ? 'missing' : state?.loading ? 'loading' : state?.error ? 'error' : 'empty'}
								missing={state?.missing.join(', ') ?? ''}
								error={state?.error ?? ''}
							/>
						{/if}
					{/if}
				</div>
			{/each}
		</div>
	{/each}
</div>
