<script lang="ts">
	import { app } from '$lib/stores/app.svelte';
	import { PlusCircle, Code, Database, TrendingUp, FileText } from 'lucide-svelte';
</script>

<svelte:head>
	<title>Data Monster</title>
</svelte:head>

<div class="home">
	<div class="hero">
		<span class="brand-mark-lg"></span>
		<h1 class="home-title">Data Monster</h1>
		<p class="home-desc">Connect data, query, explore.</p>
		<div class="home-actions">
			<a href="/connect" class="action-btn">
				<PlusCircle size={16} />
				Connect
			</a>
			{#if app.tables.length > 0}
				<a href="/data" class="action-btn">
					<Database size={16} />
					Data ({app.tables.length})
				</a>
				<a href="/query" class="action-btn">
					<Code size={16} />
					Query
				</a>
				<a href="/analyst" class="action-btn">
					<TrendingUp size={16} />
					Data Analyst
				</a>
				<a href="/pages" class="action-btn">
					<FileText size={16} />
					Pages
				</a>
			{:else}
				<span class="home-locked">Connect data to unlock Query, Data & Analyst</span>
			{/if}
		</div>
	</div>
</div>

<style>
	.home {
		flex: 1;
	}

	.hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		padding-top: 10vh;
	}

	.brand-mark-lg {
		width: 64px;
		height: 64px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-md);
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: var(--color-accent-muted);
	}

	.brand-mark-lg::after {
		content: "";
		width: 24px;
		height: 24px;
		background: var(--color-accent);
		border-radius: var(--radius-xs);
	}

	.home-title {
		margin-top: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-2xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	.home-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.home-actions {
		display: flex;
		align-items: center;
		gap: 1px;
		background: var(--color-surface);
		border: 1px solid var(--color-surface);
		border-radius: var(--radius-xs);
		overflow: hidden;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-6);
		min-height: 36px;
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		line-height: 1.4;
		color: var(--color-accent-dark);
		background: var(--color-accent-muted);
		border: none;
		text-decoration: none;
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.action-btn:hover {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.home-locked {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-3) var(--space-6);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		background: var(--color-surface-raised);
	}
</style>