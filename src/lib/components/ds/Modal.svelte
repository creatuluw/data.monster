<script lang="ts">
	let open = $state<string | null>(null);
	let previousFocus = $state<Element | null>(null);

	function openModal(id: string) {
		previousFocus = document.activeElement;
		open = id;
		document.body.style.overflow = 'hidden';
	}

	function closeModal() {
		open = null;
		document.body.style.overflow = '';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			closeModal();
			return;
		}
		if (e.key !== 'Tab') return;
		const backdrop = e.currentTarget as HTMLElement | null;
		if (!backdrop) return;
		const focusable = backdrop.querySelectorAll('button, input, [tabindex]:not([tabindex="-1"])');
		if (focusable.length === 0) return;
		const first = focusable[0] as HTMLElement;
		const last = focusable[focusable.length - 1] as HTMLElement;
		if (e.shiftKey && document.activeElement === first) {
			e.preventDefault();
			last.focus();
		} else if (!e.shiftKey && document.activeElement === last) {
			e.preventDefault();
			first.focus();
		}
	}
</script>

<div class="component-group">
	<span class="component-group-label">Overlay Dialogs</span>
	<h3 class="component-group-title">Modal</h3>
	<p class="desc">
		A focused surface for confirmations, student editing, and detail
		views. Traps focus. Returns focus on close.
	</p>
	<div class="button-row">
		<button class="btn btn-primary" onclick={() => openModal('confirm')}>Open Confirm</button>
		<button class="btn btn-secondary" onclick={() => openModal('config')}>Open Edit</button>
		<button class="btn btn-danger" onclick={() => openModal('danger')}>Open Remove</button>
	</div>
</div>

{#if open === 'confirm'}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-backdrop is-open" role="dialog" aria-modal="true" aria-labelledby="modal-confirm-title" onclick={(e) => { if (e.target === e.currentTarget) closeModal() }} onkeydown={handleKeydown}>
		<div class="modal">
			<div class="modal-header">
				<div class="modal-header-left">
					<span class="modal-header-ref">GL-MOD-01</span>
					<h2 class="modal-title" id="modal-confirm-title">Confirm Registration</h2>
				</div>
				<button class="modal-close" aria-label="Close modal" onclick={closeModal}>×</button>
			</div>
			<div class="modal-body">
				<p>
					You are about to submit assessments for
					<strong>Group 7B</strong>
					in <strong>Mathematics</strong>. This will register grades for 24
					students and update their progress records.
				</p>
				<p>
					Estimated processing:
					<span class="mono-xs">~2 sec</span>. Edit window:
					<span class="mono-xs">24 hours</span>.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn btn-ghost" onclick={closeModal}>Cancel</button>
				<button class="btn btn-primary" onclick={closeModal}>Submit</button>
			</div>
		</div>
	</div>
{/if}

{#if open === 'config'}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-backdrop is-open" role="dialog" aria-modal="true" aria-labelledby="modal-config-title" onclick={(e) => { if (e.target === e.currentTarget) closeModal() }} onkeydown={handleKeydown}>
		<div class="modal">
			<div class="modal-header">
				<div class="modal-header-left">
					<span class="modal-header-ref">GL-MOD-02</span>
					<h2 class="modal-title" id="modal-config-title">Edit Student</h2>
				</div>
				<button class="modal-close" aria-label="Close modal" onclick={closeModal}>×</button>
			</div>
			<div class="modal-body">
				<div class="field">
					<label class="field-label" for="modal-node-name">Student Name</label>
					<input class="input" type="text" id="modal-node-name" placeholder="Jan de Vries" style="width: 100%" />
				</div>
				<div class="field">
					<label class="field-label" for="modal-zone">Class</label>
					<input class="input" type="text" id="modal-zone" placeholder="7B" style="width: 100%" />
				</div>
				<div class="field">
					<label class="field-label" for="modal-weight">Student Number</label>
					<input class="input" type="number" id="modal-weight" placeholder="2024001" style="width: 100%" />
					<span class="field-hint">6-digit ID. Unique per student.</span>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn btn-ghost" onclick={closeModal}>Discard</button>
				<button class="btn btn-primary" onclick={closeModal}>Save Config</button>
			</div>
		</div>
	</div>
{/if}

{#if open === 'danger'}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-backdrop is-open" role="dialog" aria-modal="true" aria-labelledby="modal-danger-title" onclick={(e) => { if (e.target === e.currentTarget) closeModal() }} onkeydown={handleKeydown}>
		<div class="modal">
			<div class="modal-header danger-header">
				<div class="modal-header-left">
					<span class="modal-header-ref danger-ref">GL-MOD-03</span>
					<h2 class="modal-title danger-title" id="modal-danger-title">Remove Student</h2>
				</div>
				<button class="modal-close" aria-label="Close modal" onclick={closeModal}>×</button>
			</div>
			<div class="modal-body">
				<p>
					This will remove <strong>Jan de Vries</strong> from
					<strong>Group 7B</strong>. All in-progress registrations will be
					saved as draft. Submitted grades will not be affected.
				</p>
				<p class="danger-warning">▸ THIS ACTION CANNOT BE UNDONE</p>
			</div>
			<div class="modal-footer">
				<button class="btn btn-ghost" onclick={closeModal}>Cancel</button>
				<button class="btn btn-danger" onclick={closeModal}>Remove</button>
			</div>
		</div>
	</div>
{/if}

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

	.button-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-3);
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		line-height: 1.4;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		transition:
			transform var(--duration-fast) var(--ease-out-expo),
			background var(--duration-fast) ease,
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.btn:focus-visible { outline: 2px solid var(--color-accent); outline-offset: 2px; }
	.btn:active { transform: scale(0.97); }
	.btn-primary { background: var(--color-accent); color: var(--color-text-on-accent); border-color: var(--color-accent); }
	.btn-primary:hover { background: var(--color-accent-dark); border-color: var(--color-accent-dark); }
	.btn-secondary { background: transparent; color: var(--color-text); border-color: var(--color-border-strong); }
	.btn-secondary:hover { border-color: var(--color-accent); color: var(--color-accent); }
	.btn-ghost { background: transparent; color: var(--color-text-secondary); border-color: transparent; }
	.btn-ghost:hover { background: var(--color-surface-sunken); color: var(--color-text); }
	.btn-danger { background: var(--color-danger); color: oklch(0.97 0.005 22); border-color: var(--color-danger); }
	.btn-danger:hover { background: oklch(0.48 0.15 22); border-color: oklch(0.48 0.15 22); }

	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		width: 90%;
		max-width: 520px;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-6);
		border-bottom: 1px solid var(--color-border);
	}

	.danger-header {
		border-bottom-color: oklch(0.9 0.04 22);
	}

	.modal-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.modal-header-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
	}

	.danger-ref { color: var(--color-danger); }

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.danger-title { color: var(--color-danger); }

	.modal-close {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		color: var(--color-text-tertiary);
		font-size: 14px;
		line-height: 1;
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.modal-close:hover { color: var(--color-text); border-color: var(--color-border-strong); background: var(--color-surface-sunken); }
	.modal-close:focus-visible { outline: 2px solid var(--color-accent); outline-offset: 1px; }

	.modal-body {
		padding: var(--space-6);
	}

	.modal-body :global(p) {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		margin-bottom: var(--space-4);
	}

	.modal-body :global(p:last-child) {
		margin-bottom: 0;
	}

	.mono-xs {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.danger-warning {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-danger);
		letter-spacing: 0.04em;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		margin-bottom: var(--space-4);
	}

	.field:last-child {
		margin-bottom: 0;
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
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px dashed var(--color-border);
	}
</style>
