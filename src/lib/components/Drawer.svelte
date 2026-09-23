<script lang="ts">
	let open = $state<string | null>(null);
	let previousFocus = $state<Element | null>(null);

	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return { destroy() { node.remove(); } };
	}

	function openDrawer(id: string) {
		previousFocus = document.activeElement;
		open = id;
		document.body.style.overflow = 'hidden';
	}

	function closeDrawer() {
		open = null;
		document.body.style.overflow = '';
		if (previousFocus instanceof HTMLElement) previousFocus.focus();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			closeDrawer();
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
	<h3 class="component-group-title">Drawer</h3>
	<p class="desc">
		A side panel for detail views, settings, and multi-step flows that
		need more room than a modal. Traps focus. Returns focus on close.
	</p>
	<div class="button-row">
		<button class="btn btn-primary" onclick={() => openDrawer('detail')}>Open Details</button>
		<button class="btn btn-danger" onclick={() => openDrawer('danger')}>Open Remove</button>
	</div>
</div>

<div use:portal>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-backdrop" class:is-open={open === 'detail'} role="dialog" aria-modal="true" aria-labelledby="drawer-detail-title" onclick={(e) => { if (e.target === e.currentTarget) closeDrawer() }} onkeydown={handleKeydown}>
		<aside class="drawer">
			<div class="drawer-header">
				<div>
					<span class="drawer-ref">GL-001</span>
					<h2 class="drawer-title" id="drawer-detail-title">Structured Queries</h2>
				</div>
				<button class="drawer-close" aria-label="Close drawer" onclick={closeDrawer}>×</button>
			</div>
			<div class="drawer-body">
				<p>
					Every analysis starts with a well-defined query. Named templates
					and parameterized filters ensure reproducibility across dashboards
					and scheduled reports.
				</p>
				<div class="detail-grid">
					<span class="detail-label">Source</span>
					<span class="detail-value">PostgreSQL Production</span>
					<span class="detail-label">Refresh</span>
					<span class="detail-value">Every 15 min</span>
					<span class="detail-label">Owner</span>
					<span class="detail-value">Analytics Team</span>
				</div>
			</div>
			<div class="drawer-footer">
				<button class="btn btn-ghost" onclick={closeDrawer}>Close</button>
				<button class="btn btn-primary" onclick={closeDrawer}>Open Query</button>
			</div>
		</aside>
	</div>

	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="drawer-backdrop" class:is-open={open === 'danger'} role="dialog" aria-modal="true" aria-labelledby="drawer-danger-title" onclick={(e) => { if (e.target === e.currentTarget) closeDrawer() }} onkeydown={handleKeydown}>
		<aside class="drawer">
			<div class="drawer-header danger-header">
				<div>
					<span class="drawer-ref danger-ref">GL-003</span>
					<h2 class="drawer-title danger-title" id="drawer-danger-title">Remove Data Source</h2>
				</div>
				<button class="drawer-close" aria-label="Close drawer" onclick={closeDrawer}>×</button>
			</div>
			<div class="drawer-body">
				<p>
					This will remove the <strong>PostgreSQL Production</strong> data source.
					All in-progress queries will be cancelled. Saved reports will not
					be affected.
				</p>
				<p class="danger-warning">THIS ACTION CANNOT BE UNDONE</p>
			</div>
			<div class="drawer-footer">
				<button class="btn btn-ghost" onclick={closeDrawer}>Cancel</button>
				<button class="btn btn-danger" onclick={closeDrawer}>Remove</button>
			</div>
		</aside>
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
	.btn-ghost { background: transparent; color: var(--color-text-secondary); border-color: transparent; }
	.btn-ghost:hover { background: var(--color-surface-sunken); color: var(--color-text); }
	.btn-danger { background: var(--color-danger); color: oklch(0.97 0.005 25); border-color: var(--color-danger); }
	.btn-danger:hover { background: oklch(0.46 0.14 25); border-color: oklch(0.46 0.14 25); }

	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 1000;
		opacity: 0;
		visibility: hidden;
		transition:
			opacity var(--duration-slow) var(--ease-out-expo),
			visibility var(--duration-slow);
	}

	.drawer-backdrop.is-open {
		opacity: 1;
		visibility: visible;
	}

	.drawer {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: min(420px, 90vw);
		background: var(--color-surface);
		border-left: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		display: flex;
		flex-direction: column;
		transform: translateX(24px);
		transition: transform var(--duration-slow) var(--ease-out-expo);
	}

	.drawer-backdrop.is-open .drawer {
		transform: translateX(0);
	}

	.drawer-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-3);
		padding: var(--space-6);
		border-bottom: 1px solid var(--color-border);
	}

	.danger-header {
		border-bottom-color: oklch(0.9 0.04 25);
	}

	.drawer-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
	}

	.danger-ref { color: var(--color-danger); }

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-top: var(--space-1);
	}

	.danger-title { color: var(--color-danger); }

	.drawer-close {
		width: 28px;
		height: 28px;
		flex-shrink: 0;
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

	.drawer-close:hover { color: var(--color-text); border-color: var(--color-border-strong); background: var(--color-surface-sunken); }
	.drawer-close:focus-visible { outline: 2px solid var(--color-accent); outline-offset: 1px; }

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.drawer-body :global(p) {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		margin-bottom: var(--space-4);
	}

	.detail-grid {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: var(--space-2) var(--space-4);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
	}

	.detail-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.detail-value {
		font-size: var(--text-sm);
		color: var(--color-text);
	}

	.danger-warning {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-danger);
		letter-spacing: 0.04em;
	}

	.drawer-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px dashed var(--color-border);
	}
</style>
