<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Circle } from 'lucide-svelte';
	import { Plot, Dot } from 'svelteplot';

	// Helper function to generate bubble data
	function generateBubbleData(count: number, xMin: number, xMax: number, yMin: number, yMax: number, valueMin: number, valueMax: number) {
		return Array.from({ length: count }, (_, i) => ({
			x: xMin + (xMax - xMin) * (i / count) + (Math.random() - 0.5) * (xMax - xMin) * 0.2,
			y: yMin + (yMax - yMin) * (i / count) + (Math.random() - 0.5) * (yMax - yMin) * 0.2,
			value: valueMin + Math.random() * (valueMax - valueMin),
			name: `Product ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`
		}));
	}

	// Sample data for different bubble chart types
	// Basic bubble chart data with variable sizes - 1200 points
	const basicBubbleData = generateBubbleData(1200, 0, 100, 0, 100, 10000, 50000);

	// Country data for beeswarm-style bubble chart - 150 countries
	const continents = ['North America', 'Europe', 'Asia', 'South America', 'Africa', 'Oceania'];
	const countries = ['USA', 'UK', 'Japan', 'Brazil', 'Canada', 'China', 'India', 'Spain', 'Germany', 'Sweden', 'France', 'Italy', 'Australia', 'Mexico', 'Argentina', 'South Korea', 'Netherlands', 'Belgium', 'Switzerland', 'Austria'];
	const countryBubbleData: Array<{ 'Life expectancy': number; Population: number; Continent: string; Country: string }> = [];
	for (let i = 0; i < 150; i++) {
		const continent = continents[i % continents.length];
		const country = countries[i % countries.length] + (i >= countries.length ? ` ${Math.floor(i / countries.length) + 1}` : '');
		countryBubbleData.push({
			'Life expectancy': 65 + Math.random() * 25 + Math.sin(i * 0.1) * 5,
			Population: 1000000 + Math.random() * 2000000000 + Math.sin(i * 0.2) * 500000000,
			Continent: continent,
			Country: country
		});
	}

	// Sales data grouped by region - 800 points
	const regions = ['North', 'South', 'East', 'West', 'Central'];
	const categories = ['Electronics', 'Clothing', 'Food', 'Home', 'Sports'];
	const salesBubbleData: Array<{ region: string; sales: number; profit: number; category: string }> = [];
	for (let i = 0; i < 800; i++) {
		salesBubbleData.push({
			region: regions[i % regions.length],
			sales: 30000 + Math.random() * 30000 + Math.sin(i * 0.1) * 10000,
			profit: 5000 + Math.random() * 15000 + Math.sin(i * 0.12) * 5000,
			category: categories[i % categories.length]
		});
	}

	// Market share data with categories - 200 companies
	const sectors = ['Technology', 'Retail', 'Finance', 'Healthcare', 'Education', 'Automotive', 'Energy', 'Manufacturing', 'Telecom', 'Media'];
	const marketShareData: Array<{ company: string; marketShare: number; revenue: number; sector: string }> = [];
	for (let i = 0; i < 200; i++) {
		marketShareData.push({
			company: `Company ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`,
			marketShare: 5 + Math.random() * 40 + Math.sin(i * 0.15) * 10,
			revenue: 1000000 + Math.random() * 10000000 + Math.sin(i * 0.2) * 3000000,
			sector: sectors[i % sectors.length]
		});
	}

	// Time series bubble data - 500 months
	const quarters = ['Q1', 'Q2', 'Q3', 'Q4'];
	const timeSeriesBubbleData: Array<{ month: number; revenue: number; growth: number; quarter: string }> = [];
	for (let i = 0; i < 500; i++) {
		timeSeriesBubbleData.push({
			month: i + 1,
			revenue: 100000 + (i * 500) + Math.sin(i * 0.1) * 50000 + (Math.random() - 0.5) * 20000,
			growth: 3 + Math.random() * 12 + Math.sin(i * 0.15) * 3,
			quarter: quarters[i % 4]
		});
	}

	// Performance metrics with multiple dimensions - 300 departments
	const departments = ['Sales', 'Marketing', 'Support', 'Operations', 'Development', 'HR', 'Finance', 'Legal', 'IT', 'Research'];
	const performanceData: Array<{ efficiency: number; satisfaction: number; volume: number; department: string }> = [];
	for (let i = 0; i < 300; i++) {
		performanceData.push({
			efficiency: 60 + Math.random() * 35 + Math.sin(i * 0.1) * 10,
			satisfaction: 65 + Math.random() * 30 + Math.sin(i * 0.12) * 8,
			volume: 100000 + Math.random() * 200000 + Math.sin(i * 0.15) * 50000,
			department: departments[i % departments.length] + (i >= departments.length ? ` ${Math.floor(i / departments.length) + 1}` : '')
		});
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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Bubble Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicBubbleData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							r={{ range: [5, 30] }}
							x={{ label: 'X Axis' }}
							y={{ grid: true, label: 'Y Axis' }}
						>
							<Dot data={basicBubbleData} x="x" y="y" r="value" fill="#6366F1" fillOpacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">r="value"</code> for variable bubble sizes</p>
				</div>
			</div>

			<!-- 2. Beeswarm Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Beeswarm Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && countryBubbleData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							inset={20}
							r={{ range: [5, 40] }}
							x={{ type: 'linear', label: 'Life Expectancy' }}
							y={{ axis: false }}
						>
							<Dot
								data={countryBubbleData}
								x="Life expectancy"
								sort={{ channel: '-r' }}
								y={0}
								r="Population"
								dodgeY="middle"
								fill="Continent"
								fillOpacity={0.7}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">dodgeY="middle"</code> and <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">sort</code> for beeswarm layout</p>
				</div>
			</div>

			<!-- 3. Grouped Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Grouped Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && salesBubbleData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							r={{ range: [8, 35] }}
							x={{ label: 'Sales' }}
							y={{ grid: true, label: 'Profit' }}
							color={{ legend: true }}
						>
							<Dot data={salesBubbleData} x="sales" y="profit" r="sales" fill="category" fillOpacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="category"</code> for color grouping with legend</p>
				</div>
			</div>

			<!-- 4. Market Share Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Market Share Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && marketShareData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							r={{ range: [10, 40] }}
							x={{ label: 'Market Share (%)' }}
							y={{ grid: true, label: 'Revenue' }}
							color={{ legend: true }}
						>
							<Dot data={marketShareData} x="marketShare" y="revenue" r="revenue" fill="sector" fillOpacity={0.7} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Bubble size represents revenue, color represents sector</p>
				</div>
			</div>

			<!-- 5. Time Series Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Time Series Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && timeSeriesBubbleData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							r={{ range: [8, 30] }}
							x={{ label: 'Month' }}
							y={{ grid: true, label: 'Growth Rate (%)' }}
							color={{ legend: true }}
						>
							<Dot data={timeSeriesBubbleData} x="month" y="growth" r="revenue" fill="quarter" fillOpacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows revenue (bubble size) and growth rate over time, colored by quarter</p>
				</div>
			</div>

			<!-- 6. Multi-Dimensional Bubble Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Multi-Dimensional Bubble Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && performanceData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							r={{ range: [10, 35] }}
							x={{ label: 'Efficiency (%)' }}
							y={{ grid: true, label: 'Satisfaction (%)' }}
							color={{ legend: true }}
						>
							<Dot data={performanceData} x="efficiency" y="satisfaction" r="volume" fill="department" fillOpacity={0.7} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows efficiency vs satisfaction with volume as bubble size, colored by department</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Bubble Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various bubble chart styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Dot</code> component with variable radius (<code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">r</code> prop).
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/dot" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Bubble Chart Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

