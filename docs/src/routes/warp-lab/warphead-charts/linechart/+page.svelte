<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { TrendingUp } from 'lucide-svelte';
	import { Plot, LineY } from 'svelteplot';

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
	// Basic line data - 730 points (2 years of daily data)
	const basicLineData = generateTimeSeries(730, 120, 0.15, 0.15);

	// Format function for basic line chart
	function formatBasicMonth(d: number) {
		const item = basicLineData.find(item => item.month === d);
		return item?.label || String(d);
	}

	// Time series data - 365 points (1 year of daily data)
	const timeSeriesData = generateTimeSeries(365, 120, 0.2, 0.12).map((d, i) => ({
		date: i + 1,
		value: d.value,
		label: d.label
	}));

	// Multi-series data for multiple lines - 600 points
	const multiSeriesData = Array.from({ length: 600 }, (_, i) => {
		const base = 10000;
		const monthIndex = i % 12;
		const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
		return {
			month: i + 1,
			sales: base * 1.5 + (i * 20) + Math.sin(i * 0.1) * base * 0.3 + (Math.random() - 0.5) * base * 0.2,
			marketing: base + (i * 15) + Math.sin(i * 0.12) * base * 0.25 + (Math.random() - 0.5) * base * 0.15,
			support: base * 0.6 + (i * 10) + Math.sin(i * 0.08) * base * 0.2 + (Math.random() - 0.5) * base * 0.1,
			label: monthNames[monthIndex] + (i >= 12 ? ` ${Math.floor(i / 12) + 1}` : '')
		};
	});

	// Format function for multi-line chart
	function formatMultiMonth(d: number) {
		const item = multiSeriesData.find(item => item.month === d);
		return item?.label || String(d);
	}

	// Data with different colors per line - 500 points
	const coloredLineData = Array.from({ length: 500 }, (_, i) => {
		const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444'];
		return {
			month: `Q${(i % 4) + 1}`,
			revenue: 15000 + (i * 30) + Math.sin(i * 0.1) * 5000,
			color: colors[i % colors.length]
		};
	});

	// Smooth curve data with labels - 800 points
	const smoothCurveData = Array.from({ length: 800 }, (_, i) => ({
		x: i,
		y: 70 + Math.sin(i * 0.05) * 15 + Math.cos(i * 0.03) * 8 + (Math.random() - 0.5) * 3,
		label: String(i)
	}));

	// Format function for thick line chart
	function formatThickX(d: number) {
		const item = smoothCurveData.find(item => item.x === d);
		return item?.label || String(d);
	}

	// Temperature data for gradient example - 365 points (daily for a year)
	// Using numeric day values to ensure chronological ordering
	const temperatureData = Array.from({ length: 365 }, (_, i) => {
		const day = i + 1;
		const dayNames = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
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

	// Format function for time series
	function formatMonth(d: number) {
		const item = timeSeriesData.find(item => item.date === d);
		return item?.label || String(d);
	}
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Line Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Line Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Line Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicLineData.length > 0}
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
							<LineY data={basicLineData} x="month" y="value" stroke="#6366F1" strokeWidth={2} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">LineY</code> component</p>
				</div>
			</div>

			<!-- 2. Time Series Line Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Time Series Line Chart</h3>
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
							<LineY data={timeSeriesData} x="date" y="value" stroke="#10B981" strokeWidth={2} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses custom <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">tickFormat</code> for labels</p>
				</div>
			</div>

			<!-- 3. Multi-Line Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Multi-Line Chart</h3>
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
							<LineY data={multiSeriesData} x="month" y="sales" stroke="#6366F1" strokeWidth={2} />
							<LineY data={multiSeriesData} x="month" y="marketing" stroke="#10B981" strokeWidth={2} />
							<LineY data={multiSeriesData} x="month" y="support" stroke="#F59E0B" strokeWidth={2} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">LineY</code> components for multiple series</p>
				</div>
			</div>

			<!-- 4. Thick Line Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Thick Line Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && smoothCurveData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'X Axis', tickFormat: formatThickX }}
							y={{ grid: true, label: 'Y Axis', nice: true }}
						>
							<LineY data={smoothCurveData} x="x" y="y" stroke="#8B5CF6" strokeWidth={4} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">strokeWidth={4}</code> for thicker lines</p>
				</div>
			</div>

			<!-- 5. Colored Line Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Custom Color Line Chart</h3>
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
							<LineY data={temperatureData} x="day" y="temp" stroke="#EF4444" strokeWidth={3} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Custom <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">stroke</code> color for different themes</p>
				</div>
			</div>

			<!-- 6. Smooth Line Chart with Multiple Series -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Multi-Series Comparison</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && multiSeriesData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatMultiMonth }}
							y={{ grid: true, label: 'Amount' }}
						>
							<LineY data={multiSeriesData} x="month" y="sales" stroke="#6366F1" strokeWidth={2.5} />
							<LineY data={multiSeriesData} x="month" y="marketing" stroke="#10B981" strokeWidth={2.5} />
							<LineY data={multiSeriesData} x="month" y="support" stroke="#F59E0B" strokeWidth={2.5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Comparing multiple metrics with different colors</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Line Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various line chart styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">LineY</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/line" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Line Chart Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

