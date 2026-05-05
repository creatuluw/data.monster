<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import picasso from 'picasso.js';

	interface Props {
		data: Array<{
			dim: string;
			measure: number;
			target: number;
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
		// Format: [['Dim', 'Measure', 'Target'], ['A', 750, 800], ...]
		const matrixData = [
			['Dim', 'Measure', 'Target'],
			...data.map((d) => [d.dim, d.measure, d.target])
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
						key: 'd',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: { field: 'Measure' },
									end: { field: 'Target' }
								}
							}
						}
					}
				],
				scales: {
					y: {
						data: { extract: { field: 'Dim' } },
						padding: 0.2
					},
					v: {
						data: { fields: ['Measure', 'Target'] },
						expand: 0.1,
						min: 0,
						max: 100
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
						scale: 'v'
					},
					// box({ start: 0, end: 1000, width: 0.8, fill: '#eee' })
					{
						type: 'box',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: 0,
									end: 1000
								}
							}
						},
						settings: {
							major: { scale: 'y' },
							minor: { scale: 'v' },
							box: {
								fill: '#eee',
								width: 0.8
							}
						}
					},
					// box({ start: 0, end: { field: 'Target', value: v => v * 0.8 }, width: 0.8, fill: '#ccc' })
					{
						type: 'box',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: 0,
									end: { field: 'Target', value: (v: number) => v * 0.8 }
								}
							}
						},
						settings: {
							major: { scale: 'y' },
							minor: { scale: 'v' },
							box: {
								fill: '#ccc',
								width: 0.8
							}
						}
					},
					// box({ start: 0, end: { field: 'Target', value: v => v * 0.6 }, width: 0.8, fill: '#aaa' })
					{
						type: 'box',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: 0,
									end: { field: 'Target', value: (v: number) => v * 0.6 }
								}
							}
						},
						settings: {
							major: { scale: 'y' },
							minor: { scale: 'v' },
							box: {
								fill: '#aaa',
								width: 0.8
							}
						}
					},
					// box({ start: 0, end: { field: 'Measure' }, width: 0.4, fill: '#111' })
					{
						type: 'box',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: 0,
									end: { field: 'Measure' }
								}
							}
						},
						settings: {
							major: { scale: 'y' },
							minor: { scale: 'v' },
							box: {
								fill: '#111',
								width: 0.4
							}
						}
					},
					// box({ start: { field: 'Target' }, end: { field: 'Target' }, width: 0.7, fill: '#111', minHeightPx: 3 })
					{
						type: 'box',
						data: {
							extract: {
								field: 'Dim',
								props: {
									start: { field: 'Target' },
									end: { field: 'Target' }
								}
							}
						},
						settings: {
							major: { scale: 'y' },
							minor: { scale: 'v' },
							box: {
								fill: '#111',
								width: 0.7,
								minHeightPx: 3
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
				['Dim', 'Measure', 'Target'],
				...data.map((d) => [d.dim, d.measure, d.target])
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
