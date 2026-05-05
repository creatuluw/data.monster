<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{
			month: string | number;
			sales: number;
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
		// Format: [['Month', 'Sales'], ['Jan', 1106], ...]
		const matrixData = [
			['Month', 'Sales'],
			...data.map((d) => [d.month, d.sales])
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
					y: {
						data: { field: 'Sales' },
						invert: true,
						expand: 0.2
					},
					t: {
						data: { extract: { field: 'Month' } }
					}
				},
				components: [
					{
						type: 'axis',
						dock: 'left',
						scale: 'y',
						formatter: {
							type: 'd3-number',
							format: '$,.1r'
						}
					},
					{
						type: 'axis',
						dock: 'bottom',
						scale: 't'
					},
					{
						key: 'lines',
						type: 'line',
						data: {
							extract: {
								field: 'Month',
								props: {
									v: { field: 'Sales' }
								}
							}
						},
						settings: {
							coordinates: {
								major: { scale: 't' },
								minor: { scale: 'y', ref: 'v' }
							},
							layers: {
								line: {}
							}
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
				['Month', 'Sales'],
				...data.map((d) => [d.month, d.sales])
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

