<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		Chart,
		BarController,
		BarElement,
		CategoryScale,
		LinearScale,
		Title,
		Tooltip,
		Legend
	} from 'chart.js';

	// Register Chart.js components
	Chart.register(BarController, BarElement, CategoryScale, LinearScale, Title, Tooltip, Legend);

	interface Props {
		data: any[];
		xKey: string;
		yKeys: string[];
		width?: number;
		height?: number;
		borderRadius?: number;
		chartTitle?: string;
	}

	let {
		data = [],
		xKey = '',
		yKeys = [],
		width = 800,
		height = 400,
		borderRadius = 8,
		chartTitle = 'Bar Chart'
	}: Props = $props();

	let canvasElement: HTMLCanvasElement;
	let chart: Chart | null = null;

	// Generate colors for datasets
	function getColor(index: number): { bg: string; border: string } {
		const colors = [
			{ bg: 'rgba(155, 0, 255, 0.7)', border: 'rgba(155, 0, 255, 1)' }, // Purple
			{ bg: 'rgba(0, 179, 0, 0.7)', border: 'rgba(0, 179, 0, 1)' }, // Green
			{ bg: 'rgba(86, 150, 255, 0.7)', border: 'rgba(86, 150, 255, 1)' }, // Blue
			{ bg: 'rgba(255, 159, 64, 0.7)', border: 'rgba(255, 159, 64, 1)' }, // Orange
			{ bg: 'rgba(255, 99, 132, 0.7)', border: 'rgba(255, 99, 132, 1)' } // Red
		];
		return colors[index % colors.length];
	}

	function initChart() {
		if (!canvasElement || !data.length || !xKey || !yKeys.length) return;

		// Clean up existing chart
		if (chart) {
			chart.destroy();
		}

		// Extract labels from data
		const labels = data.map((item) => item[xKey]);

		// Create datasets
		const datasets = yKeys.map((key, index) => {
			const color = getColor(index);
			return {
				label: key,
				data: data.map((item) => item[key]),
				backgroundColor: color.bg,
				borderColor: color.border,
				borderWidth: 2,
				borderRadius: borderRadius,
				borderSkipped: false
			};
		});

		// Create chart
		chart = new Chart(canvasElement, {
			type: 'bar',
			data: {
				labels: labels,
				datasets: datasets
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
				plugins: {
					legend: {
						position: 'top'
					},
					title: {
						display: true,
						text: chartTitle,
						font: {
							size: 16,
							weight: 'bold'
						}
					},
					tooltip: {
						backgroundColor: 'rgba(0, 0, 0, 0.8)',
						padding: 12,
						cornerRadius: 8
					}
				},
				scales: {
					x: {
						grid: {
							display: false
						}
					},
					y: {
						beginAtZero: true,
						grid: {
							color: 'rgba(0, 0, 0, 0.05)'
						}
					}
				}
			}
		});
	}

	onMount(() => {
		initChart();
	});

	$effect(() => {
		// Re-initialize chart when data, xKey, or yKeys change
		if (data && xKey && yKeys) {
			initChart();
		}
	});

	onDestroy(() => {
		if (chart) {
			chart.destroy();
		}
	});
</script>

<div style="width: {width}px; height: {height}px; position: relative;">
	<canvas bind:this={canvasElement}></canvas>
</div>

<style>
	div {
		background-color: white;
		border-radius: 8px;
		padding: 16px;
	}
</style>

