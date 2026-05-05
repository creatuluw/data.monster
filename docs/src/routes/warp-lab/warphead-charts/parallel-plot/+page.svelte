<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { TrendingUp } from 'lucide-svelte';
	import { Plot, Line, RuleY } from 'svelteplot';

	// Sample stateage data - age distribution by state
	// Age ranges: "10-19", "20-29", "30-39", "40-49", "50-59", "60-69", "70-79", "<10", "≥80"
	const ageRanges = ['10-19', '20-29', '30-39', '40-49', '50-59', '60-69', '70-79', '<10', '≥80'];
	
	// US States list
	const states = [
		'Alabama', 'Alaska', 'Arizona', 'Arkansas', 'California', 'Colorado', 'Connecticut', 'Delaware',
		'Florida', 'Georgia', 'Hawaii', 'Idaho', 'Illinois', 'Indiana', 'Iowa', 'Kansas', 'Kentucky',
		'Louisiana', 'Maine', 'Maryland', 'Massachusetts', 'Michigan', 'Minnesota', 'Mississippi',
		'Missouri', 'Montana', 'Nebraska', 'Nevada', 'New Hampshire', 'New Jersey', 'New Mexico',
		'New York', 'North Carolina', 'North Dakota', 'Ohio', 'Oklahoma', 'Oregon', 'Pennsylvania',
		'Rhode Island', 'South Carolina', 'South Dakota', 'Tennessee', 'Texas', 'Utah', 'Vermont',
		'Virginia', 'Washington', 'West Virginia', 'Wisconsin', 'Wyoming'
	];

	// Generate stateage data - each state has data for all age ranges
	const stateage: Array<{ age: string; pop_share: number; state: string }> = [];
	
	states.forEach(state => {
		ageRanges.forEach(age => {
			// Create realistic age distribution patterns
			let pop_share = 0;
			
			if (age === '<10') {
				pop_share = 0.10 + (Math.random() * 0.06); // 10-16%
			} else if (age === '10-19') {
				pop_share = 0.11 + (Math.random() * 0.05); // 11-16%
			} else if (age === '20-29') {
				// Peak age group with more variation
				pop_share = 0.12 + (Math.random() * 0.08); // 12-20%
			} else if (age === '30-39') {
				pop_share = 0.10 + (Math.random() * 0.04); // 10-14%
			} else if (age === '40-49') {
				pop_share = 0.11 + (Math.random() * 0.05); // 11-16%
			} else if (age === '50-59') {
				pop_share = 0.12 + (Math.random() * 0.04); // 12-16%
			} else if (age === '60-69') {
				pop_share = 0.08 + (Math.random() * 0.04); // 8-12%
			} else if (age === '70-79') {
				pop_share = 0.04 + (Math.random() * 0.04); // 4-8%
			} else if (age === '≥80') {
				pop_share = 0.02 + (Math.random() * 0.04); // 2-6%
			}
			
			// Add some state-specific variation
			const stateVariation = (Math.sin(state.length * 0.1) * 0.01);
			pop_share += stateVariation;
			
			// Ensure values are within reasonable bounds (0-20%)
			pop_share = Math.max(0.01, Math.min(0.20, pop_share));
			
			stateage.push({
				age,
				pop_share, // Keep as decimal (0.01-0.20) since percent: true expects 0-1 range
				state
			});
		});
	});

	let containerWidth1 = $state(0);
	let containerEl1 = $state<HTMLDivElement>();

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
	});
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Parallel Plot</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Parallel Plot -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Age Distribution by State</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && stateage.length > 0}
						<Plot
							width={containerWidth1}
							height={400}
							marginLeft={60}
							marginBottom={60}
							marginTop={20}
							marginRight={20}
							x={{ label: 'Age range (years)' }}
							y={{
								percent: true,
								grid: true,
								label: 'Population (%)'
							}}
						>
							<Line data={stateage} x="age" y="pop_share" z="state" />
							<RuleY data={[0]} />
						</Plot>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">Line</code> component with <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">z="state"</code> for grouping</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About SveltePlot Parallel Plots</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>SveltePlot</strong> is a Svelte-native visualization framework based on the layered grammar of graphics principles.
					These examples demonstrate parallel plot styles using the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">Line</code> component with the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">z</code> prop for grouping multiple series.
				</p>
				<p>
					📚 <a href="https://svelteplot.dev/marks/line" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View SveltePlot Line Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>

