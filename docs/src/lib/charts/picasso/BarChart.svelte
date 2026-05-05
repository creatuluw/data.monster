<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{ category: string; value: number }>;
		height?: number;
		width?: number;
	}

	let { data = [], height = 400, width }: Props = $props();

	let containerEl = $state<HTMLDivElement>();
	let chartInstance: any = $state(null);

	onMount(() => {
		if (!containerEl) return;

		// Prepare matrix data for picasso.js
		// Format: [['Category', 'Value'], ['Electronics', 12000], ...]
		const matrixData = [
			['Category', 'Value'],
			...data.map((d) => [d.category, d.value])
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
						data: { field: 'Value' },
						invert: true,
						include: [0]
					},
					c: {
						data: { field: 'Value' },
						type: 'color'
					},
					t: {
						data: { extract: { field: 'Category' } },
						padding: 0.3
					}
				},
				components: [
					{
						type: 'axis',
						dock: 'left',
						scale: 'y'
					},
					{
						type: 'axis',
						dock: 'bottom',
						scale: 't'
					},
					{
						key: 'bars',
						type: 'box',
						data: {
							extract: {
								field: 'Category',
								props: {
									start: 0,
									end: { field: 'Value' }
								}
							}
						},
						settings: {
							major: { scale: 't' },
							minor: { scale: 'y' },
							box: {
								fill: { scale: 'c', ref: 'end' }
							}
						},
						brush: {
							trigger: [
								{
									on: 'tap',
									action: 'toggle',
									contexts: ['selection']
								}
							],
							consume: [
								{
									context: 'selection',
									style: {
										active: {
											fill: 'orange'
										},
										inactive: {
											fill: { scale: 'c', ref: 'end' }
										}
									}
								}
							]
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
				['Category', 'Value'],
				...data.map((d) => [d.category, d.value])
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
