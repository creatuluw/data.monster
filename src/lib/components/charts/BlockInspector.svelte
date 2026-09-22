<script lang="ts">
	import type { PageDoc } from '$lib/charts/spec-types';
	import { getChartType } from '$lib/charts/registry';
	import { availableItems, linkedTables } from '$lib/charts/relationships';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import { onMount } from 'svelte';
	import { saveMasterItem } from '$lib/central-api';
	import { extractErrorMessage } from '$lib/db-operations';
	import type { DimensionSpec, MeasureSpec } from '$lib/charts/spec-types';
	import { Plus, SquareArrowOutUpRight } from 'lucide-svelte';
	import Field from './controls/Field.svelte';
	import TextInput from './controls/TextInput.svelte';
	import NumberInput from './controls/NumberInput.svelte';
	import Select from './controls/Select.svelte';
	import Toggle from './controls/Toggle.svelte';
	import Section from './controls/Section.svelte';
	import RemoveBtn from './controls/RemoveBtn.svelte';
	import Btn from './controls/Btn.svelte';
	import ExprEditor from './ExprEditor.svelte';

	let {
		doc,
		ri,
		ci,
		bi,
		schemas,
		items,
		relationships,
		onremove,
		onItemsChanged,
		onExternalCreate
	}: {
		doc: PageDoc;
		ri: number;
		ci: number;
		bi: number;
		schemas: TableSchemas;
		items: MasterItem[];
		relationships: Relationship[];
		/** fired when the block is removed (lets the host close its drawer) */
		onremove?: () => void;
		/** fired after a master item was created on the spot — host reloads the library */
		onItemsChanged?: () => void;
		/** open the /data master-item section for full-panel creation — host navigates + returns */
		onExternalCreate?: (kind: 'dimension' | 'measure', table: string) => void;
	} = $props();

	const block = $derived(doc.rows![ri].columns![ci].blocks[bi]);
	const chart = $derived(block.type === 'chart' ? block.chart : null);
	const def = $derived(chart ? getChartType(chart.type) : null);
	const tableCols = $derived(
		block.type === 'chart' ? (schemas[chart!.source.table] ?? []) : block.type === 'table' ? (schemas[block.table] ?? []) : []
	);
	const usableItems = $derived(
		chart ? availableItems(chart.source.table, items, relationships) : []
	);
	const linked = $derived(
		chart ? linkedTables(chart.source.table, relationships).filter((t) => schemas[t]?.length) : []
	);

	const OPS = ['=', '!=', '>', '<', '>=', '<=', 'in', 'like'];
	const GRAINS = ['(none)', 'hour', 'day', 'week', 'month', 'quarter', 'year', 'day_of_week', 'month_of_year'];
	const FMTS = ['(none)', 'hours', 'usd', 'pct', 'int'];

	// create-on-the-spot master items (saved to the workspace library, chart refs them)
	let dimForm = $state({ open: false, label: '', expr: '' });
	let measForm = $state({ open: false, label: '', expr: '', fmt: '(none)' });
	let formError = $state('');

	function resetForms() {
		dimForm = { open: false, label: '', expr: '' };
		measForm = { open: false, label: '', expr: '', fmt: '(none)' };
		formError = '';
	}

	function itemId(label: string): string {
		return `mi_${chart!.source.table}_${label.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_')}`;
	}

	async function createDimension() {
		if (!chart) return;
		if (!dimForm.label.trim() || !dimForm.expr.trim()) return;
		try {
			const id = itemId(dimForm.label);
			await saveMasterItem({ id, kind: 'dimension', table: chart.source.table, label: dimForm.label.trim(), expr: dimForm.expr.trim() });
			chart.dimensions.push({ ref: id });
			resetForms();
			onItemsChanged?.();
		} catch (err) {
			formError = extractErrorMessage(err, 'Failed to save dimension');
		}
	}

	async function createMeasure() {
		if (!chart) return;
		if (!measForm.label.trim() || !measForm.expr.trim()) return;
		try {
			const id = itemId(measForm.label);
			await saveMasterItem({ id, kind: 'measure', table: chart.source.table, label: measForm.label.trim(), expr: measForm.expr.trim(), fmt: measForm.fmt === '(none)' ? undefined : measForm.fmt });
			chart.measures.push({ ref: id });
			resetForms();
			onItemsChanged?.();
		} catch (err) {
			formError = extractErrorMessage(err, 'Failed to save measure');
		}
	}

	// table-first: switching tables keeps only dims/refs still available for the new table
	function onTableChange(table: string) {
		if (!chart) return;
		chart.source.table = table;
		const okRefs = new Set(availableItems(table, items, relationships).map((i) => i.id));
		chart.dimensions = chart.dimensions.filter((d) => !('ref' in d) || okRefs.has(d.ref));
		chart.measures = chart.measures.filter((m) => !('ref' in m) || okRefs.has(m.ref));
		resetForms();
	}

	// picker value encodings: `ref:<id>` | `col:<table>:<col>` | `field:<table>:<col>`
	function dimValue(d: DimensionSpec): string {
		if ('ref' in d) return `ref:${d.ref}`;
		return `col:${d.table ?? chart!.source.table}:${d.col}`;
	}
	function dimFromValue(v: string): DimensionSpec {
		if (v.startsWith('ref:')) return { ref: v.slice(4) };
		const [, table, col] = v.split(':');
		return table === chart!.source.table ? { col } : { col, table };
	}
	function measValue(m: MeasureSpec): string {
		return 'ref' in m ? `ref:${m.ref}` : 'custom';
	}
	function measFromValue(v: string, m: MeasureSpec): MeasureSpec {
		if (v.startsWith('ref:')) return { ref: v.slice(4) };
		const [, table, col] = v.split(':');
		const linkedPick = table !== chart!.source.table;
		return {
			expr: linkedPick ? `sum("${table}"."${col}")` : `sum(${col})`,
			table: linkedPick ? table : undefined,
			label: `sum ${col}`,
			fmt: 'fmt' in m ? m.fmt : undefined
		};
	}

	function removeBlock() {
		doc.rows![ri].columns![ci].blocks.splice(bi, 1);
		onremove?.();
	}
	function addDimension() {
		if (chart) chart.dimensions.push({ col: tableCols[0] ?? '' });
	}
	function addMeasure() {
		if (chart) chart.measures.push({ expr: 'count(*)', label: 'Count' });
	}
	// opening an empty chart (fresh from the skeleton) pre-seeds one picker row per empty role,
	// so the drawer always shows the pickers — same defaults as the + add buttons; once per mount
	onMount(() => {
		if (!chart) return;
		if (chart.dimensions.length === 0) addDimension();
		if (chart.measures.length === 0) addMeasure();
	});
	function addAnnotation() {
		if (chart && def) {
			const mark = def.annotations[0];
			if (mark) chart.annotations = [...(chart.annotations ?? []), { mark, at: 0 } as never];
		}
	}
</script>

<div class="inspector">
	<Section>
		<div class="grid-2">
			<Field label="Title">
				<TextInput
					value={block.type === 'chart' ? (chart!.title ?? '') : block.type === 'table' ? (block.title ?? '') : ''}
					oncommit={(v) => { const val = v || undefined; if (block.type === 'chart') chart!.title = val; else if (block.type === 'table') block.title = val; }}
				/>
			</Field>
			{#if chart}
				<Field label="Subtitle">
					<TextInput value={chart.subtitle ?? ''} oncommit={(v) => (chart.subtitle = v || undefined)} />
				</Field>
			{/if}
		</div>

		{#if block.type === 'text'}
			<Field label="Text">
				<textarea rows="4" class="ctl-textarea" value={block.text} oninput={(e) => (block.text = (e.target as HTMLTextAreaElement).value)}></textarea>
			</Field>
		{:else if block.type === 'table'}
			<Field label="Table">
				<Select value={block.table} onchange={(v) => (block.table = v)}>
					{#each Object.keys(schemas) as t (t)}<option value={t}>{t}</option>{/each}
				</Select>
			</Field>
			<Field label="Limit" hint="Max rows fetched">
				<NumberInput min={1} value={block.limit ?? 50} oncommit={(v) => (block.limit = v)} />
			</Field>
		{:else if chart}
			<Field label="Source table">
				<Select value={chart.source.table} onchange={onTableChange}>
					{#each Object.keys(schemas) as t (t)}<option value={t}>{t}</option>{/each}
				</Select>
			</Field>
		{/if}
	</Section>

	{#if chart}
		<!-- roles: table-first — pick from ⭐ library, this table's fields, or linked tables' fields -->
		<Section title="Dimensions">
			{#snippet action()}
				<Btn variant="ghost" size="sm" onclick={addDimension}><Plus size={12} /> add</Btn>
				{#if onExternalCreate}
					<button class="data-link" title="Create in /data — full editor" onclick={() => onExternalCreate?.('dimension', chart.source.table)}>
						<SquareArrowOutUpRight size={12} /> /data
					</button>
				{/if}
			{/snippet}
			<div class="rows">
				{#each chart.dimensions as d, i (i)}
					<div class="row">
						<div class="grow">
							<Select value={dimValue(d)} onchange={(v) => { if (v === '__new') dimForm.open = true; else chart.dimensions[i] = dimFromValue(v); }}>
								{#each usableItems.filter((it) => it.kind === 'dimension') as it (it.id)}<option value={`ref:${it.id}`}>⭐ {it.label}</option>{/each}
								<optgroup label={chart.source.table}>
									{#each tableCols as c (c)}<option value={`col:${chart.source.table}:${c}`}>{c}</option>{/each}
								</optgroup>
								{#each linked as t (t)}
									<optgroup label={`${t} ⤳ linked`}>
										{#each schemas[t] ?? [] as c (c)}<option value={`col:${t}:${c}`}>{c}</option>{/each}
									</optgroup>
								{/each}
								<option value="__new">✚ Create master dimension…</option>
							</Select>
						</div>
						{#if !('ref' in d)}
							<div class="fixed-w">
								<Select small value={d.grain ?? '(none)'} onchange={(v) => { chart.dimensions[i] = { col: d.col, table: d.table, grain: v === '(none)' ? undefined : (v as never) }; }}>
									{#each GRAINS as g (g)}<option value={g}>{g}</option>{/each}
								</Select>
							</div>
						{/if}
						<RemoveBtn onclick={() => chart.dimensions.splice(i, 1)} title="Remove dimension" />
					</div>
				{/each}
			</div>
			{#if dimForm.open}
				<div class="create-form">
					<p class="create-title">New master dimension on <span class="mono">{chart.source.table}</span></p>
					<div class="grid-2">
						<TextInput placeholder="Label (e.g. Region)" bind:value={dimForm.label} />
					</div>
					<ExprEditor bind:value={dimForm.expr} kind="dimension" table={chart.source.table} columns={tableCols} masterItems={usableItems.filter((it) => it.kind === 'dimension')} placeholder="Field or expression" />
					{#if formError}<p class="form-error">{formError}</p>{/if}
					<div class="form-actions">
						<Btn variant="ghost" size="sm" onclick={() => (dimForm.open = false)}>Cancel</Btn>
						<Btn variant="primary" size="sm" onclick={createDimension}>Save to library</Btn>
					</div>
				</div>
			{/if}
		</Section>

		<Section title="Measures">
			{#snippet action()}
				<Btn variant="ghost" size="sm" onclick={addMeasure}><Plus size={12} /> add</Btn>
				{#if onExternalCreate}
					<button class="data-link" title="Create in /data — full editor" onclick={() => onExternalCreate?.('measure', chart.source.table)}>
						<SquareArrowOutUpRight size={12} /> /data
					</button>
				{/if}
			{/snippet}
			<div class="rows">
				{#each chart.measures as m, i (i)}
					<div class="card-row">
						<div class="row">
							<div class="grow">
								<Select value={measValue(m)} onchange={(v) => { if (v === '__new') measForm.open = true; else if (v !== 'custom') chart.measures[i] = measFromValue(v, m); }}>
									{#each usableItems.filter((it) => it.kind === 'measure') as it (it.id)}<option value={`ref:${it.id}`}>⭐ {it.label}</option>{/each}
									<option value="custom">✎ expression</option>
									<optgroup label={chart.source.table}>
										{#each tableCols as c (c)}<option value={`field:${chart.source.table}:${c}`}>sum({c})</option>{/each}
									</optgroup>
									{#each linked as t (t)}
										<optgroup label={`${t} ⤳ linked`}>
											{#each schemas[t] ?? [] as c (c)}<option value={`field:${t}:${c}`}>sum({c})</option>{/each}
										</optgroup>
									{/each}
									<option value="__new">✚ Create master measure…</option>
								</Select>
							</div>
							<RemoveBtn onclick={() => chart.measures.splice(i, 1)} title="Remove measure" />
						</div>
						{#if !('ref' in m)}
							<TextInput mono placeholder="expression" value={m.expr} oncommit={(v) => (chart.measures[i] = { ...m, expr: v })} />
						{/if}
						<div class="grid-2">
							<TextInput placeholder="Label" value={'label' in m ? (m.label ?? '') : ''} oncommit={(v) => (chart.measures[i] = { ...m, label: v || undefined })} />
							<Select small value={'fmt' in m ? m.fmt ?? '(none)' : '(none)'} onchange={(v) => { chart.measures[i] = { ...m, fmt: v === '(none)' ? undefined : v } as never; }}>
								{#each FMTS as f (f)}<option value={f}>{f}</option>{/each}
							</Select>
						</div>
					</div>
				{/each}
			</div>
			{#if measForm.open}
				<div class="create-form">
					<p class="create-title">New master measure on <span class="mono">{chart.source.table}</span></p>
					<div class="grid-2">
						<TextInput placeholder="Label (e.g. Revenue)" bind:value={measForm.label} />
					</div>
					<ExprEditor bind:value={measForm.expr} kind="measure" table={chart.source.table} columns={tableCols} masterItems={usableItems.filter((it) => it.kind === 'measure')} placeholder="Expression (e.g. sum(amount))" />
					<div class="fixed-w">
						<Select small bind:value={measForm.fmt}>
							{#each FMTS as f (f)}<option value={f}>{f}</option>{/each}
						</Select>
					</div>
					{#if formError}<p class="form-error">{formError}</p>{/if}
					<div class="form-actions">
						<Btn variant="ghost" size="sm" onclick={() => (measForm.open = false)}>Cancel</Btn>
						<Btn variant="primary" size="sm" onclick={createMeasure}>Save to library</Btn>
					</div>
				</div>
			{/if}
		</Section>

		<!-- schema-driven options (FR-9/Q9) -->
		{#if def}
			<Section title="Options">
				{#each def.optionsSchema as field (field.name)}
					<Field inline label={field.label}>
						{#if field.kind === 'enum'}
							<div class="fixed-w-wide">
								<Select small value={String(chart.options?.[field.name] ?? field.default ?? '')} onchange={(v) => { chart.options = { ...chart.options, [field.name]: v }; }}>
									{#each field.options ?? [] as o (o)}<option value={o}>{o}</option>{/each}
								</Select>
							</div>
						{:else if field.kind === 'boolean'}
							<Toggle checked={Boolean(chart.options?.[field.name] ?? field.default)} onchange={(v) => { chart.options = { ...chart.options, [field.name]: v }; }} />
						{:else}
							<div class="fixed-w-wide">
								<NumberInput value={field.kind === 'number' ? (chart.options?.[field.name] as number | undefined) ?? undefined : undefined} oncommit={(v) => { chart.options = { ...chart.options, [field.name]: v as never }; }} />
							</div>
						{/if}
					</Field>
				{/each}
			</Section>
		{/if}

		<!-- annotations (per-type whitelist) -->
		{#if def && def.annotations.length}
			<Section title="Annotations">
				{#snippet action()}
					<Btn variant="ghost" size="sm" onclick={addAnnotation}><Plus size={12} /> add</Btn>
				{/snippet}
				<div class="rows">
					{#each chart.annotations ?? [] as a, i (i)}
						<div class="row">
							<div class="fixed-w">
								<Select small value={a.mark} onchange={(v) => { chart.annotations![i] = { ...a, mark: v } as never; }}>
									{#each def.annotations as mk (mk)}<option value={mk}>{mk}</option>{/each}
								</Select>
							</div>
							{#if typeof a.at === 'number'}
								<div class="grow">
									<NumberInput value={a.at} oncommit={(v) => { chart.annotations![i] = { ...a, at: v ?? 0 } as never; }} />
								</div>
							{:else}
								<div class="grow">
									<TextInput mono placeholder="expression" value={a.at.expr} oncommit={(v) => { chart.annotations![i] = { ...a, at: { expr: v } } as never; }} />
								</div>
							{/if}
							<RemoveBtn onclick={() => chart.annotations!.splice(i, 1)} title="Remove annotation" />
						</div>
					{/each}
				</div>
			</Section>
		{/if}

		<Section title="Presentation">
			<Field label="Tooltip template" hint="&#123;dimension&#125; / &#123;measure&#125; placeholders">
				<TextInput mono placeholder="&#123;dimension&#125;: &#123;measure&#125;" value={chart.tooltip?.template ?? ''} oncommit={(v) => { chart.tooltip = v ? { ...chart.tooltip, template: v } : undefined; }} />
			</Field>
			<div class="grid-2">
				<Field label="Height" hint="vh fraction · blank = fit">
					<NumberInput min={0.1} max={1} step={0.05} placeholder="fit column" value={chart.heightVh ?? undefined} oncommit={(v) => (chart.heightVh = v)} />
				</Field>
				<Field label="Limit" hint="Max rows fetched">
					<NumberInput min={1} placeholder="default" value={chart.limit ?? undefined} oncommit={(v) => (chart.limit = v)} />
				</Field>
			</div>
		</Section>
	{/if}
</div>

<style>
	.inspector {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		font-size: var(--text-sm);
	}

	.grid-2 {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-2);
		min-width: 0;
	}

	.rows {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.row {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		min-width: 0;
	}

	.grow {
		flex: 1;
		min-width: 0;
	}

	.fixed-w {
		width: 108px;
		flex-shrink: 0;
	}

	.fixed-w-wide {
		width: 150px;
		flex-shrink: 0;
	}

	.card-row {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-2);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface);
	}

	.create-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-3);
		border: 1px dashed var(--color-border-strong);
		border-radius: var(--radius-sm);
		background: var(--color-surface-sunken);
	}

	.create-title {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}

	.mono {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.form-error {
		font-size: var(--text-xs);
		color: var(--color-danger);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: var(--space-2);
	}

	/* "create in /data" link next to the section add-button */
	.data-link {
		display: inline-flex;
		align-items: center;
		gap: 3px;
		height: 24px;
		padding: 0 var(--space-2);
		border: 1px dashed var(--color-border);
		border-radius: var(--radius-sm);
		background: none;
		color: var(--color-text-tertiary);
		font-size: var(--text-xs);
		cursor: pointer;
		transition:
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}
	.data-link:hover {
		color: var(--color-text);
		border-color: var(--color-text-tertiary);
	}

	:global(.ctl-textarea) {
		font-family: var(--font-body);
	}
</style>
