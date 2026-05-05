<div class="component-group">
	<span class="component-group-label">Binary Controls</span>
	<h3 class="component-group-title">Toggles</h3>
	<div class="toggle-row">
		<div class="toggle-group">
			<input class="toggle" type="checkbox" id="toggle-alerts" checked />
			<label class="toggle-label" for="toggle-alerts">Notifications</label>
		</div>
		<div class="toggle-group">
			<input class="toggle" type="checkbox" id="toggle-logs" />
			<label class="toggle-label" for="toggle-logs">Detailed View</label>
		</div>
		<div class="toggle-group">
			<input class="toggle" type="checkbox" id="toggle-auto" checked />
			<label class="toggle-label" for="toggle-auto">Auto-save</label>
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

	.toggle-row {
		display: flex;
		gap: var(--space-8);
		flex-wrap: wrap;
		align-items: center;
	}

	.toggle-group {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.toggle-label {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		cursor: pointer;
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
</style>
