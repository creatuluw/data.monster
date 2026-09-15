<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Bolt } from 'lucide-svelte';
	import ChartConfigDrawer from './ChartConfigDrawer.svelte';

	let {
		title = 'Chart',
		subtitle = '',
		selectionLabel = null,
		status = 'ok',
		error = '',
		missing = '',
		config,
		children
	}: {
		title?: string;
		subtitle?: string;
		selectionLabel?: string | null;
		status?: 'ok' | 'loading' | 'empty' | 'error' | 'missing';
		error?: string;
		missing?: string;
		config?: Snippet;
		children?: Snippet;
	} = $props();

	let configOpen = $state(false);
</script>

<!-- card frame shared by every registry chart (FR-5). mb-16: HTMLTooltip
     layout boxes overflow the card bottom row; no clipping allowed. -->
<div class="chart-card relative bg-white rounded-lg border border-zinc-200 p-6 mb-16" role="presentation">
	<div class="mb-4">
		<div class="flex items-center gap-2 flex-wrap">
			<h2 class="chart-card-title text-lg font-semibold text-zinc-900 tracking-tight">{title}</h2>
			{#if selectionLabel}
				<span class="chart-card-dot text-zinc-300" aria-hidden="true">&middot;</span>
				<span class="chart-card-sel text-xs">{selectionLabel}</span>
			{/if}
		</div>
		<p class="text-sm text-zinc-500 mt-0.5">{subtitle}</p>
	</div>
	{#if config}
		<button
			class="config-btn absolute top-3 right-3"
			onclick={() => (configOpen = !configOpen)}
			title="Configure"
			aria-label="Configure chart"
		>
			<Bolt size={14} />
		</button>
	{/if}
	<div>
		{#if status === 'loading'}
			<div class="py-16 text-center text-sm text-zinc-400">Loading…</div>
		{:else if status === 'missing'}
			<div class="py-16 text-center text-sm text-amber-600">
				Missing master item: <span class="font-mono">{missing}</span>
			</div>
		{:else if status === 'error'}
			<div class="py-16 text-center text-sm text-red-500 break-words">{error}</div>
		{:else if status === 'empty'}
			<!-- empty data poisons svelteplot scales (NaN transforms) -->
			<div class="py-16 text-center text-sm text-zinc-400">No data</div>
		{:else}
			{@render children?.()}
		{/if}
	</div>

	{#if config}
		<ChartConfigDrawer bind:open={configOpen} title={`${title} configuration`}>
			{@render config()}
		</ChartConfigDrawer>
	{/if}
</div>

<style>
	.chart-card-title {
		font-family: var(--font-display);
	}

	/* selection readout inline after the title, mono data detail */
	.chart-card-sel {
		font-family: var(--font-mono);
		color: #71717a;
		letter-spacing: 0.02em;
	}

	.config-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: 1px solid transparent;
		background: none;
		color: #a1a1aa;
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition:
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.config-btn:hover {
		color: var(--color-text, #18181b);
		border-color: #d4d4d8;
	}

	/* chart labels are data detail — mono, and never swallow mark clicks */
	.chart-card :global(text) {
		pointer-events: none;
		font-family: var(--font-mono);
	}

	.chart-card :global(.plot-footer) {
		margin-bottom: 0;
	}
</style>
