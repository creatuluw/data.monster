<script lang="ts">
	import { page as pageState } from '$app/state';
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getLibraryComponent } from '$lib/library/registry';
	import { marked } from 'marked';
	import TableRenderer from '$lib/components/charts/renderers/TableRenderer.svelte';
	import { highlightHTML } from '@speed-highlight/core';
	import '@speed-highlight/core/themes/github-light.css';
	import { Bot, Copy, Check } from 'lucide-svelte';

	setupChartRegistry();

	const id = $derived(pageState.params.id ?? '');
	const entry = $derived(getLibraryComponent(id));
	let tab = $state<'preview' | 'schema' | 'code' | 'docs'>('preview');

	// demo selection — same transient shape /pages uses ({dimension, value})
	let selected = $state<{ dimension: string; value: string } | null>(null);

	// config-drawer overrides — schema-driven, same fields the /pages inspector renders
	let optOverrides = $state<Record<string, unknown>>({});

	// schema tab: the definition as it lives in code — a JSON object
	const schemaJson = $derived(entry ? JSON.stringify(entry.def, null, 2) : '');

	// docs tab: bundled markdown → styled HTML (same pipeline as the analyst chat)
	marked.setOptions({ breaks: true, gfm: true });
	const docsHtml = $derived(entry ? (marked.parse(entry.docs, { async: false }) as string) : '');

	// code tab: path → highlighted HTML (speed-highlight/core, language by extension)
	function langOf(path: string): string {
		if (path.endsWith('.ts')) return 'ts';
		if (path.endsWith('.svelte')) return 'html'; // svelte templates tokenize as html
		if (path.endsWith('.md')) return 'md';
		return 'plain';
	}

	let highlightedCode = $state<Record<string, string>>({});
	$effect(() => {
		const e = entry;
		if (!e) {
			highlightedCode = {};
			return;
		}
		let alive = true;
		Promise.all(
				Object.entries(e.code).map(async ([path, src]) => {
					const opts = { block: true, showLineNumbers: true };
					try {
						return [path, await highlightHTML(src, langOf(path), opts)];
					} catch {
						return [path, await highlightHTML(src, 'plain', opts)];
					}
				})
			)
			.then((rows) => {
					if (alive) highlightedCode = Object.fromEntries(rows as [string, string][]);
				})
			.catch(() => {});
		return () => {
			alive = false;
		};
	});

	async function copy(text: string, key: string) {
		await navigator.clipboard.writeText(text);
		copied = key;
		setTimeout(() => (copied = null), 1500);
	}
	let copied = $state<string | null>(null);

	// LLM prompt: hand an agent everything needed to work on this component
	const llmPrompt = $derived.by(() => {
		if (!entry) return '';
		const paths = Object.keys(entry.code);
		const defPath = paths.find((p) => p.endsWith('def.ts')) ?? '';
		const pkgDir = defPath.replace('/def.ts', '');
		const files = paths.map((p) => `  - ${p}`).join('\n');
		return `Use the library-component-builder skill in this repo (.pi/skills/library-component-builder/SKILL.md).

Component: ${entry.def.type} (${entry.def.label}) — a registered /library component used in /pages.
Package: ${pkgDir}/ — def.ts, demo.ts, docs.md, index.ts (+ REPORT.md on change)
Files:
${files}
Registered in: src/lib/charts/registry-setup.svelte.ts

Task: <describe what to build, change or maintain here>

The skill enforces: interview when anything is unclear, build test-first (red-green, tests in tests/ only), keep the renderer a thin shell over pure logic, verify with npx vitest run + svelte-check + npm run build and the /library/${entry.def.type} preview, and finish with an all-PASS e2e REPORT.md in the package.`;
	});

	// merged options: option-schema defaults, then definition defaults, then demo overrides
	const demoOptions = $derived(
		entry
			? {
					...Object.fromEntries(
						entry.def.optionsSchema.filter((f) => f.default !== undefined).map((f) => [f.name, f.default])
					),
					...entry.def.defaults,
					...entry.demo.options,
					...optOverrides
				}
			: {}
	);
</script>

<svelte:head>
	<title>{entry?.def.label ?? id} — Library — Data Monster</title>
</svelte:head>

<div class="page">
	{#if entry}
		<div class="head">
			<h1 class="section-title">{entry.def.label}</h1>
			<span class="type">{entry.def.type}</span>
			<button
				class="llm-btn"
				onclick={() => copy(llmPrompt, '__llm')}
				title="Copy an LLM prompt to get an agent started on building, changing or maintaining this component (uses the library-component-builder skill)"
				aria-label="Copy LLM prompt for this component"
			>
				{#if copied === '__llm'}<Check size={14} />{:else}<Bot size={14} />{/if}
			</button>
		</div>
		<p class="desc">{entry.description}</p>

		<div class="tabs" role="tablist">
			<button class:active={tab === 'preview'} onclick={() => (tab = 'preview')}>Preview</button>
			<button class:active={tab === 'schema'} onclick={() => (tab = 'schema')}>Schema</button>
			<button class:active={tab === 'code'} onclick={() => (tab = 'code')}>Code</button>
			<button class:active={tab === 'docs'} onclick={() => (tab = 'docs')}>Docs</button>
		</div>

		{#if tab === 'preview'}
			<div class="tab-content">
				{#if (entry.blockKind ?? 'chart') === 'table'}
					<!-- the REAL TableRenderer /pages uses, fed bundled dummy rows -->
					<TableRenderer rows={entry.demo.rows} columns={entry.demo.columns ?? []} title={entry.demo.title} status="ok" error="" />
				{:else if entry.blockKind === 'text'}
					<!-- same card styling PageGrid renders text blocks with -->
					<div class="text-block">{entry.demo.text}</div>
				{:else}
				<!-- the REAL renderer /pages uses, fed bundled dummy data;
				     config snippet = schema-driven option fields, like the inspector -->
				<svelte:component
					this={entry.renderer}
					rows={entry.demo.rows}
					dimensionAliases={entry.demo.dimensionAliases}
					measureAliases={entry.demo.measureAliases}
					options={demoOptions}
					annotations={[]}
					title={entry.demo.title}
					subtitle={entry.demo.subtitle}
					selected={selected}
					onSelect={(s: { dimension: string; value: string } | null) => (selected = s)}
					colorScale={{ colorOf: () => '#888888' }}
					fmts={{} }
					heightVh={0.35}
				>
					{#snippet config()}
						<div class="space-y-3">
							<div class="space-y-1">
								<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Demo data</span>
								<p class="text-xs text-zinc-400 font-mono">{entry.demo.dimensionAliases.join(', ') || '—'} × {entry.demo.measureAliases.join(', ') || '—'} · {entry.demo.rows.length} rows</p>
							</div>
							<div class="space-y-2">
								<span class="text-xs font-medium text-zinc-500 uppercase tracking-wide">Demo options</span>
							{#each entry.def.optionsSchema as field (field.name)}
								<label class="grid grid-cols-2 gap-2 items-center">
									<span class="text-xs text-zinc-500">{field.label}</span>
									{#if field.kind === 'enum'}
										<select class="border border-zinc-300 rounded px-2 py-1" value={String(demoOptions[field.name] ?? '')} onchange={(e) => (optOverrides = { ...optOverrides, [field.name]: (e.target as HTMLSelectElement).value })}>
											{#each field.options ?? [] as o (o)}<option value={o}>{o}</option>{/each}
										</select>
									{:else if field.kind === 'boolean'}
										<input type="checkbox" checked={Boolean(demoOptions[field.name])} onchange={(e) => (optOverrides = { ...optOverrides, [field.name]: (e.target as HTMLInputElement).checked })} />
									{:else}
										<input type={field.kind === 'number' ? 'number' : 'text'} class="border border-zinc-300 rounded px-2 py-1" value={String(demoOptions[field.name] ?? '')} onchange={(e) => { const raw = (e.target as HTMLInputElement).value; optOverrides = { ...optOverrides, [field.name]: field.kind === 'number' ? (raw === '' ? undefined : Number(raw)) : raw }; }} />
									{/if}
								</label>
							{/each}
							</div>
						</div>
					{/snippet}
				</svelte:component>
				{/if}
			</div>
		{:else if tab === 'schema'}
			<div class="panel tab-content">
				<pre class="json">{schemaJson}</pre>
			</div>
		{:else if tab === 'code'}
			<div class="tab-content">
				<p class="hint">Extensions live as self-contained packages under <code>src/lib/library/components/&lt;type&gt;/</code></p>
				{#each Object.entries(entry.code) as [path, src] (path)}
					<div class="file-row">
						<span class="file-path">{path}</span>
						<button class="copy-btn" onclick={() => copy(src, path)} title="Copy source to clipboard">
							{#if copied === path}<Check size={13} />{:else}<Copy size={13} />{/if}
						</button>
					</div>
					<!-- shj-lang-* class on the wrapper is what activates the theme chrome -->
					<div class="code-block shj-lang-{langOf(path)}">{@html highlightedCode[path] ?? ''}</div>
				{/each}
			</div>
		{:else}
			<div class="panel docs prose-chat tab-content">{@html docsHtml}</div>
		{/if}
	{:else}
		<p class="muted">Unknown component "{id}".</p>
	{/if}
</div>

<style>
	.page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.head {
		display: flex;
		align-items: baseline;
		gap: var(--space-3);
	}

	.llm-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		margin-left: auto;
		border: 1px solid transparent;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.llm-btn:hover {
		color: var(--color-text, #18181b);
		border-color: var(--color-border-strong, #d4d4d8);
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.type {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-1) 0 var(--space-4) 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.tabs {
		display: flex;
		gap: var(--space-1);
		border-bottom: 1px solid var(--color-border, #e4e4e7);
		margin-bottom: var(--space-6);
	}

	.tabs button {
		padding: var(--space-2) var(--space-3);
		font-size: var(--text-sm);
		border: none;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs) var(--radius-xs) 0 0;
		border-bottom: 2px solid transparent;
	}

	.tabs button.active {
		color: var(--color-text);
		font-weight: 600;
		border-bottom-color: var(--color-accent);
	}

	/* one uniform pad around tab content */
	.tab-content {
		padding: var(--space-4);
	}

	/* text block — same card PageGrid renders */
	.text-block {
		background: #fff;
		border: 1px solid #e4e4e7;
		border-radius: var(--radius-md, 8px);
		padding: var(--space-6);
		font-size: var(--text-sm);
		color: #3f3f46;
		white-space: pre-wrap;
	}

	.panel {
		max-width: 900px;
	}

	.panel h2 {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		margin: var(--space-5) 0 var(--space-2) 0;
	}

	.panel h2:first-child {
		margin-top: 0;
	}

	.panel h2.file {
		font-family: var(--font-mono);
		font-weight: 500;
		color: var(--color-text-tertiary);
	}

	.hint {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-3) 0;
	}

	.hint code {
		font-family: var(--font-mono);
	}

	.file-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		margin: var(--space-4) 0 var(--space-1) 0;
	}

	.file-row:first-of-type {
		margin-top: 0;
	}

	.file-path {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.copy-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: 1px solid transparent;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.copy-btn:hover {
		color: var(--color-text, #18181b);
		border-color: var(--color-border-strong, #d4d4d8);
	}

	pre {
		background: var(--color-bg-inset, #fafafa);
		border: 1px solid var(--color-border, #e4e4e7);
		border-radius: var(--radius-md, 8px);
		padding: var(--space-3);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: 1.6;
		overflow-x: auto;
		white-space: pre;
		margin: 0 0 var(--space-4) 0;
	}

	/* .code-block carries shj-lang-* → theme supplies bg/border/mono/flex layout;
	   scoped overrides fit it to the app: full-width, wrapped (no scrollbars) */
	.code-block {
		margin: 0 0 var(--space-4) 0;
		font-size: var(--text-xs);
		line-height: 1.6;
		border-radius: var(--radius-md, 8px);
		border-color: var(--color-border, #e4e4e7);
		white-space: pre-wrap;
		word-break: break-word;
	}

	/* theme sets overflow-x: auto + pre on the flex row → wrap instead */
	.code-block :global(div) {
		overflow-x: hidden;
		white-space: pre-wrap;
		word-break: break-word;
	}

	pre code {
		background: none;
		font-size: inherit;
		padding: 0;
		white-space: inherit;
	}

	pre.json {
		margin-bottom: 0;
	}

	.muted {
		color: var(--color-text-tertiary);
		font-size: var(--text-sm);
	}
</style>
