<script lang="ts">
	import { BarY, BarX, Plot } from 'svelteplot';
	import type { ComponentProps } from 'svelte';

	interface BarChartProps {
		data: any[];
		x?: string;
		y?: string | string[];
		series?: string;
		title?: string;
		subtitle?: string;
		xAxisTitle?: string | boolean;
		yAxisTitle?: string | boolean;
		xFmt?: string;
		yFmt?: string;
		type?: 'stacked' | 'grouped';
		swapXY?: boolean;
		labels?: boolean;
		legend?: boolean;
		fillColor?: string;
		fillOpacity?: number;
		colorPalette?: string[];
		xGridlines?: boolean;
		yGridlines?: boolean;
		xAxisLabels?: boolean;
		yAxisLabels?: boolean;
		yMin?: number;
		yMax?: number;
		chartAreaHeight?: number;
		sort?: boolean | string;
		emptyMessage?: string;
	}

	let {
		data = [],
		x,
		y,
		series,
		title,
		subtitle,
		xAxisTitle = false,
		yAxisTitle = false,
		xFmt,
		yFmt,
		type = 'stacked',
		swapXY = false,
		labels = false,
		legend = true,
		fillColor = '#CA3500',
		fillOpacity = 1,
		colorPalette,
		xGridlines = false,
		yGridlines = true,
		xAxisLabels = true,
		yAxisLabels = true,
		yMin,
		yMax,
		chartAreaHeight = 400,
		sort = true,
		emptyMessage = 'No records'
	}: BarChartProps = $props();

	// Auto-detect x and y if not provided
	let xColumn = $derived(x || (data.length > 0 ? Object.keys(data[0])[0] : ''));
	let yColumn = $derived.by(() => {
		if (y) return Array.isArray(y) ? y[0] : y;
		if (data.length > 0) {
			const keys = Object.keys(data[0]);
			// Find first numeric column that's not x
			return keys.find((k) => k !== xColumn && typeof data[0][k] === 'number') || keys[1] || '';
		}
		return '';
	});

	// Sort data if sort is enabled
	let sortedData = $derived.by(() => {
		if (!data || data.length === 0) return data;
		
		if (sort === true) {
			// Sort by value descending
			return [...data].sort((a, b) => {
				const aVal = Number(a[yColumn]) || 0;
				const bVal = Number(b[yColumn]) || 0;
				return bVal - aVal; // descending
			});
		}
		
		return data;
	});

	let containerWidth = $state(0);
	let containerHeight = $state(chartAreaHeight);
	let containerEl = $state<HTMLDivElement>();
	let tooltipData = $state<{ x: number; y: number; datum: any } | null>(null);
	let clicked = $state<string | null>(null);

	function handleClick(event: any, datum: any) {
		clicked = datum[xColumn];
	}

	function handleMouseMove(event: any, datum: any) {
		if (containerEl) {
			const rect = containerEl.getBoundingClientRect();
			tooltipData = {
				x: event.clientX - rect.left,
				y: event.clientY - rect.top,
				datum
			};
		}
	}

	function handleMouseLeave() {
		tooltipData = null;
	}

	$effect(() => {
		if (containerEl) {
			const updateSize = () => {
				if (containerEl) {
					containerWidth = containerEl.clientWidth;
					containerHeight = chartAreaHeight;
				}
			};

			updateSize();
			const resizeObserver = new ResizeObserver(updateSize);
			resizeObserver.observe(containerEl);

			return () => resizeObserver.disconnect();
		}
	});

	// Format value based on fmt string
	function formatValue(value: any, fmt?: string): string {
		if (value == null) return '';
		
		if (fmt) {
			// Simple format handling (can be expanded)
			if (fmt.startsWith('usd')) {
				const decimals = parseInt(fmt.replace('usd', '').replace('k', '')) || 0;
				const inK = fmt.includes('k');
				const val = inK ? value / 1000 : value;
				return '$' + val.toFixed(decimals) + (inK ? 'k' : '');
			}
			if (fmt.startsWith('pct')) {
				const decimals = parseInt(fmt.replace('pct', '')) || 0;
				return (value * 100).toFixed(decimals) + '%';
			}
			if (fmt.startsWith('num')) {
				const decimals = parseInt(fmt.replace('num', '')) || 0;
				return value.toFixed(decimals);
			}
		}
		
		return typeof value === 'number' ? value.toLocaleString() : String(value);
	}
</script>

<div class="chart-container">
	{#if title || subtitle}
		<div class="chart-header">
			{#if title}
				<h3 class="chart-title">{title}</h3>
			{/if}
			{#if subtitle}
				<p class="chart-subtitle">{subtitle}</p>
			{/if}
		</div>
	{/if}

	{#if data.length === 0}
		<div class="empty-state">
			<p>{emptyMessage}</p>
		</div>
	{:else}
		<div class="chart-area" bind:this={containerEl} style="height: {chartAreaHeight}px;">
			{#if containerWidth > 0 && containerHeight > 0}
				<Plot
					width={containerWidth}
					height={containerHeight}
					marginLeft={yAxisLabels ? 60 : 20}
					marginBottom={xAxisLabels ? 60 : 20}
					marginTop={20}
					marginRight={20}
				>
					{#if swapXY}
						<BarX
							data={sortedData}
							x={yColumn}
							y={xColumn}
							fill={(d) => {
								if (colorPalette && colorPalette.length > 0) {
									const index = sortedData.findIndex((item) => item === d);
									return colorPalette[index % colorPalette.length];
								}
								return clicked === d[xColumn] ? '#222222' : fillColor;
							}}
							fillOpacity={fillOpacity}
							style="cursor: pointer;"
							onmousemove={handleMouseMove as any}
							onmouseleave={handleMouseLeave}
							onclick={handleClick as any}
						/>
					{:else}
						<BarY
							data={sortedData}
							x={xColumn}
							y={yColumn}
							fill={(d) => {
								if (colorPalette && colorPalette.length > 0) {
									const index = sortedData.findIndex((item) => item === d);
									return colorPalette[index % colorPalette.length];
								}
								return clicked === d[xColumn] ? '#222222' : fillColor;
							}}
							fillOpacity={fillOpacity}
							style="cursor: pointer;"
							onmousemove={handleMouseMove as any}
							onmouseleave={handleMouseLeave}
							onclick={handleClick as any}
						/>
					{/if}
				</Plot>
			{/if}

			<!-- Tooltip -->
			{#if tooltipData}
				<div
					class="tooltip"
					style="
						position: absolute;
						left: {tooltipData.x + 10}px;
						top: {tooltipData.y - 10}px;
						pointer-events: none;
						z-index: 1000;
					"
				>
					<div class="tooltip-title">{tooltipData.datum[xColumn]}</div>
					<div class="tooltip-value">{yColumn}: {formatValue(tooltipData.datum[yColumn], yFmt)}</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.chart-container {
		width: 100%;
		display: flex;
		flex-direction: column;
	}

	.chart-header {
		margin-bottom: 1rem;
	}

	.chart-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: rgb(17, 24, 39);
		margin: 0 0 0.25rem 0;
	}

	:global(.dark) .chart-title {
		color: rgb(243, 244, 246);
	}

	.chart-subtitle {
		font-size: 0.875rem;
		color: rgb(107, 114, 128);
		margin: 0;
	}

	:global(.dark) .chart-subtitle {
		color: rgb(156, 163, 175);
	}

	.chart-area {
		width: 100%;
		position: relative;
	}

	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		background: rgb(249, 250, 251);
		border-radius: 0.5rem;
		color: rgb(107, 114, 128);
	}

	:global(.dark) .empty-state {
		background: rgb(31, 41, 55);
		color: rgb(156, 163, 175);
	}

	.tooltip {
		background: rgb(31, 41, 55);
		border: 1px solid rgb(55, 65, 81);
		color: white;
		padding: 8px 12px;
		border-radius: 6px;
		font-size: 14px;
		line-height: 1.4;
		box-shadow:
			rgba(50, 50, 93, 0.25) 0px 2px 5px -1px,
			rgba(0, 0, 0, 0.3) 0px 1px 3px -1px;
	}

	.tooltip-title {
		font-weight: 600;
		margin-bottom: 4px;
	}

	.tooltip-value {
		color: rgb(209, 213, 219);
	}
</style>

