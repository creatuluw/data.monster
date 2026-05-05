<script lang="ts">
	import { page } from '$app/stores';
	import { Home, Database, Network, LineChart, Table, Palette, Component, Code2, BarChart3, Camera, Sparkles } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import { tabs } from '$lib/stores/tabs';

	interface Props {
		isOpen?: boolean;
		onClose?: () => void;
	}

	let { isOpen = $bindable(false), onClose }: Props = $props();

	function handleClose() {
		isOpen = false;
		onClose?.();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	// Navigation items
	const mainNavItems = [
		{
			name: 'Dashboard',
			href: '/',
			icon: Home,
			badge: '5'
		},
		{
			name: 'Data',
			href: '/data',
			icon: Database
		},
		{
			name: 'SQL Studio',
			href: '/sql-studio',
			icon: Code2
		},
		{
			name: 'Models',
			href: '/models',
			icon: Network
		},
		{
			name: 'Analysis',
			href: '/analysis',
			icon: LineChart
		},
		{
			name: 'Chart Library',
			href: '/warp-lab/chart-lib',
			icon: Sparkles,
			badge: 'NEW'
		},
		{
			name: 'Chart Examples',
			href: '/warp-lab/chart-examples',
			icon: BarChart3
		},
		{
			name: 'Table View',
			href: '/table-view',
			icon: Table
		},
		{
			name: 'Screenshot Tool',
			href: '/screenshot-tool',
			icon: Camera
		},
		{
			name: 'Design System',
			href: '/design-system',
			icon: Palette
		},
		{
			name: 'UI Kit Demo',
			href: '/color-demo',
			icon: Component
		}
	];

	const teams = [
		{ name: 'Heroicons', initial: 'H' },
		{ name: 'Tailwind Labs', initial: 'T' },
		{ name: 'Workcation', initial: 'W' }
	];

	function isActive(href: string): boolean {
		return $page.url.pathname === href;
	}

	// Handle opening in new tab
	function openInNewTab(e: MouseEvent, href: string, title: string) {
		e.preventDefault();
		e.stopPropagation();
		console.log('➕ Opening sidebar link in new tab:', title);
		
		// Add new tab to the tab bar
		tabs.addTab(title, href);
	}

	// Handle right-click to open in new tab
	function handleContextMenu(e: MouseEvent, href: string, title: string) {
		e.preventDefault();
		openInNewTab(e, href, title);
	}
</script>

<!-- Backdrop -->
{#if isOpen}
	<div
		class="fixed inset-0 bg-black/50 z-40 transition-opacity"
		onclick={handleBackdropClick}
		onkeydown={(e) => e.key === 'Escape' && handleClose()}
		role="button"
		tabindex="-1"
		aria-label="Close sidebar"
	></div>
{/if}

<!-- Sidebar -->
<div
	class="fixed inset-y-0 left-0 z-50 w-80 transform transition-transform duration-300 ease-in-out {isOpen
		? 'translate-x-0'
		: '-translate-x-full'}"
>
	<div
		class="relative flex h-full flex-col gap-y-5 overflow-y-auto border-r border-gray-200 bg-white px-6 dark:border-white/10 dark:bg-gray-900 dark:before:pointer-events-none dark:before:absolute dark:before:inset-0 dark:before:bg-black/10"
	>
		<!-- Logo/Header -->
		<div class="relative flex h-16 shrink-0 items-center justify-between">
			<span class="text-lg font-semibold text-gray-900 dark:text-white">Warphead</span>
			<button
				onclick={handleClose}
				class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
				aria-label="Close sidebar"
			>
				<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
				</svg>
			</button>
		</div>

		<!-- Navigation -->
		<nav class="relative flex flex-1 flex-col">
			<ul role="list" class="flex flex-1 flex-col gap-y-7">
				<!-- Main Navigation -->
				<li>
					<ul role="list" class="-mx-2 space-y-1">
						{#each mainNavItems as item}
							{@const Icon = item.icon}
							<li>
								<div class="group/nav relative">
									<a
										href={item.href}
										onclick={handleClose}
										oncontextmenu={(e) => handleContextMenu(e, item.href, item.name)}
										class="flex gap-x-2.5 rounded-md p-1.5 text-xs/5 font-semibold {isActive(item.href)
											? 'bg-gray-50 dark:bg-white/5 text-indigo-600 dark:text-white'
											: 'text-gray-700 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-white/5'}"
									>
										<Icon
											class="size-5 shrink-0 {isActive(item.href)
												? 'text-indigo-600 dark:text-white'
												: 'text-gray-400 group-hover/nav:text-indigo-600 dark:group-hover/nav:text-white'}"
										/>
										<span class="truncate">{item.name}</span>
										{#if item.badge}
											<span
												aria-hidden="true"
												class="ml-auto w-8 min-w-max rounded-full bg-white px-2 py-0.5 text-center text-[0.625rem]/4 font-medium whitespace-nowrap text-gray-600 outline-1 -outline-offset-1 outline-gray-200 dark:bg-gray-900 dark:text-white dark:outline-white/15"
											>
												{item.badge}
											</span>
										{/if}
									</a>
								</div>
							</li>
						{/each}
					</ul>
				</li>

				<!-- Teams Section -->
				<li>
					<div class="text-xs/5 font-semibold text-gray-400">Your teams</div>
					<ul role="list" class="-mx-2 mt-2 space-y-1">
						{#each teams as team}
							<li>
								<button
									type="button"
									class="group flex gap-x-2.5 rounded-md p-1.5 text-xs/5 font-semibold text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-400 dark:hover:bg-white/5 dark:hover:text-white w-full text-left"
								>
									<span
										class="flex size-5 shrink-0 items-center justify-center rounded-lg border border-gray-200 bg-white text-[0.625rem] font-medium text-gray-400 group-hover:border-indigo-600 group-hover:text-indigo-600 dark:border-white/15 dark:bg-white/5 dark:group-hover:border-white/20 dark:group-hover:text-white"
									>
										{team.initial}
									</span>
									<span class="truncate">{team.name}</span>
								</button>
							</li>
						{/each}
					</ul>
				</li>

				<!-- User Profile -->
				<li class="-mx-6 mt-auto">
					<button
						type="button"
						class="flex items-center gap-x-3 px-6 py-2.5 text-xs/5 font-semibold text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-white/5 w-full text-left"
					>
						<img
							src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
							alt=""
							class="size-7 rounded-full bg-gray-50 outline -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10"
						/>
						<span class="sr-only">Your profile</span>
						<span aria-hidden="true">Tom Cook</span>
					</button>
				</li>
			</ul>
		</nav>
	</div>
</div>

