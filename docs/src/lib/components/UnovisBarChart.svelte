<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { XYContainer, GroupedBar } from '@unovis/ts';
	import '@unovis/ts/styles';

	interface Props {
		data: any[];
		xKey: string;
		yKeys: string[];
		width?: number;
		height?: number;
	}

	let { data = [], xKey = '', yKeys = [], width = 800, height = 400 }: Props = $props();

	let containerElement: HTMLDivElement;
	let chart: XYContainer | null = null;

	function initChart() {
		if (!containerElement) {
			console.log('UnovisBarChart: No container element');
			return;
		}
		
		if (!data.length) {
			console.log('UnovisBarChart: No data');
			return;
		}
		
		if (!xKey || !yKeys.length) {
			console.log('UnovisBarChart: Missing xKey or yKeys', { xKey, yKeys });
			return;
		}

		console.log('UnovisBarChart: Initializing', { 
			dataLength: data.length, 
			xKey, 
			yKeys,
			sampleData: data[0],
			containerSize: {
				width: containerElement.offsetWidth,
				height: containerElement.offsetHeight
			}
		});

		// Clean up existing chart
		if (chart) {
			try {
				chart.destroy();
			} catch (e) {
				console.warn('UnovisBarChart: Error destroying chart', e);
			}
			chart = null;
		}

		try {
			// Create accessor functions
			const xAccessor = (d: any) => String(d[xKey]);
			const yAccessors = yKeys.map(key => (d: any) => {
				const val = d[key];
				return typeof val === 'number' ? val : Number(val) || 0;
			});

			console.log('UnovisBarChart: Creating bar with accessors', { xAccessor, yAccessors });

			// Create grouped bar component
			const bar = new GroupedBar({
				x: xAccessor,
				y: yAccessors
			});

			// Create container with components
			chart = new XYContainer(containerElement, {
				components: [bar],
				height,
				width
			});

			console.log('UnovisBarChart: Setting data', data);

			// Set data
			chart.setData(data);
			
			console.log('UnovisBarChart: Chart initialized successfully');
			
			// Log what was actually rendered
			setTimeout(() => {
				const svgElements = containerElement.querySelectorAll('svg');
				const barElements = containerElement.querySelectorAll('rect');
				console.log('UnovisBarChart: Rendered elements', {
					svgCount: svgElements.length,
					barCount: barElements.length,
					innerHTML: containerElement.innerHTML.slice(0, 500)
				});
			}, 100);
		} catch (error) {
			console.error('UnovisBarChart: Error initializing chart', error);
		}
	}

	onMount(() => {
		console.log('UnovisBarChart: onMount called');
		// Wait for next tick to ensure DOM is ready
		setTimeout(() => {
			initChart();
		}, 0);
	});

	$effect(() => {
		// Re-initialize chart when data, xKey, or yKeys change
		// But only if we have valid data
		if (data?.length && xKey && yKeys?.length && containerElement) {
			console.log('UnovisBarChart: $effect triggered, reinitializing');
			initChart();
		}
	});

	onDestroy(() => {
		console.log('UnovisBarChart: Destroying chart');
		if (chart) {
			try {
				chart.destroy();
			} catch (e) {
				console.warn('UnovisBarChart: Error in destroy', e);
			}
		}
	});
</script>

<div bind:this={containerElement} style="width: {width}px; height: {height}px;"></div>

<style>
	div {
		border: 1px solid #e5e7eb;
		background-color: white;
		border-radius: 8px;
	}
	
	:global(.unovis-xy-container) {
		font-family: system-ui, -apple-system, sans-serif;
	}
</style>

