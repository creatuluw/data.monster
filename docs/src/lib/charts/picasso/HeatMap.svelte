<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{
			day: string | number;
			hour: string | number;
			density: number;
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
		// Format: [['Day', 'Hour', 'Density'], ['Feb 1', 0, 5], ...]
		const matrixData = [
			['Day', 'Hour', 'Density'],
			...data.map((d) => [d.day, d.hour, d.density])
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
					days: {
						data: {
							extract: { field: 'Day' }
						}
					},
					hours: {
						data: {
							extract: { field: 'Hour' }
						}
					},
					col: {
						data: { field: 'Density' },
						type: 'threshold-color',
						range: ['#304D2A', '#53763E', '#7DA050', '#AECC61', '#E6F871', '#eee'].reverse(),
						nice: true
					}
				},
				components: [
					{
						type: 'legend-cat',
						dock: 'top',
						scale: 'col'
					},
					{
						key: 'y-axis',
						type: 'axis',
						scale: 'days',
						dock: 'left'
					},
					{
						key: 'x-axis',
						type: 'axis',
						scale: 'hours',
						dock: 'bottom'
					},
					{
						key: 'p',
						type: 'point',
						data: {
							extract: {
								field: 'Hour',
								props: {
									d: { field: 'Density' },
									group: { field: 'Day' }
								}
							}
						},
						settings: {
							x: { scale: 'hours' },
							y: { scale: 'days', ref: 'group' },
							fill: { scale: 'col', ref: 'd' },
							shape: 'square'
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
				['Day', 'Hour', 'Density'],
				...data.map((d) => [d.day, d.hour, d.density])
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

