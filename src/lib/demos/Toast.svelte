<script lang="ts">
	import { tick } from 'svelte';

	type Variant = 'default' | 'success' | 'warning' | 'danger';

	interface ToastItem {
		id: number;
		variant: Variant;
		title: string;
		message: string;
		duration: number;
		visible: boolean;
		dismissing: boolean;
	}

	let toasts = $state<ToastItem[]>([]);
	let counter = 0;

	const presets: Record<Variant, { title: string; message: string; duration: number }> = {
		default: { title: 'Notification', message: 'Query results exported to CSV.', duration: 4500 },
		success: { title: 'Analysis Complete', message: 'Q4 Revenue Report generated. 12 dashboards updated.', duration: 4500 },
		warning: { title: 'Incomplete Data', message: '3 data sources are missing refresh timestamps for this period.', duration: 5500 },
		danger: { title: 'Connection Failed', message: 'PostgreSQL Production is unreachable. 4 active queries cancelled.', duration: 6000 },
	};

	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return { destroy() { node.remove(); } };
	}

	async function showToast(variant: Variant) {
		const preset = presets[variant] || presets.default;
		const id = ++counter;
		const toast: ToastItem = { id, variant, ...preset, visible: false, dismissing: false };
		toasts = [...toasts, toast];

		await tick();

		requestAnimationFrame(() => {
			requestAnimationFrame(() => {
				toasts = toasts.map(t => t.id === id ? { ...t, visible: true } : t);
			});
		});

		setTimeout(() => dismiss(id), preset.duration);
	}

	function dismiss(id: number) {
		toasts = toasts.map(t => t.id === id ? { ...t, dismissing: true, visible: false } : t);
		setTimeout(() => {
			toasts = toasts.filter(t => t.id !== id);
		}, 500);
	}
</script>

<div class="component-group">
	<span class="component-group-label">Notification System</span>
	<h3 class="component-group-title">Toast</h3>
	<p class="desc">
		Ephemeral notifications that slide in, persist briefly, then
		dismiss. Stacks vertically from the bottom-right. No interruption to workflow.
	</p>
	<div class="toast-trigger-row">
		<button class="btn btn-primary" onclick={() => showToast('default')}>Notify</button>
		<button class="btn btn-outline btn-success" onclick={() => showToast('success')}>Success</button>
		<button class="btn btn-outline btn-warning" onclick={() => showToast('warning')}>Warning</button>
		<button class="btn btn-outline btn-danger-outline" onclick={() => showToast('danger')}>Error</button>
	</div>
</div>

<div use:portal>
	<div class="toast-container" aria-live="polite" aria-atomic="false">
		{#each toasts as toast (toast.id)}
			<div
				class="toast toast-{toast.variant}"
				class:is-visible={toast.visible}
				class:is-dismissing={toast.dismissing}
				role="status"
			>
				<div class="toast-indicator"></div>
				<div class="toast-body">
					<span class="toast-title">{toast.title}</span>
					<span class="toast-message">{toast.message}</span>
				</div>
				<button class="toast-close" aria-label="Dismiss notification" onclick={() => dismiss(toast.id)}>×</button>
				<div
					class="toast-progress toast-progress-{toast.variant}"
					style="width: 100%; transition-duration: {toast.duration}ms;"
				></div>
			</div>
		{/each}
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

	.toast-trigger-row {
		display: flex;
		flex-wrap: wrap;
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
		border: 1px solid;
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

	.btn-outline { background: transparent; }
	.btn-success { border-color: var(--color-success); color: var(--color-success); }
	.btn-warning { border-color: var(--color-warning); color: var(--color-warning); }
	.btn-danger-outline { border-color: var(--color-danger); color: var(--color-danger); }

	.toast-container {
		position: fixed;
		bottom: var(--space-6);
		right: var(--space-6);
		display: flex;
		flex-direction: column-reverse;
		gap: var(--space-2);
		z-index: 1100;
		pointer-events: none;
	}

	.toast {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-md);
		min-width: 300px;
		max-width: 420px;
		pointer-events: auto;
		position: relative;
		overflow: hidden;
		transform: translateX(110%);
		opacity: 0;
		transition:
			transform var(--duration-slow) var(--ease-out-expo),
			opacity var(--duration-slow) var(--ease-out-expo);
	}

	.toast.is-visible {
		transform: translateX(0);
		opacity: 1;
	}

	.toast.is-dismissing {
		transform: translateX(110%);
		opacity: 0;
	}

	.toast.is-visible .toast-progress {
		transform: scaleX(0);
	}

	.toast-indicator {
		width: 3px;
		align-self: stretch;
		flex-shrink: 0;
	}

	.toast-default .toast-indicator { background: var(--color-accent); }
	.toast-success .toast-indicator { background: var(--color-success); }
	.toast-warning .toast-indicator { background: var(--color-warning); }
	.toast-danger .toast-indicator { background: var(--color-danger); }

	.toast-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.toast-title {
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
	}

	.toast-message {
		font-size: var(--text-sm);
		line-height: var(--leading-normal);
		color: var(--color-text-secondary);
	}

	.toast-close {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		font-size: 12px;
		line-height: 1;
		flex-shrink: 0;
		transition: color var(--duration-fast) ease;
	}

	.toast-close:hover { color: var(--color-text); }

	.toast-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		height: 2px;
		transform-origin: left;
		transition: transform linear;
	}

	.toast-progress-default { background: var(--color-accent); }
	.toast-progress-success { background: var(--color-success); }
	.toast-progress-warning { background: var(--color-warning); }
	.toast-progress-danger { background: var(--color-danger); }

	@media (max-width: 640px) {
		.toast-container {
			left: var(--space-4);
			right: var(--space-4);
			bottom: var(--space-4);
		}
		.toast {
			min-width: 0;
			max-width: 100%;
		}
	}
</style>
