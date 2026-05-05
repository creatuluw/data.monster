<script lang="ts">
	import { BarChart3 } from 'lucide-svelte';
	import BarChart from '$lib/charts/svelteplot/BarChart.svelte';
	import HorizontalBarChart from '$lib/charts/svelteplot/HorizontalBarChart.svelte';
	import LineChart from '$lib/charts/svelteplot/LineChart.svelte';
	import AreaChart from '$lib/charts/svelteplot/AreaChart.svelte';
	import ScatterPlot from '$lib/charts/svelteplot/ScatterPlot.svelte';
	import Heatmap from '$lib/charts/svelteplot/Heatmap.svelte';

	// Sample data for different chart types
	const barChartData = [
		{ category: 'Electronics', value: 12000 },
		{ category: 'Clothing', value: 8500 },
		{ category: 'Food', value: 15000 },
		{ category: 'Books', value: 3200 },
		{ category: 'Toys', value: 6800 },
		{ category: 'Home', value: 9200 }
	];

	const horizontalBarData = [
		{ category: 'Q1', value: 45000 },
		{ category: 'Q2', value: 52000 },
		{ category: 'Q3', value: 48000 },
		{ category: 'Q4', value: 61000 }
	];

	const lineChartData = [
		{ date: 1, value: 120, label: 'Jan' },
		{ date: 2, value: 145, label: 'Feb' },
		{ date: 3, value: 138, label: 'Mar' },
		{ date: 4, value: 162, label: 'Apr' },
		{ date: 5, value: 178, label: 'May' },
		{ date: 6, value: 195, label: 'Jun' },
		{ date: 7, value: 210, label: 'Jul' },
		{ date: 8, value: 198, label: 'Aug' },
		{ date: 9, value: 225, label: 'Sep' },
		{ date: 10, value: 240, label: 'Oct' },
		{ date: 11, value: 232, label: 'Nov' },
		{ date: 12, value: 258, label: 'Dec' }
	];

	const areaChartData = [
		{ date: 'Week 1', value: 4200 },
		{ date: 'Week 2', value: 5100 },
		{ date: 'Week 3', value: 4800 },
		{ date: 'Week 4', value: 5900 },
		{ date: 'Week 5', value: 6200 },
		{ date: 'Week 6', value: 5800 },
		{ date: 'Week 7', value: 6700 },
		{ date: 'Week 8', value: 7100 }
	];

	const scatterData = [
		{ x: 10, y: 20 },
		{ x: 15, y: 35 },
		{ x: 20, y: 28 },
		{ x: 25, y: 45 },
		{ x: 30, y: 38 },
		{ x: 35, y: 52 },
		{ x: 40, y: 48 },
		{ x: 45, y: 65 },
		{ x: 50, y: 58 },
		{ x: 55, y: 72 },
		{ x: 60, y: 68 },
		{ x: 65, y: 85 },
		{ x: 70, y: 78 },
		{ x: 75, y: 92 },
		{ x: 80, y: 88 }
	];

	const heatmapData = [
		{ x: 'Mon', y: 'Morning', value: 15 },
		{ x: 'Mon', y: 'Afternoon', value: 25 },
		{ x: 'Mon', y: 'Evening', value: 18 },
		{ x: 'Tue', y: 'Morning', value: 20 },
		{ x: 'Tue', y: 'Afternoon', value: 30 },
		{ x: 'Tue', y: 'Evening', value: 22 },
		{ x: 'Wed', y: 'Morning', value: 18 },
		{ x: 'Wed', y: 'Afternoon', value: 28 },
		{ x: 'Wed', y: 'Evening', value: 20 },
		{ x: 'Thu', y: 'Morning', value: 22 },
		{ x: 'Thu', y: 'Afternoon', value: 32 },
		{ x: 'Thu', y: 'Evening', value: 24 },
		{ x: 'Fri', y: 'Morning', value: 19 },
		{ x: 'Fri', y: 'Afternoon', value: 29 },
		{ x: 'Fri', y: 'Evening', value: 21 }
	];
</script>

<div class="p-6">
	<div class="mb-6">
		<div class="flex items-center gap-3 mb-2">
			<BarChart3 class="size-8 text-indigo-600 dark:text-indigo-400" />
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">SveltePlot Chart Examples</h1>
		</div>
		<p class="text-gray-600 dark:text-gray-400">
			Explore various chart types built with SveltePlot components
		</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Bar Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Bar Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<BarChart
					data={barChartData}
					x="category"
					y="value"
					fill="#CA3500"
					xLabel="Category"
					yLabel="Value"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">BarChart</code></p>
				<p class="mt-1">Uses SveltePlot <code>BarY</code> component</p>
			</div>
		</div>

		<!-- Horizontal Bar Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Horizontal Bar Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<HorizontalBarChart
					data={horizontalBarData}
					x="value"
					y="category"
					fill="#10B981"
					xLabel="Revenue"
					yLabel="Quarter"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">HorizontalBarChart</code></p>
				<p class="mt-1">Uses SveltePlot <code>BarX</code> component</p>
			</div>
		</div>

		<!-- Line Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Line Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<LineChart
					data={lineChartData}
					x="date"
					y="value"
					stroke="#3B82F6"
					strokeWidth={2}
					xLabel="Month"
					yLabel="Sales"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">LineChart</code></p>
				<p class="mt-1">Uses SveltePlot <code>LineY</code> component</p>
			</div>
		</div>

		<!-- Area Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Area Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<AreaChart
					data={areaChartData}
					x="date"
					y="value"
					fill="#6366F1"
					fillOpacity={0.3}
					xLabel="Week"
					yLabel="Revenue"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">AreaChart</code></p>
				<p class="mt-1">Uses SveltePlot <code>AreaY</code> component</p>
			</div>
		</div>

		<!-- Scatter Plot -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Scatter Plot</h3>
			<div class="flex-1 min-h-[400px]">
				<ScatterPlot
					data={scatterData}
					x="x"
					y="y"
					fill="#6366F1"
					r={4}
					xLabel="X Axis"
					yLabel="Y Axis"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">ScatterPlot</code></p>
				<p class="mt-1">Uses SveltePlot <code>Dot</code> component</p>
			</div>
		</div>

		<!-- Heatmap -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Heatmap</h3>
			<div class="flex-1 min-h-[400px]">
				<Heatmap
					data={heatmapData}
					x="x"
					y="y"
					fill="value"
					xLabel="Day"
					yLabel="Time of Day"
					colorScheme="blues"
				/>
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">Heatmap</code></p>
				<p class="mt-1">Uses SveltePlot <code>Cell</code> component</p>
			</div>
		</div>
	</div>
</div>
