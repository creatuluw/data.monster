<div class="component-group">
	<span class="component-group-label">Data Entry</span>
	<h3 class="component-group-title">Inputs</h3>
	<div class="input-row">
		<div class="field">
			<label class="field-label" for="host-input">Data Source</label>
			<input class="input" type="text" id="host-input" placeholder="e.g. PostgreSQL Production" />
		</div>
		<div class="field">
			<label class="field-label" for="port-input">Metric Code</label>
			<input class="input" type="url" id="port-input" placeholder="e.g. REV-001" required />
			<span class="field-hint">Alphanumeric. No spaces.</span>
		</div>
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

	.input-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: var(--space-6);
		align-items: flex-start;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.field-label {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
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
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.input::placeholder {
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
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
