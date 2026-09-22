<script lang="ts">
	/**
	 * Smart DuckDB expression editor for master items (Qlik-Sense-style):
	 * - autocomplete over bound-table fields, master items and a curated DuckDB
	 *   function catalog (↑/↓ + Enter/Tab, fuzzy-ranked)
	 * - SQL syntax highlighting under the caret
	 * - starter templates per kind (share-of-total, conditional agg, date groups…)
	 * - live validation + result preview, run against the bound table
	 */
	import { tick } from 'svelte';
	import { executeQuery, extractErrorMessage, withTimeout } from '$lib/db-operations';
	import type { MasterItem } from '$lib/charts/items';
	import { DUCKDB_FUNCTIONS, CAT_LABELS, type FuncCat } from '$lib/charts/duckdb-functions';

	let {
		value = $bindable(''),
		kind = 'measure',
		table = '',
		/** bound-table fields: 'name' or { name, type } */
		columns = [] as (string | { name: string; type?: string })[],
		masterItems = [] as MasterItem[],
		placeholder = '',
		preview = true
	}: {
		value?: string;
		kind?: 'measure' | 'dimension';
		table?: string;
		columns?: (string | { name: string; type?: string })[];
		masterItems?: MasterItem[];
		placeholder?: string;
		preview?: boolean;
	} = $props();

	type Match = {
		group: 'fields' | 'items' | 'func';
		insert: string;
		name: string;
		meta: string;
		doc?: string;
		badge?: string;
		score: number;
	};

	let ta = $state<HTMLTextAreaElement | null>(null);
	let shellEl = $state<HTMLDivElement | null>(null);
	let open = $state(false);
	let active = $state(0);
	let tokStart = $state(0);
	let caret = $state(0);
	let error = $state('');
	let checking = $state(false);
	let previewVals = $state<string[] | null>(null);
	let runId = 0;

	const fields = $derived(
		columns.map((c) => (typeof c === 'string' ? { name: c, type: undefined as string | undefined } : c))
	);
	const itemMatches = $derived(
		masterItems.filter((i) => i.kind === kind && i.expr.trim())
	);

	// ── token context ─────────────────────────────────────────────────────────
	function tokenBefore(text: string, pos: number): { start: number; token: string } {
		let i = pos;
		while (i > 0 && /[A-Za-z0-9_]/.test(text[i - 1])) i--;
		return { start: i, token: text.slice(i, pos) };
	}

	function score(q: string, s: string): number {
		if (!q) return 1;
		const ls = s.toLowerCase();
		const lq = q.toLowerCase();
		if (ls.startsWith(lq)) return 4;
		if (new RegExp(`[\\s_('.]${lq.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`).test(ls)) return 3;
		if (ls.includes(lq)) return 2;
		let i = 0;
		for (const ch of ls) {
			if (ch === lq[i]) i++;
			if (i === lq.length) return 1;
		}
		return 0;
	}

	const matches = $derived.by(() => {
		const q = value.slice(tokStart, caret);
		const out: Match[] = [];
		const f: Match[] = [];
		for (const c of fields) {
			const s = score(q, c.name);
			if (s > 0) f.push({ group: 'fields', insert: c.name, name: c.name, meta: c.type ?? table, badge: 'field', score: s * 10 });
		}
		out.push(...f.sort((a, b) => b.score - a.score || a.name.localeCompare(b.name)).slice(0, 6));
		const it: Match[] = [];
		for (const i of itemMatches) {
			const s = Math.max(score(q, i.label), score(q, i.expr));
			if (s > 0)
				it.push({
					group: 'items',
					insert: i.expr,
					name: `⭐ ${i.label}`,
					meta: i.expr,
					doc: 'master item — inserts its expression',
					badge: 'master',
					score: s * 9
				});
		}
		out.push(...it.sort((a, b) => b.score - a.score).slice(0, 4));
		const fn: Match[] = [];
		for (const fdef of DUCKDB_FUNCTIONS) {
			const s = Math.max(score(q, fdef.name), score(q, fdef.sig));
			if (s > 0)
				fn.push({
					group: 'func',
					insert: fdef.snippet,
					name: fdef.sig,
					meta: CAT_LABELS[fdef.cat],
					doc: fdef.doc,
					badge: fdef.cat,
					score: s * 8
				});
		}
		out.push(...fn.sort((a, b) => b.score - a.score).slice(0, 9));
		return out;
	});

	const grouped = $derived.by(() => {
		const map = new Map<Match['group'], Match[]>();
		for (const m of matches) map.set(m.group, [...(map.get(m.group) ?? []), m]);
		return [...map.entries()];
	});

	const GROUP_TITLES = $derived.by(
		() =>
			({
				fields: `Fields on ${table || 'table'}`,
				items: 'Master items — inserts expression',
				func: 'DuckDB functions'
			}) as Record<Match['group'], string>
	);

	$effect(() => {
		if (active >= matches.length) active = Math.max(0, matches.length - 1);
	});

	function syncCaret() {
		if (!ta) return;
		const text = ta.value;
		const pos = ta.selectionStart ?? text.length;
		caret = pos;
		tokStart = tokenBefore(text, pos).start;
	}

	// clicking outside the editor closes suggestions BEFORE the click lands,
	// so the popup never swallows clicks on buttons underneath it (e.g. Save)
	function onGlobalPointerdown(e: PointerEvent) {
		if (!open) return;
		const t = e.target as Node | null;
		if (t && shellEl?.contains(t)) return;
		open = false;
	}

	function onInput() {
		if (!ta) return;
		value = ta.value; // DOM-first — bind ordering must not matter
		syncCaret();
		open = matches.length > 0;
		if (open) active = 0;
	}

	function openAll() {
		if (!ta) return;
		caret = ta.selectionStart ?? value.length;
		tokStart = caret;
		open = true;
		active = 0;
	}

	async function insert(m: Match) {
		const before = value.slice(0, tokStart);
		const after = value.slice(caret);
		let text = m.insert;
		let at = text.length;
		const marker = text.indexOf('§');
		if (marker >= 0) {
			text = text.replace('§', '');
			at = marker;
		}
		value = before + text + after;
		open = false;
		await tick();
		const pos = tokStart + at;
		ta?.focus();
		ta?.setSelectionRange(pos, pos);
		syncCaret();
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && open) {
			open = false;
			return;
		}
		if ((e.ctrlKey || e.metaKey) && e.code === 'Space') {
			e.preventDefault();
			openAll();
			return;
		}
		if (!open || matches.length === 0) return;
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			active = (active + 1) % matches.length;
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			active = (active - 1 + matches.length) % matches.length;
		} else if (e.key === 'Enter' || e.key === 'Tab') {
			e.preventDefault();
			void insert(matches[active]);
		}
	}

	function onOuterMouseDown(m: Match) {
		// mousedown (not click) so the textarea keeps focus
		return (e: MouseEvent) => {
			e.preventDefault();
			void insert(m);
		};
	}

	// ── starter templates ─────────────────────────────────────────────────────
	const templates = $derived.by(() => {
		const t: { label: string; snippet: string; title: string }[] =
			kind === 'measure'
				? [
						{ label: 'total', snippet: 'sum(§)', title: 'Sum of a column' },
						{ label: 'average', snippet: 'avg(§)', title: 'Mean of a column' },
						{ label: 'count distinct', snippet: 'count(distinct §)', title: 'Unique values' },
						{ label: 'share of total', snippet: 'sum(x) / nullif(sum(x) over (), 0)', title: 'Each row share of the grand total (window)' },
						{ label: 'conditional agg', snippet: "sum(case when cond then x end)", title: 'Aggregate only matching rows' },
						{ label: 'filtered count', snippet: 'count(*) filter (where §)', title: 'DuckDB FILTER clause' },
						{ label: 'vs last period', snippet: "x - lag(x) over (order by sort_col)", title: 'Delta vs previous row/period' }
					]
				: [
						{ label: 'month period', snippet: "date_trunc('month', §)", title: 'Group by month' },
						{ label: 'year-month', snippet: "strftime(§, '%Y-%m')", title: 'Text period label' },
						{ label: 'text cast', snippet: 'cast(§ as varchar)', title: 'Force label type' },
						{ label: 'bucket mapping', snippet: 'case when cond then § else null end', title: 'Remap values into groups' },
						{ label: 'null default', snippet: "coalesce(§, 'unknown')", title: 'Replace NULLs with a label' },
						{ label: 'numeric bucket', snippet: 'floor(x / 10) * 10', title: 'Range buckets' }
					];
		return t;
	});

	async function insertTemplate(snippet: string) {
		let text = snippet;
		let at = text.length;
		const marker = text.indexOf('§');
		if (marker >= 0) {
			text = text.replace('§', '');
			at = marker;
		}
		const base = value.trimEnd();
		const sep = base ? ' ' : '';
		value = base ? `${base}${sep}${text}` : text;
		await tick();
		const pos = (base ? base.length + sep.length : 0) + at;
		ta?.focus();
		ta?.setSelectionRange(pos, pos);
		syncCaret();
	}

	// ── highlighting ──────────────────────────────────────────────────────────
	const KW = 'select|from|where|case|when|then|else|end|and|or|not|in|is|null|over|partition|by|order|as|filter|cast|distinct|interval|between|like|limit|group|asc|desc|try';
	const TYPES = 'varchar|int|integer|double|float|decimal|numeric|date|timestamp|timestamptz|boolean|bool|bigint|smallint|time|uuid|blob';
	const HL =
		/('(?:[^']|'')*')|(--[^\n]*)|(\b\d+(?:\.\d+)?\b)|(\b(?:KW)\b)|(::\s*\w+)|(\b(?:TYPES)\b)|([a-zA-Z_][\w]*(?=\s*\())/gi
			.source
			.replace('KW', KW)
			.replace('TYPES', TYPES);

	function esc(s: string): string {
		return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
	}

	const highlighted = $derived.by(() => {
		if (!value) return '';
		const re = new RegExp(HL, 'gi');
		let out = '';
		let last = 0;
		for (const m of value.matchAll(re)) {
			const i = m.index ?? 0;
			out += esc(value.slice(last, i));
			const [full, str, com, num, kw, castOp, typ, fn] = m;
			const cls = str ? 'hl-str' : com ? 'hl-com' : num ? 'hl-num' : kw ? 'hl-kw' : castOp ? 'hl-cast' : typ ? 'hl-type' : fn ? 'hl-fn' : '';
			out += cls ? `<span class="${cls}">${esc(full)}</span>` : esc(full);
			last = i + full.length;
		}
		out += esc(value.slice(last));
		return out;
	});

	// ── live validation + preview ─────────────────────────────────────────────
	const expr = $derived(value.trim());

	// serialize: never two preview queries in flight (overlapping invokes while
	// typing are what tripped the IPC deadlock in the Tauri backend)
	let prevQuery: Promise<void> = Promise.resolve();

	$effect(() => {
		if (!preview || !table) return;
		const sql = expr;
		if (!sql) {
			error = '';
			previewVals = null;
			checking = false;
			return;
		}
		const id = ++runId;
		checking = true;
		const limit = kind === 'dimension' ? 5 : 1;
		const t = setTimeout(() => {
			prevQuery = prevQuery.then(async () => {
				if (id !== runId) return; // a newer keystroke superseded this run
				try {
					const res = await withTimeout(executeQuery(`select (${sql}) as v from "${table}" limit ${limit}`), 15000, 'validation timed out — the database is not responding');
					if (id !== runId) return;
					error = '';
					previewVals = 'data' in res
						? (res.data as unknown[][]).map((r: unknown[]) => {
								const v = r[0];
								if (v === null || v === undefined) return 'null';
								if (typeof v === 'number') return Number(v).toLocaleString('en-US', { maximumFractionDigits: 4 });
								const s = String(v);
								return s.length > 28 ? `${s.slice(0, 27)}…` : s;
							})
						: null;
				} catch (err) {
					if (id !== runId) return;
					previewVals = null;
					const msg = extractErrorMessage(err, 'Invalid expression');
					error = msg.length > 180 ? `${msg.slice(0, 179)}…` : msg;
				} finally {
					if (id === runId) checking = false;
				}
			});
		}, 500);
		return () => clearTimeout(t);
	});
</script>

<svelte:window onpointerdown={onGlobalPointerdown} />

<div class="expr-editor">
	<div class="templates" role="toolbar" aria-label="Expression templates">
		<span class="tpl-label">{kind === 'measure' ? 'measure' : 'dimension'} starters</span>
		{#each templates as t (t.label)}
			<button type="button" class="tpl" title={t.title} onclick={() => insertTemplate(t.snippet)}>{t.label}</button>
		{/each}
	</div>

<div class="shell" class:has-error={!!error} bind:this={shellEl}>
	<pre class="hl" aria-hidden="true">{@html highlighted + '\n'}</pre>
	<textarea
		bind:this={ta}
		bind:value
		{placeholder}
		spellcheck="false"
		class="input"
		oninput={onInput}
			onkeydown={onKeydown}
			onkeyup={syncCaret}
			onclick={syncCaret}
			onselect={syncCaret}
			onfocus={syncCaret}
			onblur={() => setTimeout(() => (open = false), 120)}
		></textarea>

		{#if open && matches.length > 0}
			<div class="pop" role="listbox" aria-label="Expression suggestions">
				{#each grouped as [group, list] (group)}
					<p class="pop-group">{GROUP_TITLES[group]}</p>
					{#each list as m (m.group + m.name + m.meta)}
						{@const idx = matches.indexOf(m)}
						<button
							type="button"
							class="pop-row"
							class:active={idx === active}
							role="option"
							aria-selected={idx === active}
							onmousedown={onOuterMouseDown(m)}
							onmouseenter={() => (active = idx)}
						>
							<span class="pop-name" class:mono={m.group !== 'func'}>{m.name}</span>
							<span class="pop-meta">{m.meta}</span>
							{#if m.doc}<span class="pop-doc">{m.doc}</span>{/if}
						</button>
					{/each}
				{/each}
				<p class="pop-hint">↑↓ choose · Enter/Tab insert · Esc dismiss · Ctrl+Space all</p>
			</div>
		{/if}
	</div>

	<div class="status">
		{#if checking}
			<span class="st-check">checking…</span>
		{:else if error}
			<span class="st-error" title={error}>✗ {error}</span>
		{:else if previewVals && previewVals.length}
			<span class="st-ok">✓ {kind === 'measure' ? 'result' : 'samples'}</span>
			{#each previewVals.slice(0, 5) as v, i (i)}
				<span class="chip">{v}</span>
			{/each}
		{:else if !value.trim()}
			<span class="st-hint">type a DuckDB expression — or tap a starter above · suggestions appear as you type · Ctrl+Space lists all</span>
		{/if}
	</div>
</div>

<style>
	.expr-editor {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.templates {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 4px;
	}
	.tpl-label {
		font-size: 10px;
		color: #a1a1aa;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin-right: 2px;
	}
	.tpl {
		font-size: 11px;
		padding: 1px 7px;
		border: 1px solid #e4e4e7;
		border-radius: 999px;
		background: #fff;
		color: #52525b;
		cursor: pointer;
		transition:
			color 120ms ease,
			border-color 120ms ease;
	}
	.tpl:hover {
		color: #18181b;
		border-color: #a1a1aa;
	}

	.shell {
		position: relative;
		border: 1px solid var(--color-border-strong, #d4d4d8);
		border-radius: var(--radius-xs, 6px);
		background: var(--color-surface, #fff);
	}
	.shell:focus-within {
		border-color: #a1a1aa;
		box-shadow: 0 0 0 2px rgb(24 24 27 / 6%);
	}
	.shell.has-error {
		border-color: #f87171;
	}

	/* identical metrics on both layers */
	.hl,
	.input {
		margin: 0;
		padding: 6px 8px;
		font-family: var(--font-mono, monospace);
		font-size: 12px;
		line-height: 1.5;
		white-space: pre-wrap;
		word-break: break-word;
		tab-size: 4;
	}
	.hl {
		min-height: calc(1.5em + 12px);
		color: #18181b;
		pointer-events: none;
	}
	.hl :global(.hl-kw) {
		color: #7c3aed;
		font-weight: 500;
	}
	.hl :global(.hl-str) {
		color: #0a7d33;
	}
	.hl :global(.hl-num) {
		color: #b45309;
	}
	.hl :global(.hl-com) {
		color: #94a3b8;
		font-style: italic;
	}
	.hl :global(.hl-fn) {
		color: #0369a1;
	}
	.hl :global(.hl-cast),
	.hl :global(.hl-type) {
		color: #be185d;
	}

	.input {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		resize: none;
		border: none;
		outline: none;
		background: transparent;
		color: transparent;
		caret-color: #18181b;
		overflow: hidden;
	}
	.input::placeholder {
		color: #a1a1aa;
	}

	.pop {
		position: absolute;
		left: 0;
		right: 0;
		top: calc(100% + 4px);
		z-index: 60;
		max-height: 262px;
		overflow-y: auto;
		background: #fff;
		border: 1px solid #e4e4e7;
		border-radius: 8px;
		box-shadow: 0 10px 28px rgb(0 0 0 / 12%);
		padding: 2px 0 4px;
	}
	.pop-group {
		padding: 6px 10px 2px;
		font-size: 10px;
		font-weight: 600;
		color: #a1a1aa;
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}
	.pop-row {
		display: grid;
		grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr);
		grid-template-areas:
			'name meta'
			'doc doc';
		align-items: baseline;
		gap: 0 10px;
		width: 100%;
		text-align: left;
		padding: 4px 10px;
		background: none;
		border: none;
		cursor: pointer;
	}
	.pop-row.active {
		background: #f4f4f5;
	}
	.pop-name {
		grid-area: name;
		font-size: 12.5px;
		color: #18181b;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	.pop-name.mono {
		font-family: var(--font-mono, monospace);
		font-size: 12px;
	}
	.pop-meta {
		grid-area: meta;
		font-size: 11px;
		color: #71717a;
		font-family: var(--font-mono, monospace);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		text-align: right;
	}
	.pop-doc {
		grid-area: doc;
		font-size: 11px;
		color: #a1a1aa;
	}
	.pop-hint {
		padding: 5px 10px 2px;
		font-size: 10px;
		color: #c4c4cc;
		border-top: 1px solid #f4f4f5;
		margin-top: 3px;
	}

	.status {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 4px;
		min-height: 18px;
		font-size: 11px;
	}
	.st-check {
		color: #a1a1aa;
	}
	.st-ok {
		color: #16a34a;
	}
	.st-error {
		color: #dc2626;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 100%;
	}
	.chip {
		font-family: var(--font-mono, monospace);
		font-size: 11px;
		padding: 0 6px;
		border-radius: 999px;
		background: #f4f4f5;
		border: 1px solid #e4e4e7;
		color: #3f3f46;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
