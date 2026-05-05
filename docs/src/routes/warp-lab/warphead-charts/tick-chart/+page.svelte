<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Minus } from 'lucide-svelte';
	import { Plot, TickX, TickY, RuleX, RuleY } from 'svelteplot';

	// Sample data for TickX - population share by age (similar to stateage example)
	// Dense data: every year from 0 to 100
	const stateageData = Array.from({ length: 101 }, (_, i) => {
		const age = i;
		// Create a realistic age distribution curve
		let pop_share = 0;
		if (age < 5) pop_share = 0.010 + (age * 0.001);
		else if (age < 20) pop_share = 0.015 + ((age - 5) * 0.0008);
		else if (age < 40) pop_share = 0.027 + ((age - 20) * 0.0003);
		else if (age < 60) pop_share = 0.033 - ((age - 40) * 0.0004);
		else if (age < 80) pop_share = 0.025 - ((age - 60) * 0.0005);
		else pop_share = Math.max(0.001, 0.005 - ((age - 80) * 0.0002));
		// Add some variation
		pop_share += (Math.sin(age * 0.3) * 0.002);
		return { age, pop_share: Math.max(0.001, Math.min(0.04, pop_share)) };
	});

	// Sample data for TickY - similar structure but for vertical ticks
	// Dense data: 60 categories
	const verticalTickData = Array.from({ length: 60 }, (_, i) => ({
		category: String.fromCharCode(65 + (i % 26)) + (i >= 26 ? Math.floor(i / 26) : ''),
		value: 0.05 + (Math.random() * 0.35) + (Math.sin(i * 0.2) * 0.1)
	}));

	// Sample data - temperature by day (365 days)
	const temperatureData = Array.from({ length: 365 }, (_, i) => {
		const day = i + 1;
		const month = Math.floor((day - 1) / 30.44) + 1;
		const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
		// Seasonal temperature curve with daily variation
		const baseTemp = 0.15 + (Math.sin((day / 365) * 2 * Math.PI - Math.PI / 2) * 0.15);
		const dailyVar = (Math.sin(day * 0.1) * 0.02);
		const temp = Math.max(0.01, Math.min(0.40, baseTemp + dailyVar));
		return {
			month,
			temp,
			label: monthNames[month - 1]
		};
	});

	// Format function for temperature months
	function formatMonth(d: number) {
		const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
		return monthNames[Math.min(11, Math.max(0, Math.floor(d) - 1))] || String(d);
	}

	// Sample data - revenue share by product (80 products)
	const revenueData = Array.from({ length: 80 }, (_, i) => {
		const share = (0.15 - (i * 0.0015)) + (Math.random() * 0.02) + (Math.sin(i * 0.3) * 0.01);
		return {
			product: `Product ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(2, '0')}`,
			share: Math.max(0.001, Math.min(0.20, share))
		};
	});

	// Sample data - market share by region (50 regions)
	const marketShareData = Array.from({ length: 50 }, (_, i) => {
		const regions = ['North', 'South', 'East', 'West', 'Central', 'Northeast', 'Northwest', 'Southeast', 'Southwest', 'Midwest'];
		const share = (0.12 - (i * 0.002)) + (Math.random() * 0.015) + (Math.sin(i * 0.4) * 0.008);
		return {
			region: `${regions[i % regions.length]} ${i >= regions.length ? Math.floor(i / regions.length) + 1 : ''}`.trim(),
			share: Math.max(0.001, Math.min(0.15, share))
		};
	});

	// Sample data - performance scores (100 teams)
	const performanceData = Array.from({ length: 100 }, (_, i) => {
		const score = 0.40 + (Math.random() * 0.45) + (Math.sin(i * 0.15) * 0.08);
		return {
			team: `Team ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`,
			score: Math.max(0.20, Math.min(0.98, score))
		};
	});

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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Tick Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic TickX Chart (Horizontal Ticks) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic TickX Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && stateageData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, percent: true, label: 'Population Share' }}
							y={{ label: 'Age' }}
						>
							<RuleX data={[0]} />
							<TickX data={stateageData} y="age" x="pop_share" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickX</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">RuleX</code> for reference line</p>
				</div>
			</div>

			<!-- 2. Basic TickY Chart (Vertical Ticks) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Basic TickY Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && verticalTickData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Category' }}
							y={{ grid: true, percent: true, label: 'Value' }}
						>
							<RuleY data={[0]} />
							<TickY data={verticalTickData} x="category" y="value" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickY</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">RuleY</code> for reference line</p>
				</div>
			</div>

			<!-- 3. TickX with Time Series -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. TickX with Time Series</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && temperatureData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, percent: true, label: 'Temperature Index', tickFormat: formatMonth }}
							y={{ label: 'Month' }}
						>
							<RuleX data={[0]} />
							<TickX data={temperatureData} y="month" x="temp" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickX</code> with custom tick formatting for time series</p>
				</div>
			</div>

			<!-- 4. TickY with Multiple Categories -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. TickY with Categories</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && revenueData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Product' }}
							y={{ grid: true, percent: true, label: 'Revenue Share' }}
						>
							<RuleY data={[0]} />
							<TickY data={revenueData} x="product" y="share" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickY</code> to show revenue share by product</p>
				</div>
			</div>

			<!-- 5. TickX with Market Share -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. TickX Market Share</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && marketShareData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, percent: true, label: 'Market Share' }}
							y={{ label: 'Region' }}
						>
							<RuleX data={[0]} />
							<TickX data={marketShareData} y="region" x="share" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickX</code> to visualize market share by region</p>
				</div>
			</div>

			<!-- 6. TickY Performance Scores -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. TickY Performance Scores</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && performanceData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Team' }}
							y={{ grid: true, percent: true, label: 'Performance Score' }}
						>
							<RuleY data={[0]} />
							<TickY data={performanceData} x="team" y="score" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">TickY</code> to display performance scores by team</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Tick Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various tick chart styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">TickX</code> and <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">TickY</code> components.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/examples/tick/tick-x" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot TickX Documentation</a> | 
					<a href="https://svelteplot.dev/examples/tick/tick-y" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot TickY Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

