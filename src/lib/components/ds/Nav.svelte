<nav class="nav" role="navigation" aria-label="Main navigation">
	<div class="nav-inner">
		<a href="/ui" class="nav-brand">
			<span class="brand-mark"></span>
			Hoi
			<span class="nav-seed">DS2-GLACIER-7742</span>
		</a>
		<ul class="nav-links">
			<li><a href="#colors">Colors</a></li>
			<li><a href="#type">Type</a></li>
			<li><a href="#spacing">Spacing</a></li>
			<li><a href="#components">Components</a></li>
			<li><a href="#motion">Motion</a></li>
			<li><a href="#principles">Principles</a></li>
		</ul>
	</div>
</nav>

<style>
	.nav {
		position: sticky;
		top: 0;
		z-index: 100;
		background: oklch(0.98 0.003 250 / 0.97);
		-webkit-backdrop-filter: blur(12px);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--color-border);
	}

	.nav-inner {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-3) var(--gutter);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.nav-brand {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.brand-mark {
		width: 20px;
		height: 20px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-xs);
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.brand-mark::after {
		content: "";
		width: 6px;
		height: 6px;
		background: var(--color-accent);
		border-radius: 1px;
	}

	.nav-seed {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		color: var(--color-text-tertiary);
		padding: 2px 6px;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		margin-left: var(--space-3);
		vertical-align: middle;
	}

	.nav-links {
		display: flex;
		gap: var(--space-1);
		list-style: none;
	}

	.nav-links a {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.nav-links a:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.nav-links a:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	@media (max-width: 640px) {
		.nav-links {
			display: none;
		}
		.nav-seed {
			display: none;
		}
	}
</style>
