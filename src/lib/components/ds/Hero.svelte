<header class="hero grid-bg">
	<div class="hero-classification entrance">
		<span class="class-line"></span>
		Design System Specification — v2.0
	</div>
	<h1 class="hero-title entrance entrance-d1">
		Clarity through<br /><span class="accent-word">structure</span>.
	</h1>
	<p class="hero-desc entrance entrance-d2">
		A design system for teaching, learning, and progress tracking — where
		clarity reduces cognitive load and every element serves the people
		registering, reviewing, and reporting student growth. No decoration
		without purpose. No ambiguity without consequence. Built for the
		teachers who track progress for every student.
	</p>
	<div class="hero-specs entrance entrance-d3">
		<div class="hero-spec">
			<span class="hero-spec-label">Color Space</span>
			<span class="hero-spec-value">OKLCH</span>
		</div>
		<div class="hero-spec">
			<span class="hero-spec-label">Type Scale</span>
			<span class="hero-spec-value">1.25×</span>
		</div>
		<div class="hero-spec">
			<span class="hero-spec-label">Grid Base</span>
			<span class="hero-spec-value">4 px</span>
		</div>
		<div class="hero-spec">
			<span class="hero-spec-label">Fonts</span>
			<span class="hero-spec-value">2</span>
		</div>
	</div>
</header>

<style>
	.grid-bg {
		position: relative;
	}

	.grid-bg::before {
		content: "";
		position: absolute;
		inset: 0;
		background-image:
			repeating-linear-gradient(90deg, var(--grid-color) 0 1px, transparent 1px var(--grid-spacing)),
			repeating-linear-gradient(0deg, var(--grid-color) 0 1px, transparent 1px var(--grid-spacing));
		background-size: var(--grid-spacing) var(--grid-spacing);
		pointer-events: none;
		z-index: 0;
		opacity: 0;
		transition: opacity var(--duration-slow) ease;
	}

	.grid-bg:hover::before {
		opacity: 1;
	}

	.hero {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-24) var(--gutter) var(--space-16);
		position: relative;
	}

	.hero-classification {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-6);
	}

	.class-line {
		width: 32px;
		height: 1px;
		background: var(--color-border-strong);
	}

	.hero-title {
		font-family: var(--font-display);
		font-size: var(--text-4xl);
		font-weight: 700;
		line-height: var(--leading-tight);
		letter-spacing: -0.03em;
		color: var(--color-text);
		max-width: 16ch;
	}

	.accent-word {
		color: var(--color-accent);
	}

	.hero-desc {
		margin-top: var(--space-8);
		font-size: var(--text-md);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 52ch;
	}

	.hero-specs {
		margin-top: var(--space-12);
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
		gap: 1px;
		background: var(--color-border);
		max-width: 600px;
	}

	.hero-spec {
		background: var(--color-surface);
		padding: var(--space-4) var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.hero-spec-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-text-tertiary);
	}

	.hero-spec-value {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	@media (max-width: 640px) {
		.hero {
			padding-top: var(--space-16);
		}
		.hero-specs {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
