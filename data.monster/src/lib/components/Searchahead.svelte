<script lang="ts">
	import { Search, X } from "lucide-svelte";

	type IconComponent = typeof Search;
	type Option = { icon?: IconComponent; text: string; meta?: string; value?: string; disabled?: boolean; indent?: number };

	let {
		placeholder = "Search...",
		options = [],
		maxWidth = "420px",
		noFilter = false,
		loading = false,
		hint = "",
		initialSelected = [],
		resetKey = 0,
		onselect,
		onremove,
		onquery,
	}: {
		placeholder?: string;
		options?: Option[];
		maxWidth?: string;
		noFilter?: boolean;
		loading?: boolean;
		hint?: string;
		initialSelected?: Option[];
		resetKey?: number;
		onselect?: (item: Option) => void;
		onremove?: (item: Option) => void;
		onquery?: (query: string) => void;
	} = $props();

	let query = $state("");
	let isOpen = $state(false);
	let highlightedIndex = $state(-1);
	let selected = $state<Option[]>([...initialSelected]);
	let inputWrapEl: HTMLElement | undefined = $state();
	let popoverEl: HTMLElement | undefined = $state();

	let showHint = $derived(noFilter && !query.trim() && options.length === 0 && hint);
	let filtered = $derived(
		noFilter
			? options
			: query.trim()
				? options.filter((o) =>
						o.text.toLowerCase().includes(query.toLowerCase()),
					)
				: options,
	);

	$effect(() => {
		onquery?.(query);
	});

	$effect(() => {
		resetKey;
		query = "";
		selected = [];
		isOpen = false;
		highlightedIndex = -1;
	});

	$effect(() => {
		if (!popoverEl) return;
		if (isOpen && (filtered.length > 0 || loading || showHint)) {
			popoverEl.showPopover();
			positionPopover();
		} else {
			try { popoverEl.hidePopover(); } catch {}
		}
	});

	function positionPopover() {
		if (!inputWrapEl || !popoverEl) return;
		const rect = inputWrapEl.getBoundingClientRect();
		popoverEl.style.top = `${rect.bottom + 2}px`;
		popoverEl.style.left = `${rect.left}px`;
		popoverEl.style.width = `${rect.width}px`;
	}

	function clearQuery() {
		query = "";
		isOpen = false;
	}

	function selectItem(item: Option) {
		if (item.disabled) return;
		if (selected.some((s) => s.text === item.text)) return;
		selected = [...selected, item];
		onselect?.(item);
		query = "";
		isOpen = false;
		highlightedIndex = -1;
	}

	function removeSelected(index: number) {
		const item = selected[index];
		selected = selected.filter((_, i) => i !== index);
		if (item) onremove?.(item);
	}

	function clearAll() {
		const removed = [...selected];
		selected = [];
		removed.forEach((item) => onremove?.(item));
	}

	function handleInputKeydown(e: KeyboardEvent) {
		if (e.key === "ArrowDown") {
			e.preventDefault();
			isOpen = true;
			highlightedIndex = Math.min(highlightedIndex + 1, filtered.length - 1);
		} else if (e.key === "ArrowUp") {
			e.preventDefault();
			highlightedIndex = Math.max(highlightedIndex - 1, 0);
		} else if (e.key === "Enter" && highlightedIndex >= 0) {
			e.preventDefault();
			selectItem(filtered[highlightedIndex]);
		} else if (e.key === "Backspace" && query === "" && selected.length) {
			removeSelected(selected.length - 1);
		} else if (e.key === "Escape") {
			isOpen = false;
		}
	}
</script>

<svelte:window onscroll={() => isOpen && positionPopover()} onresize={() => isOpen && positionPopover()} />

<div class="searchahead" style="max-width: {maxWidth}">
	<div class="searchahead-wrap">
		<div class="searchahead-input-wrap" bind:this={inputWrapEl} class:has-dropdown={isOpen}>
			<span class="searchahead-search-icon"><Search size={14} /></span>
			<input
				class="searchahead-input"
				type="text"
				{placeholder}
				autocomplete="off"
				aria-label="Search and select items"
				aria-expanded={isOpen}
				role="combobox"
				bind:value={query}
				onfocus={() => (isOpen = true)}
				onblur={() => setTimeout(() => (isOpen = false), 150)}
				onkeydown={handleInputKeydown}
			/>
			<button
				class="searchahead-clear"
				class:visible={query.length > 0}
				aria-label="Clear search"
				type="button"
				onclick={clearQuery}
			>
				<X size={12} />
			</button>
		</div>
		<div class="searchahead-tags">
			{#if selected.length > 1}
				<button class="searchahead-tag searchahead-tag-clear" onclick={clearAll} aria-label="Clear all filters">
					<X size={10} />
					Wissen
				</button>
			{/if}
			{#if selected.length > 0}
				{#each selected as item, i}
					<span class="searchahead-tag">
						{item.text}
						<button class="searchahead-tag-x" onclick={() => removeSelected(i)} aria-label="Remove {item.text}">
							<X size={10} />
						</button>
					</span>
				{/each}
			{:else}
				<span class="searchahead-skeleton"></span>
				<span class="searchahead-skeleton searchahead-skeleton-sm"></span>
				<span class="searchahead-skeleton searchahead-skeleton-md"></span>
			{/if}
		</div>
	</div>
</div>

<div popover="manual" class="searchahead-dropdown" bind:this={popoverEl}>
	<div class="searchahead-dropdown-header">Results</div>
	{#if loading}
		<div class="searchahead-empty">Zoeken...</div>
	{:else if showHint}
		<div class="searchahead-empty">{hint}</div>
	{:else}
		{#each filtered as option, i}
			<div
				class="searchahead-option"
				class:is-highlighted={i === highlightedIndex}
				class:is-selected={selected.some((s) => s.text === option.text)}
				class:is-disabled={option.disabled}
				class:is-child={option.indent === 1}
				role="option"
				onclick={() => selectItem(option)}
				onmouseenter={() => (highlightedIndex = i)}
			>
				<div class="searchahead-option-left">
					{#if option.icon}
						<span class="searchahead-option-icon"><option.icon size={12} /></span>
					{/if}
					{#if option.indent === 1}
						<span class="searchahead-branch"></span>
					{/if}
					<span class="searchahead-option-text">{option.text}</span>
				</div>
				{#if option.meta}
					<span class="searchahead-option-meta">{option.meta}</span>
				{/if}
			</div>
		{/each}
		{#if filtered.length === 0}
			<div class="searchahead-empty">No results found</div>
		{/if}
	{/if}
</div>

<style>
	.searchahead {
		width: 100%;
	}

	.searchahead-wrap {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.searchahead-tags {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.searchahead-tag {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		background: var(--color-accent-muted);
		border: 1px solid oklch(0.82 0.03 41);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-accent-dark);
	}

	.searchahead-tag:hover {
		background: oklch(0.88 0.03 41);
	}

	.searchahead-tag-clear {
		cursor: pointer;
		border-color: var(--color-border-strong);
		color: var(--color-text-secondary);
		background: var(--color-surface-sunken);
	}

	.searchahead-tag-clear:hover {
		background: var(--color-surface-raised);
	}

	.searchahead-tag-x {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: var(--color-accent-dark);
		opacity: 0.6;
		transition: opacity var(--duration-fast) ease;
	}

	.searchahead-tag-x:hover {
		opacity: 1;
	}

	.searchahead-skeleton {
		display: inline-block;
		height: 18px;
		width: 72px;
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
	}

	.searchahead-skeleton-sm {
		width: 48px;
	}

	.searchahead-skeleton-md {
		width: 64px;
	}

	.searchahead-input-wrap {
		position: relative;
		display: flex;
		align-items: center;
	}

	.searchahead-search-icon {
		position: absolute;
		left: var(--space-3);
		color: var(--color-text-tertiary);
		pointer-events: none;
	}

	.searchahead-input {
		width: 100%;
		padding: var(--space-2) var(--space-3) var(--space-2) var(--space-8);
		border: 1px solid var(--color-border-strong);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.searchahead-input::placeholder {
		color: var(--color-text-placeholder);
		font-size: var(--text-xs);
		font-weight: 300;
	}

	.searchahead-input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.searchahead-clear {
		position: absolute;
		right: var(--space-2);
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		line-height: 1;
		opacity: 0;
		pointer-events: none;
		transition:
			opacity var(--duration-fast) ease,
			color var(--duration-fast) ease;
	}

	.searchahead-clear.visible {
		opacity: 1;
		pointer-events: auto;
	}

	.searchahead-clear:hover {
		color: var(--color-text);
	}

	:global(.searchahead-dropdown) {
		position: fixed;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-md);
		max-height: 240px;
		overflow-y: auto;
		border-radius: var(--radius-xs);
		padding: 0;
		margin: 0;
	}

	:global(.searchahead-dropdown-header) {
		position: sticky;
		top: 0;
		background: var(--color-surface-raised);
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		border-bottom: 1px solid var(--color-border);
	}

	:global(.searchahead-option) {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	:global(.searchahead-option:hover),
	:global(.searchahead-option.is-highlighted) {
		background: var(--color-accent-muted);
	}

	:global(.searchahead-option.is-selected) {
		opacity: 0.5;
	}

	:global(.searchahead-option.is-disabled) {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.searchahead-option-left) {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	:global(.searchahead-option-icon) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: var(--color-accent);
		width: 16px;
		height: 16px;
		flex-shrink: 0;
	}

	:global(.searchahead-option.is-child) {
		padding-left: calc(var(--space-3) + 20px);
	}

	:global(.searchahead-branch) {
		position: relative;
		width: 16px;
		height: 14px;
		flex-shrink: 0;
	}

	:global(.searchahead-branch::before) {
		content: "";
		position: absolute;
		top: -6px;
		left: 0;
		border-left: 1px solid var(--color-border);
		border-bottom: 1px solid var(--color-border);
		width: 12px;
		height: 11px;
		border-bottom-left-radius: 3px;
	}

	:global(.searchahead-option-text) {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text);
	}

	:global(.searchahead-option-meta) {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	:global(.searchahead-empty) {
		padding: var(--space-6) var(--space-3);
		text-align: center;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}
</style>
