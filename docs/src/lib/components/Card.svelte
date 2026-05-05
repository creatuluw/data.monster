<script lang="ts">
	import { ArrowRight, type Icon } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import { tabs } from '$lib/stores/tabs';

	interface Props {
		title: string;
		description: string;
		href: string;
		icon?: ComponentType<Icon>;
		badge?: string;
		badgeColor?: 'green' | 'blue' | 'purple' | 'orange' | 'cyan';
	}

	let { 
		title, 
		description, 
		href, 
		icon, 
		badge,
		badgeColor = 'green'
	}: Props = $props();

	const badgeClasses = {
		green: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400',
		blue: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400',
		purple: 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400',
		orange: 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400',
		cyan: 'bg-cyan-100 text-cyan-700 dark:bg-cyan-900/30 dark:text-cyan-400'
	};

	const iconBgClasses = {
		green: 'bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-800',
		blue: 'bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-800',
		purple: 'bg-purple-50 border-purple-200 dark:bg-purple-900/20 dark:border-purple-800',
		orange: 'bg-orange-50 border-orange-200 dark:bg-orange-900/20 dark:border-orange-800',
		cyan: 'bg-cyan-50 border-cyan-200 dark:bg-cyan-900/20 dark:border-cyan-800'
	};

	const iconColorClasses = {
		green: 'text-green-600 dark:text-green-400',
		blue: 'text-blue-600 dark:text-blue-400',
		purple: 'text-purple-600 dark:text-purple-400',
		orange: 'text-orange-600 dark:text-orange-400',
		cyan: 'text-cyan-600 dark:text-cyan-400'
	};

	// Handle opening in new tab
	function openInNewTab(e: MouseEvent) {
		console.log('➕ Opening in new tab:', title);
		e.preventDefault();
		e.stopPropagation();
		
		// Add new tab to the tab bar
		tabs.addTab(title, href);
	}

	// Handle right-click to open in new tab
	function handleContextMenu(e: MouseEvent) {
		e.preventDefault();
		openInNewTab(e);
	}
</script>

<div class="group relative h-full">
	<a 
		{href}
		oncontextmenu={handleContextMenu}
		class="relative block overflow-visible rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 transition-all duration-300 ease-out hover:shadow-lg hover:shadow-slate-200/50 dark:hover:shadow-slate-950/50 hover:border-slate-300 dark:hover:border-slate-700 hover:-translate-y-0.5 h-full"
	>

	<!-- Gradient overlay on hover -->
	<div class="absolute inset-0 bg-gradient-to-br from-slate-50/50 to-transparent dark:from-slate-800/30 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
	
	<!-- Content -->
	<div class="relative flex flex-col h-full p-6">
		<!-- Floating icon in top-right -->
		{#if icon}
			{@const IconComponent = icon}
			<div class="absolute top-3 right-3">
				<div class="h-8 w-8 flex items-center justify-center border rounded-lg transition-all duration-300 group-hover:scale-110 {iconBgClasses[badgeColor]}">
					<IconComponent class="w-3.5 h-3.5 transition-colors duration-300 {iconColorClasses[badgeColor]}" />
				</div>
			</div>
		{/if}

		<!-- Badge in bottom-right if present -->
		{#if badge}
			<div class="absolute bottom-3 right-3">
				<div class="text-xs font-semibold inline-flex items-center rounded-full px-2.5 py-1 {badgeClasses[badgeColor]}">
					{badge}
				</div>
			</div>
		{/if}

		<!-- Title and description -->
		<div class="grow space-y-2 mb-4">
			<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 tracking-tight">
				{title}
			</h3>
			<p class="text-sm leading-relaxed text-slate-600 dark:text-slate-400">
				{description}
			</p>
		</div>

		<!-- Action indicator -->
		<div class="flex items-center text-sm font-medium transition-colors duration-300 {iconColorClasses[badgeColor]}">
			<span class="mr-2 group-hover:mr-3 transition-all duration-300">Explore</span>
			<ArrowRight class="w-4 h-4 transition-transform duration-300 group-hover:translate-x-1" />
		</div>
	</div>
	</a>
</div>

