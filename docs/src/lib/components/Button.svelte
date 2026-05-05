<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes, HTMLAnchorAttributes } from 'svelte/elements';

	interface BaseProps {
		variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'accent';
		size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
		fullWidth?: boolean;
		children: Snippet;
	}

	type ButtonProps = BaseProps & {
		href?: never;
	} & HTMLButtonAttributes;

	type LinkProps = BaseProps & {
		href: string;
	} & HTMLAnchorAttributes;

	type Props = ButtonProps | LinkProps;

	let {
		variant = 'primary',
		size = 'sm',
		fullWidth = false,
		href,
		class: className = '',
		children,
		...restProps
	}: Props = $props();

	// Base classes for all buttons
	const baseClasses = 'inline-flex items-center justify-center font-medium transition-colors duration-150 ease-in-out rounded-lg cursor-pointer';

	// Variant classes matching color-demo and app.css button styles
	const variantClasses = {
		primary: 'text-white bg-green-600 hover:bg-green-700 active:bg-green-800 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2',
		secondary: 'text-green-800 bg-green-100 hover:bg-green-200 active:bg-green-300 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2',
		ghost: 'text-slate-600 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-300 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-300 dark:hover:border-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2 shadow-md hover:shadow-lg',
		danger: 'text-white bg-red-600 hover:bg-red-700 active:bg-red-800 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2',
		accent: 'text-white bg-purple-600 hover:bg-purple-700 active:bg-purple-800 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2'
	};

	// Size classes
	const sizeClasses = {
		xs: 'px-2 py-1 text-xs',
		sm: 'px-3 py-1.5 text-sm',
		md: 'px-4 py-2.5 text-base',
		lg: 'px-6 py-3 text-lg',
		xl: 'px-8 py-4 text-xl'
	};

	// Width classes
	const widthClass = fullWidth ? 'w-full' : '';

	// Combined classes
	const buttonClasses = `${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${widthClass} ${className}`;

	// Disabled state handling
	const disabledClasses = 'disabled:bg-slate-400 disabled:text-slate-600 disabled:cursor-not-allowed disabled:opacity-60 disabled:hover:bg-slate-400 disabled:hover:shadow-none';
</script>

{#if href}
	<a {href} class="{buttonClasses}" {...restProps as HTMLAnchorAttributes}>
		{@render children()}
	</a>
{:else}
	<button class="{buttonClasses} {disabledClasses}" {...restProps as HTMLButtonAttributes}>
		{@render children()}
	</button>
{/if}

