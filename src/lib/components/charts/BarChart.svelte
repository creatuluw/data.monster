<script lang="ts" generics="T extends Record<string | symbol, unknown>">
	import { Plot, BarY, HTMLTooltip } from 'svelteplot';
	import type { RawValue } from 'svelteplot/types/data.js';
	import type { Snippet } from 'svelte';
	import { sameDatum } from '$lib/charts/fundament';

	let {
		data,
		category,
		value,
		tooltip,
		labelFor,
		selected = $bindable(null),
		formatY,
		color = 'oklch(0.44 0.1 158)',
		title = 'Bar chart',
		subtitle = '',
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
		/** y (number) axis tick format */
		formatY?: (v: number) => string;
		/** bar fill — DS accent by default */
		color?: string;
		/** card header */
		title?: string;
		subtitle?: string;
	} = $props();

	let chartWrap: HTMLDivElement | undefined = $state();

	// one cast: svelteplot marks want RawValue records, a generic T can't prove that
	const markData = $derived(data as unknown as Record<string | symbol, RawValue>[]);

	// selection matched on category position, never object identity —
	// svelteplot re-copies records per transform pass
	const isSel = $derived(sameDatum<T>([category]));

	function strokeOf(d: T): string {
		return selected && isSel(d, selected) ? '#000' : 'none';
	}

	function handleBarClick(_: unknown, d: Record<string | symbol, unknown>) {
		selected = d as T;
	}

	function handleChartClick(e: MouseEvent) {
		if (!(e.target as Element).closest('rect,path')) selected = null;
	}

	function handleWindowClick(e: MouseEvent) {
		const t = e.target as Element;
		if (selected && !chartWrap?.contains(t) && !t.closest?.('[data-drawer]')) selected = null;
	}
</script>

<svelte:window onclick={handleWindowClick} />

<div
	class="bar-chart relative bg-white rounded-lg border border-zinc-200 p-6 mb-16"
	bind:this={chartWrap}
	role="presentation"
	onclick={handleChartClick}
>
	<div class="mb-4">
		<h2 class="bar-chart-title text-lg font-semibold text-zinc-900 tracking-tight">{title}</h2>
		<p class="text-sm text-zinc-500 mt-0.5">{subtitle}</p>
	</div>
	{#if selected}
		<!-- value label, fixed top-right, never affects layout -->
		<div
			class="absolute top-3 right-3 z-10 bg-zinc-900 text-white text-xs rounded-md px-2.5 py-1.5 shadow-lg pointer-events-none"
		>
			{labelFor(selected)}
		</div>
	{/if}
	<div>
		{#if data.length === 0}
			<!-- empty data poisons svelteplot scales (NaN transforms), so never
			     render the Plot without rows -->
			<div class="py-16 text-center text-sm text-zinc-400">No data</div>
		{:else}
			<Plot
				padding={0}
				y={{ tickFormat: formatY ? (d) => formatY(Number(d)) : undefined, grid: true }}
				color={{ legend: false }}
				aspectRatio={2}
			>
				<BarY
					data={markData}
					x={(d: Record<string | symbol, RawValue>) => category(d as T)}
					y={(d: Record<string | symbol, RawValue>) => value(d as T)}
					fill={{ scale: false, value: color }}
					stroke={{ scale: null, value: (d: any) => strokeOf(d as T) }}
					strokeWidth={2}
					onclick={handleBarClick}
				/>
				{#snippet overlay()}
					<HTMLTooltip data={markData} x={(d: any) => category(d as T)} y={(d: any) => value(d as T)}>
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
</div>

<style>
	.bar-chart-title {
		font-family: var(--font-display);
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
