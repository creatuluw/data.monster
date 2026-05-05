<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import { BarChart3, BarChart2, Sparkles, BarChart } from 'lucide-svelte';
</script>

<PageLayout>
	<!-- Section: Visualization Libraries -->
	<section class="mb-12">
		<h2 class="font-aspekta text-2xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Visualization Libraries</h2>
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<Card
				title="Unovis"
				description="Create interactive bar charts with dimensions and metrics using the Unovis visualization library"
				href="/warp-lab/unovis"
				icon={BarChart2}
				badge="NEW"
				badgeColor="purple"
			/>
			<Card
				title="Chart.js"
				description="Create beautiful bar charts with rounded corners and customizable styling using Chart.js"
				href="/warp-lab/chart-js"
				icon={BarChart}
				badge="NEW"
				badgeColor="blue"
			/>
			<Card
				title="Chart Library"
				description="Build and customize charts using the ChartBuilder component with metrics and dimensions"
				href="/warp-lab/chart-lib"
				icon={Sparkles}
				badgeColor="cyan"
			/>
			<Card
				title="Charts svelteplot"
				description="Explore various chart types and visualization examples with interactive demonstrations"
				href="/warp-lab/charts-svelteplot"
				icon={BarChart3}
				badgeColor="green"
			/>
			<Card
				title="Headless Chart"
				description="Create fully customizable bar charts with widget-based architecture and complete element control"
				href="/warp-lab/headless"
				icon={BarChart3}
				badgeColor="orange"
			/>
			<Card
				title="Charts Picasso"
				description="Explore various chart types built with Picasso.js visualization library from Qlik"
				href="/warp-lab/charts-picasso"
				icon={BarChart3}
				badgeColor="indigo"
			/>
			<Card
				title="Warphead Charts"
				description="Create and customize charts with Warphead"
				href="/warp-lab/warphead-charts"
				icon={BarChart3}
				badgeColor="purple"
			/>
		</div>
	</section>
</PageLayout>
