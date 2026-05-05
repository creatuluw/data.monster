<script lang="ts">
	import { LineY, Plot } from 'svelteplot';

	interface Props {
		data: any[];
		x?: string;
		y?: string;
		stroke?: string;
		strokeWidth?: number;
		height?: number;
		width?: number;
		xLabel?: string;
		yLabel?: string;
		xTickFormat?: (d: any) => string;
	}

	let {
		data = [],
		x = 'date',
		y = 'value',
		stroke = '#3B82F6',
		strokeWidth = 2,
		height = 400,
		width,
		xLabel = '',
		yLabel = '',
		xTickFormat
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

	// Default month formatter if data has label property
	const monthFormatter = (d: any) => {
		if (xTickFormat) return xTickFormat(d);
		// If data items have a 'label' property, use it for formatting
		const item = data.find(item => item[x] === d);
		return item?.label !== undefined ? item.label : String(d);
	};
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
			x={{ label: xLabel, tickFormat: xTickFormat || monthFormatter }}
			y={{ grid: true, label: yLabel }}
		>
			<LineY {data} {x} {y} {stroke} {strokeWidth} />
		</Plot>
	{/if}
</div>

<style>
	.chart-container {
		width: 100%;
	}
</style>

