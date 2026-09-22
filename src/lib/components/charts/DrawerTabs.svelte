<script lang="ts">
	/** Shared Settings | Danger zone tab bar for every config drawer. */
	let {
		active,
		onchange
	}: {
		active: 'settings' | 'danger';
		onchange: (tab: 'settings' | 'danger') => void;
	} = $props();
</script>

<div class="tabs" role="tablist">
	<button
		type="button"
		role="tab"
		aria-selected={active === 'settings'}
		class="tab"
		class:on={active === 'settings'}
		onclick={() => onchange('settings')}
	>
		Settings
	</button>
	<button
		type="button"
		role="tab"
		aria-selected={active === 'danger'}
		class="tab tab-danger"
		class:on={active === 'danger'}
		onclick={() => onchange('danger')}
	>
		Danger zone
	</button>
</div>

<style>
	.tabs {
		display: flex;
		gap: 2px;
		padding: 3px;
		background: var(--color-surface-sunken);
		border-radius: var(--radius-md);
	}

	.tab {
		flex: 1;
		height: 26px;
		border: none;
		border-radius: var(--radius-sm);
		background: transparent;
		color: var(--color-text-secondary);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 500;
		cursor: pointer;
		transition:
			background 120ms ease,
			color 120ms ease,
			box-shadow 120ms ease;
	}

	.tab:hover:not(.on) {
		color: var(--color-text);
	}

	.tab-danger:hover:not(.on) {
		color: var(--color-danger);
	}

	.tab.on {
		background: var(--color-surface);
		color: var(--color-text);
		box-shadow: var(--shadow-sm);
	}

	.tab-danger.on {
		color: var(--color-danger);
	}

	.tab:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}
</style>
