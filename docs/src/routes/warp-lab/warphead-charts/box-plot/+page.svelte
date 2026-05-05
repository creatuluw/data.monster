<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Box } from 'lucide-svelte';
	import { Plot, BoxY, BoxX } from 'svelteplot';

	// Helper function to generate box plot data (multiple values per category)
	function generateBoxData(categories: string[], valuesPerCategory: number, baseValueFn: (catIdx: number) => number, spreadFn: (catIdx: number) => number) {
		const data: Array<{ [key: string]: string | number }> = [];
		categories.forEach((category, catIdx) => {
			const baseValue = baseValueFn(catIdx);
			const spread = spreadFn(catIdx);
			for (let i = 0; i < valuesPerCategory; i++) {
				data.push({
					[Object.keys(data[0] || {})[0] || 'category']: category,
					[Object.keys(data[0] || {})[1] || 'value']: baseValue + (Math.random() - 0.5) * spread + Math.sin(i * 0.1) * (spread * 0.2)
				});
			}
		});
		return data;
	}

	// Sample data: Vehicle fuel economy by class - 20 classes x 50 vehicles each (1000 points)
	const vehicleClasses = ['compact', 'midsize', 'suv', 'pickup', 'minivan', 'sedan', 'coupe', 'convertible', 'wagon', 'hatchback', 'truck', 'van', 'crossover', 'luxury', 'sports', 'hybrid', 'electric', 'diesel', 'gasoline', 'plugin'];
	const mpgData: Array<{ class: string; hwy: number }> = [];
	vehicleClasses.forEach((vehicleClass, idx) => {
		const baseMPG = [35, 28, 20, 18, 24, 30, 32, 28, 26, 33, 19, 22, 25, 27, 29, 45, 50, 30, 26, 48][idx % 20];
		for (let i = 0; i < 50; i++) {
			mpgData.push({
				class: vehicleClass,
				hwy: baseMPG + (Math.random() - 0.5) * 8 + Math.sin(i * 0.2) * 3
			});
		}
	});

	// Sample data: Olympic athletes' weights by sport - 15 sports x 40 athletes each (600 points)
	const sports = ['Swimming', 'Basketball', 'Gymnastics', 'Weightlifting', 'Running', 'Cycling', 'Tennis', 'Soccer', 'Volleyball', 'Baseball', 'Football', 'Wrestling', 'Boxing', 'Martial Arts', 'Track & Field'];
	const olympiansData: Array<{ sport: string; weight: number }> = [];
	sports.forEach((sport, idx) => {
		const baseWeight = [75, 95, 55, 120, 65, 70, 72, 78, 82, 85, 100, 90, 75, 70, 68][idx % 15];
		for (let i = 0; i < 40; i++) {
			olympiansData.push({
				sport,
				weight: baseWeight + (Math.random() - 0.5) * 20 + Math.sin(i * 0.15) * 8
			});
		}
	});

	// Sample data: Sales performance by department - 15 departments x 40 sales each (600 points)
	const departments = ['Electronics', 'Clothing', 'Food', 'Home', 'Sports', 'Books', 'Toys', 'Health', 'Beauty', 'Automotive', 'Garden', 'Tools', 'Office', 'School', 'Baby'];
	const salesData: Array<{ department: string; sales: number }> = [];
	departments.forEach((department, idx) => {
		const baseSales = 35000 + (idx * 2000);
		for (let i = 0; i < 40; i++) {
			salesData.push({
				department,
				sales: baseSales + (Math.random() - 0.5) * 15000 + Math.sin(i * 0.1) * 5000
			});
		}
	});

	// Sample data: Test scores by subject - 12 subjects x 50 students each (600 points)
	const subjects = ['Math', 'Science', 'English', 'History', 'Art', 'Music', 'PE', 'Geography', 'Chemistry', 'Physics', 'Biology', 'Literature'];
	const testScoresData: Array<{ subject: string; score: number }> = [];
	subjects.forEach((subject, idx) => {
		const baseScore = [85, 78, 92, 72, 80, 82, 88, 75, 79, 81, 77, 90][idx % 12];
		for (let i = 0; i < 50; i++) {
			testScoresData.push({
				subject,
				score: baseScore + (Math.random() - 0.5) * 20 + Math.sin(i * 0.12) * 6
			});
		}
	});

	// Sample data: Product prices by category - 10 categories x 40 products each (400 points)
	const priceCategories = ['Premium', 'Standard', 'Budget', 'Luxury', 'Economy', 'Professional', 'Enterprise', 'Starter', 'Advanced', 'Ultimate'];
	const productPricesData: Array<{ category: string; price: number }> = [];
	priceCategories.forEach((category, idx) => {
		const basePrice = [300, 100, 30, 500, 50, 200, 400, 75, 150, 600][idx % 10];
		for (let i = 0; i < 40; i++) {
			productPricesData.push({
				category,
				price: basePrice + (Math.random() - 0.5) * (basePrice * 0.5) + Math.sin(i * 0.1) * (basePrice * 0.2)
			});
		}
	});

	// Sample data: Temperature by region - 12 regions x 30 measurements each (360 points)
	const regions = ['Tropical', 'Temperate', 'Arctic', 'Desert', 'Mediterranean', 'Continental', 'Oceanic', 'Mountain', 'Polar', 'Subtropical', 'Tundra', 'Savanna'];
	const temperatureData: Array<{ region: string; temp: number }> = [];
	regions.forEach((region, idx) => {
		const baseTemp = [28, 18, -5, 35, 22, 15, 20, 10, -10, 25, -2, 30][idx % 12];
		for (let i = 0; i < 30; i++) {
			temperatureData.push({
				region,
				temp: baseTemp + (Math.random() - 0.5) * 12 + Math.sin(i * 0.2) * 5
			});
		}
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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Box Plot</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic BoxY Chart (Vertical) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic BoxY Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && mpgData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							grid
							x={{ type: 'band', label: 'Vehicle Class' }}
							y={{ grid: true, label: 'Highway MPG' }}
							inset={10}
						>
							<BoxY
								data={mpgData}
								x="class"
								y="hwy"
								fill="#6366F1"
								tickMinMax
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BoxY</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">tickMinMax</code></p>
				</div>
			</div>

			<!-- 2. BoxY with Custom Styling -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. BoxY with Custom Median Styling</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && mpgData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							grid
							x={{ type: 'band', label: 'Vehicle Class' }}
							y={{ grid: true, label: 'Highway MPG' }}
							inset={10}
						>
							<BoxY
								data={mpgData}
								x="class"
								y="hwy"
								fill="class"
								tickMinMax
								tickMedian={{
									stroke: 'var(--svelteplot-bg)',
									strokeWidth: 3
								}}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="class"</code> and custom <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">tickMedian</code> styling</p>
				</div>
			</div>

			<!-- 3. BoxY with Sales Data -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. BoxY - Sales Performance</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && salesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							grid
							x={{ type: 'band', label: 'Department' }}
							y={{ grid: true, label: 'Sales ($)' }}
							inset={10}
						>
							<BoxY
								data={salesData}
								x="department"
								y="sales"
								fill="#10B981"
								tickMinMax
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BoxY</code> for sales distribution analysis</p>
				</div>
			</div>

			<!-- 4. Basic BoxX Chart (Horizontal) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Basic BoxX Chart</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && olympiansData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							inset={10}
							x={{
								axis: 'both',
								grid: true,
								type: 'log',
								label: 'Weight (kg)'
							}}
							y={{ label: 'Sport' }}
						>
							<BoxX
								data={olympiansData}
								x="weight"
								y="sport"
								bar={{ fillOpacity: 0.5 }}
								tickMinMax
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BoxX</code> component with log scale</p>
				</div>
			</div>

			<!-- 5. BoxX with Test Scores -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. BoxX - Test Scores</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && testScoresData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							inset={10}
							x={{
								axis: 'both',
								grid: true,
								label: 'Score'
							}}
							y={{ label: 'Subject' }}
						>
							<BoxX
								data={testScoresData}
								x="score"
								y="subject"
								fill="#F59E0B"
								bar={{ fillOpacity: 0.6 }}
								tickMinMax
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BoxX</code> for score distribution</p>
				</div>
			</div>

			<!-- 6. BoxX with Product Prices -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. BoxX - Product Prices</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && productPricesData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={120}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							inset={10}
							x={{
								axis: 'both',
								grid: true,
								label: 'Price ($)'
							}}
							y={{ label: 'Category' }}
						>
							<BoxX
								data={productPricesData}
								x="price"
								y="category"
								fill="#8B5CF6"
								bar={{ fillOpacity: 0.5 }}
								tickMinMax
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">BoxX</code> for price distribution analysis</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Box Plots</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various box plot styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">BoxY</code> and <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">BoxX</code> components.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/box" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Box Plot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

