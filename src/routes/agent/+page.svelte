<script lang="ts">
	import { marked } from 'marked';
	import { loadPrompts, injectWorkspace } from '$lib/agent-prompts';
	import { app } from '$lib/stores/app.svelte';
	import Drawer from '$lib/components/Drawer.svelte';
	import { Copy, Check, BookOpen, Rocket, Database, LayoutDashboard, Sigma, Eraser } from 'lucide-svelte';

	const prompts = loadPrompts();
	let drawerOpen = $state(false);
	let prompt = $state<(typeof prompts)[number] | null>(null);
	let copied = $state(false);

	const ICONS: Record<string, typeof Rocket> = {
		onboarding: Rocket,
		'ingest-csv': Database,
		'dashboard-interview': LayoutDashboard,
		'add-measure': Sigma,
		explain: BookOpen,
		cleanup: Eraser
	};

	const GOAL_LABELS: Record<string, string> = {
		data: 'Data',
		content: 'Content',
		insights: 'Insights',
		actions: 'Actions'
	};

	function showPrompt(p: (typeof prompts)[number]) {
		prompt = p;
		copied = false;
		drawerOpen = true;
	}

	function closeDrawer() {
		drawerOpen = false;
	}

	function promptText(p: (typeof prompts)[number]): string {
		return injectWorkspace(p.body, app.workspacePath);
	}

	function render(body: string): string {
		return marked.parse(body, { async: false }) as string;
	}

	async function copy() {
		if (!prompt) return;
		try {
			await navigator.clipboard.writeText(promptText(prompt));
			copied = true;
			setTimeout(() => (copied = false), 1500);
		} catch {
			// clipboard unavailable — the text is selectable anyway
		}
	}
</script>



<div class="skills-page">
	<div class="section-header">
		<h1 class="section-title page-title">Skills</h1>
	</div>

	<div class="skills-grid">
		{#each prompts as p (p.file)}
			{@const Icon = ICONS[p.file] ?? BookOpen}
			<button class="skill-card" onclick={() => showPrompt(p)} title="Open prompt">
				<div class="skill-icon">
					<Icon size={18} />
				</div>
				<div class="skill-info">
					<span class="skill-title">{p.title}</span>
					<span class="skill-desc">{p.description}</span>
					<div class="flex gap-1 flex-wrap">
						{#each [GOAL_LABELS[p.goal], ...p.tags].filter(Boolean).slice(0, 3) as tag (tag)}
							<span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-100 text-zinc-500">{tag}</span>
						{/each}
					</div>
				</div>
			</button>
		{/each}
	</div>
</div>

{#snippet drawerFooter()}
	<button class="copy-btn" onclick={copy} title={copied ? 'Copied' : 'Copy prompt'}>
		{#if copied}<Check size={14} class="text-green-600" /> Copied{:else}<Copy size={14} /> Copy{/if}
	</button>
{/snippet}

<Drawer
	bind:open={drawerOpen}
	title={prompt?.title ?? ''}
	width="50vw"
	onClosed={closeDrawer}
	footer={drawerFooter}
>
	{#if prompt}
		<p class="prompt-desc">{prompt.description}</p>
		<div class="prose-chat prompt-md">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html render(promptText(prompt))}
		</div>
	{/if}
</Drawer>

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

	.skills-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-3);
		margin-top: var(--space-6);
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

	
	
	
	
	
	
	.copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		border-radius: var(--radius-xs);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
		background: transparent;
		border: 1px solid var(--color-border-strong);
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.copy-btn:hover {
		color: var(--color-accent);
		border-color: var(--color-accent);
		background: var(--color-surface-sunken);
	}

	.prompt-desc {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		margin: 0 0 var(--space-4) 0;
	}

	.prompt-md {
		font-size: var(--text-xs);
	}

	.copy-btn:hover {
		background: var(--color-accent, #d7e6f5);
	}

	
	.close-btn:hover {
		background: var(--color-accent-muted, #eef4fa);
		color: var(--color-text);
	}

	</style>
