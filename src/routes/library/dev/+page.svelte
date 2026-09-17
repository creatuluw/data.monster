<script lang="ts">
	import { Terminal, FolderTree, GitBranch, ClipboardCheck, Bot } from 'lucide-svelte';
</script>

<svelte:head>
	<title>For developers — Library — Data Monster</title>
</svelte:head>

<div class="page">
	<div class="head">
		<h1 class="section-title">For developers</h1>
	</div>
	<p class="desc">
		How to build a library component. One registration makes it available app-wide:
		this library, the page-editor add-block picker, and a schema-driven config panel.
	</p>

	<div class="callout">
		<div class="callout-icon"><Bot size={18} /></div>
		<div>
			<strong>Fastest path:</strong> use the <code>library-component-builder</code> skill
			(<code>.pi/skills/library-component-builder/</code>). Give it your idea — it interviews
			you, builds test-first, registers, and only reports done when the e2e report passes.
		</div>
	</div>

	<section>
		<h2><FolderTree size={15} /> The package</h2>
		<p>Every component is a self-contained folder — own definition, logic, data, docs and report:</p>
		<pre><code>src/lib/library/components/&lt;type&gt;/
├── def.ts        # roles + optionsSchema + hooks + annotations
├── demo.ts       # dummy rows + aliases (previews need no DuckDB)
├── docs.md       # usage notes (Docs tab)
├── index.ts      # wires def + REAL renderer + demo + source refs
└── REPORT.md     # e2e test report — all PASS, or it isn't done</code></pre>
		<p>
			The renderer lives in <code>src/lib/components/charts/renderers/</code> — demos render the
			real component, never a clone.
		</p>
	</section>

	<section>
		<h2><GitBranch size={15} /> Register</h2>
		<p>One import + one call in <code>src/lib/charts/registry-setup.svelte.ts</code>:</p>
		<pre><code>import pie from '$lib/library/components/pie';

registerLibraryComponent(pie);   // inside setupChartRegistry()
// and add [pie.def.type]: pie.renderer to chartRenderers</code></pre>
		<p>That single registration feeds the library grid, add-block picker, config panel and validation.</p>
	</section>

	<section>
		<h2><Terminal size={15} /> Build test-first</h2>
		<ul>
			<li>Red → green, one slice per cycle: failing test first, then the minimum code to pass.</li>
			<li>Agree the seams up front; pure logic gets unit tests, the Svelte renderer stays a thin shell verified by e2e.</li>
			<li>Tests live in <code>tests/</code> — never inside <code>src/</code>.</li>
			<li>Verify with <code>npx vitest run</code>, <code>npx svelte-check --threshold error</code>, <code>npm run build</code>.</li>
		</ul>
	</section>

	<section>
		<h2><ClipboardCheck size={15} /> Done = e2e report passing</h2>
		<p>Every component ships <code>REPORT.md</code> with concrete evidence:</p>
		<ul>
			<li><strong>Definitions</strong> — roles/options validated by unit tests; full suite green.</li>
			<li><strong>UI</strong> — card on /library, Preview renders demo rows, add-block works on a page, config panel shows the schema fields.</li>
			<li><strong>Code/data/logic</strong> — the checklist from the skill's CONTRACT reference, every box justified.</li>
		</ul>
		<p class="muted">Any FAIL or missing report = not done. Report format: <code>.pi/skills/library-component-builder/references/E2E-REPORT.md</code>.</p>
	</section>
</div>

<style>
	.page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
		max-width: 80ch;
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-1) 0 var(--space-4) 0;
		line-height: var(--leading-relaxed);
	}

	.callout {
		display: flex;
		gap: var(--space-3);
		align-items: flex-start;
		padding: var(--space-3) var(--space-4);
		border: 1px solid var(--color-accent, oklch(0.44 0.1 158 / 0.35));
		border-left-width: 3px;
		border-radius: var(--radius-md, 8px);
		background: var(--color-surface, #fff);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		margin-bottom: var(--space-5);
	}

	.callout-icon {
		color: var(--color-accent);
		margin-top: 2px;
	}

	section {
		margin-bottom: var(--space-6);
	}

	h2 {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		margin: 0 0 var(--space-2) 0;
	}

	h2 :global(svg) {
		color: var(--color-text-tertiary);
	}

	p,
	li {
		font-size: var(--text-sm);
		color: var(--color-text-secondary, #52525b);
		line-height: var(--leading-relaxed);
	}

	ul {
		padding-left: var(--space-5);
		margin: var(--space-2) 0;
	}

	code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
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
		margin: var(--space-2) 0;
	}

	.muted {
		color: var(--color-text-tertiary);
	}
</style>
