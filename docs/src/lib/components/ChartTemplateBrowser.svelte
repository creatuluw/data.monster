<script lang="ts">
	import { BarChart3, Search, Tag, Sparkles, Copy, Check, Code, FileCode, X } from 'lucide-svelte';
	import {
		CHART_TEMPLATES,
		getAllCategories,
		getAllTags,
		type ChartTemplate
	} from '$lib/viz/chart-templates';
	import Button from './Button.svelte';

	interface Props {
		onSelect?: (template: ChartTemplate) => void;
		onClose?: () => void;
	}

	let { onSelect, onClose }: Props = $props();

	let searchQuery = $state('');
	let selectedCategory = $state<ChartTemplate['category'] | 'all'>('all');
	let selectedTags = $state<string[]>([]);
	let copiedId = $state<string | null>(null);
	let viewingTemplate = $state<ChartTemplate | null>(null);
	let implementationTab = $state<'template' | 'example'>('template');

	const categories = getAllCategories();
	const allTags = getAllTags();

	// Filtered templates
	const filteredTemplates = $derived.by(() => {
		let filtered = CHART_TEMPLATES;

		// Filter by category
		if (selectedCategory !== 'all') {
			filtered = filtered.filter((t) => t.category === selectedCategory);
		}

		// Filter by tags
		if (selectedTags.length > 0) {
			filtered = filtered.filter((t) => selectedTags.every((tag) => t.tags.includes(tag)));
		}

		// Filter by search
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(
				(t) =>
					t.name.toLowerCase().includes(query) ||
					t.description.toLowerCase().includes(query) ||
					t.tags.some((tag) => tag.toLowerCase().includes(query))
			);
		}

		return filtered;
	});

	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter((t) => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
	}

	function handleSelect(template: ChartTemplate) {
		onSelect?.(template);
	}

	function viewDetails(template: ChartTemplate) {
		viewingTemplate = template;
		implementationTab = 'template';
	}

	async function copyCode(code: string) {
		await navigator.clipboard.writeText(code);
		copiedId = 'copy-active';
		setTimeout(() => (copiedId = null), 2000);
	}

	async function copyTemplate(template: ChartTemplate, event: MouseEvent) {
		event.stopPropagation();
		await navigator.clipboard.writeText(template.rawCode);
		copiedId = template.id;
		setTimeout(() => (copiedId = null), 2000);
	}

	const categoryLabels: Record<ChartTemplate['category'] | 'all', string> = {
		all: 'All Templates',
		basic: 'Basic',
		comparison: 'Comparison',
		trend: 'Trends',
		distribution: 'Distribution',
		relationship: 'Relationships',
		composition: 'Composition'
	};
</script>

<div class="flex flex-col h-full bg-white dark:bg-slate-900">
	<!-- Header -->
	<div class="px-6 py-4 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-1">
					Chart Templates
				</h2>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Choose from {CHART_TEMPLATES.length} production-ready templates
				</p>
			</div>
			{#if onClose}
				<Button variant="ghost" size="sm" onclick={onClose}> Close </Button>
			{/if}
		</div>

		<!-- Search -->
		<div class="relative">
			<Search
				class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 pointer-events-none"
			/>
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search templates..."
				class="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500"
			/>
		</div>
	</div>

	<!-- Filters -->
	<div class="px-6 py-3 border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
		<!-- Category Pills -->
		<div class="flex items-center gap-2 mb-3">
			<span class="text-xs font-medium text-slate-600 dark:text-slate-400">Category:</span>
			<div class="flex flex-wrap gap-2">
				{#each ['all', ...categories] as category}
					<button
						type="button"
						onclick={() => (selectedCategory = category as any)}
						class="px-3 py-1 text-xs rounded-full transition-colors {selectedCategory ===
						category
							? 'bg-indigo-500 text-white'
							: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:border-indigo-400'}"
					>
						{categoryLabels[category as keyof typeof categoryLabels]}
					</button>
				{/each}
			</div>
		</div>

		<!-- Tag Filters -->
		<div class="flex items-center gap-2">
			<span class="text-xs font-medium text-slate-600 dark:text-slate-400">Tags:</span>
			<div class="flex flex-wrap gap-2">
				{#each allTags.slice(0, 8) as tag}
					<button
						type="button"
						onclick={() => toggleTag(tag)}
						class="inline-flex items-center gap-1 px-2 py-1 text-xs rounded-full transition-colors {selectedTags.includes(
							tag
						)
							? 'bg-purple-500 text-white'
							: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:border-purple-400'}"
					>
						<Tag class="w-3 h-3" />
						{tag}
					</button>
				{/each}
			</div>
		</div>

		{#if selectedTags.length > 0 || selectedCategory !== 'all' || searchQuery}
			<div class="mt-2 flex items-center justify-between text-xs">
				<span class="text-slate-600 dark:text-slate-400">
					{filteredTemplates.length} template{filteredTemplates.length !== 1 ? 's' : ''} found
				</span>
				<button
					type="button"
					onclick={() => {
						selectedTags = [];
						selectedCategory = 'all';
						searchQuery = '';
					}}
					class="text-indigo-600 dark:text-indigo-400 hover:underline"
				>
					Clear filters
				</button>
			</div>
		{/if}
	</div>

	<!-- Template Grid -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if filteredTemplates.length === 0}
			<div class="flex items-center justify-center h-full">
				<div class="text-center max-w-md">
					<BarChart3 class="w-12 h-12 text-slate-300 dark:text-slate-600 mx-auto mb-3" />
					<p class="text-slate-600 dark:text-slate-400">
						No templates match your filters. Try adjusting your search.
					</p>
				</div>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each filteredTemplates as template}
					<div
						class="group relative text-left bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-4 hover:border-indigo-400 dark:hover:border-indigo-500 hover:shadow-lg transition-all duration-200"
					>
						<!-- Header -->
						<div class="flex items-start justify-between mb-2">
							<div
								class="flex items-center gap-2 flex-1 min-w-0 cursor-pointer"
								onclick={() => viewDetails(template)}
								onkeydown={(e) => e.key === 'Enter' && viewDetails(template)}
								role="button"
								tabindex="0"
							>
								<div
									class="p-2 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 text-white"
								>
									<BarChart3 class="w-4 h-4" />
								</div>
								<div class="flex-1 min-w-0">
									<h3
										class="font-semibold text-slate-900 dark:text-slate-100 text-sm truncate group-hover:text-indigo-600 dark:group-hover:text-indigo-400"
									>
										{template.name}
									</h3>
									<span
										class="text-xs text-slate-500 dark:text-slate-400 capitalize inline-block"
									>
										{template.category}
									</span>
								</div>
							</div>
							<button
								type="button"
								onclick={(e) => copyTemplate(template, e)}
								class="p-1.5 rounded text-slate-400 hover:text-indigo-600 hover:bg-indigo-50 dark:hover:bg-indigo-900/20 transition-colors"
								title="Copy code"
							>
								{#if copiedId === template.id}
									<Check class="w-4 h-4 text-green-600" />
								{:else}
									<Copy class="w-4 h-4" />
								{/if}
							</button>
						</div>

						<!-- Description -->
						<div
							class="cursor-pointer"
							onclick={() => viewDetails(template)}
							onkeydown={(e) => e.key === 'Enter' && viewDetails(template)}
							role="button"
							tabindex="0"
						>
							<p class="text-xs text-slate-600 dark:text-slate-400 mb-3 line-clamp-2">
								{template.description}
							</p>

							<!-- Tags -->
							<div class="flex flex-wrap gap-1 mb-3">
								{#each template.tags.slice(0, 3) as tag}
									<span
										class="inline-flex items-center gap-1 px-2 py-0.5 text-xs rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400"
									>
										{tag}
									</span>
								{/each}
								{#if template.tags.length > 3}
									<span
										class="inline-flex items-center gap-1 px-2 py-0.5 text-xs rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400"
									>
										+{template.tags.length - 3}
									</span>
								{/if}
							</div>
						</div>

						<!-- Actions -->
						<div class="flex gap-2">
							<button
								type="button"
								onclick={() => viewDetails(template)}
								class="flex-1 text-center py-1.5 px-3 bg-indigo-50 dark:bg-indigo-900/20 text-indigo-600 dark:text-indigo-400 text-xs font-medium rounded transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900/30"
							>
								View Details
							</button>
							<button
								type="button"
								onclick={() => handleSelect(template)}
								class="flex-1 text-center py-1.5 px-3 bg-purple-50 dark:bg-purple-900/20 text-purple-600 dark:text-purple-400 text-xs font-medium rounded transition-colors hover:bg-purple-100 dark:hover:bg-purple-900/30"
							>
								Use Template
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- Template Detail Modal -->
{#if viewingTemplate}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
		onclick={() => (viewingTemplate = null)}
		onkeydown={(e) => e.key === 'Escape' && (viewingTemplate = null)}
		role="button"
		tabindex="-1"
	>
		<div
			class="w-full max-w-5xl h-[85vh] bg-white dark:bg-slate-900 rounded-lg shadow-2xl flex flex-col"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-label="Template details"
			tabindex="0"
		>
			<!-- Header -->
			<div
				class="px-6 py-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between"
			>
				<div>
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
						{viewingTemplate.name}
					</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						{viewingTemplate.description}
					</p>
				</div>
				<button
					type="button"
					onclick={() => (viewingTemplate = null)}
					class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Tabs -->
			<div class="px-6 border-b border-slate-200 dark:border-slate-700 flex gap-4">
				<button
					type="button"
					onclick={() => (implementationTab = 'template')}
					class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {implementationTab ===
					'template'
						? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
						: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
				>
					<div class="flex items-center gap-2">
						<Code class="w-4 h-4" />
						Raw SveltePlot
					</div>
				</button>
				{#if viewingTemplate.usageExample}
					<button
						type="button"
						onclick={() => (implementationTab = 'example')}
						class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {implementationTab ===
						'example'
							? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
							: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
					>
						<div class="flex items-center gap-2">
							<FileCode class="w-4 h-4" />
							Usage Example
						</div>
					</button>
				{/if}
			</div>

			<!-- Code Content -->
			<div class="flex-1 overflow-hidden p-6">
				<div class="h-full bg-slate-50 dark:bg-slate-950 rounded-lg border border-slate-200 dark:border-slate-800 overflow-hidden flex flex-col">
					<!-- Code Header -->
					<div class="px-4 py-2 bg-slate-100 dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
						<span class="text-xs font-mono text-slate-600 dark:text-slate-400">
							{implementationTab === 'template' ? 'Raw SveltePlot Code (Edit in Code Editor)' : 'Compiled Component (Use in Analysis Views)'}
						</span>
						<button
							type="button"
							onclick={() =>
								copyCode(implementationTab === 'template' ? (viewingTemplate?.rawCode || '') : (viewingTemplate?.usageExample || ''))}
							class="flex items-center gap-1.5 px-2 py-1 text-xs rounded hover:bg-slate-200 dark:hover:bg-slate-800 transition-colors text-slate-700 dark:text-slate-300"
						>
							{#if copiedId === 'copy-active'}
								<Check class="w-3 h-3 text-green-600" />
								<span>Copied!</span>
							{:else}
								<Copy class="w-3 h-3" />
								<span>Copy</span>
							{/if}
						</button>
					</div>

					<!-- Code Display -->
					<div class="flex-1 overflow-auto p-4">
						<pre
							class="text-xs font-mono text-slate-800 dark:text-slate-200 whitespace-pre-wrap">{implementationTab === 'template' ? (viewingTemplate?.rawCode || '') : (viewingTemplate?.usageExample || '')}</pre>
					</div>
				</div>
			</div>

			<!-- Actions -->
			<div class="px-6 py-4 border-t border-slate-200 dark:border-slate-700 flex justify-between items-center">
				<div class="flex flex-wrap gap-2">
					{#each viewingTemplate.tags as tag}
						<span
							class="inline-flex items-center gap-1 px-2 py-1 text-xs rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400"
						>
							<Tag class="w-3 h-3" />
							{tag}
						</span>
					{/each}
				</div>
				<div class="flex gap-2">
					<Button variant="secondary" onclick={() => (viewingTemplate = null)}>
						Close
					</Button>
					<Button
						variant="primary"
						onclick={() => {
							handleSelect(viewingTemplate!);
							viewingTemplate = null;
						}}
					>
						<Sparkles class="w-4 h-4 mr-1.5" />
						Use Template
					</Button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>

