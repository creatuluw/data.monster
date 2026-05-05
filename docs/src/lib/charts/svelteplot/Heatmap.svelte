<script lang="ts">
	import { Cell, Plot } from 'svelteplot';

	interface Props {
		data: any[];
		x?: string;
		y?: string;
		fill?: string;
		height?: number;
		width?: number;
		xLabel?: string;
		yLabel?: string;
		colorScheme?: string;
	}

	let {
		data = [],
		x = 'x',
		y = 'y',
		fill = 'value',
		height = 400,
		width,
		xLabel = '',
		yLabel = '',
		colorScheme = 'blues'
	}: Props = $props();

	let containerWidth = $state(0);
	let containerEl = $state<HTMLDivElement>();

	$effect(() => {
		if (containerEl) {
			const updateSize = () => {
				if (containerEl) {
					containerWidth = containerEl.clientWidth;
				}
			};

			updateSize();
			const resizeObserver = new ResizeObserver(updateSize);
			resizeObserver.observe(containerEl);

			return () => resizeObserver.disconnect();
		}
	});
</script>

<div class="chart-container" bind:this={containerEl}>
	{#if containerWidth > 0 && data.length > 0}
		<Plot
			width={width || containerWidth}
			height={height}
			marginLeft={60}
			marginBottom={60}
			marginTop={20}
			marginRight={20}
			x={{ label: xLabel }}
			y={{ label: yLabel }}
			color={{ scheme: colorScheme }}
		>
			<Cell {data} {x} {y} {fill} />
		</Plot>
	{/if}
</div>

<style>
	.chart-container {
		width: 100%;
	}
</style>

