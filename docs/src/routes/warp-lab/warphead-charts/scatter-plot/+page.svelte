<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { ChartScatter } from 'lucide-svelte';
	import { Plot, Dot } from 'svelteplot';

	// Helper function to generate large scatter datasets
	function generateScatterData(count: number, xMin: number, xMax: number, yMin: number, yMax: number, correlation: number = 0): Array<{ x: number; y: number }> {
		const data: Array<{ x: number; y: number }> = [];
		for (let i = 0; i < count; i++) {
			const x = xMin + (xMax - xMin) * (i / count) + (Math.random() - 0.5) * (xMax - xMin) * 0.1;
			const y = yMin + (yMax - yMin) * (i / count) * correlation + (Math.random() - 0.5) * (yMax - yMin) * (1 - Math.abs(correlation));
			data.push({ x, y });
		}
		return data;
	}

	function generateClusteredData(clusterCount: number, pointsPerCluster: number): Array<{ x: number; y: number }> {
		const data: Array<{ x: number; y: number }> = [];
		for (let c = 0; c < clusterCount; c++) {
			const centerX = 20 + (c * 30) + Math.random() * 20;
			const centerY = 20 + (c * 25) + Math.random() * 20;
			for (let i = 0; i < pointsPerCluster; i++) {
				data.push({
					x: centerX + (Math.random() - 0.5) * 15,
					y: centerY + (Math.random() - 0.5) * 15
				});
			}
		}
		return data;
	}

	// Sample data for different scatter plot types
	// Basic scatter plot data - 1200 points
	const basicScatterData = generateScatterData(1200, 0, 100, 0, 100, 0.3);

	// Correlation data (positive correlation) - 1500 points
	const correlationData = generateScatterData(1500, 0, 100, 0, 100, 0.85);

	// Multi-series scatter plot data - 500 points per series
	const series1Data = generateScatterData(500, 0, 50, 20, 60, 0.6).map(d => ({ ...d, series: 'Group A' }));
	const series2Data = generateScatterData(500, 0, 50, 10, 50, 0.5).map(d => ({ ...d, series: 'Group B' }));
	const series3Data = generateScatterData(500, 0, 50, 40, 80, 0.7).map(d => ({ ...d, series: 'Group C' }));

	// Data with different point sizes - 1000 points
	const sizeData = generateScatterData(1000, 0, 100, 0, 100, 0.4).map(d => ({
		...d,
		size: 3 + Math.random() * 12
	}));

	// Data with dynamic colors - 1200 points
	const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#84CC16'];
	const coloredScatterData = generateScatterData(1200, 0, 100, 0, 100, 0.3).map(d => ({
		...d,
		color: colors[Math.floor(Math.random() * colors.length)]
	}));

	// Clustered data (multiple clusters) - 1200 points across 8 clusters
	const clusteredData = generateClusteredData(8, 150);

	let containerWidth1 = $state(0);
	let containerWidth2 = $state(0);
	let containerWidth3 = $state(0);
	let containerWidth4 = $state(0);
	let containerWidth5 = $state(0);
	let containerWidth6 = $state(0);

	let containerEl1 = $state<HTMLDivElement>();
	let containerEl2 = $state<HTMLDivElement>();
	let containerEl3 = $state<HTMLDivElement>();
	let containerEl4 = $state<HTMLDivElement>();
	let containerEl5 = $state<HTMLDivElement>();
	let containerEl6 = $state<HTMLDivElement>();

	function setupResizeObserver(el: HTMLDivElement | undefined, setWidth: (w: number) => void) {
		if (!el) return;
		
		const updateSize = () => {
			if (el) {
				const width = el.clientWidth || el.offsetWidth;
				if (width > 0) {
					setWidth(width);
				}
			}
		};
		
		// Initial measurement
		updateSize();
		
		// Use requestAnimationFrame to ensure DOM is ready
		requestAnimationFrame(() => {
			updateSize();
			const resizeObserver = new ResizeObserver(updateSize);
			resizeObserver.observe(el);
		});
	}

	$effect(() => {
		setupResizeObserver(containerEl1, (w) => containerWidth1 = w);
		setupResizeObserver(containerEl2, (w) => containerWidth2 = w);
		setupResizeObserver(containerEl3, (w) => containerWidth3 = w);
		setupResizeObserver(containerEl4, (w) => containerWidth4 = w);
		setupResizeObserver(containerEl5, (w) => containerWidth5 = w);
		setupResizeObserver(containerEl6, (w) => containerWidth6 = w);
	});
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Scatter Plot</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicScatterData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={basicScatterData} x="x" y="y" fill="#6366F1" r={5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> component</p>
				</div>
			</div>

			<!-- 2. Correlation Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Correlation Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && correlationData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Variable X' }}
							y={{ grid: true, label: 'Variable Y' }}
						>
							<Dot data={correlationData} x="x" y="y" fill="#10B981" r={6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows positive correlation between variables</p>
				</div>
			</div>

			<!-- 3. Multi-Series Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Multi-Series Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && series1Data.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={series1Data} x="x" y="y" fill="#6366F1" r={5} />
							<Dot data={series2Data} x="x" y="y" fill="#10B981" r={5} />
							<Dot data={series3Data} x="x" y="y" fill="#F59E0B" r={5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> components for different series</p>
				</div>
			</div>

			<!-- 4. Variable Size Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Variable Size Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && sizeData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={sizeData} x="x" y="y" fill="#8B5CF6" r="size" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">r="size"</code> for variable point sizes</p>
				</div>
			</div>

			<!-- 5. Multi-Color Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Multi-Color Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && coloredScatterData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={coloredScatterData} x="x" y="y" fill="color" r={6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="color"</code> for dynamic colors</p>
				</div>
			</div>

			<!-- 6. Clustered Scatter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Clustered Scatter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && clusteredData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={clusteredData} x="x" y="y" fill="#EF4444" r={5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows data clusters with distinct groupings</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Scatter Plots</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various scatter plot styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Dot</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/dot" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Scatter Plot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

