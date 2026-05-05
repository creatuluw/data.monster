<script lang="ts">
	import { BarChart3 } from 'lucide-svelte';
	import BarChart from '$lib/charts/picasso/BarChart.svelte';
	import StackedAreaChart from '$lib/charts/picasso/StackedAreaChart.svelte';
	import ScatterPlot from '$lib/charts/picasso/ScatterPlot.svelte';
	import BulletChart from '$lib/charts/picasso/BulletChart.svelte';
	import LineChart from '$lib/charts/picasso/LineChart.svelte';
	import HeatMap from '$lib/charts/picasso/HeatMap.svelte';

	// Sample data for bar chart
	const barChartData = [
		{ category: 'Electronics', value: 12000 },
		{ category: 'Clothing', value: 8500 },
		{ category: 'Food', value: 15000 },
		{ category: 'Books', value: 3200 },
		{ category: 'Toys', value: 6800 },
		{ category: 'Home', value: 9200 }
	];

	// Sample data for stacked area chart
	const stackedAreaData = [
		{ date: 'Jan', Series1: 100, Series2: 200, Series3: 150 },
		{ date: 'Feb', Series1: 120, Series2: 180, Series3: 170 },
		{ date: 'Mar', Series1: 150, Series2: 220, Series3: 190 },
		{ date: 'Apr', Series1: 180, Series2: 250, Series3: 210 },
		{ date: 'May', Series1: 200, Series2: 280, Series3: 230 },
		{ date: 'Jun', Series1: 220, Series2: 300, Series3: 250 }
	];

	const stackedAreaSeries = ['Series1', 'Series2', 'Series3'];

	// Sample data for scatter plot
	const scatterData = [
		{ month: 'Jan', sales: 1106, margin: 7, year: '2020' },
		{ month: 'Feb', sales: 5444, margin: 53, year: '2020' },
		{ month: 'Mar', sales: 147, margin: 64, year: '2020' },
		{ month: 'Apr', sales: 7499, margin: 47, year: '2020' },
		{ month: 'May', sales: 430, margin: 62, year: '2020' },
		{ month: 'Jun', sales: 9735, margin: 13, year: '2020' },
		{ month: 'Jan', sales: 1500, margin: 10, year: '2021' },
		{ month: 'Feb', sales: 6200, margin: 58, year: '2021' },
		{ month: 'Mar', sales: 300, margin: 72, year: '2021' },
		{ month: 'Apr', sales: 8200, margin: 52, year: '2021' },
		{ month: 'May', sales: 600, margin: 68, year: '2021' },
		{ month: 'Jun', sales: 10500, margin: 18, year: '2021' }
	];

	// Sample data for bullet chart
	const bulletData = [
		{ dim: 'Revenue', measure: 750, target: 800 },
		{ dim: 'Profit', measure: 550, target: 600 },
		{ dim: 'Orders', measure: 450, target: 500 },
		{ dim: 'Customers', measure: 320, target: 350 }
	];

	// Sample data for line chart - monthly data
	const lineChartData = [
		{ month: 'Jan', sales: 1106 },
		{ month: 'Feb', sales: 5444 },
		{ month: 'Mar', sales: 147 },
		{ month: 'Apr', sales: 7499 },
		{ month: 'May', sales: 430 },
		{ month: 'Jun', sales: 9735 },
		{ month: 'Jul', sales: 3200 },
		{ month: 'Aug', sales: 4800 },
		{ month: 'Sep', sales: 6200 },
		{ month: 'Oct', sales: 7500 },
		{ month: 'Nov', sales: 8900 },
		{ month: 'Dec', sales: 10500 }
	];

	// Sample data for heatmap
	const heatmapData = [
		{ day: 'Feb 1', hour: 0, density: 5 },
		{ day: 'Feb 1', hour: 1, density: 8 },
		{ day: 'Feb 1', hour: 2, density: 12 },
		{ day: 'Feb 1', hour: 3, density: 15 },
		{ day: 'Feb 1', hour: 4, density: 18 },
		{ day: 'Feb 1', hour: 5, density: 22 },
		{ day: 'Feb 1', hour: 6, density: 10 },
		{ day: 'Feb 1', hour: 7, density: 7 },
		{ day: 'Feb 1', hour: 8, density: 5 },
		{ day: 'Feb 1', hour: 9, density: 12 },
		{ day: 'Feb 1', hour: 10, density: 16 },
		{ day: 'Feb 1', hour: 11, density: 19 },
		{ day: 'Feb 1', hour: 12, density: 21 },
		{ day: 'Feb 1', hour: 13, density: 18 },
		{ day: 'Feb 1', hour: 14, density: 14 },
		{ day: 'Feb 1', hour: 15, density: 11 },
		{ day: 'Feb 1', hour: 16, density: 9 },
		{ day: 'Feb 1', hour: 17, density: 6 },
		{ day: 'Feb 1', hour: 18, density: 4 },
		{ day: 'Feb 1', hour: 19, density: 7 },
		{ day: 'Feb 1', hour: 20, density: 13 },
		{ day: 'Feb 1', hour: 21, density: 17 },
		{ day: 'Feb 1', hour: 22, density: 14 },
		{ day: 'Feb 1', hour: 23, density: 9 },
		{ day: 'Feb 2', hour: 0, density: 6 },
		{ day: 'Feb 2', hour: 1, density: 9 },
		{ day: 'Feb 2', hour: 2, density: 13 },
		{ day: 'Feb 2', hour: 3, density: 16 },
		{ day: 'Feb 2', hour: 4, density: 20 },
		{ day: 'Feb 2', hour: 5, density: 23 },
		{ day: 'Feb 2', hour: 6, density: 11 },
		{ day: 'Feb 2', hour: 7, density: 8 },
		{ day: 'Feb 2', hour: 8, density: 6 },
		{ day: 'Feb 2', hour: 9, density: 13 },
		{ day: 'Feb 2', hour: 10, density: 17 },
		{ day: 'Feb 2', hour: 11, density: 20 },
		{ day: 'Feb 2', hour: 12, density: 22 },
		{ day: 'Feb 2', hour: 13, density: 19 },
		{ day: 'Feb 2', hour: 14, density: 15 },
		{ day: 'Feb 2', hour: 15, density: 12 },
		{ day: 'Feb 2', hour: 16, density: 10 },
		{ day: 'Feb 2', hour: 17, density: 7 },
		{ day: 'Feb 2', hour: 18, density: 5 },
		{ day: 'Feb 2', hour: 19, density: 8 },
		{ day: 'Feb 2', hour: 20, density: 14 },
		{ day: 'Feb 2', hour: 21, density: 18 },
		{ day: 'Feb 2', hour: 22, density: 15 },
		{ day: 'Feb 2', hour: 23, density: 10 },
		{ day: 'Feb 3', hour: 0, density: 4 },
		{ day: 'Feb 3', hour: 1, density: 7 },
		{ day: 'Feb 3', hour: 2, density: 11 },
		{ day: 'Feb 3', hour: 3, density: 14 },
		{ day: 'Feb 3', hour: 4, density: 17 },
		{ day: 'Feb 3', hour: 5, density: 21 },
		{ day: 'Feb 3', hour: 6, density: 9 },
		{ day: 'Feb 3', hour: 7, density: 6 },
		{ day: 'Feb 3', hour: 8, density: 4 },
		{ day: 'Feb 3', hour: 9, density: 11 },
		{ day: 'Feb 3', hour: 10, density: 15 },
		{ day: 'Feb 3', hour: 11, density: 18 },
		{ day: 'Feb 3', hour: 12, density: 20 },
		{ day: 'Feb 3', hour: 13, density: 17 },
		{ day: 'Feb 3', hour: 14, density: 13 },
		{ day: 'Feb 3', hour: 15, density: 10 },
		{ day: 'Feb 3', hour: 16, density: 8 },
		{ day: 'Feb 3', hour: 17, density: 5 },
		{ day: 'Feb 3', hour: 18, density: 3 },
		{ day: 'Feb 3', hour: 19, density: 6 },
		{ day: 'Feb 3', hour: 20, density: 12 },
		{ day: 'Feb 3', hour: 21, density: 16 },
		{ day: 'Feb 3', hour: 22, density: 13 },
		{ day: 'Feb 3', hour: 23, density: 8 }
	];
</script>

<div class="p-6">
	<div class="mb-6">
		<div class="flex items-center gap-3 mb-2">
			<BarChart3 class="size-8 text-indigo-600 dark:text-indigo-400" />
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Picasso.js Chart Examples</h1>
		</div>
		<p class="text-gray-600 dark:text-gray-400">
			Explore chart types built with Picasso.js visualization library
		</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Bar Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Bar Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<BarChart data={barChartData} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">BarChart</code></p>
				<p class="mt-1">Uses Picasso.js <code>box</code> component</p>
			</div>
		</div>

		<!-- Stacked Area Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Stacked Area Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<StackedAreaChart data={stackedAreaData} series={stackedAreaSeries} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>
					Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">StackedAreaChart</code>
				</p>
				<p class="mt-1">Uses Picasso.js <code>line-area</code> component with stacking</p>
			</div>
		</div>

		<!-- Scatter Plot -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Scatter Plot</h3>
			<div class="flex-1 min-h-[400px]">
				<ScatterPlot data={scatterData} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">ScatterPlot</code></p>
				<p class="mt-1">Uses Picasso.js <code>point</code> component with color legend</p>
			</div>
		</div>

		<!-- Bullet Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Bullet Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<BulletChart data={bulletData} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">BulletChart</code></p>
				<p class="mt-1">Uses Picasso.js <code>box</code> components for measure, target, and ranges</p>
			</div>
		</div>

		<!-- Line Chart -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Line Chart</h3>
			<div class="flex-1 min-h-[400px]">
				<LineChart data={lineChartData} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">LineChart</code></p>
				<p class="mt-1">Uses Picasso.js <code>line</code> component with d3 formatters</p>
			</div>
		</div>

		<!-- Heat Map -->
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-lg p-6 flex flex-col min-h-[500px]">
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">Heat Map</h3>
			<div class="flex-1 min-h-[400px]">
				<HeatMap data={heatmapData} height={400} />
			</div>
			<div class="mt-4 text-xs text-gray-500 dark:text-gray-400">
				<p>Component: <code class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">HeatMap</code></p>
				<p class="mt-1">Uses Picasso.js <code>point</code> component with square shape and threshold-color scale</p>
			</div>
		</div>
	</div>
</div>
