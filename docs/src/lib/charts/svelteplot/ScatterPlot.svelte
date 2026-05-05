<script lang="ts">
	import { Dot, Plot } from 'svelteplot';

	interface Props {
		data: any[];
		x?: string;
		y?: string;
		fill?: string;
		r?: number;
		height?: number;
		width?: number;
		xLabel?: string;
		yLabel?: string;
	}

	let {
		data = [],
		x = 'x',
		y = 'y',
		fill = '#6366F1',
		r = 3,
		height = 400,
		width,
		xLabel = '',
		yLabel = ''
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
			x={{ grid: true, label: xLabel }}
			y={{ grid: true, label: yLabel }}
		>
			<Dot {data} {x} {y} {fill} {r} />
		</Plot>
	{/if}
</div>

<style>
	.chart-container {
		width: 100%;
	}
</style>

