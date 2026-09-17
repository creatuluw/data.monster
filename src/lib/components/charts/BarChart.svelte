<script lang="ts" generics="T extends Record<string | symbol, unknown>">
	import { Plot, BarX, HTMLTooltip } from 'svelteplot';
	import type { RawValue } from 'svelteplot/types/data.js';
	import type { Snippet } from 'svelte';
	import { Bolt } from 'lucide-svelte';
	import ChartConfigDrawer from './ChartConfigDrawer.svelte';
	import { sameDatum } from '$lib/charts/fundament';

	let {
		data,
		category,
		value,
		tooltip,
		labelFor,
		selected = $bindable(null),
		formatValue,
		color = '#888888',
		heightVh = 0.3,
		title = 'Bar chart',
		subtitle = '',
		config,
	}: {
		/** rows to plot (pre-aggregated, e.g. via buildBars) */
		data: T[];
		/** band axis accessor — the bar's category */
		category: (d: T) => string;
		/** bar length accessor */
		value: (d: T) => number;
		/** hover popover content */
		tooltip: Snippet<[T]>;
		/** selected-bar pill text */
		labelFor: (d: T) => string;
		selected?: T | null;
		/** value (number, x) axis tick format */
		formatValue?: (v: number) => string;
		/** bar fill when deselected (DS green when selected) */
		color?: string;
		/** plot height as a fraction of viewport height (default 30vh) */
		heightVh?: number;
		/** card header */
		title?: string;
		subtitle?: string;
		/** optional config panel fields — renders a Bolt button top-right that opens the drawer */
		config?: Snippet;
	} = $props();

	let chartWrap: HTMLDivElement | undefined = $state();
	let innerHeight = $state(800);

	// default: 20% of viewport height, override per chart via heightVh
	const plotHeight = $derived(innerHeight * heightVh);

	// one cast: svelteplot marks want RawValue records, a generic T can't prove that
	const markData = $derived(data as unknown as Record<string | symbol, RawValue>[]);

	// selection matched on category position, never object identity —
	// svelteplot re-copies records per transform pass
	const isSel = $derived(sameDatum<T>([category]));

	// y band domain from data order, so the caller's sort wins over svelteplot's
	// default alphabetical ordinal sort; reverse puts domain[0] on top
	const yDomain = $derived(markData.map((d) => category(d as T)));

	function fillOf(d: T): string {
		return selected && isSel(d, selected) ? 'oklch(0.44 0.1 158)' : color;
	}

	function handleBarClick(_: unknown, d: Record<string | symbol, unknown>) {
		selected = d as T;
	}

	function handleChartClick(e: MouseEvent) {
		if (!(e.target as Element).closest('rect,path,button')) selected = null;
	}

	let configOpen = $state(false);

	function handleWindowClick(e: MouseEvent) {
		const t = e.target as Element;
		if (selected && !chartWrap?.contains(t) && !t.closest?.('[data-drawer]')) selected = null;
	}
</script>

<svelte:window onclick={handleWindowClick} bind:innerHeight />

<div
	class="bar-chart relative bg-white rounded-lg border border-zinc-200 p-6 mb-16"
	bind:this={chartWrap}
	role="presentation"
	onclick={handleChartClick}
>
	<div class="mb-4">
		<div class="flex items-center gap-2 flex-wrap">
			<h2 class="bar-chart-title text-lg font-semibold text-zinc-900 tracking-tight">{title}</h2>
			{#if selected}
				<span class="bar-chart-dot text-zinc-300" aria-hidden="true">&middot;</span>
				<span class="bar-chart-sel text-xs">{labelFor(selected)}</span>
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
		{#if data.length === 0}
			<!-- empty data poisons svelteplot scales (NaN transforms), so never
			     render the Plot without rows -->
			<div class="py-16 text-center text-sm text-zinc-400">No data</div>
		{:else}
			<Plot
				padding={0}
				x={{ tickFormat: formatValue ? (d) => formatValue(Number(d)) : undefined, grid: true }}
				y={{ padding: 0.3, domain: yDomain, reverse: true }}
				color={{ legend: false }}
				height={plotHeight}
			>
				<BarX
					data={markData}
					x={(d: Record<string | symbol, RawValue>) => value(d as T)}
					y={(d: Record<string | symbol, RawValue>) => category(d as T)}
					fill={{ scale: null, value: (d: any) => fillOf(d as T) }}
					onclick={handleBarClick}
				/>
				{#snippet overlay()}
					<HTMLTooltip data={markData} x={(d: any) => value(d as T)} y={(d: any) => category(d as T)}>
						{#snippet children({ datum })}
							{#if datum}
								<div
									class="bg-zinc-900 text-white text-xs rounded-md px-2.5 py-1.5 shadow-lg whitespace-nowrap -translate-x-1/2 -translate-y-[calc(100%+8px)]"
								>
									{@render tooltip(datum as T)}
								</div>
							{/if}
						{/snippet}
					</HTMLTooltip>
				{/snippet}
			</Plot>
		{/if}
	</div>

	{#if config}
		<ChartConfigDrawer bind:open={configOpen} title={`${title} configuration`}>
			{@render config()}
		</ChartConfigDrawer>
	{/if}
</div>

<style>
	.bar-chart-title {
		font-family: var(--font-display);
		font-weight: 700;
	}

	/* selected-bar readout inline after the title, mono data detail */
	.bar-chart-sel {
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

	/* chart labels are data detail — mono, and never swallow bar clicks */
	.bar-chart :global(text) {
		pointer-events: none;
		font-family: var(--font-mono);
	}

	.bar-chart :global(.plot-footer) {
		margin-bottom: 0;
	}
</style>
