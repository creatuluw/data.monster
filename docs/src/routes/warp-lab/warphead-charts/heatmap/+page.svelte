<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Grid3x3 } from 'lucide-svelte';
	import { Plot, Cell, Text } from 'svelteplot';

	// Sample data: Simpsons episodes with season/episode and IMDB ratings - 800 episodes (40 seasons x 20 episodes)
	const simpsonsData: Array<{ episode: number; season: number; imdb_rating: number; title: string }> = [];
	for (let season = 1; season <= 40; season++) {
		for (let episode = 1; episode <= 20; episode++) {
			const baseRating = 8.0 + Math.sin(season * 0.2) * 1.0 + Math.sin(episode * 0.3) * 0.5;
			simpsonsData.push({
				episode,
				season,
				imdb_rating: Math.max(6.0, Math.min(10.0, baseRating + (Math.random() - 0.5) * 0.8)),
				title: `Season ${season} Episode ${episode}`
			});
		}
	}

	// Sample data: Seattle weather for 3 years (1095 days) - expanded calendar heatmap
	const seattleData: Array<{ date: Date; temp_max: number; day: number; month: number }> = [];
	const startDate = new Date('2015-01-01');
	for (let i = 0; i < 1095; i++) {
		const date = new Date(startDate);
		date.setDate(date.getDate() + i);
		const dayOfMonth = date.getDate();
		const month = date.getMonth();
		// Seasonal temperature variation with daily noise
		const seasonalBase = 10 + Math.sin((i / 365.25) * 2 * Math.PI - Math.PI / 2) * 12;
		const temp_max = seasonalBase + (Math.random() - 0.5) * 8 + Math.sin(i * 0.1) * 2;
		seattleData.push({ date, temp_max: Math.max(0, Math.min(35, temp_max)), day: dayOfMonth, month });
	}

	// Format function for month labels
	function formatMonth(locale: string, format: string) {
		return (d: number) => {
			const date = new Date(2015, d, 1);
			return date.toLocaleDateString(locale, { month: format as any });
		};
	}

	// Simple grid data for basic heatmap - 30x30 grid (900 cells)
	const rows = Array.from({ length: 30 }, (_, i) => String.fromCharCode(65 + (i % 26)) + (i >= 26 ? Math.floor(i / 26) : ''));
	const cols = Array.from({ length: 30 }, (_, i) => String.fromCharCode(88 + (i % 3)) + (i >= 3 ? Math.floor(i / 3) : ''));
	const gridData: Array<{ row: string; col: string; value: number }> = [];
	rows.forEach((row, rIdx) => {
		cols.forEach((col, cIdx) => {
			gridData.push({
				row,
				col,
				value: 70 + Math.sin(rIdx * 0.2) * 20 + Math.cos(cIdx * 0.3) * 15 + (Math.random() - 0.5) * 10
			});
		});
	});

	// Sales performance data by region and quarter - 20 regions x 20 quarters (400 cells)
	const regions = ['North', 'South', 'East', 'West', 'Central', 'Northeast', 'Northwest', 'Southeast', 'Southwest', 'Midwest', 'Pacific', 'Atlantic', 'Mountain', 'Plains', 'Coastal', 'Inland', 'Urban', 'Rural', 'Suburban', 'Metro'];
	const salesData: Array<{ region: string; quarter: string; sales: number }> = [];
	for (let year = 2015; year <= 2019; year++) {
		for (let q = 1; q <= 4; q++) {
			regions.forEach((region, rIdx) => {
				salesData.push({
					region,
					quarter: `Q${q} ${year}`,
					sales: 100000 + (rIdx * 5000) + ((year - 2015) * 10000) + (q * 2000) + Math.sin(rIdx * 0.5) * 20000 + (Math.random() - 0.5) * 10000
				});
			});
		}
	}

	// Activity data by hour and day - 24 hours x 7 days (168 cells)
	const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
	const activityData: Array<{ hour: number; day: string; activity: number }> = [];
	days.forEach((day, dIdx) => {
		for (let hour = 0; hour < 24; hour++) {
			const baseActivity = hour >= 6 && hour <= 18 ? 50 + (hour - 6) * 3 : 10 + Math.abs(hour - 12) * 2;
			const dayMultiplier = dIdx < 5 ? 1.2 : 0.8; // Weekdays higher
			activityData.push({
				hour,
				day,
				activity: Math.max(0, Math.min(100, baseActivity * dayMultiplier + (Math.random() - 0.5) * 15))
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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Heatmap</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Simpsons Episodes Heatmap with Text Labels -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Episode Ratings Heatmap</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && simpsonsData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							grid
							aspectRatio={1}
							x={{ type: 'band', axis: 'top' }}
							y={{ type: 'band' }}
							color={{ type: 'quantile', scheme: 'PiYG' }}
						>
							<Cell
								data={simpsonsData}
								x="episode"
								y="season"
								fill="imdb_rating"
								inset={0.5}
							/>
							<Text
								data={simpsonsData}
								y="season"
								x="episode"
								fontSize={9}
								fill="black"
								text={(d) => d.imdb_rating?.toFixed(1)}
								title={(d) => d.title}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> and <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Text</code> components with quantile color scheme</p>
				</div>
			</div>

			<!-- 2. Calendar Heatmap (Seattle Weather) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Calendar Heatmap</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && seattleData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							padding={0}
							aspectRatio={1}
							y={{
								ticks: [0, 1, 2],
								tickFormat: formatMonth('en', 'short')
							}}
						>
							<Cell
								data={seattleData}
								filter={(d) => d.date.getUTCFullYear() === 2015}
								x={(d) => d.date.getUTCDate()}
								y={(d) => d.date.getUTCMonth()}
								fill="temp_max"
								inset={0.5}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> component with date-based filtering</p>
				</div>
			</div>

			<!-- 3. Basic Grid Heatmap -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Basic Grid Heatmap</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && gridData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'band', label: 'Column' }}
							y={{ type: 'band', label: 'Row' }}
							color={{ scheme: 'Blues' }}
						>
							<Cell
								data={gridData}
								x="col"
								y="row"
								fill="value"
								inset={0.5}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> component with band scales</p>
				</div>
			</div>

			<!-- 4. Sales Performance Heatmap -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Sales Performance Heatmap</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && salesData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={80}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'band', label: 'Quarter' }}
							y={{ type: 'band', label: 'Region' }}
							color={{ scheme: 'YlOrRd' }}
						>
							<Cell
								data={salesData}
								x="quarter"
								y="region"
								fill="sales"
								inset={0.5}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> component with YlOrRd color scheme</p>
				</div>
			</div>

			<!-- 5. Activity Heatmap by Hour and Day -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Activity Heatmap</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && activityData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'band', label: 'Hour' }}
							y={{ type: 'band', label: 'Day' }}
							color={{ scheme: 'Viridis' }}
						>
							<Cell
								data={activityData}
								x="hour"
								y="day"
								fill="activity"
								inset={0.5}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> component with Viridis color scheme</p>
				</div>
			</div>

			<!-- 6. Heatmap with Custom Inset -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Heatmap with Custom Styling</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && gridData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ type: 'band', label: 'Column' }}
							y={{ type: 'band', label: 'Row' }}
							color={{ scheme: 'RdYlGn' }}
						>
							<Cell
								data={gridData}
								x="col"
								y="row"
								fill="value"
								inset={0.2}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Cell</code> component with custom inset value</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Heatmaps</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various heatmap styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Cell</code> component.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/cell" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Cell Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

