<script lang="ts">
	import { onMount } from 'svelte';
	
	interface Props {
		text: string;
		position?: 'top' | 'bottom' | 'left' | 'right';
		size?: 'sm' | 'md' | 'lg';
		delay?: number;
		children?: any;
	}
	
	let { 
		text, 
		position = 'top', 
		size = 'sm', 
		delay = 200,
		children 
	}: Props = $props();
	
	let showTooltip = $state(false);
	let timeoutId: number | null = null;
	
	// Size variants
	const sizeClasses = {
		sm: 'text-xs px-2 py-1',
		md: 'text-sm px-3 py-1.5',
		lg: 'text-base px-4 py-2'
	};
	
	// Position variants
	const positionClasses = {
		top: 'bottom-full left-1/2 -translate-x-1/2 mb-2',
		bottom: 'top-full left-1/2 -translate-x-1/2 mt-2',
		left: 'right-full top-1/2 -translate-y-1/2 mr-2',
		right: 'left-full top-1/2 -translate-y-1/2 ml-2'
	};
	
	// Arrow position variants
	const arrowClasses = {
		top: 'top-full left-1/2 -translate-x-1/2 border-t-slate-900 dark:border-t-slate-700',
		bottom: 'bottom-full left-1/2 -translate-x-1/2 border-b-slate-900 dark:border-b-slate-700',
		left: 'left-full top-1/2 -translate-y-1/2 border-l-slate-900 dark:border-l-slate-700',
		right: 'right-full top-1/2 -translate-y-1/2 border-r-slate-900 dark:border-r-slate-700'
	};
	
	function handleMouseEnter() {
		if (timeoutId) {
			clearTimeout(timeoutId);
		}
		timeoutId = window.setTimeout(() => {
			showTooltip = true;
		}, delay);
	}
	
	function handleMouseLeave() {
		if (timeoutId) {
			clearTimeout(timeoutId);
		}
		showTooltip = false;
	}
	
	onMount(() => {
		return () => {
			if (timeoutId) {
				clearTimeout(timeoutId);
			}
		};
	});
</script>

<div 
	role="presentation"
	class="relative inline-block"
	onmouseenter={handleMouseEnter}
	onmouseleave={handleMouseLeave}
>
	{@render children?.()}
	
	{#if showTooltip}
		<div 
			class="absolute z-50 pointer-events-none {positionClasses[position]}"
			role="tooltip"
		>
			<div class="bg-slate-900 dark:bg-slate-700 text-white rounded shadow-lg whitespace-nowrap {sizeClasses[size]}">
				{text}
			</div>
			<!-- Arrow -->
			<div class="absolute {arrowClasses[position]} w-0 h-0 border-4 border-transparent"></div>
		</div>
	{/if}
</div>

<style>
	/* Ensure tooltip doesn't interfere with pointer events */
	:global(.tooltip-wrapper) {
		display: inline-block;
	}
</style>

