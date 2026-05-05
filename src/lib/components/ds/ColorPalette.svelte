<script lang="ts">
	function copySwatch(event: MouseEvent, value: string) {
		navigator.clipboard.writeText(value).catch(() => {});
		const swatch = event.currentTarget as HTMLElement;
		swatch.classList.add("copied");
		setTimeout(() => swatch.classList.remove("copied"), 1200);
	}
</script>

<section class="section" id="colors" aria-labelledby="colors-title">
	<div class="section-header entrance">
		<span class="section-number">01</span>
		<h2 class="section-title" id="colors-title">Color Palette</h2>
	</div>

	<div class="color-grid entrance entrance-d1">
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.69 0.16 41)')} aria-label="Copy accent color token">
			<div class="color-swatch-fill swatch-accent" data-token="--color-accent"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Accent</span>
				<span class="color-swatch-value">oklch(0.69 0.16 41)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.78 0.09 41)')} aria-label="Copy accent light color token">
			<div class="color-swatch-fill swatch-accent-light" data-token="--color-accent-light"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Accent Light</span>
				<span class="color-swatch-value">oklch(0.78 0.09 41)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.44 0.14 41)')} aria-label="Copy accent dark color token">
			<div class="color-swatch-fill swatch-accent-dark" data-token="--color-accent-dark"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Accent Dark</span>
				<span class="color-swatch-value">oklch(0.44 0.14 41)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.93 0.03 41)')} aria-label="Copy accent muted color token">
			<div class="color-swatch-fill swatch-accent-muted" data-token="--color-accent-muted"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Accent Muted</span>
				<span class="color-swatch-value">oklch(0.93 0.03 41)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>

		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.98 0.003 250)')} aria-label="Copy surface color token">
			<div class="color-swatch-fill swatch-surface" data-token="--color-surface"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Surface</span>
				<span class="color-swatch-value">oklch(0.98 0.003 250)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.96 0.005 250)')} aria-label="Copy surface raised color token">
			<div class="color-swatch-fill swatch-surface-raised" data-token="--color-surface-raised"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Raised</span>
				<span class="color-swatch-value">oklch(0.96 0.005 250)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.935 0.006 250)')} aria-label="Copy surface sunken color token">
			<div class="color-swatch-fill swatch-surface-sunken" data-token="--color-surface-sunken"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Sunken</span>
				<span class="color-swatch-value">oklch(0.935 0.006 250)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>

		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.14 0.01 250)')} aria-label="Copy text color token">
			<div class="color-swatch-fill swatch-text" data-token="--color-text"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Text Primary</span>
				<span class="color-swatch-value">oklch(0.14 0.01 250)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.40 0.012 250)')} aria-label="Copy text secondary color token">
			<div class="color-swatch-fill swatch-text-secondary" data-token="--color-text-secondary"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Text Secondary</span>
				<span class="color-swatch-value">oklch(0.40 0.012 250)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>

		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.68 0.14 160)')} aria-label="Copy success color token">
			<div class="color-swatch-fill swatch-success" data-token="--color-success"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Success</span>
				<span class="color-swatch-value">oklch(0.68 0.14 160)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.76 0.13 80)')} aria-label="Copy warning color token">
			<div class="color-swatch-fill swatch-warning" data-token="--color-warning"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Warning</span>
				<span class="color-swatch-value">oklch(0.76 0.13 80)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
		<button class="color-swatch" onclick={(e) => copySwatch(e, 'oklch(0.58 0.17 22)')} aria-label="Copy danger color token">
			<div class="color-swatch-fill swatch-danger" data-token="--color-danger"></div>
			<div class="color-swatch-info">
				<span class="color-swatch-name">Danger</span>
				<span class="color-swatch-value">oklch(0.58 0.17 22)</span>
			</div>
			<div class="copy-confirm">Copied</div>
		</button>
	</div>
</section>

<style>
	.color-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1px;
		background: var(--color-border);
	}

	.color-swatch {
		background: var(--color-surface);
		overflow: hidden;
		cursor: pointer;
		transition: background var(--duration-fast) ease;
		position: relative;
		border: none;
		text-align: left;
		font-family: inherit;
		padding: 0;
	}

	.color-swatch:hover {
		background: var(--color-surface-raised);
	}

	.color-swatch:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: -2px;
		z-index: 1;
	}

	.color-swatch-fill {
		height: 80px;
		position: relative;
	}

	.color-swatch-fill::after {
		content: attr(data-token);
		position: absolute;
		bottom: 6px;
		right: 8px;
		font-family: var(--font-mono);
		font-size: 8px;
		letter-spacing: 0.06em;
		color: oklch(1 0 0 / 0.6);
		mix-blend-mode: difference;
	}

	.color-swatch-info {
		padding: var(--space-3) var(--space-4);
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.color-swatch-name {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.color-swatch-value {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.copy-confirm {
		position: absolute;
		inset: 0;
		background: oklch(0.69 0.16 41 / 0.9);
		display: flex;
		align-items: center;
		justify-content: center;
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text-on-accent);
		opacity: 0;
		transition: opacity var(--duration-fast) ease;
		pointer-events: none;
	}

	.color-swatch.copied .copy-confirm {
		opacity: 1;
	}

	.swatch-accent { background: var(--color-accent); }
	.swatch-accent-light { background: var(--color-accent-light); }
	.swatch-accent-dark { background: var(--color-accent-dark); }
	.swatch-accent-muted { background: var(--color-accent-muted); }
	.swatch-surface { background: var(--color-surface); border-bottom: 1px solid var(--color-border); }
	.swatch-surface-raised { background: var(--color-surface-raised); border-bottom: 1px solid var(--color-border); }
	.swatch-surface-sunken { background: var(--color-surface-sunken); border-bottom: 1px solid var(--color-border); }
	.swatch-text { background: var(--color-text); }
	.swatch-text-secondary { background: var(--color-text-secondary); }
	.swatch-success { background: var(--color-success); }
	.swatch-warning { background: var(--color-warning); }
	.swatch-danger { background: var(--color-danger); }
</style>
