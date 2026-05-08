<script lang="ts">
	let {
		checked = $bindable(false),
		label = "",
		id,
	}: {
		checked?: boolean;
		label?: string;
		id?: string;
	} = $props();
</script>

<div class="toggle-group">
	<input
		{id}
		type="checkbox"
		class="toggle"
		bind:checked
		role="switch"
		aria-checked={checked}
	/>
	{#if label}
		<label class="toggle-label" for={id}>{label}</label>
	{/if}
</div>

<style>
	.toggle-group {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.toggle {
		position: relative;
		width: 40px;
		height: 20px;
		appearance: none;
		-webkit-appearance: none;
		background: var(--color-border-strong);
		border-radius: var(--radius-xs);
		border: 1px solid var(--color-border);
		cursor: pointer;
		transition:
			background var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.toggle::after {
		content: "";
		position: absolute;
		top: 2px;
		left: 2px;
		width: 14px;
		height: 14px;
		background: var(--color-surface);
		border-radius: 1px;
		transition: transform var(--duration-base) var(--ease-out-expo);
		box-shadow: 0 1px 2px oklch(0.22 0.005 250 / 0.2);
	}

	.toggle:checked {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}

	.toggle:checked::after {
		transform: translateX(20px);
	}

	.toggle:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	.toggle-label {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		cursor: pointer;
	}
</style>
