<script lang="ts">
	/** Drawer control kit — boolean switch (keyboard accessible, role=switch). */
	let {
		checked = $bindable(false),
		label,
		onchange
	}: {
		checked?: boolean;
		/** optional visible label; the switch is self-explanatory without */
		label?: string;
		onchange?: (v: boolean) => void;
	} = $props();

	function toggle() {
		checked = !checked;
		onchange?.(checked);
	}
</script>

<button type="button" class="toggle" class:on={checked} role="switch" aria-checked={checked} aria-label={label} onclick={toggle}>
	<span class="knob"></span>
</button>

<style>
	.toggle {
		position: relative;
		width: 28px;
		height: 16px;
		flex-shrink: 0;
		border: 1px solid var(--color-border-strong);
		border-radius: var(--radius-full);
		background: var(--color-surface-sunken);
		cursor: pointer;
		padding: 0;
		transition:
			background 120ms ease,
			border-color 120ms ease;
	}

	.toggle .knob {
		position: absolute;
		top: 1px;
		left: 1px;
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--color-surface);
		box-shadow: var(--shadow-sm);
		transition: transform 140ms var(--ease-out-quart, cubic-bezier(0.25, 1, 0.5, 1));
	}

	.toggle.on {
		background: var(--color-accent);
		border-color: var(--color-accent-dark);
	}

	.toggle.on .knob {
		transform: translateX(12px);
	}

	.toggle:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}
</style>
