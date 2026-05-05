<script lang="ts">
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		trigger?: Snippet;
		children?: Snippet;
		align?: 'left' | 'right';
		sideOffset?: number;
	}

	let {
		trigger,
		children,
		align = 'left',
		sideOffset = 8
	}: Props = $props();

	let isOpen = $state(false);
	let triggerRef = $state<HTMLButtonElement | null>(null);
	let contentRef = $state<HTMLDivElement | null>(null);

	function toggleMenu() {
		isOpen = !isOpen;
	}

	function closeMenu() {
		isOpen = false;
	}

	function handleClickOutside(event: MouseEvent) {
		if (
			triggerRef &&
			contentRef &&
			!triggerRef.contains(event.target as Node) &&
			!contentRef.contains(event.target as Node)
		) {
			closeMenu();
		}
	}

	function handleEscape(event: KeyboardEvent) {
		if (event.key === 'Escape' && isOpen) {
			closeMenu();
		}
	}

	onMount(() => {
		document.addEventListener('mousedown', handleClickOutside);
		document.addEventListener('keydown', handleEscape);

		return () => {
			document.removeEventListener('mousedown', handleClickOutside);
			document.removeEventListener('keydown', handleEscape);
		};
	});
</script>

<div class="relative inline-block">
	<!-- Trigger Button -->
	<button
		bind:this={triggerRef}
		onclick={toggleMenu}
		type="button"
		class="inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium transition-colors
			bg-white dark:bg-slate-800 
			border border-slate-300 dark:border-slate-700
			text-slate-700 dark:text-slate-300
			hover:bg-slate-50 dark:hover:bg-slate-700
			focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-slate-900"
		aria-expanded={isOpen}
		aria-haspopup="true"
	>
		{#if trigger}
			{@render trigger()}
		{:else}
			<span>Menu</span>
			<svg
				class="ml-2 h-4 w-4 transition-transform {isOpen ? 'rotate-180' : ''}"
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 20 20"
				fill="currentColor"
			>
				<path
					fill-rule="evenodd"
					d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
					clip-rule="evenodd"
				/>
			</svg>
		{/if}
	</button>

	<!-- Dropdown Content -->
	{#if isOpen}
		<div
			bind:this={contentRef}
			class="absolute z-50 mt-{sideOffset / 4} min-w-[16rem] rounded-lg border shadow-lg
				bg-white dark:bg-slate-800
				border-slate-200 dark:border-slate-700
				{align === 'right' ? 'right-0' : 'left-0'}"
			role="menu"
			aria-orientation="vertical"
		>
			<div class="py-1">
				{#if children}
					{@render children()}
				{/if}
			</div>
		</div>
	{/if}
</div>

