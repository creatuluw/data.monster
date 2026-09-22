<script lang="ts">
	import { marked } from 'marked';
	import { loadPrompts } from '$lib/agent-prompts';
	import { Copy, Check, BookOpen } from 'lucide-svelte';

	const prompts = loadPrompts();
	let copied = $state('');

	const GOALS: { key: string; label: string; blurb: string }[] = [
		{ key: 'data', label: 'Create data', blurb: 'Get data into the workspace and shape it.' },
		{ key: 'content', label: 'Create content', blurb: 'Build report pages and dashboards.' },
		{ key: 'insights', label: 'Get insights', blurb: 'Reusable analysis building blocks.' },
		{ key: 'actions', label: 'Take actions', blurb: 'Understand and keep the workspace healthy.' }
	];

	function render(body: string): string {
		return marked.parse(body, { async: false }) as string;
	}

	async function copy(body: string) {
		try {
			await navigator.clipboard.writeText(body);
			copied = body;
			setTimeout(() => (copied = ''), 1500);
		} catch {
			// clipboard unavailable — the text is selectable anyway
		}
	}
</script>

<div class="page-shell" style="padding: var(--space-6);">
	<div class="mb-6">
		<h1 class="page-title">Agent prompts</h1>
		<p class="text-sm text-zinc-500 mt-1">
			Copy a prompt into your coding agent (Claude Code, Cursor, Codex, …) to have it
			build content for this app by writing files in your workspace. The agent needs
			access to the workspace folder — paste its path where the prompt says so.
		</p>
		<p class="text-xs text-zinc-400 mt-2 inline-flex items-center gap-1">
			<BookOpen size={12} />
			The prompts teach the agent to read <code class="font-mono">README.md</code> and
			<code class="font-mono">dm/docs/</code> in your workspace first — that's where the
			app's agent documentation lives.
		</p>
	</div>

	{#each GOALS as g (g.key)}
		<section class="mt-6">
			<h2 class="font-semibold text-zinc-900">{g.label}</h2>
			<p class="text-xs text-zinc-500 mt-0.5">{g.blurb}</p>
			<div class="grid gap-4 md:grid-cols-2 mt-3">
			{#each prompts.filter((p) => p.goal === g.key) as p (p.file)}
			<div class="bg-white border border-zinc-200 rounded-xl p-4 flex flex-col gap-2">
				<div class="flex items-start justify-between gap-2">
					<div>
						<h2 class="font-semibold text-zinc-900 text-sm">{p.title}</h2>
						<p class="text-xs text-zinc-500 mt-0.5">{p.description}</p>
					</div>
					<button
						class="p-2 rounded-md text-zinc-400 hover:text-zinc-900 hover:bg-zinc-100 shrink-0"
						onclick={() => copy(p.body)}
						title={copied === p.body ? 'Copied' : 'Copy prompt'}
						aria-label={copied === p.body ? 'Copied' : 'Copy prompt'}
					>
						{#if copied === p.body}<Check size={14} class="text-green-600" />{:else}<Copy size={14} />{/if}
					</button>
				</div>
				<div class="flex gap-1 flex-wrap">
					{#each p.tags as tag (tag)}
						<span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-100 text-zinc-500">{tag}</span>
					{/each}
				</div>
				<div class="prose-chat text-xs max-h-64 overflow-y-auto border-t border-zinc-100 pt-2">
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html render(p.body)}
				</div>
			</div>
			{/each}
			</div>
		</section>
	{/each}
</div>
