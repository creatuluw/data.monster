<script lang="ts">
	import { Home, LogIn } from "lucide-svelte";
	import Button from "./Button.svelte";

	let {
		code,
		title,
		message,
		homeUrl = "/",
		loginUrl,
		showHomeButton = true,
		showLoginButton = false,
		children,
	}: {
		code: number;
		title: string;
		message: string;
		homeUrl?: string;
		loginUrl?: string;
		showHomeButton?: boolean;
		showLoginButton?: boolean;
		children?: import("svelte").Snippet;
	} = $props();

	let codeColor = $derived(
		code === 401
			? "var(--color-warning)"
			: code === 403
				? "var(--color-danger)"
				: code === 404
					? "var(--color-text-tertiary)"
					: "var(--color-danger)",
	);
</script>

<div class="error-page">
	<div class="error-container entrance">
		<div class="error-code" style:color={codeColor}>{code}</div>

		<div class="error-divider"></div>

		<h1 class="error-title">{title}</h1>
		<p class="error-message">{message}</p>

		{#if children}
			<div class="error-extra">
				{@render children()}
			</div>
		{/if}

		<div class="error-actions">
			{#if showHomeButton}
				<Button variant="secondary" href={homeUrl}>
					<Home size={14} />
					Terug naar home
				</Button>
			{/if}
			{#if showLoginButton && loginUrl}
				<Button variant="primary" href={loginUrl}>
					<LogIn size={14} />
					Inloggen
				</Button>
			{/if}
		</div>
	</div>
</div>

<style>
	.error-page {
		min-height: 100dvh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-24) var(--gutter);
		background: var(--color-surface-sunken);
	}

	.error-container {
		width: min(480px, calc(100vw - 2 * var(--gutter)));
		background: var(--color-surface);
		padding: var(--space-12) var(--space-8);
		border: 1px solid var(--color-border);
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.error-code {
		font-family: var(--font-display);
		font-size: var(--text-4xl);
		font-weight: 700;
		line-height: 1;
		letter-spacing: -0.04em;
	}

	.error-divider {
		width: 48px;
		height: 2px;
		background: var(--color-border-strong);
		margin: var(--space-6) 0;
	}

	.error-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
		margin-bottom: var(--space-3);
	}

	.error-message {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 36ch;
	}

	.error-extra {
		margin-top: var(--space-6);
		width: 100%;
		text-align: left;
	}

	.error-actions {
		margin-top: var(--space-8);
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}
</style>
