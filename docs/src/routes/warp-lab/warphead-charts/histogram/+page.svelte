<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { BarChart3 } from 'lucide-svelte';
	import { Plot, RectY, CustomMark, binX, stackY } from 'svelteplot';

	// Sample data - olympians with weight data
	const olympians = Array.from({ length: 500 }, () => ({
		weight: 50 + Math.random() * 100 + Math.sin(Math.random() * Math.PI * 2) * 15
	}));

	// Additional sample datasets for different histogram examples
	// Age distribution data
	const ageData = Array.from({ length: 1000 }, () => ({
		age: 18 + Math.random() * 50 + Math.sin(Math.random() * Math.PI * 2) * 10
	}));

	// Height distribution data
	const heightData = Array.from({ length: 800 }, () => ({
		height: 150 + Math.random() * 50 + Math.cos(Math.random() * Math.PI * 2) * 10
	}));

	// Income distribution data
	const incomeData = Array.from({ length: 600 }, () => ({
		income: 20000 + Math.random() * 80000 + Math.sin(Math.random() * Math.PI * 2) * 20000
	}));

	// Temperature data
	const temperatureData = Array.from({ length: 365 }, () => ({
		temp: 15 + Math.random() * 20 + Math.sin((Math.random() * Math.PI * 2)) * 5
	}));

	// Score distribution data
	const scoreData = Array.from({ length: 1200 }, () => ({
		score: 40 + Math.random() * 40 + Math.cos(Math.random() * Math.PI * 2) * 8
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
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Histogram</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Histogram with Custom Path -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Histogram with Custom Path</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && olympians.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Weight' }}
							y={{ zero: true, grid: true, label: 'Count' }}
						>
							{@const binned = binX(
								{ data: olympians, x: 'weight' },
								{ y: 'count', interval: 3 }
							)}
							<RectY {...binned} opacity={0.2} />
							<CustomMark {...stackY(binned)}>
								{#snippet marks({ records })}
									<path
										d="M{records[0].x1},{records[0].y1} {records
											.map((r) => `V${r.y2}H${r.x2}`)
											.join(' ')}"
										stroke="currentColor"
										fill="none" />
								{/snippet}
							</CustomMark>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">binX</code>, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">RectY</code>, <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">stackY</code>, and <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">CustomMark</code> components</p>
				</div>
			</div>

			<!-- 2. Age Distribution Histogram -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Age Distribution Histogram</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && ageData.length > 0}
						<Plot
							width={containerWidth2}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Age' }}
							y={{ zero: true, grid: true, label: 'Frequency' }}
						>
							{@const binned = binX(
								{ data: ageData, x: 'age' },
								{ y: 'count', interval: 5 }
							)}
							<RectY {...binned} fill="#10B981" opacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">binX</code> with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">RectY</code> for frequency distribution</p>
				</div>
			</div>

			<!-- 3. Height Distribution Histogram -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Height Distribution Histogram</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && heightData.length > 0}
						<Plot
							width={containerWidth3}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Height (cm)' }}
							y={{ zero: true, grid: true, label: 'Count' }}
						>
							{@const binned = binX(
								{ data: heightData, x: 'height' },
								{ y: 'count', interval: 4 }
							)}
							<RectY {...binned} fill="#F59E0B" opacity={0.7} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Histogram with custom interval and opacity</p>
				</div>
			</div>

			<!-- 4. Income Distribution Histogram -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Income Distribution Histogram</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && incomeData.length > 0}
						<Plot
							width={containerWidth4}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Income ($)' }}
							y={{ zero: true, grid: true, label: 'Frequency' }}
						>
							{@const binned = binX(
								{ data: incomeData, x: 'income' },
								{ y: 'count', interval: 10000 }
							)}
							<RectY {...binned} fill="#8B5CF6" opacity={0.6} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Histogram with larger interval bins for income data</p>
				</div>
			</div>

			<!-- 5. Temperature Distribution Histogram -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Temperature Distribution Histogram</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && temperatureData.length > 0}
						<Plot
							width={containerWidth5}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Temperature (°C)' }}
							y={{ zero: true, grid: true, label: 'Count' }}
						>
							{@const binned = binX(
								{ data: temperatureData, x: 'temp' },
								{ y: 'count', interval: 2 }
							)}
							<RectY {...binned} fill="#EF4444" opacity={0.5} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Temperature distribution with smaller interval bins</p>
				</div>
			</div>

			<!-- 6. Score Distribution with Custom Path -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Score Distribution with Custom Path</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && scoreData.length > 0}
						<Plot
							width={containerWidth6}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Score' }}
							y={{ zero: true, grid: true, label: 'Frequency' }}
						>
							{@const binned = binX(
								{ data: scoreData, x: 'score' },
								{ y: 'count', interval: 2 }
							)}
							<RectY {...binned} fill="#6366F1" opacity={0.2} />
							<CustomMark {...stackY(binned)}>
								{#snippet marks({ records })}
									<path
										d="M{records[0].x1},{records[0].y1} {records
											.map((r) => `V${r.y2}H${r.x2}`)
											.join(' ')}"
										stroke="#6366F1"
										strokeWidth={2}
										fill="none" />
								{/snippet}
							</CustomMark>
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Combines <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">RectY</code> with custom path outline</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Histograms</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate various histogram styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">binX</code>, <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">RectY</code>, <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">stackY</code>, and <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">CustomMark</code> components.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>


