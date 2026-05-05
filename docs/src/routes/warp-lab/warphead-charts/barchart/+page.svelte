<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { BarChart3 } from 'lucide-svelte';
	import { Plot, BarY, BarX } from 'svelteplot';

	// Sample data for different chart types
	// Basic bar data - 80 categories
	const categories = ['Electronics', 'Clothing', 'Food', 'Books', 'Toys', 'Home', 'Sports', 'Automotive', 'Health', 'Beauty', 'Garden', 'Tools', 'Furniture', 'Appliances', 'Jewelry', 'Music', 'Movies', 'Games', 'Software', 'Hardware', 'Office', 'School', 'Baby', 'Pet', 'Travel', 'Outdoor', 'Fitness', 'Art', 'Crafts', 'Collectibles', 'Antiques', 'Vintage', 'Handmade', 'Custom', 'Premium', 'Standard', 'Budget', 'Luxury', 'Basic', 'Deluxe', 'Pro', 'Enterprise', 'Starter', 'Advanced', 'Expert', 'Master', 'Elite', 'Ultimate', 'Supreme', 'Classic', 'Modern', 'Retro', 'Futuristic', 'Traditional', 'Contemporary', 'Minimalist', 'Maximalist', 'Eco', 'Organic', 'Natural', 'Synthetic', 'Digital', 'Analog', 'Hybrid', 'Pure', 'Mixed', 'Original', 'Replica', 'Authentic', 'Genuine', 'Premium', 'Exclusive', 'Limited', 'Special', 'Regular', 'Standard', 'Custom'];
	const basicBarData = Array.from({ length: 80 }, (_, i) => ({
		category: categories[i] || `Category ${i + 1}`,
		value: 5000 + Math.random() * 20000 + Math.sin(i * 0.2) * 5000
	}));

	// Horizontal bar data - 60 quarters (15 years)
	const horizontalBarData = Array.from({ length: 60 }, (_, i) => ({
		category: `Q${(i % 4) + 1} ${2020 + Math.floor(i / 4)}`,
		value: 40000 + (i * 500) + Math.sin(i * 0.1) * 10000 + (Math.random() - 0.5) * 5000
	}));

	// Multi-series data for grouped/stacked charts - 72 months (6 years)
	const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
	const multiSeriesData = Array.from({ length: 72 }, (_, i) => {
		const monthIndex = i % 12;
		const year = Math.floor(i / 12);
		return {
			category: `${monthNames[monthIndex]} ${2020 + year}`,
			sales: 10000 + (i * 200) + Math.sin(i * 0.1) * 5000 + (Math.random() - 0.5) * 2000,
			marketing: 7000 + (i * 150) + Math.sin(i * 0.12) * 3000 + (Math.random() - 0.5) * 1500,
			support: 4000 + (i * 100) + Math.sin(i * 0.08) * 2000 + (Math.random() - 0.5) * 1000
		};
	});

	// Data with different colors per bar - 100 products
	const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#84CC16'];
	const coloredBarData = Array.from({ length: 100 }, (_, i) => ({
		category: `Product ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`,
		value: 8000 + Math.random() * 15000 + Math.sin(i * 0.15) * 4000,
		color: colors[i % colors.length]
	}));

	// Stacked data format - restructured for SveltePlot stacked bars - 60 quarters
	const products = ['Product A', 'Product B', 'Product C', 'Product D', 'Product E'];
	const stackedData: Array<{ category: string; value: number; series: string }> = [];
	for (let q = 0; q < 60; q++) {
		const quarter = `Q${(q % 4) + 1} ${2020 + Math.floor(q / 4)}`;
		products.forEach((product, pIdx) => {
			stackedData.push({
				category: quarter,
				value: 5000 + (q * 100) + (pIdx * 2000) + Math.sin(q * 0.1) * 3000 + (Math.random() - 0.5) * 1000,
				series: product
			});
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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Bar Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Vertical Bar Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Vertical Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicBarData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Category' }}
							y={{ grid: true, label: 'Value' }}
						>
							<BarY data={basicBarData} x="category" y="value" fill="#6366F1" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BarY</code> component</p>
				</div>
			</div>

			<!-- 2. Horizontal Bar Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Horizontal Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && horizontalBarData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ grid: true, label: 'Value' }}
							y={{ label: 'Quarter' }}
						>
							<BarX data={horizontalBarData} x="value" y="category" fill="#10B981" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BarX</code> component</p>
				</div>
			</div>

			<!-- 3. Grouped Bar Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Grouped Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && multiSeriesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month' }}
							y={{ grid: true, label: 'Amount' }}
						>
							<BarY data={multiSeriesData} x="category" y="sales" fill="#6366F1" />
							<BarY data={multiSeriesData} x="category" y="marketing" fill="#10B981" />
							<BarY data={multiSeriesData} x="category" y="support" fill="#F59E0B" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BarY</code> components for grouping</p>
				</div>
			</div>

			<!-- 4. Stacked Bar Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Stacked Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && stackedData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Quarter' }}
							y={{ grid: true, label: 'Value' }}
						>
							<BarY data={stackedData} x="category" y="value" fill="series" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BarY</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="series"</code> for automatic stacking</p>
				</div>
			</div>

			<!-- 5. Colored Bar Chart (Different color per bar) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Multi-Color Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && coloredBarData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Product' }}
							y={{ grid: true, label: 'Revenue' }}
						>
							<BarY data={coloredBarData} x="category" y="value" fill="color" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="color"</code> for dynamic colors</p>
				</div>
			</div>

			<!-- 6. Gradient Bar Chart -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Gradient Bar Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && basicBarData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Category' }}
							y={{ grid: true, label: 'Value' }}
						>
							<BarY data={basicBarData} x="category" y="value" fill="url(#gradient)" />
							<defs>
								<linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#8B5CF6;stop-opacity:1" />
									<stop offset="100%" style="stop-color:#6366F1;stop-opacity:1" />
								</linearGradient>
							</defs>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses SVG gradient with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="url(#gradient)"</code></p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Bar Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various bar chart styles using <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">BarY</code> and <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">BarX</code> components.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>
