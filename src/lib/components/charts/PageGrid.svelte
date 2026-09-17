<script lang="ts">
	import type { Block, PageDoc, PageRow } from '$lib/charts/spec-types';
	import { rowColumns } from '$lib/charts/spec-types';
	import type { createPageRuntime } from '$lib/charts/page-runtime.svelte';
	import { chartRenderers, setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getChartType } from '$lib/charts/registry';
	import TableRenderer from './renderers/TableRenderer.svelte';
	import SkeletonSetup from './SkeletonSetup.svelte';
	import type { MasterItem } from '$lib/charts/items';
	import type { Relationship } from '$lib/charts/relationships';
	import type { TableSchemas } from '$lib/charts/query/compile';
	import ChartCard from './ChartCard.svelte';
	import { Bolt, Settings2, Plus, GripHorizontal, GripVertical } from 'lucide-svelte';

	setupChartRegistry();

	let {
		doc,
		runtime,
		configureId = null,
		onConfigure,
		onConfigureRow,
		onConfigureColumn,
		schemas,
		items,
		relationships,
		onCreateMasterItem,
		onAdd,
		onAddRow
	}: {
		doc: PageDoc;
		runtime: ReturnType<typeof createPageRuntime>;
		/** focused config mode: render only this block, full width (editor supplies the 50vw drawer) */
		configureId?: string | null;
		onConfigure?: (id: string) => void;
		onConfigureRow?: (ri: number) => void;
		onConfigureColumn?: (ri: number, ci: number) => void;
		/** data context for the in-chart skeleton setup (optional — falls back to drawer-opening buttons) */
		schemas?: TableSchemas;
		items?: MasterItem[];
		relationships?: Relationship[];
		/** host-side master-item creation for the in-chart ✚ flows; resolves to the new item id */
		onCreateMasterItem?: (kind: 'dimension' | 'measure', table: string, label: string, expr: string) => Promise<string>;
		/** + Component clicked — host opens the component picker; ci = first empty column (or 0 to spawn a new one) */
		onAdd?: (ri: number, ci: number) => void;
		/** + Row clicked — host appends a row (and autosaves) */
		onAddRow?: () => void;
	} = $props();

	function stateFor(id: string) {
		return runtime.states[id];
	}

	/** drag the row's bottom grip to set its height (px) */
	function startRowDrag(e: PointerEvent, row: PageRow) {
		const grip = e.currentTarget as HTMLElement;
		const shell = grip.closest('.row-shell') as HTMLElement | null;
		if (!shell) return;
		const startY = e.clientY;
		const startH = row.height ?? shell.offsetHeight;
		grip.setPointerCapture(e.pointerId);
		document.body.style.cursor = 'row-resize';
		document.body.style.userSelect = 'none';
		const move = (ev: PointerEvent) => {
			// drag down = taller (same direction the border moves)
			row.height = Math.max(80, Math.round(startH + (ev.clientY - startY)));
		};
		const up = () => {
			grip.removeEventListener('pointermove', move);
			grip.removeEventListener('pointerup', up);
			grip.removeEventListener('pointercancel', up);
			document.body.style.cursor = '';
			document.body.style.userSelect = '';
		};
		grip.addEventListener('pointermove', move);
		grip.addEventListener('pointerup', up);
		grip.addEventListener('pointercancel', up);
	}

	const clampSpan = (n: number) => Math.min(12, Math.max(1, n));

	/** drag the column's grip sideways to resize — takes/gives width from the next sibling */
	function startColDrag(e: PointerEvent, ri: number, ci: number) {
		const grip = e.currentTarget as HTMLElement;
		const grid = grip.closest('.grid') as HTMLElement | null;
		if (!grid) return;
		const cols = doc.rows![ri].columns;
		if (!cols) return;
		const rect = grid.getBoundingClientRect();
		const unit = rect.width / 12;
		grip.setPointerCapture(e.pointerId);
		document.body.style.cursor = 'col-resize';
		document.body.style.userSelect = 'none';
		const move = (ev: PointerEvent) => {
			// right edge of the column follows the pointer: span = pointer position in 12ths
			let target = clampSpan(Math.round((ev.clientX - rect.left) / unit));
			const cur = cols[ci].span ?? 12;
			if (target === cur) return;
			const sib = cols[ci + 1];
			if (sib) {
				const sibSpan = sib.span ?? 12;
				const room = cur + sibSpan - 1; // keep the sibling ≥ 1 unit
				if (target > room) target = room;
				sib.span = cur + sibSpan - target;
			} else {
				const others = cols.reduce((n, c, i) => (i === ci ? n : n + (c.span ?? 12)), 0);
				target = Math.min(target, Math.max(1, 12 - others));
			}
			cols[ci].span = target;
		};
		const up = () => {
			grip.removeEventListener('pointermove', move);
			grip.removeEventListener('pointerup', up);
			grip.removeEventListener('pointercancel', up);
			document.body.style.cursor = '';
			document.body.style.userSelect = '';
		};
		grip.addEventListener('pointermove', move);
		grip.addEventListener('pointerup', up);
		grip.addEventListener('pointercancel', up);
	}
</script>

{#snippet renderBlock(block: Block, id: string, fillPx = 0)}
	{@const state = stateFor(id)}
	{#if block.type === 'text'}
		<div class="bg-white rounded-lg border border-zinc-200 p-6 text-sm text-zinc-700 whitespace-pre-wrap">
			{block.text}
		</div>
	{:else if block.type === 'table'}
		<TableRenderer
			rows={state?.rows ?? []}
			columns={block.columns ?? []}
			title={block.title ?? block.table}
			status={state?.error ? 'error' : state?.loading ? 'loading' : 'ok'}
			error={state?.error ?? ''}
		/>
	{:else if block.type === 'chart'}
		{@const def = getChartType(block.chart.type)}
		{@const Renderer = chartRenderers[block.chart.type]}
		{@const sel = runtime.selection && runtime.selection.blockId === id ? runtime.selection : null}
		{#if Renderer && state && !state.missing.length && !state.loading && !state.error && state.dimensionAliases.length > 0}
			<Renderer
				rows={state.rows}
				status={state.loading ? 'loading' : state.error ? 'error' : undefined}
				error={state.error}
				dimensionAliases={state.dimensionAliases}
				measureAliases={state.measureAliases}
				options={state.options}
				annotations={state.annotations}
				title={block.chart.title ?? def?.label ?? block.chart.type}
				subtitle={block.chart.subtitle ?? ''}
				tooltip={block.chart.tooltip}
				selected={sel ? { dimension: sel.dimension, value: sel.value } : null}
				onSelect={(s) => runtime.select(id, s)}
				colorScale={runtime.colorScale}
				fmts={runtime.fmtsFor(state)}
				heightVh={block.chart.heightVh ?? 0.3}
				heightPx={block.chart.heightVh === undefined ? fillPx : 0}
			/>
		{:else if state?.unconfigured}
			<!-- setup skeleton: no data until every role minimum is met -->
			<ChartCard title={block.chart.title ?? def?.label ?? block.chart.type} subtitle={block.chart.subtitle ?? ''} status="setup">
				{#if schemas && items && relationships}
					<SkeletonSetup chart={block.chart} {schemas} {items} {relationships} {onCreateMasterItem} />
				{:else}
				<div class="flex flex-col items-center justify-center gap-3 py-6 px-4 rounded-lg border border-dashed border-zinc-300 bg-zinc-50/60">
					<div class="flex flex-col gap-1.5 w-full max-w-56" aria-hidden="true">
						<div class="h-2.5 rounded bg-zinc-200 animate-pulse w-1/3"></div>
						<div class="h-8 rounded bg-zinc-200/70 animate-pulse"></div>
						<div class="h-8 rounded bg-zinc-200/50 animate-pulse"></div>
					</div>
					<p class="text-xs text-zinc-500">Pick a table, then add dimensions and measures.</p>
					<div class="flex gap-2">
						<button class="px-3 py-1.5 rounded-md text-xs font-medium bg-zinc-900 text-white hover:bg-zinc-700 inline-flex items-center gap-1" onclick={(e) => { e.stopPropagation(); onConfigure?.(id); }}>
							<Plus size={12} /> Add dimension
						</button>
						<button class="px-3 py-1.5 rounded-md text-xs font-medium bg-white border border-zinc-300 text-zinc-700 hover:bg-zinc-100 inline-flex items-center gap-1" onclick={(e) => { e.stopPropagation(); onConfigure?.(id); }}>
							<Plus size={12} /> Add measure
						</button>
					</div>
					</div>
				{/if}
			</ChartCard>
		{:else}
			<ChartCard
				title={block.chart.title ?? block.chart.type}
				subtitle={block.chart.subtitle ?? ''}
				status={state?.missing.length ? 'missing' : state?.loading ? 'loading' : state?.error ? 'error' : 'empty'}
				missing={state?.missing.join(', ') ?? ''}
				error={state?.error ?? ''}
			/>
		{/if}
	{/if}
{/snippet}

{#if configureId !== null}
	<!-- focused config mode: only the configured block, full width of the left half -->
	{#each doc.rows ?? [] as row, ri}
		{#each rowColumns(row) as col, ci}
			{#each col.blocks as block, bi}
				{#if `r${ri}-c${ci}-b${bi}` === configureId}
					{@render renderBlock(block, configureId)}
				{/if}
			{/each}
		{/each}
	{/each}
{:else}
	<div class="space-y-5 pb-16 pt-2">
		{#if !(doc.rows ?? []).length}
			<!-- empty page: the same + Row button as under existing rows -->
			<button class="add-row-btn" onclick={() => onAddRow?.()}>
				<Plus size={12} /> Row
			</button>
		{/if}
		{#each doc.rows ?? [] as row, ri}
			<!-- row silhouette: thin gray border, always visible -->
			<section class="row-shell">
				<!-- top-left border cluster: [Row settings] [+ Component] — same badge style -->
				<div class="row-edge-cluster">
					<button class="edge-btn" onclick={() => onConfigureRow?.(ri)} title="Row settings">
						<Settings2 size={11} /> Row {ri + 1}
					</button>
					<button class="edge-btn" onclick={() => onAdd?.(ri, Math.max(0, rowColumns(row).findIndex((c) => c.blocks.length === 0)))} title="Add a component to this row">
						<Plus size={11} /> Component
					</button>
				</div>
				<!-- row height grip, bottom-center on the border -->
				<div
					class="grip grip-row"
					role="separator"
					aria-label="Drag to adjust row height"
					tabindex="-1"
					onpointerdown={(e) => startRowDrag(e, row)}
				>
					<GripHorizontal size={13} />
				</div>
				<div class="grid grid-cols-12 gap-3">
					{#each rowColumns(row) as col, ci}
						{@const colH = col.height ?? row.height}
						<div class="col-shell" style={`grid-column: span ${col.span ?? 12} / span ${col.span ?? 12};${colH !== undefined ? ` height: ${colH}px;` : ''}`}>
							<button
							class="edge-btn edge-btn-col"
							onclick={() => {
								// single-column row: the column IS the row — one shared drawer (both labels open it)
								if (rowColumns(row).length === 1) onConfigureRow?.(ri);
								else onConfigureColumn?.(ri, ci);
							}}
							title={rowColumns(row).length === 1 ? 'Row settings' : 'Column settings'}
						>
								<Settings2 size={11} />
							</button>
							<!-- column width grip, bottom-right of the column -->
							<div
								class="grip grip-col"
								role="separator"
								aria-label="Drag to resize column"
								tabindex="-1"
								onpointerdown={(e) => startColDrag(e, ri, ci)}
							>
								<GripVertical size={13} />
							</div>
							<div class="col-inner" class:col-empty={col.blocks.length === 0} style={colH !== undefined ? 'height: 100%; overflow: auto;' : ''}>
								<div class="space-y-3">
									{#each col.blocks as block, bi}
										{@const id = `r${ri}-c${ci}-b${bi}`}
										<div class="relative">
											<button
												class="config-open-btn"
												onclick={(e) => { e.stopPropagation(); onConfigure?.(id); }}
												title="Configure"
												aria-label="Configure block"
											>
												<Bolt size={14} />
											</button>
											<!-- fill the fixed column height unless the block sets its own heightVh (~110px card chrome) -->
											{@render renderBlock(block, id, colH !== undefined ? Math.max(120, colH - 110) : 0)}
										</div>
									{/each}
								</div>
							</div>
						</div>
					{/each}
				</div>
			</section>
			<!-- + Row, top-left just below this row -->
			<button class="add-row-btn" onclick={() => onAddRow?.()}>
				<Plus size={12} /> Row
			</button>
		{/each}
	</div>
{/if}

<style>
	/* row silhouette — thin gray border so an empty row is still visible */
	.row-shell {
		position: relative;
		border: 1px solid #e4e4e7;
		border-radius: var(--radius-md, 8px);
		padding: 16px 12px 12px;
		background: transparent;
	}

	/* empty column: dotted grid inside the silhouette + room to see it */
	.col-inner {
		display: flex;
		flex-direction: column;
		min-height: 36px;
	}
	.col-inner > :first-child {
		flex: 1;
	}
	.col-empty {
		min-height: 140px;
		border-radius: var(--radius-sm, 6px);
		background-image: radial-gradient(circle, #d4d4d8 1.2px, transparent 1.2px);
		background-size: 16px 16px;
	}

	/* config badges floating on the row border (row left, column right) */
	.edge-btn {
		position: absolute;
		z-index: 20;
		display: inline-flex;
		align-items: center;
		gap: 4px;
		height: 20px;
		padding: 0 8px 0 6px;
		border: 1px solid #e4e4e7;
		border-radius: 999px;
		background: white;
		color: #a1a1aa;
		font-size: 10px;
		font-weight: 500;
		cursor: pointer;
		transition: color var(--duration-fast, 150ms) ease, border-color var(--duration-fast, 150ms) ease;
	}
	.edge-btn:hover {
		color: var(--color-text, #18181b);
		border-color: #d4d4d8;
	}
	/* top-left border cluster: [Row settings] [+ Component] */
	.row-edge-cluster {
		position: absolute;
		top: -11px;
		left: 10px;
		display: flex;
		gap: 4px;
		z-index: 20;
	}
	/* edge-btn is absolute by default (corner badges) — inside the cluster they flow */
	.row-edge-cluster .edge-btn {
		position: static;
	}
	/* + Row button under each row, top-left */
	.add-row-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 3px 10px;
		margin-left: 2px;
		border: 1px dashed #d4d4d8;
		border-radius: var(--radius-xs, 6px);
		background: transparent;
		color: #a1a1aa;
		font-size: 12px;
		cursor: pointer;
		transition: color var(--duration-fast, 150ms) ease, border-color var(--duration-fast, 150ms) ease;
	}
	.add-row-btn:hover {
		color: #52525b;
		border-color: #a1a1aa;
	}

	.edge-btn-col {
		top: -11px;
		right: 4px;
		padding: 0 5px;
	}

	/* drag grips: row height (bottom-center), column width (bottom-right) */
	.grip {
		position: absolute;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		height: 20px;
		border: 1px solid #e4e4e7;
		border-radius: 999px;
		background: white;
		color: #a1a1aa;
		z-index: 20;
		touch-action: none;
		user-select: none;
		transition: color var(--duration-fast, 150ms) ease, border-color var(--duration-fast, 150ms) ease;
	}
	.grip:hover {
		color: var(--color-text, #18181b);
		border-color: #d4d4d8;
	}
	.grip-row {
		bottom: -11px;
		left: 50%;
		transform: translateX(-50%);
		width: 28px;
		cursor: row-resize;
	}
	.grip-col {
		bottom: 4px;
		right: 4px;
		width: 20px;
		cursor: col-resize;
		opacity: 0;
	}
	.col-shell:hover .grip-col {
		opacity: 1;
	}



	.config-open-btn {
		position: absolute;
		top: var(--space-3, 12px);
		right: var(--space-3, 12px);
		/* above the svelteplot overlay svgs, which otherwise swallow pointer hits
		   at the card's top-right (elementFromPoint returned the plot svg) */
		z-index: 30;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1, 4px);
		border: 1px solid transparent;
		background: white;
		color: #a1a1aa;
		cursor: pointer;
		border-radius: var(--radius-xs, 4px);
		transition:
			color var(--duration-fast, 150ms) ease,
			border-color var(--duration-fast, 150ms) ease;
	}

	.config-open-btn:hover {
		color: var(--color-text, #18181b);
		border-color: #d4d4d8;
	}
</style>
