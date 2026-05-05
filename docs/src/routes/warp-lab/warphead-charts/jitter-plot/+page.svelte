<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Plot, Dot } from 'svelteplot';

	// Dense sample data similar to penguins dataset for faceted beeswarm
	// Adelie: 2800-4800g (Females: 2800-4000g, Males: 3200-4800g)
	// Chinstrap: 2800-4800g (Females: 2800-4200g, Males: 3200-4800g)
	// Gentoo: 3800-6300g (Females: 3800-5200g, Males: 4500-6300g)
	// Creating a very dense dataset with many overlapping values to show jittering effect
	function generateDensePenguinData(): Array<{ species: string; body_mass_g: number; sex: string }> {
		const data: Array<{ species: string; body_mass_g: number; sex: string }> = [];
		
		// Helper to add multiple entries with slight variations
		function addDenseEntries(species: string, sex: string, baseMasses: number[], countPerMass: number) {
			for (const baseMass of baseMasses) {
				for (let i = 0; i < countPerMass; i++) {
					// Add slight variations (±20g) to create density
					const variation = (i % 5) * 10 - 20;
					data.push({ species, body_mass_g: baseMass + variation, sex });
				}
			}
		}
		
		// Adelie - Female (2800-4000g) - very dense
		addDenseEntries('Adelie', 'Female', [2850, 2900, 2950, 3000, 3050, 3100, 3150, 3200, 3250, 3300, 3350, 3400, 3450, 3500, 3550, 3600, 3650, 3700, 3750, 3800, 3850, 3900, 3950, 4000], 8);
		
		// Adelie - Male (3200-4800g) - very dense
		addDenseEntries('Adelie', 'Male', [3200, 3300, 3400, 3500, 3600, 3700, 3800, 3900, 4000, 4100, 4200, 4300, 4400, 4500, 4600, 4700, 4800], 8);
		
		// Chinstrap - Female (2800-4200g) - very dense
		addDenseEntries('Chinstrap', 'Female', [2800, 2900, 3000, 3100, 3200, 3300, 3400, 3500, 3600, 3700, 3800, 3900, 4000, 4100, 4200], 8);
		
		// Chinstrap - Male (3200-4800g) - very dense
		addDenseEntries('Chinstrap', 'Male', [3200, 3300, 3400, 3500, 3600, 3700, 3800, 3900, 4000, 4100, 4200, 4300, 4400, 4500, 4600, 4700, 4800], 8);
		
		// Gentoo - Female (3800-5200g) - very dense
		addDenseEntries('Gentoo', 'Female', [3800, 3900, 4000, 4100, 4200, 4300, 4400, 4500, 4600, 4700, 4800, 4900, 5000, 5100, 5200], 9);
		
		// Gentoo - Male (4500-6300g) - very dense
		addDenseEntries('Gentoo', 'Male', [4500, 4600, 4700, 4800, 4900, 5000, 5100, 5200, 5300, 5400, 5500, 5600, 5700, 5800, 5900, 6000, 6100, 6200, 6300], 9);
		
		return data;
	}
	
	const penguinsData = generateDensePenguinData();

	// Basic jitter plot data (single category) - 500 points
	const basicJitterData = Array.from({ length: 500 }, () => ({
		category: 'Group A',
		value: 20 + Math.random() * 20 + Math.sin(Math.random() * Math.PI * 2) * 5
	}));

	// Multi-category jitter plot data - 1000 points
	const treatments = ['Treatment A', 'Treatment B', 'Treatment C', 'Treatment D'];
	const groups = ['Control', 'Treatment'];
	const multiCategoryJitterData: Array<{ category: string; value: number; group: string }> = [];
	for (let i = 0; i < 1000; i++) {
		const treatment = treatments[i % treatments.length];
		const group = groups[i % 2];
		const baseValue = group === 'Control' ? 15 : 25;
		multiCategoryJitterData.push({
			category: treatment,
			value: baseValue + Math.random() * 15 + Math.sin(i * 0.1) * 5,
			group
		});
	}

	// Time series jitter plot data - 1200 points (12 months x 100 samples)
	const monthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
	const types = ['Type A', 'Type B', 'Type C'];
	const timeSeriesJitterData: Array<{ month: string; value: number; type: string }> = [];
	for (let m = 0; m < 12; m++) {
		for (let i = 0; i < 100; i++) {
			types.forEach((type, tIdx) => {
				const baseValue = 40 + (m * 3) + (tIdx * 5);
				timeSeriesJitterData.push({
					month: monthNames[m],
					value: baseValue + Math.random() * 10 + Math.sin(i * 0.2) * 3,
					type
				});
			});
		}
	}

	// Colored jitter plot data - 800 points
	const regions = ['Region 1', 'Region 2', 'Region 3', 'Region 4', 'Region 5'];
	const colors = ['#6366F1', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6'];
	const coloredJitterData: Array<{ category: string; value: number; color: string }> = [];
	for (let i = 0; i < 800; i++) {
		const region = regions[i % regions.length];
		const color = colors[i % colors.length];
		coloredJitterData.push({
			category: region,
			value: 90 + (i % regions.length) * 20 + Math.random() * 20 + Math.sin(i * 0.1) * 10,
			color
		});
	}

	// Simple grouped jitter plot - 600 points
	const groupsList = ['A', 'B', 'C', 'D', 'E', 'F'];
	const groupedJitterData: Array<{ group: string; value: number }> = [];
	for (let i = 0; i < 600; i++) {
		const group = groupsList[i % groupsList.length];
		const baseValue = 15 + (i % groupsList.length) * 10;
		groupedJitterData.push({
			group,
			value: baseValue + Math.random() * 8 + Math.sin(i * 0.15) * 4
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
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Jitter Plot</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Faceted Beeswarm (from SveltePlot example) -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Faceted Beeswarm</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && penguinsData.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							color={{ legend: true }}
							fx={{ axis: 'bottom' }}
							r={{ zero: false }}
							y={{ grid: true, label: 'Body Mass (g)' }}
						>
							<Dot
								data={penguinsData}
								fx="species"
								x={0}
								y="body_mass_g"
								fill="sex"
								dodgeX="middle"
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fx</code> faceting and <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">dodgeX="middle"</code></p>
				</div>
			</div>

			<!-- 2. Basic Jitter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Basic Jitter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && basicJitterData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Category' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot
								data={basicJitterData}
								x="category"
								y="value"
								fill="#6366F1"
								dodgeX="middle"
								r={4}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Dot</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">dodgeX="middle"</code> for jittering</p>
				</div>
			</div>

			<!-- 3. Multi-Category Jitter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Multi-Category Jitter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && multiCategoryJitterData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							color={{ legend: true }}
							x={{ label: 'Treatment' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot
								data={multiCategoryJitterData}
								x="category"
								y="value"
								fill="group"
								dodgeX="middle"
								r={4}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="group"</code> to color by group with jittering</p>
				</div>
			</div>

			<!-- 4. Time Series Jitter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Time Series Jitter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && timeSeriesJitterData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							color={{ legend: true }}
							x={{ label: 'Month' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot
								data={timeSeriesJitterData}
								x="month"
								y="value"
								fill="type"
								dodgeX="middle"
								r={4}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Time series data with jittering to show distribution over time</p>
				</div>
			</div>

			<!-- 5. Colored Jitter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Colored Jitter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && coloredJitterData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Region' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot
								data={coloredJitterData}
								x="category"
								y="value"
								fill="color"
								dodgeX="middle"
								r={4}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">fill="color"</code> for dynamic colors with jittering</p>
				</div>
			</div>

			<!-- 6. Grouped Jitter Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Grouped Jitter Plot</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && groupedJitterData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Group' }}
							y={{ grid: true, label: 'Value' }}
						>
							<Dot
								data={groupedJitterData}
								x="group"
								y="value"
								fill="#8B5CF6"
								dodgeX="middle"
								r={5}
							/>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Simple grouped jitter plot showing distribution across groups</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Jitter Plots</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various jitter plot styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Dot</code> component with <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">dodgeX</code> for horizontal jittering and <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">fx</code> for faceting.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/examples/dot/dodge-faceted" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Jitter Plot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

