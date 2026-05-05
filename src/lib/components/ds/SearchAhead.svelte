<script lang="ts">
	interface SaItem {
		id: string;
		label: string;
		meta: string;
		icon: string;
	}

	const saData: SaItem[] = [
		{ id: 'stu-01', label: 'Emma Bakker', meta: 'Group 7B', icon: '◉' },
		{ id: 'stu-02', label: 'Jan de Vries', meta: 'Group 7B', icon: '◉' },
		{ id: 'stu-03', label: 'Sophie Jansen', meta: 'Group 7A', icon: '◉' },
		{ id: 'stu-04', label: 'Lucas Visser', meta: 'Group 8A', icon: '◉' },
		{ id: 'sub-wis', label: 'Wiskunde', meta: 'WIS-301', icon: '◆' },
		{ id: 'sub-tal', label: 'Taal', meta: 'TAL-201', icon: '◆' },
		{ id: 'sub-lez', label: 'Lezen', meta: 'LEZ-102', icon: '◆' },
		{ id: 'sub-snk', label: 'Snappet', meta: 'SNK-401', icon: '◆' },
		{ id: 'grp-7a', label: 'Group 7A', meta: '28 students', icon: '▨' },
		{ id: 'grp-7b', label: 'Group 7B', meta: '24 students', icon: '▨' },
		{ id: 'grp-8a', label: 'Group 8A', meta: '26 students', icon: '▨' },
		{ id: 'grp-8b', label: 'Group 8B', meta: '22 students', icon: '▨' },
		{ id: 'tea-01', label: 'Marie Kosters', meta: 'Teacher', icon: '◈' },
		{ id: 'tea-02', label: 'Peter Hendriks', meta: 'Teacher', icon: '◈' },
	];

	let query = $state('');
	let selected = $state<string[]>([]);
	let highlighted = $state(-1);
	let dropdownOpen = $state(false);

	let filtered = $derived(
		saData
			.filter(item => !selected.includes(item.id))
			.filter(item => {
				if (!query.trim()) return true;
				const q = query.toLowerCase();
				return item.label.toLowerCase().includes(q) || item.meta.toLowerCase().includes(q);
			})
	);

	function highlightText(text: string, q: string) {
		if (!q) return text;
		const regex = new RegExp(`(${q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
		return text.replace(regex, '<mark>$1</mark>');
	}

	function open() { dropdownOpen = true; }
	function close() { dropdownOpen = false; highlighted = -1; }

	function select(id: string) {
		selected = [...selected, id];
		query = '';
		dropdownOpen = true;
		highlighted = -1;
	}

	function deselect(id: string) {
		selected = selected.filter(s => s !== id);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			highlighted = Math.min(highlighted + 1, filtered.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			highlighted = Math.max(highlighted - 1, 0);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (highlighted >= 0 && filtered[highlighted]) {
				select(filtered[highlighted].id);
			}
		} else if (e.key === 'Escape') {
			close();
		} else if (e.key === 'Backspace' && query === '' && selected.length) {
			deselect(selected[selected.length - 1]);
		}
	}

	function handleMouseover(i: number) {
		highlighted = i;
	}
</script>

<div class="component-group">
	<span class="component-group-label">Filtered Selection</span>
	<h3 class="component-group-title">Search-ahead Select</h3>
	<p class="desc">
		Type to filter. Arrow keys to navigate. Enter to select. Backspace
		on empty input to remove last chip.
	</p>

	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="searchahead" onfocusout={(e) => { if (!e.currentTarget.contains(e.relatedTarget as Node)) close() }}>
		<div class="searchahead-input-wrap" class:has-dropdown={dropdownOpen}>
			<input
				class="searchahead-input"
				type="text"
				placeholder="Search students, subjects, groups..."
				autocomplete="off"
				aria-label="Search and select items"
				aria-expanded={dropdownOpen}
				role="combobox"
				bind:value={query}
				onfocus={open}
				oninput={() => { open(); highlighted = -1; }}
				onkeydown={handleKeydown}
			/>
			{#if query}
				<button class="searchahead-clear visible" aria-label="Clear search" type="button" onclick={() => { query = ''; }}>
					×
				</button>
			{:else}
				<button class="searchahead-clear" aria-label="Clear search" type="button" tabindex="-1">×</button>
			{/if}
			{#if dropdownOpen}
				<div class="searchahead-dropdown is-open" role="listbox">
					<div class="searchahead-dropdown-header">Results ({filtered.length})</div>
					{#if filtered.length === 0}
						<div class="searchahead-empty">// No matching results</div>
					{:else}
						{#each filtered as item, i}
							<div
								class="searchahead-option"
								class:is-highlighted={i === highlighted}
								role="option"
								onclick={() => select(item.id)}
								onmouseover={() => handleMouseover(i)}
							>
								<span class="searchahead-option-left">
									<span class="searchahead-option-icon">{item.icon}</span>
									<span class="searchahead-option-text">{@html highlightText(item.label, query.toLowerCase().trim())}</span>
								</span>
								<span class="searchahead-option-meta">{item.meta}</span>
							</div>
						{/each}
					{/if}
				</div>
			{/if}
		</div>
		{#if selected.length > 0}
			<div class="searchahead-selection" aria-label="Selected items">
				{#each selected as id}
					<button class="searchahead-chip" onclick={() => deselect(id)}>
						{saData.find(d => d.id === id)?.label}
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.component-group {
		position: relative;
	}

	.component-group-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-6);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.component-group-label::before {
		content: "\25A0";
		color: var(--color-accent);
		font-size: 7px;
	}

	.component-group-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-6);
	}

	.desc {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		margin-bottom: var(--space-4);
		max-width: 52ch;
	}

	.searchahead {
		position: relative;
		width: 100%;
		max-width: 420px;
	}

	.searchahead-input-wrap {
		position: relative;
		display: flex;
		align-items: center;
	}

	.searchahead-input-wrap::before {
		content: "\2315";
		position: absolute;
		left: var(--space-3);
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		pointer-events: none;
		z-index: 1;
	}

	.searchahead-input {
		width: 100%;
		padding: var(--space-2) var(--space-3) var(--space-2) var(--space-8);
		border: 1px solid var(--color-border-strong);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.searchahead-input::placeholder { color: var(--color-text-tertiary); }
	.searchahead-input:focus { outline: none; border-color: var(--color-accent); box-shadow: 0 0 0 2px var(--color-accent-muted); }

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
		transition: opacity var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.searchahead-clear.visible { opacity: 1; pointer-events: auto; }
	.searchahead-clear:hover { color: var(--color-text); }

	.searchahead-dropdown {
		position: absolute;
		top: calc(100% + 2px);
		left: 0;
		right: 0;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-md);
		max-height: 240px;
		overflow-y: auto;
		z-index: 50;
	}

	.searchahead-dropdown-header {
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

	.searchahead-option {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.searchahead-option:hover,
	.searchahead-option.is-highlighted {
		background: var(--color-accent-muted);
	}

	.searchahead-option-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.searchahead-option-icon {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-accent);
		width: 16px;
		text-align: center;
	}

	.searchahead-option-text {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
	}

	.searchahead-option-text :global(mark) {
		background: none;
		color: var(--color-accent);
		font-weight: 700;
	}

	.searchahead-option-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.searchahead-empty {
		padding: var(--space-6) var(--space-3);
		text-align: center;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.searchahead-selection {
		margin-top: var(--space-4);
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		min-height: 24px;
	}

	.searchahead-chip {
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
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.searchahead-chip:hover { background: oklch(0.88 0.03 41); }
	.searchahead-chip::after { content: "\00D7"; font-size: 11px; margin-left: var(--space-1); color: var(--color-accent-light); }

	@media (max-width: 640px) {
		.searchahead { max-width: 100%; }
	}
</style>
