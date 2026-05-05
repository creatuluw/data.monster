<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		onclick?: () => void;
		disabled?: boolean;
		icon?: Snippet;
		children?: Snippet;
		shortcut?: string;
		variant?: 'default' | 'danger';
	}

	let {
		onclick,
		disabled = false,
		icon,
		children,
		shortcut,
		variant = 'default'
	}: Props = $props();

	const variantClasses = {
		default: 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700',
		danger: 'text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950/30'
	};
</script>

<button
	type="button"
	{onclick}
	{disabled}
	class="w-full flex items-center px-3 py-2 text-sm transition-colors
		{variantClasses[variant]}
		{disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}
		focus:outline-none focus:bg-slate-100 dark:focus:bg-slate-700"
	role="menuitem"
>
	{#if icon}
		<span class="mr-2 flex-shrink-0">
			{@render icon()}
		</span>
	{/if}
	
	<span class="flex-1 text-left">
		{#if children}
			{@render children()}
		{/if}
	</span>

	{#if shortcut}
		<span class="ml-4 flex items-center gap-0.5 text-xs text-slate-500 dark:text-slate-400">
			{#each shortcut.split('+') as key}
				<kbd class="inline-flex items-center justify-center min-w-[1.25rem] h-5 px-1 rounded border
					bg-slate-100 dark:bg-slate-700
					border-slate-300 dark:border-slate-600
					text-slate-700 dark:text-slate-300
					font-mono text-[10px]">
					{key}
				</kbd>
			{/each}
		</span>
	{/if}
</button>

