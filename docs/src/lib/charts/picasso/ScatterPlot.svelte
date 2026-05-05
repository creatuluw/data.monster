<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{
			month: string | number;
			sales: number;
			margin: number;
			year: string | number;
			size?: number;
		}>;
		height?: number;
		width?: number;
	}

	let { data = [], height = 400, width }: Props = $props();

	let containerEl = $state<HTMLDivElement>();
	let chartInstance: any = $state(null);

	onMount(() => {
		if (!containerEl) return;

		// Prepare matrix data for picasso.js
		// Format: [['Month', 'Sales', 'Margin', 'Year'], ['Jan', 1106, 7, '2020'], ...]
		const matrixData = [
			['Month', 'Sales', 'Margin', 'Year'],
			...data.map((d) => [d.month, d.sales, d.margin, d.year])
		];

		chartInstance = picasso.chart({
			element: containerEl,
			data: [
				{
					type: 'matrix',
					data: matrixData
				}
			],
			settings: {
				scales: {
					s: {
						data: { field: 'Sales' },
						expand: 0.2,
						invert: true
					},
					m: {
						data: { field: 'Margin' },
						expand: 0.1
					},
					col: {
						data: { extract: { field: 'Year' } },
						type: 'color'
					}
				},
				components: [
					{
						key: 'y-axis',
						type: 'axis',
						scale: 's',
						dock: 'left'
					},
					{
						type: 'legend-cat',
						dock: 'right',
						scale: 'col'
					},
					{
						key: 'x-axis',
						type: 'axis',
						scale: 'm',
						dock: 'bottom'
					},
					{
						key: 'p',
						type: 'point',
						data: {
							extract: {
								field: 'Month',
								props: {
									y: { field: 'Sales' },
									x: { field: 'Margin' },
									group: { field: 'Year' }
								}
							}
						},
						settings: {
							x: { scale: 'm' },
							y: { scale: 's' },
							shape: 'circle',
							size: () => Math.random(),
							strokeWidth: 2,
							stroke: '#fff',
							opacity: 0.8,
							fill: { scale: 'col', ref: 'group' }
						}
					}
				]
			}
		});

		return () => {
			if (chartInstance) {
				chartInstance.destroy();
				chartInstance = null;
			}
		};
	});

	onDestroy(() => {
		if (chartInstance) {
			chartInstance.destroy();
		}
	});

	$effect(() => {
		if (chartInstance && data.length > 0) {
			const matrixData = [
				['Month', 'Sales', 'Margin', 'Year'],
				...data.map((d) => [d.month, d.sales, d.margin, d.year])
			];

			chartInstance.update({
				data: [
					{
						type: 'matrix',
						data: matrixData
					}
				]
			});
		}
	});
</script>

<div class="chart-container" bind:this={containerEl} style="width: {width ? width + 'px' : '100%'}; height: {height}px;"></div>

<style>
	.chart-container {
		width: 100%;
		height: 100%;
		position: relative;
		overflow: hidden;
	}
</style>

