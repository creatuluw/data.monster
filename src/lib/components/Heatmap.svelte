<script lang="ts" generics="T extends Record<string | symbol, unknown>">
	import { Plot, Cell, HTMLTooltip, Text, ColorLegend } from "svelteplot";
	import type { ColorScheme } from "svelteplot/types/colorScheme.js";
	import type { RawValue } from "svelteplot/types/data.js";
	import type { Snippet } from "svelte";

	let {
		data,
		x,
		y,
		value,
		threshold,
		scheme = "orrd",
		isEmpty,
		tooltip,
		labelFor,
		label,
		selected = $bindable(null),
		xTicks,
		formatX,
		yTicks,
		formatY,
		title = "Heatmap",
		subtitle = "",
	}: {
		/** rows to plot */
		data: T[];
		/** column accessor (e.g. day of month) */
		x: (d: T) => number;
		/** row accessor (e.g. month 0-11) */
		y: (d: T) => number;
		/** color value accessor */
		value: (d: T) => number;
		/** color threshold domain */
		threshold: number[];
		scheme?: ColorScheme;
		/** cells matching this predicate render as no-data (fixed color) */
		isEmpty?: (d: T) => boolean;
		/** hover popover content */
		tooltip: Snippet<[T]>;
		/** selected-cell pill text */
		labelFor: (d: T) => string;
		/** centered in-cell value label (e.g. utilization %) */
		label?: (d: T) => string;
		selected?: T | null;
		/** axis ticks + tick labels for the x data axis */
		xTicks?: number[];
		formatX?: (v: number) => string;
		/** axis ticks + tick labels for the y data axis */
		yTicks?: number[];
		formatY?: (v: number) => string;
		/** card header (override per chart) */
		title?: string;
		subtitle?: string;
	} = $props();

	let clientWidth = $state(500);
	let chartWrap: HTMLDivElement | undefined = $state();

	// flip axes on narrow containers for readability.
	const ax = $derived(clientWidth < 600 ? "y" : "x");
	const ay = $derived(clientWidth < 600 ? "x" : "y");

	// one cast: svelteplot marks want RawValue records, a generic T can't prove that
	const markData = $derived(data as unknown as Record<string | symbol, RawValue>[]);
	const emptyData = $derived(
		isEmpty ? markData.filter((d) => isEmpty(d as T)) : [],
	);
	const filledData = $derived(
		isEmpty ? markData.filter((d) => !isEmpty(d as T)) : markData,
	);

	function handleCellClick(_: unknown, d: Record<string | symbol, unknown>) {
		selected = d as T;
	}

	// per-datum stroke channel — survives svelteplot re-renders (DOM attribute
	// pokes get wiped by its scale-measurement passes). Records are re-copied
	// per transform pass, so match on grid position, not identity.
	function strokeOf(d: T): string {
		return selected && x(d) === x(selected) && y(d) === y(selected)
			? "#000"
			: "none";
	}

	function handleChartClick(e: MouseEvent) {
		// two .plot-body elements exist (legend + main) — check the target
		if (!(e.target as Element).closest("rect")) selected = null;
	}

	function handleWindowClick(e: MouseEvent) {
		// don't deselect when interacting with an open drawer (e.g. the detail
		// panel opened from a cell click) — only true outside clicks clear
		const t = e.target as Element;
		if (selected && !chartWrap?.contains(t) && !t.closest?.("[data-drawer]"))
			selected = null;
	}
</script>

<svelte:window onclick={handleWindowClick} />

<!-- mb-16: HTMLTooltip layout boxes extend below their cells even though
     the visual is translated up — on the bottom row they overflowed the card
     and made the page scrollbar flicker. No overflow clipping: top-row and
     first/last-column popups must render outside the card. -->
<div
	class="heatmap relative bg-white rounded-lg border border-zinc-200 p-6 mb-16"
	bind:this={chartWrap}
	role="presentation"
	onclick={handleChartClick}
>
	<div class="mb-4">
		<h2 class="heatmap-title text-lg font-semibold text-zinc-900 tracking-tight">
			{title}
		</h2>
		<p class="text-sm text-zinc-500 mt-0.5">
			{subtitle}
		</p>
	</div>
	{#if selected}
		<!-- value label, fixed top-right, never affects layout -->
		<div
			class="absolute top-3 right-3 z-10 bg-zinc-900 text-white text-xs rounded-md px-2.5 py-1.5 shadow-lg pointer-events-none"
		>
			{labelFor(selected)}
		</div>
	{/if}
	<div bind:clientWidth>
		{#if data.length === 0}
			<!-- empty data poisons svelteplot scales (NaN transforms), so never
			     render the Plot without rows -->
			<div class="py-16 text-center text-sm text-zinc-400">No data</div>
		{:else}
			<Plot
				padding={0}
				color={{
					scheme,
					type: "threshold",
					domain: threshold,
					legend: false,
				}}
				aspectRatio={clientWidth < 600 ? 2 : 1}
				{...{
					[ax]: {
						ticks: xTicks,
						tickFormat: formatX,
					},
					[ay]: {
						ticks: yTicks,
						tickFormat: formatY,
						axis: clientWidth < 600 ? "both" : "left",
					},
				}}
			>
				<Cell
					data={filledData}
					{...{
						[ax]: x,
						[ay]: y,
					}}
					fill={value}
					stroke={{ scale: null, value: strokeOf }}
					strokeWidth={2}
					inset={0.5}
					onclick={handleCellClick}
				/>
				{#if isEmpty}
					<!-- no-data cells: fixed color, off the threshold scale -->
					<Cell
						data={emptyData}
						{...{
							[ax]: x,
							[ay]: y,
						}}
						fill="#eee"
						stroke={{ scale: null, value: strokeOf }}
						strokeWidth={2}
						inset={0.5}
						onclick={handleCellClick}
					/>
				{/if}
				{#if label}
					<!-- value label on top of filled cells; white on dark cells past the
						 3rd threshold stop, dark gray below it -->
					<Text
						data={filledData}
						{...{
							[ax]: x,
							[ay]: y,
						}}
						text={(d: any) => label(d as T)}
						fontSize={13}
						fontWeight={600}
						fill={{
							scale: null,
							value: (d: any) =>
								value(d as T) >= (threshold[2] ?? 0.75) ? "#fff" : "#3f3f46",
						}}
					/>
				{/if}
				{#snippet overlay()}
					<HTMLTooltip data={markData} {...{ [ax]: x, [ay]: y }}>
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
				{#snippet footer()}
					<div class="flex justify-end">
						<ColorLegend class={null} />
					</div>
				{/snippet}
			</Plot>
		{/if}
	</div>
</div>

<style>
	.heatmap-title {
		font-family: var(--font-display);
	}

	/* cell labels must not swallow clicks/hovers meant for the cell rects */
	.heatmap :global(text) {
		pointer-events: none;
		font-family: var(--font-mono);
	}

	/* svelteplot puts margin-bottom: 2rem on the legend footer */
	.heatmap :global(.plot-footer) {
		margin-bottom: 0;
	}
</style>
