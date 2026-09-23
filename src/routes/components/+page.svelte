<script lang="ts">
	import { loadAppComponents } from '$lib/app-components';
	import { drawerResize } from '$lib/components/drawer-resize';
	import { Copy, Check, X, Box, Shapes, BarChart3 } from 'lucide-svelte';

	const components = loadAppComponents();
	let open = $state<(typeof components)[number] | null>(null);
	let drawerShown = $state(false);
	let copied = $state(false);

	function iconFor(dir: string): typeof Box {
		if (dir === 'charts') return BarChart3;
		if (dir === 'ds') return Shapes;
		return Box;
	}

	function dirLabel(dir: string): string {
		return dir || 'components';
	}

	function show(c: (typeof components)[number]) {
		open = c;
		copied = false;
		requestAnimationFrame(() => (drawerShown = true));
	}

	function closeDrawer() {
		drawerShown = false;
		setTimeout(() => (open = null), 200);
	}

	async function copy() {
		if (!open) return;
		try {
			await navigator.clipboard.writeText(open.source);
			copied = true;
			setTimeout(() => (copied = false), 1500);
		} catch {
			// clipboard unavailable — the text is selectable anyway
		}
	}
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && open && closeDrawer()} />

<div class="page">
	<div class="section-header">
		<h1 class="section-title page-title">Components</h1>
	</div>

	<p class="section-subtitle">
		Every component the app's UI is built with, from <code>src/lib/components/</code>.
		Click a card to view its source.
	</p>

	<div class="grid">
		{#each components as c (c.path)}
			<button class="card" onclick={() => show(c)} title="Open source">
				<div class="card-icon"><Box size={18} /></div>
				<div class="card-body">
					<span class="card-title">{c.name}</span>
					<span class="card-desc">{c.lines} lines · {dirLabel(c.dir)}</span>
				</div>
			</button>
		{/each}
	</div>
</div>

{#if open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-overlay" class:drawer-overlay-visible={drawerShown} onclick={closeDrawer} onkeydown={() => {}}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<section class="drawer" class:drawer-open={drawerShown} use:drawerResize onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
			<div class="drawer-header">
				<div class="drawer-heading">
					<h2 class="drawer-title">{open.name}</h2>
					<span class="drawer-path">{open.path} · {open.lines} lines</span>
				</div>
				<div class="flex items-center gap-1">
					<button class="copy-btn" onclick={copy} title={copied ? 'Copied' : 'Copy source'}>
						{#if copied}<Check size={14} class="text-green-600" /> Copied{:else}<Copy size={14} /> Copy{/if}
					</button>
					<button class="close-btn" onclick={closeDrawer} title="Close" aria-label="Close">
						<X size={16} />
					</button>
				</div>
			</div>
			<div class="drawer-body">
				<pre class="source">{open.source}</pre>
			</div>
		</section>
	</div>
{/if}

<style>
	.page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
	}

	.section-title {
		margin: 0;
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-1) 0 var(--space-6) 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.section-subtitle code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
	}

	/* same card as /labs and /library */
	.card {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-align: left;
		color: var(--color-text);
		background: var(--color-surface);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
		box-sizing: border-box;
	}

	.card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.card-icon {
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

	.card-body {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
		line-height: var(--leading-snug);
	}

	.drawer-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0 0 0 / 0.3);
		z-index: 200;
		opacity: 0;
		transition: opacity var(--duration-base) ease;
		pointer-events: none;
	}

	.drawer-overlay-visible {
		opacity: 1;
		pointer-events: auto;
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 50vw;
		max-width: 100vw;
		background: var(--color-surface);
		border-left: 1px solid var(--color-border);
		z-index: 201;
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform var(--duration-base) var(--ease-out-expo);
		box-shadow: var(--shadow-lg);
	}

	.drawer-open {
		transform: translateX(0);
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-5);
		border-bottom: 1px solid var(--color-border);
	}

	.drawer-heading {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
	}

	.drawer-title {
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.drawer-path {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		border-radius: var(--radius-md, 6px);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-accent-dark, #1e3a5f);
		background: var(--color-accent-muted, #eef4fa);
	}

	.copy-btn:hover {
		background: var(--color-accent, #d7e6f5);
	}

	.close-btn {
		display: inline-flex;
		padding: 6px;
		border-radius: var(--radius-md, 6px);
		color: var(--color-text-muted, #71717a);
	}

	.close-btn:hover {
		background: var(--color-accent-muted, #eef4fa);
		color: var(--color-text);
	}

	.drawer-body {
		flex: 1;
		overflow: auto;
		padding: var(--space-5);
	}

	.source {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		white-space: pre;
		margin: 0;
	}
</style>
