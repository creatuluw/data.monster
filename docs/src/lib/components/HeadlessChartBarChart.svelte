<script lang="ts">
	import Widget from '@meursyphus/flitter-svelte';
	import { BarChart } from '@meursyphus/headless-chart';
	import { Container, BoxDecoration, EdgeInsets, BorderRadius } from '@meursyphus/flitter';

	interface Props {
		data: any[];
		xKey: string;
		yKeys: string[];
		width?: number;
		height?: number;
		chartTitle?: string;
		direction?: 'vertical' | 'horizontal';
	}

	let {
		data = [],
		xKey = '',
		yKeys = [],
		width = 800,
		height = 400,
		chartTitle = 'Bar Chart',
		direction = 'vertical'
	}: Props = $props();

	// Echarts-inspired colors
	function getColor(index: number): string {
		const colors = [
			'#5470c6', // Blue (Echarts style)
			'#91cc75', // Green
			'#fac858', // Yellow
			'#ee6666', // Red
			'#73c0de', // Light Blue
			'#3ba272', // Dark Green
			'#fc8452', // Orange
			'#9a60b4'  // Purple
		];
		return colors[index % colors.length];
	}

	// Transform data to Headless Chart format
	const chartData = $derived.by(() => {
		if (!data.length || !xKey || !yKeys.length) {
			return null;
		}

		const labels = data.map((item) => String(item[xKey]));

		const datasets = yKeys.map((key, index) => ({
			legend: key,
			values: data.map((item) => {
				const val = item[key];
				return typeof val === 'number' ? val : Number(val) || 0;
			})
		}));

		return {
			labels,
			datasets
		};
	});

	// Create the chart widget - Echarts-inspired simple implementation
	const chartWidget = $derived.by(() => {
		if (!chartData) return null;

		try {
			return BarChart({
				data: chartData,
				title: chartTitle,
				direction,
				custom: {
					bar: ({ datasetIndex }) => {
						const color = getColor(datasetIndex);
						
						return Container({
							width: Infinity,
							height: Infinity,
							margin: EdgeInsets.symmetric({ horizontal: 2 }),
							decoration: new BoxDecoration({
								color: color,
								borderRadius: BorderRadius.circular(2)
							})
						});
					}
				}
			});
		} catch (error) {
			console.error('Error creating Headless Chart:', error);
			console.error('Chart data:', chartData);
			return null;
		}
	});
</script>

{#if chartWidget}
	<div class="chart-container" style="width: {width}px; height: {height}px;">
		<div class="chart-wrapper">
			<Widget 
				width="{width - 32}px" 
				height="{height - 32}px" 
				widget={chartWidget}
			/>
		</div>
	</div>
{:else if chartData === null}
	<div class="flex items-center justify-center" style="width: {width}px; height: {height}px;">
		<p class="text-slate-500 dark:text-slate-400">No data available</p>
	</div>
{:else}
	<div class="flex items-center justify-center" style="width: {width}px; height: {height}px;">
		<p class="text-slate-500 dark:text-slate-400">Loading chart...</p>
	</div>
{/if}

<style>
	.chart-container {
		background-color: white;
		border-radius: 8px;
		padding: 16px;
		position: relative;
		overflow: visible;
	}

	:global(.dark) .chart-container {
		background-color: rgb(30, 41, 59);
	}

	.chart-wrapper {
		width: 100%;
		height: 100%;
		position: relative;
	}
</style>
