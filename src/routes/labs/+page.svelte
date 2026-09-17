<script lang="ts">
	import {
	Grid3x3,
	BarChart3,
	ChartLine,
	ChartArea,
	ChartScatter,
	Workflow,
	Waypoints,
	Boxes,
	Brain,
	ChartGantt,
	ChartPie,
	CircleDashed,
	CircleDotDashed,
	Compass,
	Fan,
	GitCommitHorizontal,
	GitFork,
	Grid2x2,
	History,
	IterationCw,
	Layers,
	LayoutGrid,
	ListTree,
	Orbit,
	Radar,
	Route,
	Share2,
	Shell,
	Sun,
	Table2,
	Target,
	Trophy
} from 'lucide-svelte';
</script>

<svelte:head>
	<title>Labs — Data Monster</title>
</svelte:head>

<div class="labs-page">
	<div class="section-header">
		<h1 class="section-title">Chart types</h1>
	</div>

	<p class="section-subtitle">
		One card per chart type we are building here, each with its own playground. Heatmap first.
	</p>

	<div class="labs-grid">
		<a href="/labs/bar-chart" class="lab-card">
			<div class="lab-icon">
				<BarChart3 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Barchart</span>
				<span class="lab-desc">svelteplot BarY — hover tooltip, click-to-select, mono chart labels, empty-data guard. Top 5 regions + Other.</span>
			</div>
		</a>

		<a href="/labs/line-chart" class="lab-card">
			<div class="lab-icon">
				<ChartLine size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Line chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>

		<a href="/labs/area-chart" class="lab-card">
			<div class="lab-icon">
				<ChartArea size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Area chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>

		<a href="/labs/scatter-chart" class="lab-card">
			<div class="lab-icon">
				<ChartScatter size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Scatter chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>

		<a href="/labs/heatmap" class="lab-card">
			<div class="lab-icon">
				<Grid3x3 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Heatmap</span>
				<span class="lab-desc">Threshold-colored grid with in-cell labels, hover tooltips, click-to-select and no-data cells. Built on svelteplot.</span>
			</div>
		</a>

		<a href="/labs/parallel-coordinates" class="lab-card">
			<div class="lab-icon">
				<Workflow size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Parallel coordinates</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>

		<a href="/labs/sankey" class="lab-card">
			<div class="lab-icon">
				<Waypoints size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Sankey</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/pie-chart" class="lab-card">
			<div class="lab-icon">
				<ChartPie size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Pie chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/venn-diagram" class="lab-card">
			<div class="lab-icon">
				<Orbit size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Venn diagram</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/concentric-diagram" class="lab-card">
			<div class="lab-icon">
				<CircleDotDashed size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Concentric diagram</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/circular-chart" class="lab-card">
			<div class="lab-icon">
				<IterationCw size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Circular chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/bubble-chart" class="lab-card">
			<div class="lab-icon">
				<CircleDashed size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Bubble chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/bubble-race-chart" class="lab-card">
			<div class="lab-icon">
				<Trophy size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Bubble race chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/sunburst-chart" class="lab-card">
			<div class="lab-icon">
				<Sun size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Sunburst chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/fan-chart" class="lab-card">
			<div class="lab-icon">
				<Fan size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Fan chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/windrose-chart" class="lab-card">
			<div class="lab-icon">
				<Compass size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Windrose chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/tape-diagram" class="lab-card">
			<div class="lab-icon">
				<Layers size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Tape diagram</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/gantt-diagram" class="lab-card">
			<div class="lab-icon">
				<ChartGantt size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Gantt diagram</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/tree-map" class="lab-card">
			<div class="lab-icon">
				<LayoutGrid size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Tree map</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/grid" class="lab-card">
			<div class="lab-icon">
				<Grid2x2 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Grid</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/periodic-table" class="lab-card">
			<div class="lab-icon">
				<Table2 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Periodic table</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/arc-diagram" class="lab-card">
			<div class="lab-icon">
				<GitCommitHorizontal size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Arc diagram</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/chord-chart" class="lab-card">
			<div class="lab-icon">
				<Share2 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Chord chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/radar-chart" class="lab-card">
			<div class="lab-icon">
				<Radar size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Radar chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/polar-grid" class="lab-card">
			<div class="lab-icon">
				<Target size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Polar grid</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/spiral-graph" class="lab-card">
			<div class="lab-icon">
				<Shell size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Spiral graph</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/timeline" class="lab-card">
			<div class="lab-icon">
				<History size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Timeline</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/flow-chart" class="lab-card">
			<div class="lab-icon">
				<Route size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Flow chart</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/binary-tree" class="lab-card">
			<div class="lab-icon">
				<GitFork size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Binary tree</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/mind-map" class="lab-card">
			<div class="lab-icon">
				<Brain size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Mind map</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/decision-tree" class="lab-card">
			<div class="lab-icon">
				<ListTree size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Decision tree</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>
		<a href="/labs/block-scheme" class="lab-card">
			<div class="lab-icon">
				<Boxes size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Block scheme</span>
				<span class="lab-desc">Placeholder — coming soon.</span>
			</div>
		</a>

	</div>
</div>

<style>
	.labs-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent-muted);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		white-space: nowrap;
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-3) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.labs-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-3);
		margin-top: var(--space-6);
	}

	.lab-card {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-decoration: none;
		color: var(--color-text);
		background: var(--color-surface);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
		box-sizing: border-box;
	}

	.lab-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.lab-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: var(--radius-sm);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.lab-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.lab-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
	}

	.lab-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: var(--leading-snug);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
