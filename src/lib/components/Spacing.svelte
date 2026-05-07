<section class="section" id="spacing" aria-labelledby="spacing-title">
	<div class="section-header entrance">
		<span class="section-number">03</span>
		<h2 class="section-title" id="spacing-title">Spacing Scale</h2>
	</div>

	<div class="spacing-specimen entrance entrance-d1">
		<div class="spacing-row">
			<span class="spacing-label">--space-1</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 4.2%"></div>
			</div>
			<span class="spacing-px">4px</span>
			<span class="spacing-rem">0.25r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-2</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 8.3%"></div>
			</div>
			<span class="spacing-px">8px</span>
			<span class="spacing-rem">0.5r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-3</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 12.5%"></div>
			</div>
			<span class="spacing-px">12px</span>
			<span class="spacing-rem">0.75r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-4</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 16.7%"></div>
			</div>
			<span class="spacing-px">16px</span>
			<span class="spacing-rem">1r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-6</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 25%"></div>
			</div>
			<span class="spacing-px">24px</span>
			<span class="spacing-rem">1.5r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-8</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 33.3%"></div>
			</div>
			<span class="spacing-px">32px</span>
			<span class="spacing-rem">2r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-12</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 50%"></div>
			</div>
			<span class="spacing-px">48px</span>
			<span class="spacing-rem">3r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-16</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 66.7%"></div>
			</div>
			<span class="spacing-px">64px</span>
			<span class="spacing-rem">4r</span>
		</div>
		<div class="spacing-row">
			<span class="spacing-label">--space-24</span>
			<div class="spacing-bar-track">
				<div class="spacing-bar-fill" style="width: 100%"></div>
			</div>
			<span class="spacing-px">96px</span>
			<span class="spacing-rem">6r</span>
		</div>
	</div>
</section>

<style>
	.spacing-specimen {
		display: flex;
		flex-direction: column;
	}

	.spacing-row {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-2) 0;
		border-bottom: 1px dashed var(--color-border);
	}

	.spacing-row:last-child {
		border-bottom: none;
	}

	.spacing-label {
		font-family: var(--font-mono);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.06em;
		color: var(--color-text-secondary);
		min-width: 72px;
		text-align: right;
	}

	.spacing-bar-track {
		flex: 1;
		height: 16px;
		background: var(--color-surface-sunken);
		position: relative;
		overflow: hidden;
	}

	.spacing-bar-fill {
		height: 100%;
		background: var(--color-accent);
		opacity: 0.55;
		transition: width var(--duration-slow) var(--ease-out-expo);
		position: relative;
	}

	.spacing-bar-fill::after {
		content: "";
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: 1px;
		background: var(--color-accent);
		opacity: 1;
	}

	.spacing-px {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
		min-width: 32px;
	}

	.spacing-rem {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-accent-light);
		letter-spacing: 0.02em;
		min-width: 48px;
	}

	@media (max-width: 640px) {
		.spacing-label {
			min-width: 56px;
			font-size: 9px;
		}
		.spacing-rem {
			display: none;
		}
	}
</style>
