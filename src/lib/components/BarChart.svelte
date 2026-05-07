<script lang="ts">
	import { PanelLeftClose, PanelRightClose } from 'lucide-svelte';

	let {
		data = [],
		labelKey = '',
		valueKey = '',
		title = '',
		tagLabel = '',
		selected = $bindable(new Set<string>()),
		onselect = (label: string) => {},
		onaction = () => {},
		fill = false,
		drawerOpen = false
	}: {
		data: Record<string, unknown>[];
		labelKey: string;
		valueKey: string;
		title?: string;
		tagLabel?: string;
		selected?: Set<string>;
		onselect?: (label: string) => void;
		onaction?: () => void;
		fill?: boolean;
		drawerOpen?: boolean;
	} = $props();

	let maxVal = $derived(Math.max(...data.map((d) => Number(d[valueKey]) || 0), 1));

	let longestValue = $derived(
		data.reduce((max: number, row: Record<string, unknown>) => {
			const s = fmt(Number(row[valueKey]) || 0);
			return s.length > max ? s.length : max;
		}, 0)
	);

	function fmt(n: number): string {
		if (n >= 1_000_000) return (n / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M';
		if (n >= 1_000) return (n / 1_000).toFixed(1).replace(/\.0$/, '') + 'K';
		return n.toFixed(0);
	}

	let pct = (row: Record<string, unknown>) => {
		const v = Number(row[valueKey]) || 0;
		return maxVal > 0 ? (v / maxVal) * 100 : 0;
	};

	function handleClick(label: string) {
		const next = new Set(selected);
		if (next.has(label)) {
			next.delete(label);
		} else {
			next.add(label);
		}
		selected = next;
		onselect(label);
	}

	let hasSelection = $derived(selected.size > 0);
</script>

{#if data.length > 0}
	<article class="chart-card" class:chart-fill={fill}>
		<div class="chart-header">
			<div class="chart-header-left">
				{#if title}
					<h3 class="chart-title">{title}</h3>
				{/if}
				{#if tagLabel}
					<span class="chart-tag">{tagLabel}</span>
				{/if}
			</div>
			<button class="chart-action" class:chart-action-active={drawerOpen} onclick={onaction} aria-label={drawerOpen ? 'Close drawer' : 'Open export drawer'}>
				{#if drawerOpen}
					<PanelRightClose size={18} />
				{:else}
					<PanelLeftClose size={18} />
				{/if}
			</button>
		</div>

		<hr class="chart-divider" />

		<div class="chart-body">
			{#each data as row}
				{@const label = String(row[labelKey] ?? '')}
				{@const isActive = hasSelection && selected.has(label)}
				{@const isDimmed = hasSelection && !selected.has(label)}
				<button
					class="chart-row"
					class:active={isActive}
					class:dimmed={isDimmed}
					onclick={() => handleClick(label)}
				>
					<span class="chart-label">{label}</span>
					<div class="chart-track">
						<div
							class="chart-bar"
							class:bar-active={isActive}
							class:bar-dimmed={isDimmed}
							style="width:calc({pct(row)}% - 10px)"
						></div>
					</div>
					<span class="chart-value" style="width:{Math.min(longestValue, 20)}ch">{fmt(Number(row[valueKey]) || 0)}</span>
				</button>
			{/each}
		</div>
	</article>
{/if}

<style>
	.chart-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		display: flex;
		flex-direction: column;
		height: 460px;
	}

	.chart-fill {
		height: 100%;
		border-radius: 0;
		border: none;
	}

	.chart-header {
		padding: var(--space-3) var(--space-4);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		flex-shrink: 0;
	}

	.chart-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		min-width: 0;
	}

	.chart-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin: 0;
	}

	.chart-tag {
		display: inline-flex;
		align-items: center;
		padding: 2px var(--space-2);
		border: 1px solid var(--color-border);
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		line-height: 1.5;
		border-radius: var(--radius-xs);
		background: var(--color-surface-sunken);
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.chart-action {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		flex-shrink: 0;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.chart-action:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.chart-action-active {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.chart-action:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.chart-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
		margin: 0;
		flex-shrink: 0;
	}

	.chart-body {
		flex: 1;
		min-height: 0;
		padding: var(--space-2) var(--space-4) var(--space-3);
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.chart-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		min-height: 20px;
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		text-align: left;
		width: 100%;
		font: inherit;
		color: inherit;
		outline: none;
	}

	.chart-row:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
		border-radius: 2px;
	}

	.chart-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		width: 15ch;
		max-width: 15ch;
		flex-shrink: 0;
		text-align: right;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		letter-spacing: -0.01em;
	}

	.chart-track {
		flex: 1;
		min-width: 0;
		height: 14px;
		background: var(--color-surface-raised);
		border-radius: 2px;
		overflow: hidden;
	}

	.chart-bar {
		height: 100%;
		background: var(--color-accent);
		border-radius: 2px;
		transition: width 0.4s var(--ease-out-expo), background 0.15s ease;
		min-width: 2px;
	}

	.bar-active {
		background: #3b82f6;
	}

	.bar-dimmed {
		background: #ccc;
	}

	.chart-value {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		flex-shrink: 0;
		max-width: 20ch;
		text-align: right;
		letter-spacing: 0.02em;
	}
</style>
