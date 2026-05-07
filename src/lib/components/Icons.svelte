<script lang="ts">
	import {
		Home, BarChart3, Database, Search, Filter, TrendingUp, Bell, Settings,
		User, Download, CheckCircle, AlertTriangle, XCircle, Info, Clock,
		PlusCircle, PenLine, Trash2, RefreshCw, Share2, PieChart,
		ArrowDownNarrowWide, Upload, Bookmark, Link, Lock, Copy, ShieldCheck,
		Loader, ArrowLeft, Table, FileText, CloudUpload, Server, Code, Calendar,
		Tag, Eye, Maximize, Users, Globe, Map, Cpu, Play, Pause, Square,
		FastForward, Share, LayoutGrid
	} from 'lucide-svelte';

	type IconEntry = { name: string; component: any };

	const row1: IconEntry[] = [
		{ name: 'Home', component: Home },
		{ name: 'BarChart3', component: BarChart3 },
		{ name: 'Database', component: Database },
		{ name: 'Search', component: Search },
		{ name: 'Filter', component: Filter },
		{ name: 'TrendingUp', component: TrendingUp },
		{ name: 'Bell', component: Bell },
		{ name: 'Settings', component: Settings },
		{ name: 'User', component: User },
		{ name: 'Download', component: Download },
	];

	const row2: IconEntry[] = [
		{ name: 'CheckCircle', component: CheckCircle },
		{ name: 'AlertTriangle', component: AlertTriangle },
		{ name: 'XCircle', component: XCircle },
		{ name: 'Info', component: Info },
		{ name: 'Clock', component: Clock },
		{ name: 'PlusCircle', component: PlusCircle },
		{ name: 'PenLine', component: PenLine },
		{ name: 'Trash2', component: Trash2 },
		{ name: 'RefreshCw', component: RefreshCw },
		{ name: 'Share2', component: Share2 },
	];

	const row3: IconEntry[] = [
		{ name: 'PieChart', component: PieChart },
		{ name: 'ArrowDownNarrowWide', component: ArrowDownNarrowWide },
		{ name: 'Upload', component: Upload },
		{ name: 'Bookmark', component: Bookmark },
		{ name: 'Link', component: Link },
		{ name: 'Lock', component: Lock },
		{ name: 'Copy', component: Copy },
		{ name: 'ShieldCheck', component: ShieldCheck },
		{ name: 'Loader', component: Loader },
		{ name: 'ArrowLeft', component: ArrowLeft },
	];

	const row4: IconEntry[] = [
		{ name: 'Table', component: Table },
		{ name: 'FileText', component: FileText },
		{ name: 'CloudUpload', component: CloudUpload },
		{ name: 'Server', component: Server },
		{ name: 'Code', component: Code },
		{ name: 'Calendar', component: Calendar },
		{ name: 'Tag', component: Tag },
		{ name: 'Eye', component: Eye },
		{ name: 'Maximize', component: Maximize },
		{ name: 'Maximize (fullscreen)', component: Maximize },
	];

	const row5: IconEntry[] = [
		{ name: 'Users', component: Users },
		{ name: 'Globe', component: Globe },
		{ name: 'Map', component: Map },
		{ name: 'Cpu', component: Cpu },
		{ name: 'Play', component: Play },
		{ name: 'Pause', component: Pause },
		{ name: 'Square (stop)', component: Square },
		{ name: 'FastForward', component: FastForward },
		{ name: 'Share (export)', component: Share },
		{ name: 'LayoutGrid', component: LayoutGrid },
	];
</script>

<section class="section" id="icons" aria-labelledby="icons-title">
	<div class="section-header entrance">
		<span class="section-number">07</span>
		<h2 class="section-title" id="icons-title">Iconography</h2>
	</div>

	<div class="icon-intro entrance entrance-d1">
		<span class="icon-intro-label">Required Library</span>
		<p class="icon-intro-desc">
			All icons use <strong>Lucide</strong> — a consistent, open-source icon set
			with clean stroke-based designs. No custom SVGs, no emoji, no icon fonts.
			This ensures consistent optical weight, pixel alignment, and a single source
			of truth across the entire interface.
		</p>
		<div class="icon-install">
			<code class="code code-block">npm install lucide-svelte</code>
		</div>
		<div class="icon-usage">
			<code class="code code-block">&lt;BarChart3 size={20} /&gt;</code>
		</div>
	</div>

	<div class="icon-showcase entrance entrance-d2">
		<div class="icon-row">
			{#each row1 as item}
				<div class="icon-cell">
					<svelte:component this={item.component} size={32} />
					<span class="icon-name">{item.name}</span>
				</div>
			{/each}
		</div>
		<div class="icon-row">
			{#each row2 as item}
				<div class="icon-cell">
					<svelte:component this={item.component} size={32} />
					<span class="icon-name">{item.name}</span>
				</div>
			{/each}
		</div>
		<div class="icon-row">
			{#each row3 as item}
				<div class="icon-cell">
					<svelte:component this={item.component} size={32} />
					<span class="icon-name">{item.name}</span>
				</div>
			{/each}
		</div>
		<div class="icon-row">
			{#each row4 as item}
				<div class="icon-cell">
					<svelte:component this={item.component} size={32} />
					<span class="icon-name">{item.name}</span>
				</div>
			{/each}
		</div>
		<div class="icon-row">
			{#each row5 as item}
				<div class="icon-cell">
					<svelte:component this={item.component} size={32} />
					<span class="icon-name">{item.name}</span>
				</div>
			{/each}
		</div>
	</div>

	<div class="icon-rules entrance entrance-d3">
		<span class="icon-category-label">Usage Rules</span>
		<div class="rules-grid">
			<div class="rule">
				<span class="rule-icon"><CheckCircle size={16} /></span>
				<span class="rule-text">Use Lucide's default stroke style for consistency</span>
			</div>
			<div class="rule">
				<span class="rule-icon"><CheckCircle size={16} /></span>
				<span class="rule-text">Default size: 20px. Small: 16px. Large: 24px.</span>
			</div>
			<div class="rule">
				<span class="rule-icon"><XCircle size={16} /></span>
				<span class="rule-text">Never mix icon sets. Lucide only.</span>
			</div>
			<div class="rule">
				<span class="rule-icon"><CheckCircle size={16} /></span>
				<span class="rule-text">Icons supplement labels. Never replace them.</span>
			</div>
		</div>
	</div>
</section>

<style>
	.section {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-16) var(--gutter);
		position: relative;
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
		margin-bottom: var(--space-12);
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: var(--space-1) var(--space-2);
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
		line-height: var(--leading-snug);
		color: var(--color-text);
	}

	.icon-intro {
		margin-bottom: var(--space-12);
	}

	.icon-intro-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-accent);
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-4);
	}

	.icon-intro-label::before {
		content: "\25A0";
		font-size: 7px;
	}

	.icon-intro-desc {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 60ch;
		margin-bottom: var(--space-4);
	}

	.code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		background: var(--color-surface-sunken);
		padding: 1px var(--space-2);
		border-radius: var(--radius-xs);
	}

	.code-block {
		display: block;
		padding: var(--space-2) var(--space-3);
	}

	.icon-install,
	.icon-usage {
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-xs);
		display: inline-block;
		margin-bottom: var(--space-2);
	}

	.icon-showcase {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		padding: var(--space-6);
		margin-bottom: var(--space-12);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.icon-row {
		display: grid;
		grid-template-columns: repeat(10, 1fr);
		gap: var(--space-1);
	}

	.icon-cell {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-4) var(--space-2);
		border-radius: var(--radius-xs);
		transition: background var(--duration-fast) ease;
		cursor: default;
	}

	.icon-cell:hover {
		background: var(--color-surface-sunken);
	}

	.icon-cell :global(svg) {
		color: var(--color-text);
	}

	.icon-name {
		font-family: var(--font-mono);
		font-size: 7px;
		color: var(--color-text-tertiary);
		text-align: center;
		letter-spacing: 0.02em;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 100%;
	}

	.icon-rules {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		padding: var(--space-8);
	}

	.icon-category-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		display: block;
		margin-bottom: var(--space-4);
	}

	.rules-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
		gap: var(--space-4);
	}

	.rule {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
	}

	.rule-icon :global(svg) {
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.rule-text {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		line-height: var(--leading-normal);
	}

	@media (max-width: 640px) {
		.icon-row {
			grid-template-columns: repeat(5, 1fr);
		}
	}
</style>
