<script lang="ts">
	import { marked } from 'marked';
	import { loadPrompts, injectWorkspace } from '$lib/agent-prompts';
	import { app } from '$lib/stores/app.svelte';
	import { drawerResize } from '$lib/components/drawer-resize';
	import { Copy, Check, BookOpen, X, Rocket, Database, LayoutDashboard, Sigma, Eraser } from 'lucide-svelte';

	const prompts = loadPrompts();
	let open = $state<(typeof prompts)[number] | null>(null);
	let drawerShown = $state(false);
	let copied = $state(false);

	const ICONS: Record<string, typeof Rocket> = {
		onboarding: Rocket,
		'ingest-csv': Database,
		'dashboard-interview': LayoutDashboard,
		'add-measure': Sigma,
		explain: BookOpen,
		cleanup: Eraser
	};

	const GOALS: { key: string; label: string; blurb: string }[] = [
		{ key: 'data', label: 'Create data', blurb: 'Get data into the workspace and shape it.' },
		{ key: 'content', label: 'Create content', blurb: 'Build report pages and dashboards.' },
		{ key: 'insights', label: 'Get insights', blurb: 'Reusable analysis building blocks.' },
		{ key: 'actions', label: 'Take actions', blurb: 'Understand and keep the workspace healthy.' }
	];

	function showPrompt(p: (typeof prompts)[number]) {
		open = p;
		copied = false;
		requestAnimationFrame(() => (drawerShown = true));
	}

	function closeDrawer() {
		drawerShown = false;
		setTimeout(() => (open = null), 200);
	}

	function promptText(p: (typeof prompts)[number]): string {
		return injectWorkspace(p.body, app.workspacePath);
	}

	function render(body: string): string {
		return marked.parse(body, { async: false }) as string;
	}

	async function copy() {
		if (!open) return;
		try {
			await navigator.clipboard.writeText(promptText(open));
			copied = true;
			setTimeout(() => (copied = false), 1500);
		} catch {
			// clipboard unavailable — the text is selectable anyway
		}
	}
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && open && closeDrawer()} />

<div class="skills-page">
	<div class="section-header">
		<h1 class="section-title page-title">Skills</h1>
	</div>

	<p class="section-subtitle">
		One card per skill: copy its prompt into your coding agent (Claude Code, Cursor, Codex, …)
		and it builds content for this app by writing files in your workspace.
		{#if app.workspacePath}
			Your workspace path <code class="font-mono">{app.workspacePath}</code> is already filled into the prompts.
		{:else}
			No workspace open — the prompts contain a placeholder where you paste the folder path.
		{/if}
		The prompts teach the agent to read <code class="font-mono">README.md</code> and
		<code class="font-mono">dm/docs/</code> in your workspace first.
	</p>

	{#each GOALS as g (g.key)}
		<section>
			<h2 class="goal-title">{g.label}</h2>
			<p class="goal-blurb">{g.blurb}</p>
			<div class="skills-grid">
				{#each prompts.filter((p) => p.goal === g.key) as p (p.file)}
					{@const Icon = ICONS[p.file] ?? BookOpen}
					<button class="skill-card" onclick={() => showPrompt(p)} title="Open prompt">
						<div class="skill-icon">
							<Icon size={18} />
						</div>
						<div class="skill-info">
							<span class="skill-title">{p.title}</span>
							<span class="skill-desc">{p.description}</span>
							<div class="flex gap-1 flex-wrap">
								{#each p.tags as tag (tag)}
									<span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-100 text-zinc-500">{tag}</span>
								{/each}
							</div>
						</div>
					</button>
				{/each}
			</div>
		</section>
	{/each}
</div>

{#if open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-overlay" class:drawer-overlay-visible={drawerShown} onclick={closeDrawer} onkeydown={() => {}}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<section class="drawer" class:drawer-open={drawerShown} use:drawerResize onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
			<div class="drawer-header">
				<h2 class="drawer-title">{open.title}</h2>
				<div class="flex items-center gap-1">
					<button class="copy-btn" onclick={copy} title={copied ? 'Copied' : 'Copy prompt'}>
						{#if copied}<Check size={14} class="text-green-600" /> Copied{:else}<Copy size={14} /> Copy{/if}
					</button>
					<button class="close-btn" onclick={closeDrawer} title="Close" aria-label="Close">
						<X size={16} />
					</button>
				</div>
			</div>
			<div class="drawer-body">
				<p class="text-xs text-zinc-500">{open.description}</p>
				<div class="prose-chat text-xs">
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html render(promptText(open))}
				</div>
			</div>
		</section>
	</div>
{/if}

<style>
	.skills-page {
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
		margin: var(--space-3) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.goal-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		margin: var(--space-6) 0 0 0;
	}

	.goal-blurb {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin: 2px 0 0 0;
	}

	.skills-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-3);
		margin-top: var(--space-3);
	}

	.skill-card {
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

	.skill-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.skill-icon {
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

	.skill-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.skill-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
	}

	.skill-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: var(--leading-snug);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
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

	.drawer-title {
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
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
		overflow-y: auto;
		padding: var(--space-5);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
</style>
