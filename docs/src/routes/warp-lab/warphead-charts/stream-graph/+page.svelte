<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Waves } from 'lucide-svelte';
	import { Plot, AreaY } from 'svelteplot';

	// RIAA-like data structure for streamgraph
	// Format: { year, revenue, format, group }
	// The 'format' field is used for z-axis grouping, 'group' is used for fill colors
	// Expanded to 30 years (1995-2024) - 120 data points (30 years x 4 formats)
	const riaaData = [
		// 1995
		{ year: 1995, revenue: 12.3, format: 'CD', group: 'CD' },
		{ year: 1995, revenue: 0.1, format: 'Vinyl', group: 'Vinyl' },
		{ year: 1995, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 1995, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 1996
		{ year: 1996, revenue: 13.1, format: 'CD', group: 'CD' },
		{ year: 1996, revenue: 0.1, format: 'Vinyl', group: 'Vinyl' },
		{ year: 1996, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 1996, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 1997
		{ year: 1997, revenue: 13.8, format: 'CD', group: 'CD' },
		{ year: 1997, revenue: 0.1, format: 'Vinyl', group: 'Vinyl' },
		{ year: 1997, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 1997, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 1998
		{ year: 1998, revenue: 14.2, format: 'CD', group: 'CD' },
		{ year: 1998, revenue: 0.1, format: 'Vinyl', group: 'Vinyl' },
		{ year: 1998, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 1998, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 1999
		{ year: 1999, revenue: 14.8, format: 'CD', group: 'CD' },
		{ year: 1999, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 1999, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 1999, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2000
		{ year: 2000, revenue: 14.6, format: 'CD', group: 'CD' },
		{ year: 2000, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2000, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 2000, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2001
		{ year: 2001, revenue: 13.3, format: 'CD', group: 'CD' },
		{ year: 2001, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2001, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 2001, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2002
		{ year: 2002, revenue: 12.9, format: 'CD', group: 'CD' },
		{ year: 2002, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2002, revenue: 0.0, format: 'Digital', group: 'Digital' },
		{ year: 2002, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2003
		{ year: 2003, revenue: 11.8, format: 'CD', group: 'CD' },
		{ year: 2003, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2003, revenue: 0.1, format: 'Digital', group: 'Digital' },
		{ year: 2003, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2004
		{ year: 2004, revenue: 11.2, format: 'CD', group: 'CD' },
		{ year: 2004, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2004, revenue: 0.2, format: 'Digital', group: 'Digital' },
		{ year: 2004, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2005
		{ year: 2005, revenue: 10.5, format: 'CD', group: 'CD' },
		{ year: 2005, revenue: 0.2, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2005, revenue: 0.5, format: 'Digital', group: 'Digital' },
		{ year: 2005, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2006
		{ year: 2006, revenue: 9.4, format: 'CD', group: 'CD' },
		{ year: 2006, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2006, revenue: 1.0, format: 'Digital', group: 'Digital' },
		{ year: 2006, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2007
		{ year: 2007, revenue: 7.5, format: 'CD', group: 'CD' },
		{ year: 2007, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2007, revenue: 1.7, format: 'Digital', group: 'Digital' },
		{ year: 2007, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2008
		{ year: 2008, revenue: 5.5, format: 'CD', group: 'CD' },
		{ year: 2008, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2008, revenue: 2.9, format: 'Digital', group: 'Digital' },
		{ year: 2008, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2009
		{ year: 2009, revenue: 4.3, format: 'CD', group: 'CD' },
		{ year: 2009, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2009, revenue: 3.4, format: 'Digital', group: 'Digital' },
		{ year: 2009, revenue: 0.0, format: 'Streaming', group: 'Streaming' },
		// 2010
		{ year: 2010, revenue: 3.4, format: 'CD', group: 'CD' },
		{ year: 2010, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2010, revenue: 3.4, format: 'Digital', group: 'Digital' },
		{ year: 2010, revenue: 0.5, format: 'Streaming', group: 'Streaming' },
		// 2011
		{ year: 2011, revenue: 3.1, format: 'CD', group: 'CD' },
		{ year: 2011, revenue: 0.3, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2011, revenue: 3.0, format: 'Digital', group: 'Digital' },
		{ year: 2011, revenue: 1.0, format: 'Streaming', group: 'Streaming' },
		// 2012
		{ year: 2012, revenue: 2.8, format: 'CD', group: 'CD' },
		{ year: 2012, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2012, revenue: 2.9, format: 'Digital', group: 'Digital' },
		{ year: 2012, revenue: 1.8, format: 'Streaming', group: 'Streaming' },
		// 2013
		{ year: 2013, revenue: 2.4, format: 'CD', group: 'CD' },
		{ year: 2013, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2013, revenue: 2.8, format: 'Digital', group: 'Digital' },
		{ year: 2013, revenue: 2.8, format: 'Streaming', group: 'Streaming' },
		// 2014
		{ year: 2014, revenue: 2.0, format: 'CD', group: 'CD' },
		{ year: 2014, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2014, revenue: 2.6, format: 'Digital', group: 'Digital' },
		{ year: 2014, revenue: 4.0, format: 'Streaming', group: 'Streaming' },
		// 2015
		{ year: 2015, revenue: 1.8, format: 'CD', group: 'CD' },
		{ year: 2015, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2015, revenue: 2.3, format: 'Digital', group: 'Digital' },
		{ year: 2015, revenue: 5.2, format: 'Streaming', group: 'Streaming' },
		// 2016
		{ year: 2016, revenue: 1.5, format: 'CD', group: 'CD' },
		{ year: 2016, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2016, revenue: 2.0, format: 'Digital', group: 'Digital' },
		{ year: 2016, revenue: 7.0, format: 'Streaming', group: 'Streaming' },
		// 2017
		{ year: 2017, revenue: 1.2, format: 'CD', group: 'CD' },
		{ year: 2017, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2017, revenue: 1.8, format: 'Digital', group: 'Digital' },
		{ year: 2017, revenue: 9.8, format: 'Streaming', group: 'Streaming' },
		// 2018
		{ year: 2018, revenue: 1.0, format: 'CD', group: 'CD' },
		{ year: 2018, revenue: 0.4, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2018, revenue: 1.5, format: 'Digital', group: 'Digital' },
		{ year: 2018, revenue: 12.5, format: 'Streaming', group: 'Streaming' },
		// 2019
		{ year: 2019, revenue: 0.8, format: 'CD', group: 'CD' },
		{ year: 2019, revenue: 0.5, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2019, revenue: 1.2, format: 'Digital', group: 'Digital' },
		{ year: 2019, revenue: 15.2, format: 'Streaming', group: 'Streaming' },
		// 2020
		{ year: 2020, revenue: 0.6, format: 'CD', group: 'CD' },
		{ year: 2020, revenue: 0.6, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2020, revenue: 1.0, format: 'Digital', group: 'Digital' },
		{ year: 2020, revenue: 18.2, format: 'Streaming', group: 'Streaming' },
		// 2021
		{ year: 2021, revenue: 0.5, format: 'CD', group: 'CD' },
		{ year: 2021, revenue: 0.7, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2021, revenue: 0.9, format: 'Digital', group: 'Digital' },
		{ year: 2021, revenue: 20.5, format: 'Streaming', group: 'Streaming' },
		// 2022
		{ year: 2022, revenue: 0.4, format: 'CD', group: 'CD' },
		{ year: 2022, revenue: 0.8, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2022, revenue: 0.8, format: 'Digital', group: 'Digital' },
		{ year: 2022, revenue: 22.8, format: 'Streaming', group: 'Streaming' },
		// 2023
		{ year: 2023, revenue: 0.3, format: 'CD', group: 'CD' },
		{ year: 2023, revenue: 0.9, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2023, revenue: 0.7, format: 'Digital', group: 'Digital' },
		{ year: 2023, revenue: 25.1, format: 'Streaming', group: 'Streaming' },
		// 2024
		{ year: 2024, revenue: 0.3, format: 'CD', group: 'CD' },
		{ year: 2024, revenue: 1.0, format: 'Vinyl', group: 'Vinyl' },
		{ year: 2024, revenue: 0.6, format: 'Digital', group: 'Digital' },
		{ year: 2024, revenue: 27.5, format: 'Streaming', group: 'Streaming' }
	];

	// Alternative data: Technology adoption over time - 20 years (2005-2024) - 80 data points
	const formats = ['Desktop', 'Mobile', 'Tablet', 'Smart TV'];
	const techAdoptionData: Array<{ year: number; revenue: number; format: string; group: string }> = [];
	for (let year = 2005; year <= 2024; year++) {
		const yearIndex = year - 2005;
		formats.forEach((format, fIdx) => {
			let revenue = 0;
			if (format === 'Desktop') {
				revenue = 52 - (yearIndex * 2) + Math.sin(yearIndex * 0.2) * 3;
			} else if (format === 'Mobile') {
				revenue = Math.max(0, 1 + (yearIndex * 3.2) + Math.sin(yearIndex * 0.15) * 2);
			} else if (format === 'Tablet') {
				revenue = year >= 2010 ? Math.max(0, (yearIndex - 5) * 1.4 + Math.sin((yearIndex - 5) * 0.1) * 1) : 0;
			} else if (format === 'Smart TV') {
				revenue = year >= 2011 ? Math.max(0, (yearIndex - 6) * 1.7 + Math.sin((yearIndex - 6) * 0.12) * 1.5) : 0;
			}
			techAdoptionData.push({ year, revenue: Math.max(0, revenue), format, group: format });
		});
	}

	// Sales data by product category - 10 years (2015-2024) quarterly - 160 data points (40 quarters x 4 categories)
	const categories = ['Electronics', 'Clothing', 'Food', 'Books'];
	const salesData: Array<{ year: number; revenue: number; format: string; group: string }> = [];
	for (let year = 2015; year <= 2024; year++) {
		for (let quarter = 0; quarter < 4; quarter++) {
			const yearQuarter = year + (quarter * 0.25);
			const timeIndex = ((year - 2015) * 4) + quarter;
			categories.forEach((category, cIdx) => {
				const baseRevenue = [10000, 7000, 4500, 2500][cIdx];
				const growth = timeIndex * [200, 150, 100, 50][cIdx];
				const seasonal = Math.sin(quarter * Math.PI / 2) * [2000, 1500, 1000, 500][cIdx];
				salesData.push({
					year: yearQuarter,
					revenue: baseRevenue + growth + seasonal + (Math.random() - 0.5) * 1000,
					format: category,
					group: category
				});
			});
		}
	}

	let containerWidth1 = $state(0);
	let containerWidth2 = $state(0);
	let containerWidth3 = $state(0);
	let containerWidth4 = $state(0);

	let containerEl1 = $state<HTMLDivElement>();
	let containerEl2 = $state<HTMLDivElement>();
	let containerEl3 = $state<HTMLDivElement>();
	let containerEl4 = $state<HTMLDivElement>();

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
	});
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Streamgraph</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Streamgraph (RIAA-style) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Music Format Revenue (RIAA-style)</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && riaaData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={0}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, label: 'Year' }}
							y={{ axis: false }}
							color={{ legend: true }}
						>
							<AreaY
								data={riaaData}
								x="year"
								y="revenue"
								z="format"
								curve="basis"
								fill="group"
								opacity={0.8}
								stack={{ offset: 'wiggle', order: 'inside-out' }}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">AreaY</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">stack={{ offset: 'wiggle', order: 'inside-out' }}</code></p>
				</div>
			</div>

			<!-- 2. Technology Adoption Streamgraph -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Technology Adoption Over Time</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && techAdoptionData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={0}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, label: 'Year' }}
							y={{ axis: false }}
							color={{ legend: true }}
						>
							<AreaY
								data={techAdoptionData}
								x="year"
								y="revenue"
								z="format"
								curve="basis"
								fill="group"
								opacity={0.8}
								stack={{ offset: 'wiggle', order: 'inside-out' }}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows device usage trends with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">curve="basis"</code> smoothing</p>
				</div>
			</div>

			<!-- 3. Sales Data Streamgraph -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Sales by Product Category</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && salesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={0}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, label: 'Quarter' }}
							y={{ axis: false }}
							color={{ legend: true }}
						>
							<AreaY
								data={salesData}
								x="year"
								y="revenue"
								z="format"
								curve="basis"
								fill="group"
								opacity={0.8}
								stack={{ offset: 'wiggle', order: 'inside-out' }}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Quarterly sales data with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="group"</code> for automatic coloring</p>
				</div>
			</div>

			<!-- 4. Streamgraph with Custom Opacity -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. High Opacity Streamgraph</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && riaaData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={0}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, label: 'Year' }}
							y={{ axis: false }}
							color={{ legend: true }}
						>
							<AreaY
								data={riaaData}
								x="year"
								y="revenue"
								z="format"
								curve="basis"
								fill="group"
								opacity={0.95}
								stack={{ offset: 'wiggle', order: 'inside-out' }}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Higher <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">opacity={0.95}</code> for more solid appearance</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Streamgraphs</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					Streamgraphs are a variation of stacked area charts with a "wiggle" offset that centers the baseline, creating flowing, organic shapes.
					These examples demonstrate streamgraph visualizations using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">AreaY</code> component with <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">stack={{ offset: 'wiggle', order: 'inside-out' }}</code>.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/examples/area/streamgraph" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Streamgraph Example</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

