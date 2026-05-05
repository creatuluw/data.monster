<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { ArrowRight } from 'lucide-svelte';
	import { Plot, Arrow, Text } from 'svelteplot';

	// Basic arrow data - 50 arrows
	const basicArrowData = Array.from({ length: 50 }, (_, i) => ({
		x1: i * 2,
		y1: 10 + Math.sin(i * 0.2) * 5,
		x2: (i * 2) + 3 + Math.random() * 2,
		y2: 15 + Math.sin(i * 0.2) * 5 + (Math.random() - 0.5) * 3
	}));

	// Population and inequality data - 200 cities
	const metros: Array<{ POP_1980: number; R90_10_1980: number; POP_2015: number; R90_10_2015: number; name: string }> = [];
	for (let i = 0; i < 200; i++) {
		const basePop = 500000 + Math.random() * 3000000;
		metros.push({
			POP_1980: basePop,
			R90_10_1980: 3.0 + Math.random() * 3.0 + Math.sin(i * 0.1) * 1.0,
			POP_2015: basePop * (1.1 + Math.random() * 0.3),
			R90_10_2015: 3.5 + Math.random() * 3.5 + Math.sin(i * 0.12) * 1.2,
			name: `City ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`
		});
	}

	// Sales performance data - 300 companies
	const salesData: Array<{ revenue_2020: number; profit_2020: number; revenue_2023: number; profit_2023: number; company: string }> = [];
	for (let i = 0; i < 300; i++) {
		const baseRevenue = 200000 + Math.random() * 1000000;
		salesData.push({
			revenue_2020: baseRevenue,
			profit_2020: baseRevenue * 0.1 + (Math.random() - 0.5) * baseRevenue * 0.05,
			revenue_2023: baseRevenue * (1.3 + Math.random() * 0.4),
			profit_2023: baseRevenue * 0.12 + (Math.random() - 0.5) * baseRevenue * 0.06,
			company: `Company ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`
		});
	}

	// Temperature change data - 150 locations
	const locations = ['Coastal', 'Mountain', 'Desert', 'Forest', 'Urban', 'Rural', 'Suburban', 'Tropical', 'Arctic', 'Temperate'];
	const temperatureData: Array<{ temp_jan: number; humidity_jan: number; temp_jul: number; humidity_jul: number; location: string }> = [];
	for (let i = 0; i < 150; i++) {
		const location = locations[i % locations.length] + (i >= locations.length ? ` ${Math.floor(i / locations.length) + 1}` : '');
		const baseTemp = [15, 5, 20, 10, 12, 8, 11, 25, -10, 18][i % locations.length];
		temperatureData.push({
			temp_jan: baseTemp + (Math.random() - 0.5) * 5,
			humidity_jan: 50 + Math.random() * 30 + Math.sin(i * 0.1) * 10,
			temp_jul: baseTemp + 10 + (Math.random() - 0.5) * 8,
			humidity_jul: 55 + Math.random() * 30 + Math.sin(i * 0.12) * 12,
			location
		});
	}

	// Market share transition - 200 brands
	const marketData: Array<{ share_2020: number; growth_2020: number; share_2023: number; growth_2023: number; brand: string }> = [];
	for (let i = 0; i < 200; i++) {
		const baseShare = 5 + Math.random() * 30;
		marketData.push({
			share_2020: baseShare,
			growth_2020: 2 + Math.random() * 8 + Math.sin(i * 0.1) * 2,
			share_2023: baseShare * (1.1 + Math.random() * 0.3),
			growth_2023: 3 + Math.random() * 10 + Math.sin(i * 0.12) * 3,
			brand: `Brand ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`
		});
	}

	// Customer satisfaction over time - 250 products
	const satisfactionData: Array<{ customers_2020: number; satisfaction_2020: number; customers_2023: number; satisfaction_2023: number; product: string }> = [];
	for (let i = 0; i < 250; i++) {
		const baseCustomers = 500 + Math.random() * 2000;
		satisfactionData.push({
			customers_2020: baseCustomers,
			satisfaction_2020: 3.0 + Math.random() * 1.5 + Math.sin(i * 0.1) * 0.3,
			customers_2023: baseCustomers * (1.2 + Math.random() * 0.6),
			satisfaction_2023: 3.5 + Math.random() * 1.2 + Math.sin(i * 0.12) * 0.4,
			product: `Product ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`
		});
	}

	// Highlight state for interactivity
	let hl1 = $state<any>(null);
	let hl2 = $state<any>(null);
	let hl3 = $state<any>(null);
	let hl4 = $state<any>(null);
	let hl5 = $state<any>(null);
	let hl6 = $state<any>(null);

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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Delta Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Arrow -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Arrow</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicArrowData.length > 0}
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
							<Arrow
								data={basicArrowData}
								x1="x1"
								y1="y1"
								x2="x2"
								y2="y2"
								stroke="#6366F1"
								strokeWidth={2}
								markerEnd="arrow"
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Arrow</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">x1</code>, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">y1</code>, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">x2</code>, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">y2</code> coordinates</p>
				</div>
			</div>

			<!-- 2. Population & Inequality with Color Scheme -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Population & Inequality with Color Scheme</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && metros.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'log', label: 'Population' }}
							y={{ label: 'Inequality', type: 'point' }}
							color={{
								scheme: 'BuRd',
								label: 'Change in inequality from 1980 to 2015',
								legend: true,
								tickFormat: '+f'
							}}
						>
							<Arrow
								data={metros}
								x1="POP_1980"
								y1="R90_10_1980"
								x2="POP_2015"
								y2="R90_10_2015"
								stroke={(d) => d.R90_10_2015 - d.R90_10_1980}
								bend
								opacity={{
									scale: null,
									value: (d) => !hl2 || hl2.name === d.name ? 1 : 0.1
								}}
								onmouseenter={(evt, d) => (hl2 = d)}
								onmouseleave={() => (hl2 = null)}
							/>
							<Text
								data={metros}
								x="POP_2015"
								y="R90_10_2015"
								filter={(d) => hl2 ? hl2.name === d.name : false}
								text="name"
								fill="currentColor"
								stroke="var(--svelteplot-bg)"
								strokeWidth={4}
								lineAnchor="bottom"
								dy={-6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">color</code> scheme, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">bend</code>, and interactive <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">opacity</code></p>
				</div>
			</div>

			<!-- 3. Sales Performance Transition -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Sales Performance Transition</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && salesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Revenue' }}
							y={{ grid: true, label: 'Profit' }}
							color={{
								scheme: 'Viridis',
								label: 'Profit Change',
								legend: true
							}}
						>
							<Arrow
								data={salesData}
								x1="revenue_2020"
								y1="profit_2020"
								x2="revenue_2023"
								y2="profit_2023"
								stroke={(d) => d.profit_2023 - d.profit_2020}
								bend
								opacity={{
									scale: null,
									value: (d) => !hl3 || hl3.company === d.company ? 1 : 0.1
								}}
								onmouseenter={(evt, d) => (hl3 = d)}
								onmouseleave={() => (hl3 = null)}
							/>
							<Text
								data={salesData}
								x="revenue_2023"
								y="profit_2023"
								filter={(d) => hl3 ? hl3.company === d.company : false}
								text="company"
								fill="currentColor"
								stroke="var(--svelteplot-bg)"
								strokeWidth={4}
								lineAnchor="bottom"
								dy={-6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows revenue and profit changes with color-coded transitions</p>
				</div>
			</div>

			<!-- 4. Temperature & Humidity Change -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Temperature & Humidity Change</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && temperatureData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Temperature (°C)' }}
							y={{ grid: true, label: 'Humidity (%)' }}
							color={{
								scheme: 'RdYlBu',
								label: 'Temperature Change',
								legend: true
							}}
						>
							<Arrow
								data={temperatureData}
								x1="temp_jan"
								y1="humidity_jan"
								x2="temp_jul"
								y2="humidity_jul"
								stroke={(d) => d.temp_jul - d.temp_jan}
								bend
								opacity={{
									scale: null,
									value: (d) => !hl4 || hl4.location === d.location ? 1 : 0.1
								}}
								onmouseenter={(evt, d) => (hl4 = d)}
								onmouseleave={() => (hl4 = null)}
							/>
							<Text
								data={temperatureData}
								x="temp_jul"
								y="humidity_jul"
								filter={(d) => hl4 ? hl4.location === d.location : false}
								text="location"
								fill="currentColor"
								stroke="var(--svelteplot-bg)"
								strokeWidth={4}
								lineAnchor="bottom"
								dy={-6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Seasonal changes from January to July with interactive labels</p>
				</div>
			</div>

			<!-- 5. Market Share Transition -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Market Share Transition</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && marketData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Market Share (%)' }}
							y={{ grid: true, label: 'Growth Rate (%)' }}
							color={{
								scheme: 'Spectral',
								label: 'Share Change',
								legend: true,
								tickFormat: '+f'
							}}
						>
							<Arrow
								data={marketData}
								x1="share_2020"
								y1="growth_2020"
								x2="share_2023"
								y2="growth_2023"
								stroke={(d) => d.share_2023 - d.share_2020}
								bend
								opacity={{
									scale: null,
									value: (d) => !hl5 || hl5.brand === d.brand ? 1 : 0.1
								}}
								onmouseenter={(evt, d) => (hl5 = d)}
								onmouseleave={() => (hl5 = null)}
							/>
							<Text
								data={marketData}
								x="share_2023"
								y="growth_2023"
								filter={(d) => hl5 ? hl5.brand === d.brand : false}
								text="brand"
								fill="currentColor"
								stroke="var(--svelteplot-bg)"
								strokeWidth={4}
								lineAnchor="bottom"
								dy={-6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Market share and growth rate changes with color-coded transitions</p>
				</div>
			</div>

			<!-- 6. Customer Satisfaction Growth -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Customer Satisfaction Growth</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && satisfactionData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'log', label: 'Customers' }}
							y={{ label: 'Satisfaction Score', type: 'point' }}
							color={{
								scheme: 'YlGnBu',
								label: 'Satisfaction Change',
								legend: true,
								tickFormat: '+f'
							}}
						>
							<Arrow
								data={satisfactionData}
								x1="customers_2020"
								y1="satisfaction_2020"
								x2="customers_2023"
								y2="satisfaction_2023"
								stroke={(d) => d.satisfaction_2023 - d.satisfaction_2020}
								bend
								opacity={{
									scale: null,
									value: (d) => !hl6 || hl6.product === d.product ? 1 : 0.1
								}}
								onmouseenter={(evt, d) => (hl6 = d)}
								onmouseleave={() => (hl6 = null)}
							/>
							<Text
								data={satisfactionData}
								x="customers_2023"
								y="satisfaction_2023"
								filter={(d) => hl6 ? hl6.product === d.product : false}
								text="product"
								fill="currentColor"
								stroke="var(--svelteplot-bg)"
								strokeWidth={4}
								lineAnchor="bottom"
								dy={-6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Log scale with customer growth and satisfaction improvements</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Delta Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various delta chart styles (showing changes over time) using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Arrow</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/arrow" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Arrow Chart Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

