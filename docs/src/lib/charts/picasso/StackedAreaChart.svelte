<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{ date: string | number; [series: string]: string | number }>;
		series: string[];
		height?: number;
		width?: number;
	}

	let { data = [], series = [], height = 400, width }: Props = $props();

	let containerEl = $state<HTMLDivElement>();
	let chartInstance: any = $state(null);

	onMount(() => {
		if (!containerEl || series.length === 0) return;

		// Transform data to long format: [{ date: 'Jan', line: 'Series1', end: 100 }, ...]
		const longData = data.flatMap((d) =>
			series.map((s) => ({
				date: d.date,
				line: s,
				end: d[s] || 0
			}))
		);

		// Prepare matrix data
		const matrixData = [
			['Date', 'Line', 'End'],
			...longData.map((d) => [d.date, d.line, d.end])
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
				collections: [
					{
						key: 'stacked',
						data: {
							extract: {
								field: 'Date',
								props: {
									line: { field: 'Line' },
									end: { field: 'End' }
								}
							},
							stack: {
								stackKey: (d: any) => d.value,
								value: (d: any) => d.end.value
							}
						}
					}
				],
				scales: {
					y: {
						data: {
							collection: {
								key: 'stacked'
							}
						},
						invert: true,
						expand: 0.2,
						min: 0
					},
					t: {
						data: { extract: { field: 'Date' } }
					},
					color: {
						data: { extract: { field: 'Line' } },
						type: 'color'
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
						key: 'lines',
						type: 'line',
						data: {
							collection: 'stacked'
						},
						settings: {
							coordinates: {
								major: { scale: 't' },
								minor0: { scale: 'y', ref: 'start' },
								minor: { scale: 'y', ref: 'end' },
								layerId: { ref: 'line' }
							},
							layers: {
								curve: 'monotone',
								line: {
									show: false
								},
								area: {
									fill: { scale: 'color', ref: 'line' }
								}
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
		if (chartInstance && data.length > 0 && series.length > 0) {
			// Transform data to long format
			const longData = data.flatMap((d) =>
				series.map((s) => ({
					date: d.date,
					line: s,
					end: d[s] || 0
				}))
			);

			const matrixData = [
				['Date', 'Line', 'End'],
				...longData.map((d) => [d.date, d.line, d.end])
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

