<script lang="ts">
	import type { Block, PageDoc } from '$lib/charts/spec-types';
	import type { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import { chartRenderers, setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getChartType } from '$lib/charts/registry';
	import TableRenderer from './renderers/TableRenderer.svelte';
	import ChartCard from './ChartCard.svelte';
	import { Bolt } from 'lucide-svelte';

	setupChartRegistry();

	let {
		doc,
		runtime,
		configureId = null,
		onConfigure
	}: {
		doc: PageDoc;
		runtime: ReturnType<typeof createPageRuntime>;
		/** focused config mode: render only this block, full width (editor supplies the 50vw drawer) */
		configureId?: string | null;
		onConfigure?: (id: string) => void;
	} = $props();

	function stateFor(ri: number, bi: number) {
		return runtime.states[`r${ri}-b${bi}`];
	}

	function blockId(ri: number, bi: number) {
		return `r${ri}-b${bi}`;
	}
</script>

{#snippet renderBlock(block: Block, id: string)}
	{@const [ri, bi] = id.replace('r', '').split('-b').map(Number)}
	{@const state = stateFor(ri, bi)}
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
			<ChartCard
				title={block.chart.title ?? block.chart.type}
				subtitle={block.chart.subtitle ?? ''}
				status={state?.missing.length ? 'missing' : state?.loading ? 'loading' : state?.error ? 'error' : 'empty'}
				missing={state?.missing.join(', ') ?? ''}
				error={state?.error ?? ''}
			/>
		{/if}
	{/if}
{/snippet}

{#if configureId !== null}
	<!-- focused config mode: only the configured block, full width of the left half -->
	{#each doc.rows ?? [] as row, ri}
		{#each row.blocks as block, bi (ri + '-' + bi)}
			{#if blockId(ri, bi) === configureId}
				{@render renderBlock(block, blockId(ri, bi))}
			{/if}
		{/each}
	{/each}
{:else}
	<div class="space-y-4">
		{#each doc.rows ?? [] as row, ri}
			<div class="grid grid-cols-12 gap-4 items-start">
				{#each row.blocks as block, bi (ri + '-' + bi)}
					{@const id = blockId(ri, bi)}
					{@const span = block.span ?? 12}
					<div class="relative" style={`grid-column: span ${span} / span ${span};`}>
						<button
							class="config-open-btn"
							onclick={(e) => { e.stopPropagation(); onConfigure?.(id); }}
							title="Configure"
							aria-label="Configure block"
						>
							<Bolt size={14} />
						</button>
						{@render renderBlock(block, id)}
					</div>
				{/each}
			</div>
		{/each}
	</div>
{/if}

<style>
	.config-open-btn {
		position: absolute;
		top: var(--space-3, 12px);
		right: var(--space-3, 12px);
		z-index: 10;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1, 4px);
		border: 1px solid transparent;
		background: white;
		color: #a1a1aa;
		cursor: pointer;
		border-radius: var(--radius-xs, 4px);
		transition:
			color var(--duration-fast, 150ms) ease,
			border-color var(--duration-fast, 150ms) ease;
	}

	.config-open-btn:hover {
		color: var(--color-text, #18181b);
		border-color: #d4d4d8;
	}
</style>
