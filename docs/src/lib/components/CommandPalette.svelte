<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import {
		Home,
		Database,
		Network,
		LineChart,
		Table,
		Palette,
		Component,
		DatabaseZap,
		Code2,
		BarChart3,
		Camera,
		Upload,
		Link,
		FileText,
		Search,
		TableProperties,
		GitBranch,
		BarChart,
		TrendingUp,
		Layers,
		FunctionSquare,
		Bookmark,
		Eye,
		PlusCircle,
		FileUp,
		Globe,
		Settings,
		PieChart,
		TrendingDown,
		Activity,
		Sparkles
	} from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	interface Props {
		isOpen?: boolean;
	}

	let { isOpen = $bindable(false) }: Props = $props();

	interface CommandItem {
		id: string;
		title: string;
		description?: string;
		path: string;
		icon: ComponentType;
		category: string;
		keywords: string[];
	}

	// All available routes and features
	const commands: CommandItem[] = [
		// Dashboard
		{
			id: 'home',
			title: 'Dashboard',
			description: 'Go to main dashboard',
			path: '/',
			icon: Home,
			category: 'Navigation',
			keywords: ['dashboard', 'home', 'main', 'overview']
		},

		// Data Routes
		{
			id: 'data',
			title: 'Data',
			description: 'Manage your data sources',
			path: '/data',
			icon: Database,
			category: 'Data',
			keywords: ['data', 'sources', 'files', 'databases']
		},
		{
			id: 'data-upload',
			title: 'Upload Data',
			description: 'Upload CSV, JSON, or Parquet files',
			path: '/data/upload',
			icon: Upload,
			category: 'Data',
			keywords: ['upload', 'import', 'csv', 'json', 'parquet', 'file']
		},
		{
			id: 'data-connect',
			title: 'Connect to Database',
			description: 'Connect to PostgreSQL or other databases',
			path: '/data/connect',
			icon: Link,
			category: 'Data',
			keywords: ['connect', 'database', 'postgres', 'postgresql', 'remote']
		},
		{
			id: 'data-remote',
			title: 'Load Remote File',
			description: 'Load data from remote URLs',
			path: '/data/remote',
			icon: Globe,
			category: 'Data',
			keywords: ['remote', 'url', 'http', 'https', 'download', 'external']
		},
		{
			id: 'data-query',
			title: 'Query Data',
			description: 'Run SQL queries on your data',
			path: '/data/query',
			icon: Search,
			category: 'Data',
			keywords: ['query', 'sql', 'search', 'filter']
		},
		{
			id: 'data-create',
			title: 'Create Table',
			description: 'Create a new table manually',
			path: '/data/create',
			icon: PlusCircle,
			category: 'Data',
			keywords: ['create', 'table', 'new', 'schema']
		},
		{
			id: 'data-export',
			title: 'Export Data',
			description: 'Export data to various formats',
			path: '/data/export',
			icon: FileUp,
			category: 'Data',
			keywords: ['export', 'download', 'save', 'csv', 'json', 'parquet']
		},

		// Database Tools
		{
			id: 'database-explorer',
			title: 'Database Explorer',
			description: 'Browse database schema and tables',
			path: '/database-explorer',
			icon: DatabaseZap,
			category: 'Tools',
			keywords: ['database', 'explorer', 'schema', 'browse', 'tables']
		},
		{
			id: 'sql-studio',
			title: 'SQL Studio',
			description: 'Advanced SQL editor and query builder',
			path: '/sql-studio',
			icon: Code2,
			category: 'Tools',
			keywords: ['sql', 'studio', 'editor', 'query', 'code']
		},
		{
			id: 'table-view',
			title: 'Table View',
			description: 'View and edit table data',
			path: '/table-view',
			icon: Table,
			category: 'Tools',
			keywords: ['table', 'view', 'grid', 'data', 'rows']
		},

		// Models
		{
			id: 'models',
			title: 'Models',
			description: 'Manage data models and relationships',
			path: '/models',
			icon: Network,
			category: 'Models',
			keywords: ['models', 'relationships', 'schema']
		},
		{
			id: 'models-datamodel',
			title: 'Data Model Canvas',
			description: 'Visual data model designer',
			path: '/models/datamodel',
			icon: GitBranch,
			category: 'Models',
			keywords: ['datamodel', 'canvas', 'visual', 'designer', 'relationships', 'erd']
		},
		{
			id: 'models-tables',
			title: 'Tables',
			description: 'Manage database tables',
			path: '/models/tables',
			icon: TableProperties,
			category: 'Models',
			keywords: ['tables', 'schema', 'structure']
		},
		{
			id: 'models-dimensions',
			title: 'Dimensions',
			description: 'Define and manage dimensions',
			path: '/models/dimensions',
			icon: Layers,
			category: 'Models',
			keywords: ['dimensions', 'attributes', 'columns']
		},
		{
			id: 'models-metrics',
			title: 'Metrics',
			description: 'Create and manage metrics',
			path: '/models/metrics',
			icon: TrendingUp,
			category: 'Models',
			keywords: ['metrics', 'kpi', 'measures', 'calculations']
		},
		{
			id: 'models-functions',
			title: 'User-Defined Functions',
			description: 'Create custom SQL functions',
			path: '/models/functions',
			icon: FunctionSquare,
			category: 'Models',
			keywords: ['functions', 'udf', 'custom', 'sql', 'macros']
		},
		{
			id: 'models-sql',
			title: 'SQL Models',
			description: 'Manage SQL-based models',
			path: '/models/sql',
			icon: Code2,
			category: 'Models',
			keywords: ['sql', 'models', 'views', 'queries']
		},
		{
			id: 'models-analytics',
			title: 'Analytics Models',
			description: 'Build analytical models',
			path: '/models/analytics',
			icon: Activity,
			category: 'Models',
			keywords: ['analytics', 'analysis', 'insights']
		},
		{
			id: 'models-manage',
			title: 'Manage Models',
			description: 'Organize and configure models',
			path: '/models/manage',
			icon: Settings,
			category: 'Models',
			keywords: ['manage', 'organize', 'settings', 'config']
		},

		// Analysis
		{
			id: 'analysis',
			title: 'Analysis',
			description: 'Create visualizations and insights',
			path: '/analysis',
			icon: LineChart,
			category: 'Analysis',
			keywords: ['analysis', 'charts', 'visualizations', 'insights']
		},
		{
			id: 'chart-lib',
			title: 'Chart Library',
			description: 'Visual chart builder with Evidence.dev-style API',
			path: '/warp-lab/chart-lib',
			icon: Sparkles,
			category: 'Analysis',
			keywords: ['chart', 'library', 'builder', 'visualization', 'evidence', 'gui', 'creator', 'new']
		},
		{
			id: 'analysis-canvas',
			title: 'Analysis Canvas',
			description: 'Interactive analysis workspace',
			path: '/analysis/canvas',
			icon: Palette,
			category: 'Analysis',
			keywords: ['canvas', 'workspace', 'interactive', 'freeform']
		},
		{
			id: 'analysis-charts',
			title: 'Charts',
			description: 'Build custom charts',
			path: '/analysis/charts',
			icon: BarChart,
			category: 'Analysis',
			keywords: ['charts', 'graphs', 'visualizations', 'plot']
		},
		{
			id: 'analysis-components',
			title: 'Chart Components',
			description: 'Library of chart components',
			path: '/analysis/components',
			icon: Component,
			category: 'Analysis',
			keywords: ['components', 'library', 'widgets', 'charts']
		},
		{
			id: 'analysis-slides',
			title: 'Presentation Slides',
			description: 'Create presentation slides',
			path: '/analysis/slides',
			icon: FileText,
			category: 'Analysis',
			keywords: ['slides', 'presentation', 'deck', 'powerpoint']
		},

		// Other Tools
		{
			id: 'bookmarks',
			title: 'Bookmarks',
			description: 'Saved queries and views',
			path: '/bookmarks',
			icon: Bookmark,
			category: 'Tools',
			keywords: ['bookmarks', 'saved', 'favorites', 'starred']
		},
		{
			id: 'findings',
			title: 'Findings',
			description: 'Discoveries and insights',
			path: '/findings',
			icon: Eye,
			category: 'Tools',
			keywords: ['findings', 'insights', 'discoveries', 'notes']
		},
		{
			id: 'chart-examples',
			title: 'Chart Examples',
			description: 'Gallery of chart examples',
			path: '/warp-lab/chart-examples',
			icon: BarChart3,
			category: 'Tools',
			keywords: ['examples', 'gallery', 'templates', 'charts']
		},
		{
			id: 'screenshot-tool',
			title: 'Screenshot Tool',
			description: 'Capture screenshots of the app',
			path: '/screenshot-tool',
			icon: Camera,
			category: 'Tools',
			keywords: ['screenshot', 'capture', 'image', 'snap']
		},

		// Development
		{
			id: 'style-guide',
			title: 'Style Guide',
			description: 'Design system and components',
			path: '/style-guide',
			icon: Palette,
			category: 'Development',
			keywords: ['style', 'guide', 'design', 'system', 'components']
		},
		{
			id: 'color-demo',
			title: 'UI Kit Demo',
			description: 'UI component demonstrations',
			path: '/color-demo',
			icon: Component,
			category: 'Development',
			keywords: ['demo', 'ui', 'kit', 'components', 'colors']
		}
	];

	let searchQuery = $state('');
	let selectedIndex = $state(0);

	// Filter commands based on search query
	const filteredCommands = $derived.by(() => {
		if (!searchQuery.trim()) {
			return commands;
		}

		const query = searchQuery.toLowerCase().trim();
		return commands.filter((cmd) => {
			return (
				cmd.title.toLowerCase().includes(query) ||
				cmd.description?.toLowerCase().includes(query) ||
				cmd.keywords.some((kw) => kw.includes(query)) ||
				cmd.category.toLowerCase().includes(query)
			);
		});
	});

	// Group commands by category
	const groupedCommands = $derived.by(() => {
		const groups = new Map<string, CommandItem[]>();

		filteredCommands.forEach((cmd) => {
			if (!groups.has(cmd.category)) {
				groups.set(cmd.category, []);
			}
			groups.get(cmd.category)!.push(cmd);
		});

		return groups;
	});

	function handleKeydown(e: KeyboardEvent) {
		if (!isOpen) return;

		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
				scrollToSelected();
				break;
			case 'ArrowUp':
				e.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, 0);
				scrollToSelected();
				break;
			case 'Enter':
				e.preventDefault();
				if (filteredCommands[selectedIndex]) {
					executeCommand(filteredCommands[selectedIndex]);
				}
				break;
			case 'Escape':
				e.preventDefault();
				closeModal();
				break;
		}
	}

	function scrollToSelected() {
		const selectedElement = document.querySelector(`[data-command-index="${selectedIndex}"]`);
		if (selectedElement) {
			selectedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
		}
	}

	function executeCommand(command: CommandItem) {
		goto(command.path);
		closeModal();
	}

	function closeModal() {
		isOpen = false;
		searchQuery = '';
		selectedIndex = 0;
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			closeModal();
		}
	}

	// Reset selection when search changes
	$effect(() => {
		if (searchQuery) {
			selectedIndex = 0;
		}
	});

	// Add keyboard listener when component mounts
	$effect(() => {
		if (typeof window !== 'undefined') {
			window.addEventListener('keydown', handleKeydown);
			return () => window.removeEventListener('keydown', handleKeydown);
		}
	});

	// Global keyboard shortcut (Ctrl+K or Cmd+K)
	$effect(() => {
		if (typeof window === 'undefined') return;

		function handleGlobalKeydown(e: KeyboardEvent) {
			if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
				e.preventDefault();
				isOpen = !isOpen;
			}
		}

		window.addEventListener('keydown', handleGlobalKeydown);
		return () => window.removeEventListener('keydown', handleGlobalKeydown);
	});
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 bg-gray-500/25 dark:bg-gray-900/50 z-[100] transition-opacity"
		onclick={handleBackdropClick}
		role="presentation"
	></div>

	<!-- Modal -->
	<div
		class="fixed inset-0 w-screen overflow-y-auto p-4 sm:p-6 md:p-20 z-[101] focus:outline-none"
		tabindex="0"
		role="dialog"
		aria-modal="true"
		aria-labelledby="command-palette-title"
	>
		<div
			class="mx-auto max-w-2xl transform overflow-hidden rounded-xl bg-white/80 shadow-2xl outline-1 outline-black/5 backdrop-blur-sm transition-all dark:bg-gray-900/80 dark:-outline-offset-1 dark:outline-white/10"
		>
			<!-- Search Input -->
			<div class="grid grid-cols-1">
				<!-- svelte-ignore a11y_autofocus -->
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search features, pages, and actions..."
					autofocus
					class="col-start-1 row-start-1 h-12 w-full bg-transparent pr-4 pl-11 text-base text-gray-900 outline-hidden placeholder:text-gray-500 sm:text-sm dark:text-white dark:placeholder:text-gray-400 border-0 focus:ring-0"
				/>
				<svg
					viewBox="0 0 20 20"
					fill="currentColor"
					aria-hidden="true"
					class="pointer-events-none col-start-1 row-start-1 ml-4 size-5 self-center text-gray-900/40 dark:text-gray-500"
				>
					<path
						d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z"
						clip-rule="evenodd"
						fill-rule="evenodd"
					/>
				</svg>
			</div>

			<!-- Results -->
			<div class="max-h-80 scroll-py-2 overflow-y-auto">
				{#if filteredCommands.length === 0}
					<!-- No Results -->
					<div class="px-6 py-14 text-center sm:px-14">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
							aria-hidden="true"
							class="mx-auto size-6 text-gray-900/40 dark:text-gray-500"
						>
							<path
								d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
						</svg>
						<p class="mt-4 text-sm text-gray-900 dark:text-gray-200">
							No results found for "{searchQuery}". Try a different search term.
						</p>
					</div>
				{:else}
					<!-- Grouped Results -->
					<div class="divide-y divide-gray-500/10 dark:divide-white/10">
						{#each [...groupedCommands.entries()] as [category, items], groupIndex}
							<div class="p-2">
								<h2 class="mt-2 mb-2 px-3 text-xs font-semibold text-gray-900 dark:text-gray-200">
									{category}
								</h2>
								<div class="text-sm text-gray-700 dark:text-gray-300">
									{#each items as command, itemIndex}
										{@const globalIndex = filteredCommands.indexOf(command)}
										{@const Icon = command.icon}
										{@const isSelected = globalIndex === selectedIndex}
										<button
											type="button"
											data-command-index={globalIndex}
											onclick={() => executeCommand(command)}
											onmouseenter={() => (selectedIndex = globalIndex)}
											class="group flex w-full cursor-pointer items-center rounded-md px-3 py-2 select-none focus:outline-hidden {isSelected
												? 'bg-gray-900/5 text-gray-900 dark:bg-white/5 dark:text-white'
												: 'hover:bg-gray-900/5 hover:text-gray-900 dark:hover:bg-white/5 dark:hover:text-white'}"
										>
											<Icon
												class="size-6 flex-none {isSelected
													? 'text-gray-900 dark:text-white'
													: 'text-gray-900/40 group-hover:text-gray-900 dark:text-gray-500 dark:group-hover:text-white'}"
											/>
											<div class="ml-3 flex-auto text-left">
												<div class="truncate font-medium">{command.title}</div>
												{#if command.description}
													<div class="truncate text-xs text-gray-500 dark:text-gray-400">
														{command.description}
													</div>
												{/if}
											</div>
											{#if isSelected}
												<span
													aria-hidden="true"
													class="ml-3 flex-none text-xs text-gray-500 dark:text-gray-400"
												>
													↵
												</span>
											{/if}
										</button>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer with keyboard shortcuts -->
			<div
				class="border-t border-gray-500/10 dark:border-white/10 px-4 py-2 text-xs text-gray-500 dark:text-gray-400 flex items-center justify-between"
			>
				<div class="flex items-center gap-4">
					<span class="flex items-center gap-1">
						<kbd
							class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 font-mono"
							>↑</kbd
						>
						<kbd
							class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 font-mono"
							>↓</kbd
						>
						<span>to navigate</span>
					</span>
					<span class="flex items-center gap-1">
						<kbd
							class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 font-mono"
							>↵</kbd
						>
						<span>to select</span>
					</span>
					<span class="flex items-center gap-1">
						<kbd
							class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 font-mono"
							>esc</kbd
						>
						<span>to close</span>
					</span>
				</div>
				<span class="flex items-center gap-1">
					<kbd
						class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 font-mono"
						>Ctrl+K</kbd
					>
					<span>to open</span>
				</span>
			</div>
		</div>
	</div>
{/if}

