<footer class="footer">
	<span class="footer-brand">Hoi Design System</span>
	<div class="footer-meta">
		<span>SEED: DS2-GLACIER-7742</span>
		<span>v2.0</span>
		<span>OKLCH</span>
	</div>
	<ul class="footer-links">
		<li><a href="#colors">Colors</a></li>
		<li><a href="#type">Type</a></li>
		<li><a href="#components">Components</a></li>
	</ul>
</footer>

<style>
	.footer {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-12) var(--gutter) var(--space-8);
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: var(--space-4);
		border-top: 1px solid var(--color-border);
	}

	.footer-brand {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text-tertiary);
	}

	.footer-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		color: var(--color-text-tertiary);
		display: flex;
		gap: var(--space-6);
	}

	.footer-links {
		display: flex;
		gap: var(--space-4);
		list-style: none;
	}

	.footer-links a {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		text-decoration: none;
		transition: color var(--duration-fast) ease;
	}

	.footer-links a:hover {
		color: var(--color-accent);
	}

	@media (max-width: 640px) {
		.footer {
			flex-direction: column;
			align-items: flex-start;
		}
		.footer-meta {
			flex-direction: column;
			gap: var(--space-2);
		}
	}
</style>
