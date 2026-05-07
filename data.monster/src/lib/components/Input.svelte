<script lang="ts">
	let {
		type = "text",
		placeholder = "",
		label,
		id,
		hint,
		required = false,
		disabled = false,
		value = $bindable(""),
		onblur,
	}: {
		type?: string;
		placeholder?: string;
		label?: string;
		id?: string;
		hint?: string;
		required?: boolean;
		disabled?: boolean;
		value?: string;
		onblur?: (e: FocusEvent) => void;
	} = $props();
</script>

<div class="field">
	{#if label}
		<label class="field-label" for={id}>{label}</label>
	{/if}
	<input
		{type}
		{id}
		{placeholder}
		{required}
		{disabled}
		bind:value
		{onblur}
		class="input"
	/>
	{#if hint}
		<span class="field-hint">{hint}</span>
	{/if}
</div>

<style>
	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.field-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.input {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.input::placeholder {
		color: var(--color-text-placeholder);
		font-size: var(--text-xs);
		font-weight: 300;
	}

	.input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.input:invalid:not(:placeholder-shown) {
		border-color: var(--color-danger);
		box-shadow: 0 0 0 2px oklch(0.95 0.03 22);
	}
</style>
