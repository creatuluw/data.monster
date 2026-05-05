<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Plot, Dot, GridY } from 'svelteplot';

	// Sample data for different dot plot types
	// Basic dot data - 100 categories
	const basicDotData = Array.from({ length: 100 }, (_, i) => ({
		category: String.fromCharCode(65 + (i % 26)) + (i >= 26 ? Math.floor(i / 26) : ''),
		value: 20 + Math.random() * 60 + Math.sin(i * 0.2) * 20
	}));

	// Language data for log scale example - 200 languages
	const languageNames = ['Mandarin Chinese', 'Spanish', 'English', 'Hindi', 'Arabic', 'Portuguese', 'Bengali', 'Russian', 'Japanese', 'Punjabi', 'German', 'Javanese', 'Wu Chinese', 'Malay', 'Telugu', 'Vietnamese', 'Turkish', 'Italian', 'Tamil', 'Urdu'];
	const languagesData: Array<{ Language: string; 'Total speakers': number }> = [];
	for (let i = 0; i < 200; i++) {
		const baseSpeakers = 10000000 * Math.pow(10, Math.random() * 2);
		languagesData.push({
			Language: languageNames[i % languageNames.length] + (i >= languageNames.length ? ` ${Math.floor(i / languageNames.length) + 1}` : ''),
			'Total speakers': Math.max(1000000, Math.min(2000000000, baseSpeakers))
		});
	}

	// Multi-series dot plot data - 120 quarters (30 years)
	const multiSeriesData = Array.from({ length: 120 }, (_, i) => ({
		category: `Q${(i % 4) + 1} ${2020 + Math.floor(i / 4)}`,
		sales: 100 + (i * 2) + Math.sin(i * 0.1) * 30 + (Math.random() - 0.5) * 15,
		marketing: 70 + (i * 1.5) + Math.sin(i * 0.12) * 20 + (Math.random() - 0.5) * 10,
		support: 40 + (i * 1) + Math.sin(i * 0.08) * 15 + (Math.random() - 0.5) * 8
	}));

	// Time series dot plot data - 365 days
	const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
	const timeSeriesData = Array.from({ length: 365 }, (_, i) => {
		const baseTemp = 20 + Math.sin((i / 365) * 2 * Math.PI - Math.PI / 2) * 12;
		const monthIndex = Math.floor(i / 30.44);
		return {
			month: i + 1,
			temperature: baseTemp + (Math.random() - 0.5) * 5,
			label: monthNames[monthIndex % 12]
		};
	});

	// Data with variable sizes - 800 points
	const variableSizeData = Array.from({ length: 800 }, (_, i) => ({
		x: (i % 40) * 2.5,
		y: Math.floor(i / 40) * 5,
		size: 3 + Math.random() * 15 + Math.sin(i * 0.1) * 5
	}));

	// Colored dot plot data - 150 products
	const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#84CC16'];
	const coloredDotData = Array.from({ length: 150 }, (_, i) => ({
		category: `Product ${String.fromCharCode(65 + (i % 26))}${i >= 26 ? Math.floor(i / 26) : ''}-${String(i + 1).padStart(3, '0')}`,
		value: 80 + Math.random() * 150 + Math.sin(i * 0.15) * 40,
		color: colors[i % colors.length]
	}));

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
		const item = timeSeriesData.find(item => item.month === d);
		return item?.label || String(d);
	}
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Dot Plot</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Dot Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && basicDotData.length > 0}
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
							<Dot data={basicDotData} x="category" y="value" fill="#6366F1" r={8} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> component with basic configuration</p>
				</div>
			</div>

			<!-- 2. Log Scale Dot Plot (from SveltePlot example) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Log Scale Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && languagesData.length > 0}
						<Plot
							frame
							inset={20}
							width={containerWidth2}
							height={400}
							x={{
								type: 'log',
								axis: 'both',
								label: 'NUMBER OF SPEAKERS',
								labelAnchor: 'center'
							}}
							y={{ type: 'point', label: '' }}
						>
							<GridY strokeDasharray="1,3" strokeOpacity={0.5} />
							<Dot
								data={languagesData.filter(
									(d) => d['Total speakers'] >= 70e6
								)}
								fill="currentColor"
								sort={{ channel: '-x' }}
								y="Language"
								x="Total speakers"
								r={6}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">log</code> scale on x-axis and <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">point</code> scale on y-axis with sorting</p>
				</div>
			</div>

			<!-- 3. Multi-Series Dot Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Multi-Series Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && multiSeriesData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Quarter' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot data={multiSeriesData} x="category" y="sales" fill="#6366F1" r={7} />
							<Dot data={multiSeriesData} x="category" y="marketing" fill="#10B981" r={7} />
							<Dot data={multiSeriesData} x="category" y="support" fill="#F59E0B" r={7} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> components for different series</p>
				</div>
			</div>

			<!-- 4. Time Series Dot Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Time Series Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && timeSeriesData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Month', tickFormat: formatMonth }}
							y={{ grid: true, label: 'Temperature (°C)' }}
						>
							<Dot data={timeSeriesData} x="month" y="temperature" fill="#EF4444" r={8} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Time series data with custom month labels using <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">tickFormat</code></p>
				</div>
			</div>

			<!-- 5. Variable Size Dot Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Variable Size Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && variableSizeData.length > 0}
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
							<Dot data={variableSizeData} x="x" y="y" fill="#8B5CF6" r="size" />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">r="size"</code> for variable dot sizes</p>
				</div>
			</div>

			<!-- 6. Multi-Color Dot Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Multi-Color Dot Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && coloredDotData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Product' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot data={coloredDotData} x="category" y="value" fill="color" r={9} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="color"</code> for dynamic colors per dot</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Dot Plots</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various dot plot styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Dot</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/dot" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Dot Plot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

