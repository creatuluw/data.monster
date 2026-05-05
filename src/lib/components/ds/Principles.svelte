<section class="section" id="principles" aria-labelledby="principles-title">
	<div class="section-header entrance">
		<span class="section-number">06</span>
		<h2 class="section-title" id="principles-title">Design Principles</h2>
	</div>

	<div class="principles-grid entrance entrance-d1">
		<div class="principle">
			<span class="principle-ref">GL-P01</span>
			<h3 class="principle-title">Structure enables focus</h3>
			<p class="principle-desc">
				A consistent interface lets teachers focus on teaching and students
				on learning. When every pattern is predictable — buttons, colors,
				layouts — cognitive load drops and attention stays on what matters:
				student progress.
			</p>
		</div>
		<div class="principle">
			<span class="principle-ref">GL-P02</span>
			<h3 class="principle-title">Clarity is care</h3>
			<p class="principle-desc">
				Teachers work under time pressure with many students. Ambiguous
				labels, unclear states, and visual noise aren't minor flaws — they
				steal time from the people who need it most. Precision in the
				interface is respect for the educator.
			</p>
		</div>
		<div class="principle">
			<span class="principle-ref">GL-P03</span>
			<h3 class="principle-title">Motion conveys progress</h3>
			<p class="principle-desc">
				Every transition answers a question: what just changed? When a
				registration is saved or a report generated, the motion confirms
				the action. Industrial easing. No bounce. No decoration. Progress
				feels deliberate.
			</p>
		</div>
		<div class="principle">
			<span class="principle-ref">GL-P04</span>
			<h3 class="principle-title">Tinted neutrality</h3>
			<p class="principle-desc">
				Pure gray is a missed opportunity. Neutrals tinted toward the
				system's hue create cohesion below conscious perception. The cold
				blue undertone gives the interface a calm, focused character suited
				for long sessions of data entry and review.
			</p>
		</div>
	</div>
</section>

<style>
	.principles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: var(--space-6);
	}

	.principle {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-8);
		transition: box-shadow var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.principle:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.principle-ref {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
		margin-bottom: var(--space-4);
	}

	.principle-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-3);
	}

	.principle-desc {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 38ch;
	}

	@media (max-width: 640px) {
		.principles-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
