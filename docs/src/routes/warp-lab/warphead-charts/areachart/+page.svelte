<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { AreaChart } from 'lucide-svelte';
	import { Plot, AreaY } from 'svelteplot';

	// Helper function to generate time series data
	function generateTimeSeries(count: number, baseValue: number, trend: number, noise: number = 0.1) {
		const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
		return Array.from({ length: count }, (_, i) => {
			const value = baseValue + (trend * i) + (Math.sin(i * 0.1) * baseValue * noise) + (Math.random() - 0.5) * baseValue * noise;
			const monthIndex = i % 12;
			return {
				month: i + 1,
				value: Math.max(0, value),
				label: monthNames[monthIndex] + (i >= 12 ? ` ${Math.floor(i / 12) + 1}` : '')
			};
		});
	}

	// Sample data for different chart types
	// Using numeric month values to ensure chronological ordering
	// Basic area data - 730 points (2 years of daily data)
	const basicAreaData = generateTimeSeries(730, 120, 0.15, 0.15);

	// Format function for basic area chart
	function formatBasicMonth(d: number) {
		const item = basicAreaData.find(item => item.month === d);
		return item?.label || String(d);
	}

	// Time series data - 365 points (1 year of daily data)
	const timeSeriesData = generateTimeSeries(365, 120, 0.2, 0.12).map((d, i) => ({
		date: i + 1,
		value: d.value,
		label: d.label
	}));

	// Multi-series data for multiple areas - 600 points
	const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
	const multiSeriesData = Array.from({ length: 600 }, (_, i) => {
		const base = 10000;
		const monthIndex = i % 12;
		return {
			month: i + 1,
			sales: base * 1.5 + (i * 20) + Math.sin(i * 0.1) * base * 0.3 + (Math.random() - 0.5) * base * 0.2,
			marketing: base + (i * 15) + Math.sin(i * 0.12) * base * 0.25 + (Math.random() - 0.5) * base * 0.15,
			support: base * 0.6 + (i * 10) + Math.sin(i * 0.08) * base * 0.2 + (Math.random() - 0.5) * base * 0.1,
			label: monthNames[monthIndex] + (i >= 12 ? ` ${Math.floor(i / 12) + 1}` : '')
		};
	});

	// Format function for multi-area chart
	function formatMultiMonth(d: number) {
		const item = multiSeriesData.find(item => item.month === d);
		return item?.label || String(d);
	}

	// Stacked data format - restructured for SveltePlot stacked areas - 500 months
	const products = ['Product A', 'Product B', 'Product C', 'Product D', 'Product E'];
	const stackedData: Array<{ month: number; value: number; series: string; label: string }> = [];
	for (let m = 0; m < 500; m++) {
		const monthIndex = m % 12;
		const label = monthNames[monthIndex] + (m >= 12 ? ` ${Math.floor(m / 12) + 1}` : '');
		products.forEach((product, pIdx) => {
			stackedData.push({
				month: m + 1,
				value: 5000 + (m * 50) + (pIdx * 2000) + Math.sin(m * 0.1) * 3000 + (Math.random() - 0.5) * 1000,
				series: product,
				label
			});
		});
	}

	// Format function for stacked area chart
	function formatStackedMonth(d: number) {
		const item = stackedData.find(item => item.month === d);
		return item?.label || String(d);
	}

	// Data with different colors per area - 500 points
	const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444'];
	const coloredAreaData = Array.from({ length: 500 }, (_, i) => ({
		month: `Q${(i % 4) + 1}`,
		revenue: 15000 + (i * 30) + Math.sin(i * 0.1) * 5000,
		color: colors[i % colors.length]
	}));

	// Temperature data for gradient example - 365 points (daily for a year)
	// Using numeric day values to ensure chronological ordering
	const dayNames = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
	const temperatureData = Array.from({ length: 365 }, (_, i) => {
		const baseTemp = 22 + Math.sin((i / 365) * 2 * Math.PI) * 8;
		return {
			day: (i % 7) + 1,
			temp: baseTemp + (Math.random() - 0.5) * 4,
			label: dayNames[i % 7]
		};
	});

	// Format function for temperature chart
	function formatDay(d: number) {
		const item = temperatureData.find(item => item.day === d);
		return item?.label || String(d);
	}

	// Format function for time series
	function formatMonth(d: number) {
		const item = timeSeriesData.find(item => item.date === d);
		return item?.label || String(d);
	}

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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Area Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicAreaData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatBasicMonth }}
							y={{ grid: true, label: 'Value' }}
						>
							<AreaY data={basicAreaData} x="month" y="value" fill="#6366F1" fillOpacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">AreaY</code> component</p>
				</div>
			</div>

			<!-- 2. Time Series Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Time Series Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && timeSeriesData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatMonth }}
							y={{ grid: true, label: 'Value' }}
						>
							<AreaY data={timeSeriesData} x="date" y="value" fill="#10B981" fillOpacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses custom <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">tickFormat</code> for labels</p>
				</div>
			</div>

			<!-- 3. Multi-Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Multi-Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && multiSeriesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatMultiMonth }}
							y={{ grid: true, label: 'Amount' }}
						>
							<AreaY data={multiSeriesData} x="month" y="sales" fill="#6366F1" fillOpacity={0.5} />
							<AreaY data={multiSeriesData} x="month" y="marketing" fill="#10B981" fillOpacity={0.5} />
							<AreaY data={multiSeriesData} x="month" y="support" fill="#F59E0B" fillOpacity={0.5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">AreaY</code> components for multiple series</p>
				</div>
			</div>

			<!-- 4. Stacked Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Stacked Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && stackedData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatStackedMonth }}
							y={{ grid: true, label: 'Value' }}
						>
							<AreaY data={stackedData} x="month" y="value" fill="series" fillOpacity={0.7} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">AreaY</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="series"</code> for automatic stacking</p>
				</div>
			</div>

			<!-- 5. Custom Color Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Custom Color Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && temperatureData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Day', tickFormat: formatDay }}
							y={{ grid: true, label: 'Temperature (°C)' }}
						>
							<AreaY data={temperatureData} x="day" y="temp" fill="#EF4444" fillOpacity={0.5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Custom <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill</code> color for different themes</p>
				</div>
			</div>

			<!-- 6. Gradient Area Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Gradient Area Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && basicAreaData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatBasicMonth }}
							y={{ grid: true, label: 'Value' }}
						>
							<AreaY data={basicAreaData} x="month" y="value" fill="url(#areaGradient)" fillOpacity={0.8} />
							<defs>
								<linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#8B5CF6;stop-opacity:1" />
									<stop offset="100%" style="stop-color:#6366F1;stop-opacity:0.3" />
								</linearGradient>
							</defs>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses SVG gradient with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="url(#areaGradient)"</code></p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Area Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various area chart styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">AreaY</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/area" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Area Chart Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

