<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { FieldFunction } from '$lib/db-operations';
	import { getFunctionsForType } from '$lib/field-functions/library';

	let {
		columnName = '',
		columnType = '',
		fieldFunctions = [] as FieldFunction[],
		activeFunctions = [] as string[],
		onclose,
		onapply
	}: {
		columnName: string;
		columnType: string;
		fieldFunctions: FieldFunction[];
		activeFunctions: string[];
		onclose: () => void;
		onapply: (functionIds: string[]) => void;
	} = $props();

	let drawerOpen = $state(false);
	let selected = $state<Set<string>>(new Set());

	$effect(() => {
		if (columnName) {
			drawerOpen = true;
			selected = new Set(activeFunctions);
		}
	});

	function handleClose() {
		drawerOpen = false;
		setTimeout(() => onclose(), 200);
	}

	function toggle(fnId: string) {
		const next = new Set(selected);
		if (next.has(fnId)) {
			next.delete(fnId);
		} else {
			next.add(fnId);
		}
		selected = next;
		onapply([...next]);
	}

	const available = $derived(getFunctionsForType(fieldFunctions, columnType));

	function typeColor(type: string): string {
		const t = type.toLowerCase();
		if (t.includes('int')) return 'int';
		if (t.includes('float') || t.includes('double') || t.includes('decimal') || t.includes('numeric') || t.includes('real')) return 'float';
		if (t.includes('bool')) return 'bool';
		if (t.includes('date') || t.includes('time') || t.includes('timestamp')) return 'time';
		if (t.includes('blob') || t.includes('byte') || t.includes('binary')) return 'binary';
		return 'str';
	}
</script>

{#if columnName}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-overlay" class:drawer-overlay-visible={drawerOpen} onclick={handleClose} onkeydown={() => {}}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="drawer" class:drawer-open={drawerOpen} onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
			<div class="drawer-header">
				<h2 class="drawer-title">Add function</h2>
				<button class="drawer-close" onclick={handleClose} title="Close">
					<X size={16} />
				</button>
			</div>

			<div class="drawer-body">
				<div class="drawer-col-info">
					<span class="col-info-name">{columnName}</span>
					<span class="col-type col-type-{typeColor(columnType)}">{columnType}</span>
				</div>

				{#if available.length === 0}
					<div class="drawer-empty">
						<span>No functions available for {columnType}</span>
					</div>
				{:else}
					<div class="fn-list">
						{#each available as fn}
							{@const isActive = selected.has(fn.id)}
							<button
								class="fn-item"
								class:fn-item-active={isActive}
								onclick={() => toggle(fn.id)}
							>
								<div class="fn-check">
									{#if isActive}
										<span class="fn-check-mark">&check;</span>
									{/if}
								</div>
								<div class="fn-info">
									<span class="fn-label">{fn.label}</span>
									{#if fn.description}
										<span class="fn-desc">{fn.description}</span>
									{/if}
								</div>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
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
		width: 30vw;
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
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.01em;
	}

	.drawer-close {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: none;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.drawer-close:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.drawer-col-info {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3);
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
	}

	.col-info-name {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		word-break: break-all;
	}

	.col-type {
		font-size: 9px;
		font-weight: 600;
		padding: 1px var(--space-1);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		flex-shrink: 0;
		margin-left: auto;
	}

	.col-type-str {
		background: oklch(0.93 0.02 250);
		color: oklch(0.35 0.04 250);
	}

	.col-type-int {
		background: oklch(0.93 0.06 155);
		color: oklch(0.30 0.08 155);
	}

	.col-type-float {
		background: oklch(0.93 0.06 200);
		color: oklch(0.30 0.08 200);
	}

	.col-type-bool {
		background: oklch(0.93 0.04 310);
		color: oklch(0.35 0.08 310);
	}

	.col-type-time {
		background: oklch(0.93 0.06 60);
		color: oklch(0.30 0.08 60);
	}

	.col-type-binary {
		background: oklch(0.93 0.02 30);
		color: oklch(0.35 0.03 30);
	}

	.drawer-empty {
		padding: var(--space-8) 0;
		text-align: center;
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.fn-list {
		display: flex;
		flex-direction: column;
		gap: 1px;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface-raised);
		overflow: hidden;
	}

	.fn-item {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-3);
		border: none;
		background: none;
		width: 100%;
		cursor: pointer;
		font-family: inherit;
		text-align: left;
		transition: background var(--duration-fast) ease;
	}

	.fn-item:not(:last-child) {
		border-bottom: 1px solid var(--color-border);
	}

	.fn-item:hover {
		background: var(--color-surface-sunken);
	}

	.fn-item-active {
		background: var(--color-accent-muted);
	}

	.fn-item-active:hover {
		background: oklch(0.88 0.03 250);
	}

	.fn-check {
		width: 16px;
		height: 16px;
		border: 1px solid var(--color-border-strong);
		border-radius: var(--radius-xs);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		margin-top: 1px;
		background: var(--color-surface);
	}

	.fn-item-active .fn-check {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}

	.fn-check-mark {
		font-size: 10px;
		color: white;
		line-height: 1;
	}

	.fn-info {
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.fn-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text);
		font-weight: 600;
	}

	.fn-item-active .fn-label {
		color: var(--color-accent-dark);
	}

	.fn-desc {
		font-family: var(--font-body);
		font-size: 9px;
		color: var(--color-text-tertiary);
	}
</style>
