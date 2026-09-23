<script lang="ts">
	import { marked } from 'marked';
	import { loadPrompts, injectWorkspace } from '$lib/agent-prompts';
	import { app } from '$lib/stores/app.svelte';
	import { drawerResize } from '$lib/components/drawer-resize';
	import { Copy, Check, BookOpen, X } from 'lucide-svelte';

	const prompts = loadPrompts();
	let open = $state<(typeof prompts)[number] | null>(null);
	let drawerShown = $state(false);
	let copied = $state(false);

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

<div class="page-shell" style="padding: var(--space-6);">
	<div class="mb-6">
		<h1 class="page-title">Skills</h1>
		<p class="text-sm text-zinc-500 mt-1">
			Pick a skill, copy its prompt into your coding agent (Claude Code, Cursor, Codex, …)
			and it builds content for this app by writing files in your workspace. The agent
			needs access to the workspace folder.
		</p>
		<p class="text-xs text-zinc-400 mt-2 inline-flex items-center gap-1">
			<BookOpen size={12} />
			The prompts teach the agent to read <code class="font-mono">README.md</code> and
			<code class="font-mono">dm/docs/</code> in your workspace first — that's where the
			app's agent documentation lives.
		</p>
		<p class="text-xs mt-1 {app.workspacePath ? 'text-green-700' : 'text-zinc-400'}">
			{#if app.workspacePath}
				✓ Your workspace path <code class="font-mono">{app.workspacePath}</code> is already filled into the prompts.
			{:else}
				No workspace open — the prompts contain a placeholder where you paste the folder path.
			{/if}
		</p>
	</div>

	{#each GOALS as g (g.key)}
		<section class="mt-6">
			<h2 class="font-semibold text-zinc-900">{g.label}</h2>
			<p class="text-xs text-zinc-500 mt-0.5">{g.blurb}</p>
			<div class="grid gap-4 md:grid-cols-2 mt-3">
				{#each prompts.filter((p) => p.goal === g.key) as p (p.file)}
					<button class="skill-card" onclick={() => showPrompt(p)} title="Open prompt">
						<div>
							<h2 class="font-semibold text-zinc-900 text-sm">{p.title}</h2>
							<p class="text-xs text-zinc-500 mt-0.5">{p.description}</p>
						</div>
						<div class="flex gap-1 flex-wrap">
							{#each p.tags as tag (tag)}
								<span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-100 text-zinc-500">{tag}</span>
							{/each}
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
	.skill-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		text-align: left;
		background: #fff;
		border: 1px solid var(--color-border, #e4e4e7);
		border-radius: var(--radius-lg, 0.75rem);
		padding: var(--space-4);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.skill-card:hover {
		border-color: var(--color-accent);
		box-shadow: var(--shadow-sm, 0 1px 2px rgb(0 0 0 / 0.05));
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
