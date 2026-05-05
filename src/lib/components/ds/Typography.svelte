<section class="section" id="type" aria-labelledby="type-title">
	<div class="section-header entrance">
		<span class="section-number">02</span>
		<h2 class="section-title" id="type-title">Typography</h2>
	</div>

	<div class="type-specimen entrance entrance-d1">
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-4xl</span>
				<span class="type-token-value">clamp(3.2, 7vw, 5.06)</span>
			</div>
			<span class="type-sample-display specimen-4xl">Glacier</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-3xl</span>
				<span class="type-token-value">clamp(2.8, 5.5vw, 3.815)</span>
			</div>
			<span class="type-sample-display specimen-3xl">Specification</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-2xl</span>
				<span class="type-token-value">2.441rem / 700</span>
			</div>
			<span class="type-sample-display specimen-2xl">Subsystem Header</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-xl</span>
				<span class="type-token-value">1.953rem / 600</span>
			</div>
			<span class="type-sample-display specimen-xl">Component Title</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-lg</span>
				<span class="type-token-value">1.563rem / 600</span>
			</div>
			<span class="type-sample-display specimen-lg">Section Label</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-md</span>
				<span class="type-token-value">1.25rem / 500</span>
			</div>
			<span class="type-sample-display specimen-md">Subsection</span>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-base</span>
				<span class="type-token-value">1rem / 400</span>
			</div>
			<p class="type-sample-body specimen-base">
				Manrope brings geometric clarity with generous spacing and a
				friendly character. Its clean lines and balanced x-height keep
				paragraphs comfortable for long sessions, while the slightly
				informal touch keeps text from feeling clinical or cold.
			</p>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-sm</span>
				<span class="type-token-value">0.8125rem / 400</span>
			</div>
			<p class="type-sample-body specimen-sm">
				Small body for metadata, timestamps, and secondary readings.
				Subordinate to the main flow but never disposable.
			</p>
		</div>
		<div class="type-row">
			<div class="type-meta">
				<span class="type-token-name">--text-xs</span>
				<span class="type-token-value">0.6875rem / 600</span>
			</div>
			<span class="type-sample-display specimen-xs">Classification / Reference</span>
		</div>
	</div>

	<div class="font-pair-grid entrance entrance-d2">
		<div class="font-pair-card">
			<span class="font-pair-label">Display Font</span>
			<h3 class="font-pair-name">Source Serif 4</h3>
			<span class="font-pair-classification">Variable optical serif / Angle-accent terminals</span>
			<p class="font-pair-sample display-sample">
				ABCDEFGHIJKLMNOPQRSTUVWXYZ<br />
				abcdefghijklmnopqrstuvwxyz<br />
				0123456789<br />
				<em>Precision. Warmth. Memory.</em>
			</p>
		</div>
		<div class="font-pair-card">
			<span class="font-pair-label">Body Font</span>
			<h3 class="font-pair-name body-font">Manrope</h3>
			<span class="font-pair-classification">Variable sans / Geometric / Expanded metrics</span>
			<p class="font-pair-sample body-sample">
				abcdefghijklmnopqrstuvwxyz<br />
				ABCDEFGHIJKLMNOPQRSTUVWXYZ<br />
				0123456789<br />
				Clear. Open. Kind.
			</p>
		</div>
	</div>
</section>

<style>
	.type-specimen {
		display: flex;
		flex-direction: column;
	}

	.type-row {
		display: grid;
		grid-template-columns: 140px 1fr;
		gap: var(--space-6);
		align-items: baseline;
		padding: var(--space-6) 0;
		border-bottom: 1px dashed var(--color-border);
	}

	.type-row:last-child {
		border-bottom: none;
	}

	.type-meta {
		display: flex;
		flex-direction: column;
		gap: 3px;
		position: relative;
	}

	.type-token-name {
		font-family: var(--font-mono);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.08em;
		color: var(--color-accent);
	}

	.type-token-value {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.type-sample-display {
		font-family: var(--font-display);
		line-height: var(--leading-tight);
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	.type-sample-body {
		font-family: var(--font-body);
		line-height: var(--leading-normal);
		color: var(--color-text);
		max-width: 62ch;
	}

	.specimen-4xl { font-size: var(--text-4xl); font-weight: 800; letter-spacing: -0.04em; }
	.specimen-3xl { font-size: var(--text-3xl); font-weight: 700; letter-spacing: -0.03em; }
	.specimen-2xl { font-size: var(--text-2xl); font-weight: 700; }
	.specimen-xl { font-size: var(--text-xl); font-weight: 600; }
	.specimen-lg { font-size: var(--text-lg); font-weight: 600; }
	.specimen-md { font-size: var(--text-md); font-weight: 500; }
	.specimen-base { font-size: var(--text-base); font-weight: 400; }
	.specimen-sm { font-size: var(--text-sm); font-weight: 400; }
	.specimen-xs { font-size: var(--text-xs); font-weight: 600; letter-spacing: 0.08em; }

	.font-pair-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1px;
		background: var(--color-border);
		margin-top: var(--space-12);
	}

	.font-pair-card {
		background: var(--color-surface);
		padding: var(--space-8);
	}

	.font-pair-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-4);
	}

	.font-pair-name {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
		margin-bottom: var(--space-2);
	}

	.body-font {
		font-family: var(--font-body);
	}

	.font-pair-classification {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-6);
	}

	.font-pair-sample {
		font-size: var(--text-md);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 38ch;
	}

	.display-sample {
		font-family: var(--font-display);
		font-weight: 600;
		letter-spacing: -0.02em;
	}

	.body-sample {
		font-family: var(--font-body);
		font-weight: 400;
	}

	@media (max-width: 640px) {
		.type-row {
			grid-template-columns: 1fr;
			gap: var(--space-2);
		}
		.type-meta {
			flex-direction: row;
			gap: var(--space-3);
		}
		.font-pair-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
