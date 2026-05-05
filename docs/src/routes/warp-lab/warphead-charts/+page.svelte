<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import { BarChart3, TrendingUp, ChartScatter, AreaChart, ArrowRight, Grid3x3, Circle, CircleDot, Dot, Activity, Waves, Box, Minus, LayoutList, GitBranch } from 'lucide-svelte';
</script>

<PageLayout>
	<section class="mb-12">

		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<Card
				title="Barchart"
				description="Create and customize charts with Warphead"
				href="/warp-lab/warphead-charts/barchart"
				icon={BarChart3}
				badgeColor="purple"
			/>
			<Card
				title="Linechart"
				description="Create and customize line charts with Warphead"
				href="/warp-lab/warphead-charts/linechart"
				icon={TrendingUp}
				badgeColor="blue"
			/>
			<Card
				title="Scatter Plot"
				description="Create and customize scatter plots with Warphead"
				href="/warp-lab/warphead-charts/scatter-plot"
				icon={ChartScatter}
				badgeColor="green"
			/>
			<Card
				title="Area Chart"
				description="Create and customize area charts with Warphead"
				href="/warp-lab/warphead-charts/areachart"
				icon={AreaChart}
				badgeColor="orange"
			/>
			<Card
				title="Delta Chart"
				description="Create and customize delta charts with Warphead"
				href="/warp-lab/warphead-charts/deltachart"
				icon={ArrowRight}
				badgeColor="red"
			/>
			<Card
				title="Heatmap"
				description="Create and customize heatmaps with Warphead"
				href="/warp-lab/warphead-charts/heatmap"
				icon={Grid3x3}
				badgeColor="indigo"
			/>
			<Card
				title="Circle Pack"
				description="Create and customize circle pack charts with Warphead"
				href="/warp-lab/warphead-charts/circle-pack"
				icon={Circle}
				badgeColor="teal"
			/>
			<Card
				title="Bubble Chart"
				description="Create and customize bubble charts with Warphead"
				href="/warp-lab/warphead-charts/bubble-chart"
				icon={CircleDot}
				badgeColor="pink"
			/>
			<Card
				title="Dot Plot"
				description="Create and customize dot plots with Warphead"
				href="/warp-lab/warphead-charts/dot-plot"
				icon={Dot}
				badgeColor="violet"
			/>
			<Card
				title="Jitter Plot"
				description="Create and customize jitter plots with Warphead"
				href="/warp-lab/warphead-charts/jitter-plot"
				icon={Activity}
				badgeColor="cyan"
			/>
			<Card
				title="Stream Graph"
				description="Create and customize streamgraphs with Warphead"
				href="/warp-lab/warphead-charts/stream-graph"
				icon={Waves}
				badgeColor="amber"
			/>
			<Card
				title="Box Plot"
				description="Create and customize box plots with Warphead"
				href="/warp-lab/warphead-charts/box-plot"
				icon={Box}
				badgeColor="rose"
			/>
			<Card
				title="Tick Chart"
				description="Create and customize tick charts with Warphead"
				href="/warp-lab/warphead-charts/tick-chart"
				icon={Minus}
				badgeColor="emerald"
			/>
			<Card
				title="Histogram"
				description="Create and customize histograms with Warphead"
				href="/warp-lab/warphead-charts/histogram"
				icon={LayoutList}
				badgeColor="sky"
			/>
			<Card
				title="Parallel Plot"
				description="Create and customize parallel plots with Warphead"
				href="/warp-lab/warphead-charts/parallel-plot"
				icon={GitBranch}
				badgeColor="lime"
			/>
		</div>
	</section>
</PageLayout>

