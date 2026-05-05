<script lang="ts">
	import { Component, Search, Tag, Copy, Check, Code, X, BookOpen, Loader } from 'lucide-svelte';
	import Button from './Button.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import type { ComponentTemplate } from '$lib/types/component-templates';

	interface Props {
		onSelect?: (template: ComponentTemplate) => void;
		onClose?: () => void;
	}

	let { onSelect, onClose }: Props = $props();

	let searchQuery = $state('');
	let selectedTags = $state<string[]>([]);
	let copiedId = $state<string | null>(null);
	let viewingTemplate = $state<ComponentTemplate | null>(null);
	let codeTab = $state<'html' | 'css' | 'js'>('html');
	let templates = $state<ComponentTemplate[]>([]);
	let isLoading = $state(true);
	let iframeRef = $state<HTMLIFrameElement>();

	// Load templates from database
	onMount(async () => {
		try {
			const result = await invoke<string>('list_component_templates');
			templates = JSON.parse(result);
		} catch (error) {
			console.error('Failed to load component templates:', error);
		} finally {
			isLoading = false;
		}
	});

	// Get all unique tags
	const allTags = $derived(
		Array.from(
			new Set(
				templates
					.filter((t) => t.tags)
					.flatMap((t) => t.tags!.split(',').map((tag) => tag.trim()).filter((tag) => tag))
			)
		).sort()
	);

	// Filtered templates
	const filteredTemplates = $derived.by(() => {
		let filtered = templates;

		// Filter by tags
		if (selectedTags.length > 0) {
			filtered = filtered.filter((t) =>
				selectedTags.every((tag) =>
					t.tags?.split(',').map((t) => t.trim()).includes(tag)
				)
			);
		}

		// Filter by search
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(
				(t) =>
					t.component_name.toLowerCase().includes(query) ||
					(t.description && t.description.toLowerCase().includes(query)) ||
					(t.tags && t.tags.toLowerCase().includes(query))
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

	function handleSelect(template: ComponentTemplate) {
		onSelect?.(template);
	}

	function viewDetails(template: ComponentTemplate) {
		viewingTemplate = template;
		codeTab = 'html';
		
		// Render in iframe
		setTimeout(() => {
			if (iframeRef) {
				const doc = iframeRef.contentDocument || iframeRef.contentWindow?.document;
				if (doc) {
					const frameworkTags = template.frameworks?.split(',').map(f => f.trim()).filter(f => f) || [];
					const hasTailwind = frameworkTags.includes('tailwind');
					const hasAlpine = frameworkTags.includes('alpinejs');
					
					const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  ${hasTailwind ? '<script src="https://cdn.tailwindcss.com"><\/script>' : ''}
  ${hasAlpine ? '<script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"><\/script>' : ''}
  <style>${template.css_code || ''}</style>
</head>
<body class="p-6">
  ${template.html_code}
  <script>${template.js_code || ''}<\/script>
</body>
</html>`;
					
					doc.open();
					doc.write(html);
					doc.close();
				}
			}
		}, 100);
	}

	async function copyCode(code: string) {
		await navigator.clipboard.writeText(code);
		copiedId = 'copy-active';
		setTimeout(() => (copiedId = null), 2000);
	}

	async function copyTemplate(template: ComponentTemplate, event: MouseEvent) {
		event.stopPropagation();
		await navigator.clipboard.writeText(template.html_code);
		copiedId = template.slug;
		setTimeout(() => (copiedId = null), 2000);
	}
</script>

<div class="flex flex-col h-full bg-white dark:bg-slate-900">
	<!-- Header -->
	<div class="px-6 py-4 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 mb-1">
					Component Templates
				</h2>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Choose from {templates.length} UI component templates
				</p>
			</div>
			{#if onClose}
				<Button variant="ghost" size="sm" onclick={onClose}>
					<X class="w-4 h-4" />
				</Button>
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
				placeholder="Search components..."
				class="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500"
			/>
		</div>
	</div>

	<!-- Filters -->
	<div class="px-6 py-3 border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
		<!-- Tag Filters -->
		<div class="flex items-center gap-2">
			<span class="text-xs font-medium text-slate-600 dark:text-slate-400">Tags:</span>
			<div class="flex flex-wrap gap-2">
				{#each allTags.slice(0, 10) as tag}
					<button
						type="button"
						onclick={() => toggleTag(tag)}
						class="inline-flex items-center gap-1 px-2 py-1 text-xs rounded-full transition-colors {selectedTags.includes(
							tag
						)
							? 'bg-indigo-500 text-white'
							: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:border-indigo-400'}"
					>
						<Tag class="w-3 h-3" />
						{tag}
					</button>
				{/each}
			</div>
		</div>

		{#if selectedTags.length > 0 || searchQuery}
			<div class="mt-2 flex items-center justify-between text-xs">
				<span class="text-slate-600 dark:text-slate-400">
					{filteredTemplates.length} template{filteredTemplates.length !== 1 ? 's' : ''} found
				</span>
				<button
					type="button"
					onclick={() => {
						selectedTags = [];
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
		{#if isLoading}
			<div class="flex items-center justify-center h-full">
				<Loader class="w-8 h-8 text-slate-400 animate-spin" />
			</div>
		{:else if filteredTemplates.length === 0}
			<div class="flex items-center justify-center h-full">
				<div class="text-center max-w-md">
					<Component class="w-12 h-12 text-slate-300 dark:text-slate-600 mx-auto mb-3" />
					<p class="text-slate-600 dark:text-slate-400">
						No components match your filters. Try adjusting your search.
					</p>
				</div>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each filteredTemplates as template}
					<div
						class="group relative bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-4 hover:shadow-lg hover:border-indigo-400 dark:hover:border-indigo-600 transition-all cursor-pointer"
						onclick={() => viewDetails(template)}
					>
						<!-- Header -->
						<div class="flex items-start justify-between mb-3">
							<div class="flex-1">
								<div class="flex items-center gap-2 mb-1">
									<Component class="w-4 h-4 text-indigo-500 flex-shrink-0" />
									<h3 class="font-semibold text-slate-900 dark:text-slate-100 text-sm">
										{template.component_name}
									</h3>
								</div>
								<p class="text-xs text-slate-600 dark:text-slate-400 line-clamp-2">
									{template.description}
								</p>
							</div>
							<button
								type="button"
								onclick={(e) => copyTemplate(template, e)}
								class="opacity-0 group-hover:opacity-100 p-1.5 text-slate-400 hover:text-indigo-500 transition-all"
								title="Copy HTML"
							>
								{#if copiedId === template.slug}
									<Check class="w-4 h-4 text-green-500" />
								{:else}
									<Copy class="w-4 h-4" />
								{/if}
							</button>
						</div>

						<!-- Tags -->
						{#if template.tags}
							<div class="flex flex-wrap gap-1 mb-3">
								{#each template.tags.split(',').slice(0, 3) as tag}
									<span
										class="inline-flex items-center px-2 py-0.5 text-xs rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400"
									>
										{tag.trim()}
									</span>
								{/each}
							</div>
						{/if}

						<!-- Frameworks -->
						{#if template.frameworks}
							<div class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
								<Code class="w-3 h-3" />
								{template.frameworks.split(',').map(f => f.trim()).join(', ')}
							</div>
						{/if}

						<!-- Use Template Button -->
						<button
							type="button"
							onclick={(e) => {
								e.stopPropagation();
								handleSelect(template);
							}}
							class="mt-3 w-full px-3 py-1.5 text-xs font-medium text-white bg-indigo-600 hover:bg-indigo-700 rounded transition-colors"
						>
							Use Template
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- Detail View Modal -->
{#if viewingTemplate}
	<div
		class="fixed inset-0 bg-black/60 flex items-center justify-center z-[80] p-4"
		onclick={() => (viewingTemplate = null)}
		role="button"
		tabindex="-1"
	>
		<div
			class="bg-white dark:bg-slate-900 rounded-lg shadow-2xl w-full max-w-6xl h-[85vh] overflow-hidden flex flex-col"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			tabindex="0"
		>
			<!-- Modal Header -->
			<div class="px-6 py-4 border-b border-slate-200 dark:border-slate-700">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
							{viewingTemplate.component_name}
						</h3>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							{viewingTemplate.description}
						</p>
					</div>
					<div class="flex items-center gap-2">
						<Button
							variant="primary"
							size="sm"
							onclick={() => {
								handleSelect(viewingTemplate!);
								viewingTemplate = null;
							}}
						>
							Use Template
						</Button>
						<Button variant="ghost" size="sm" onclick={() => (viewingTemplate = null)}>
							<X class="w-4 h-4" />
						</Button>
					</div>
				</div>
			</div>

			<!-- Modal Content -->
			<div class="flex-1 flex overflow-hidden">
				<!-- Left: Preview -->
				<div class="flex-1 border-r border-slate-200 dark:border-slate-700 overflow-hidden">
					<div class="h-full flex flex-col">
						<div class="px-4 py-2 bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
							<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Preview</span>
						</div>
						<div class="flex-1 overflow-auto bg-slate-50 dark:bg-slate-950">
							<iframe
								bind:this={iframeRef}
								title="Component Preview"
								sandbox="allow-scripts"
								class="w-full h-full border-0"
							></iframe>
						</div>
					</div>
				</div>

				<!-- Right: Code -->
				<div class="w-[500px] flex flex-col overflow-hidden">
					<!-- Code Tabs -->
					<div class="flex border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800">
						<button
							type="button"
							onclick={() => (codeTab = 'html')}
							class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {codeTab === 'html'
								? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
								: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
						>
							HTML
						</button>
						{#if viewingTemplate.css_code}
							<button
								type="button"
								onclick={() => (codeTab = 'css')}
								class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {codeTab === 'css'
									? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
									: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
							>
								CSS
							</button>
						{/if}
						{#if viewingTemplate.js_code}
							<button
								type="button"
								onclick={() => (codeTab = 'js')}
								class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {codeTab === 'js'
									? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
									: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
							>
								JavaScript
							</button>
						{/if}
					</div>

					<!-- Code Content -->
					<div class="flex-1 overflow-auto bg-slate-900 text-slate-100 p-4 relative">
						<button
							type="button"
							onclick={() => copyCode(codeTab === 'html' ? viewingTemplate!.html_code : codeTab === 'css' ? (viewingTemplate!.css_code || '') : (viewingTemplate!.js_code || ''))}
							class="absolute top-2 right-2 px-3 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800 hover:bg-slate-700 rounded transition-colors"
						>
							{#if copiedId === 'copy-active'}
								<Check class="w-3 h-3 inline mr-1" />
								Copied!
							{:else}
								<Copy class="w-3 h-3 inline mr-1" />
								Copy
							{/if}
						</button>
						<pre class="text-sm leading-relaxed overflow-x-auto"><code>{#if codeTab === 'html'}{viewingTemplate.html_code}{:else if codeTab === 'css'}{viewingTemplate.css_code || '/* No CSS code */'}{:else}{viewingTemplate.js_code || '// No JavaScript code'}{/if}</code></pre>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

