<script lang="ts">
	import type { ChartDataPoint, ChartOptions, ColorScale, BarClickDetail } from './types';

	interface Props {
		data: ChartDataPoint[];
		colorScale?: ColorScale;
		options?: ChartOptions;
		onbarclick?: (detail: BarClickDetail) => void;
	}

	let { data, colorScale, options = {}, onbarclick }: Props = $props();

	let containerRef: HTMLDivElement | undefined = $state();

	const BAR_HEIGHT = $derived(options.barHeight ?? 28);
	const BAR_GAP = $derived(options.barGap ?? 4);
	const LABEL_WIDTH = $derived(options.labelWidth ?? 100);
	const PADDING_RIGHT = $derived(60);
	const PADDING_LEFT = $derived(8);
	const PADDING_TOP = $derived(options.title ? 36 : 8);
	const PADDING_BOTTOM = $derived(8);

	let sorted = $derived(() => {
		const order = options.sortOrder ?? 'asc';
		return [...data].sort((a, b) =>
			order === 'asc' ? a.value - b.value : b.value - a.value
		);
	});

	let maxValue = $derived(Math.max(1, ...data.map((d) => d.value)));
	let chartWidth = $derived(800);
	let chartHeight = $derived(
		PADDING_TOP + data.length * (BAR_HEIGHT + BAR_GAP) - BAR_GAP + PADDING_BOTTOM
	);

	let barAreaWidth = $derived(chartWidth - LABEL_WIDTH - PADDING_LEFT - PADDING_RIGHT);

	function barWidth(value: number): number {
		return (value / maxValue) * barAreaWidth;
	}

	function barY(index: number): number {
		return PADDING_TOP + index * (BAR_HEIGHT + BAR_GAP);
	}

	function valueX(value: number): number {
		return LABEL_WIDTH + PADDING_LEFT + barWidth(value) + 8;
	}

	function getColor(group: string, index: number): string {
		if (colorScale && colorScale[group]) return colorScale[group];
		const palette = [
			'oklch(0.69 0.16 41)',
			'oklch(0.58 0.18 250)',
			'oklch(0.68 0.14 160)',
			'oklch(0.62 0.17 145)',
			'oklch(0.78 0.09 41)',
			'oklch(0.58 0.2 280)',
			'oklch(0.72 0.13 190)',
			'oklch(0.66 0.16 130)',
			'oklch(0.58 0.17 22)',
			'oklch(0.76 0.13 80)'
		];
		return palette[index % palette.length];
	}

	function handleClick(point: ChartDataPoint, index: number) {
		onbarclick?.({ group: point.group, index });
	}

	function formatValue(value: number): string {
		if (options.valueFormatter) return options.valueFormatter(value);
		return value.toLocaleString();
	}

	$effect(() => {
		if (!containerRef) return;
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				chartWidth = entry.contentRect.width;
			}
		});
		observer.observe(containerRef);
		return () => observer.disconnect();
	});
</script>

<div class="chart-container" bind:this={containerRef}>
	{#if options.title}
		<span class="chart-title">{options.title}</span>
	{/if}
	<svg
		width="100%"
		height={chartHeight}
		viewBox="0 0 {chartWidth} {chartHeight}"
		role="img"
		aria-label={options.title ?? 'Bar chart'}
	>
		{#each sorted() as point, i (point.group)}
			<g class="bar-row" role="button" tabindex="0" onclick={() => handleClick(point, i)} onkeydown={(e) => e.key === 'Enter' && handleClick(point, i)}>
				<text
					x={LABEL_WIDTH - 8}
					y={barY(i) + BAR_HEIGHT / 2}
					dominant-baseline="central"
					text-anchor="end"
					class="bar-label"
				>
					{point.group}
				</text>
				<rect
					x={LABEL_WIDTH + PADDING_LEFT}
					y={barY(i)}
					width={barWidth(point.value)}
					height={BAR_HEIGHT}
					rx="2"
					fill={getColor(point.group, i)}
					class="bar-fill"
				/>
				<text
					x={valueX(point.value)}
					y={barY(i) + BAR_HEIGHT / 2}
					dominant-baseline="central"
					class="bar-value"
				>
					{formatValue(point.value)}
				</text>
			</g>
		{/each}
	</svg>
</div>

<style>
	.chart-container {
		width: 100%;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		box-shadow: var(--shadow-sm);
	}

	.chart-title {
		display: block;
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text-secondary);
		margin-bottom: var(--space-2);
		letter-spacing: -0.01em;
	}

	.bar-row {
		cursor: pointer;
	}

	.bar-row:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.bar-fill {
		transition: width 0.4s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s ease;
	}

	.bar-row:hover .bar-fill {
		opacity: 0.85;
	}

	.bar-label {
		font-family: var(--font-mono);
		font-size: 10px;
		fill: var(--color-text-secondary);
		letter-spacing: 0.02em;
	}

	.bar-value {
		font-family: var(--font-mono);
		font-size: 10px;
		fill: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}
</style>
