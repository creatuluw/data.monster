<script lang="ts">
	import { analyst } from '$lib/stores/analyst.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { marked } from 'marked';

	marked.setOptions({ breaks: true, gfm: true });

	let messagesEl: HTMLDivElement | undefined = $state();
	let inputEl: HTMLTextAreaElement | undefined = $state();
	let inputText = $state('');
	let editSql = $state('');
	let leftWidth = $state(30);
	let queryHeight = $state(200);
	let resizing: 'horizontal' | 'vertical' | null = $state(null);

	onMount(() => {
		if (analyst.selectedTables.length === 0) {
			goto('/analyst');
			return;
		}
		inputEl?.focus();
	});

	let activeQuery = $derived.by(() => {
		if (analyst.activeQueryId) {
			return analyst.queries.find((q) => q.id === analyst.activeQueryId) ?? null;
		}
		if (analyst.queries.length > 0) {
			return analyst.queries[analyst.queries.length - 1];
		}
		return null;
	});

	$effect(() => {
		if (activeQuery) {
			editSql = activeQuery.sql;
		}
	});

	function handleSend() {
		const text = inputText.trim();
		if (!text || analyst.streaming) return;
		inputText = '';
		analyst.sendMessage(text);
		scrollToBottom();
	}

	function handleSuggestion(text: string) {
		inputText = text;
		handleSend();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}

	function scrollToBottom() {
		requestAnimationFrame(() => {
			if (messagesEl) {
				messagesEl.scrollTop = messagesEl.scrollHeight;
			}
		});
	}

	$effect(() => {
		if (analyst.streamingContent || analyst.messages.length) {
			scrollToBottom();
		}
	});

	type RenderPart = { type: 'text'; content: string } | { type: 'sql'; content: string; partial: boolean };

	function renderParts(content: string): RenderPart[] {
		const parts: RenderPart[] = [];
		const regex = /```sql\s*\n([\s\S]*?)```/g;
		let lastIndex = 0;
		let match;

		while ((match = regex.exec(content)) !== null) {
			if (match.index > lastIndex) {
				const text = content.slice(lastIndex, match.index).trim();
				if (text) parts.push({ type: 'text', content: text });
			}
			parts.push({ type: 'sql', content: match[1].trim(), partial: false });
			lastIndex = match.index + match[0].length;
		}

		if (lastIndex < content.length) {
			const remaining = content.slice(lastIndex);
			const partialMatch = remaining.match(/```sql\s*\n([\s\S]*)$/);
			if (partialMatch && analyst.streaming) {
				const textBefore = remaining.slice(0, remaining.indexOf('```sql')).trim();
				if (textBefore) parts.push({ type: 'text', content: textBefore });
				parts.push({ type: 'sql', content: partialMatch[1].trim(), partial: true });
			} else {
				const text = remaining.trim();
				if (text) parts.push({ type: 'text', content: text });
			}
		}

		return parts;
	}

	function renderMarkdown(text: string): string {
		return marked.parse(text, { async: false }) as string;
	}

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}

	function handleRerun() {
		if (!activeQuery || !editSql.trim()) return;
		analyst.rerunQuery(activeQuery.id, editSql.trim());
	}

	function handleRerunKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
			e.preventDefault();
			handleRerun();
		}
	}

	function scrollToMessage(messageId: string) {
		const el = document.getElementById(`msg-${messageId}`);
		if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
	}

	function getQueryLabel(index: number, query: (typeof analyst.queries)[0]): string {
		const answerNum = analyst.messages
			.filter((m) => m.role === 'assistant')
			.findIndex((m) => m.id === query.messageRefId);
		return `Q${index + 1}` + (answerNum >= 0 ? ` \u00B7 A${answerNum + 1}` : '');
	}

	function startHorizontalResize(e: MouseEvent) {
		e.preventDefault();
		resizing = 'horizontal';
		const container = (e.currentTarget as HTMLElement).parentElement!;

		function onMove(ev: MouseEvent) {
			const rect = container.getBoundingClientRect();
			const pct = ((ev.clientX - rect.left) / rect.width) * 100;
			leftWidth = Math.min(Math.max(pct, 15), 55);
		}

		function onUp() {
			resizing = null;
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		}

		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
	}

	function startVerticalResize(e: MouseEvent) {
		e.preventDefault();
		resizing = 'vertical';
		const rightPanel = document.getElementById('right-panel');
		if (!rightPanel) return;
		const tabBar = rightPanel.querySelector('[data-tab-bar]') as HTMLElement;

		function onMove(ev: MouseEvent) {
			if (!rightPanel || !tabBar) return;
			const panelRect = rightPanel.getBoundingClientRect();
			const tabBarBottom = tabBar.getBoundingClientRect().bottom;
			const y = ev.clientY - tabBarBottom;
			const maxH = panelRect.bottom - tabBarBottom - 60;
			queryHeight = Math.min(Math.max(y, 60), maxH);
		}

		function onUp() {
			resizing = null;
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		}

		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
	}
</script>

<svelte:head>
	<title>Data Analyst — Data Monster</title>
</svelte:head>

<div
	class="flex overflow-hidden {resizing ? 'select-none' : ''}"
	style="height: calc(100% + 2rem); margin: -1rem -1.5rem; width: calc(100% + 3rem);"
>
	<!-- Left Panel: Chat -->
	<div
		class="flex min-w-0 flex-col border-r border-sand-200 bg-sand-50"
		style="width: {leftWidth}%;"
	>
		<!-- Chat Header -->
		<div class="flex shrink-0 items-center justify-between border-b border-sand-200 bg-white px-4 py-3">
			<div class="flex items-center gap-2">
				<div class="flex h-7 w-7 items-center justify-center rounded-md bg-sage-600">
					<svg class="h-3.5 w-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0 1 12 15a9.065 9.065 0 0 0-6.23.693L5 14.5m14.8.8 1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0 1 12 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5" />
					</svg>
				</div>
				<div>
					<h2 class="text-sm font-semibold text-sand-800">Data Analyst</h2>
					<p class="text-[10px] text-sand-400">{analyst.selectedTables.length} table{analyst.selectedTables.length !== 1 ? 's' : ''} in context</p>
				</div>
			</div>
			<a
				href="/analyst"
				class="text-xs text-sand-400 transition-colors hover:text-sage-600"
				title="Change tables"
			>
				<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
					<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
				</svg>
			</a>
		</div>

		<!-- Messages -->
		<div bind:this={messagesEl} class="flex-1 overflow-y-auto px-4 py-3">
			{#if analyst.messages.length === 0 && !analyst.streaming}
				<div class="flex flex-col gap-4 py-6">
					<div class="rounded-lg border border-sand-200 bg-white px-4 py-4 shadow-sm">
						<h3 class="mb-2 text-sm font-semibold text-sand-700">Welcome to Data Analyst</h3>
						<p class="text-xs leading-relaxed text-sand-400"
							>I'm a Flue-powered agent that analyzes your data with SQL. Ask me anything about the
							{analyst.selectedTables.length}
							{analyst.selectedTables.length !== 1 ? 'tables' : 'table'} you selected.</p
						>
					</div>

					<p class="text-[11px] font-medium uppercase tracking-wider text-sand-400">Try asking</p>

					{#each [
						'Give me an overview of the data',
						'What are the key patterns and trends?',
						'Show me summary statistics',
						'What are the outliers?'
					] as suggestion}
						<button
							onclick={() => handleSuggestion(suggestion)}
							class="rounded-lg border border-sand-200 bg-white px-3 py-2 text-left text-xs text-sand-600 shadow-sm transition-all hover:border-sage-300 hover:bg-sage-50"
						>
							{suggestion}
						</button>
					{/each}
				</div>
			{/if}

			{#each analyst.messages as message (message.id)}
				<div id="msg-{message.id}" class="mb-3">
					{#if message.role === 'user'}
						<div class="rounded-lg bg-sage-100 px-3 py-2 text-sm text-sage-900">
							{message.content}
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each renderParts(message.content) as part}
								{#if part.type === 'text'}
									<div class="prose-chat text-sm leading-relaxed">
										{@html renderMarkdown(part.content)}
									</div>
								{:else}
									<div class="rounded-md border border-sand-200 bg-sand-50 px-3 py-2">
										<div class="mb-1 flex items-center justify-between">
											<span class="text-[10px] font-medium uppercase tracking-wider text-sand-400">SQL</span>
											<span class="text-[10px] text-sand-300">{part.content.split('\n').length} line{part.content.split('\n').length !== 1 ? 's' : ''}</span>
										</div>
										<pre class="overflow-x-auto text-xs leading-relaxed text-sand-700"><code>{part.content}</code></pre>
									</div>
								{/if}
							{/each}
						</div>
					{/if}
				</div>
			{/each}

			{#if analyst.streaming && analyst.streamingContent}
				<div class="mb-3">
					<div class="flex flex-col gap-2">
						{#each renderParts(analyst.streamingContent) as part}
							{#if part.type === 'text'}
								<div class="prose-chat text-sm leading-relaxed">
									{@html renderMarkdown(part.content)}
								</div>
							{:else}
								<div class="rounded-md border border-sand-200 bg-sand-50 px-3 py-2">
									<div class="mb-1 flex items-center justify-between">
										<span class="text-[10px] font-medium uppercase tracking-wider text-sand-400">SQL</span>
										{#if part.partial}
											<span class="text-[10px] text-copper-500">writing...</span>
										{/if}
									</div>
									<pre class="overflow-x-auto text-xs leading-relaxed text-sand-700"><code>{part.content}{#if part.partial}<span class="animate-pulse text-sage-400">|</span>{/if}</code></pre>
								</div>
							{/if}
						{/each}
						<span class="inline-block h-4 w-1 animate-pulse bg-sage-400"></span>
					</div>
				</div>
			{:else if analyst.streaming}
				<div class="mb-3 flex items-center gap-2 text-sand-400">
					<svg class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
						<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
						<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
					</svg>
					<span class="text-xs">Thinking...</span>
				</div>
			{/if}
		</div>

		<!-- Input -->
		<div class="shrink-0 border-t border-sand-200 bg-white px-4 py-3">
			<div class="flex items-end gap-2">
				<textarea
					bind:this={inputEl}
					bind:value={inputText}
					onkeydown={handleKeydown}
					placeholder="Ask about your data..."
					rows="1"
					disabled={analyst.streaming}
					class="max-h-24 min-h-[36px] flex-1 resize-none rounded-lg border border-sand-200 bg-white px-3 py-2 text-sm leading-relaxed text-sand-800 shadow-sm transition-colors placeholder:text-sand-300 focus:border-sage-400 focus:outline-none focus:ring-2 focus:ring-sage-200 disabled:opacity-50"
					oninput={() => {
						if (inputEl) {
							inputEl.style.height = 'auto';
							inputEl.style.height = Math.min(inputEl.scrollHeight, 96) + 'px';
						}
					}}
				></textarea>
				<button
					onclick={handleSend}
					disabled={!inputText.trim() || analyst.streaming}
					aria-label="Send message"
					class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-sage-600 text-white shadow-sm transition-all hover:bg-sage-700 disabled:cursor-not-allowed disabled:opacity-40"
				>
					<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0 1 21.485 12 59.77 59.77 0 0 1 3.27 20.876L5.999 12Zm0 0h7.5" />
					</svg>
				</button>
			</div>
			<p class="mt-1 text-[10px] text-sand-300">Enter to send, Shift+Enter for new line</p>
		</div>
	</div>

	<!-- Horizontal Resize Handle -->
	<div
		class="group relative z-10 flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-sand-200 transition-colors hover:bg-sage-400 {resizing === 'horizontal' ? 'bg-sage-500' : ''}"
		onmousedown={startHorizontalResize}
		role="separator"
		aria-orientation="vertical"
	>
		<div class="h-8 w-0.5 rounded-full bg-sand-300 transition-colors group-hover:bg-white {resizing === 'horizontal' ? 'bg-white' : ''}"></div>
	</div>

	<!-- Right Panel: Data -->
	<div id="right-panel" class="flex min-w-0 flex-1 flex-col bg-white">
		{#if analyst.queries.length > 0}
			<!-- Tab Bar -->
			<div data-tab-bar class="flex shrink-0 items-center overflow-x-auto border-b border-sand-200 bg-sand-50">
				{#each analyst.queries as query, i}
					{@const label = getQueryLabel(i, query)}
					<button
						onclick={() => analyst.setActiveQuery(query.id)}
						class="relative flex shrink-0 items-center gap-1.5 border-b-2 px-3 py-2 text-xs font-medium transition-colors {query.id === analyst.activeQueryId
							? 'border-sage-600 bg-white text-sage-700'
							: 'border-transparent text-sand-400 hover:bg-sand-100 hover:text-sand-600'}"
					>
						<span>{label}</span>
						{#if query.error}
							<span class="h-1.5 w-1.5 rounded-full bg-red-400"></span>
						{:else if query.result}
							<span class="h-1.5 w-1.5 rounded-full bg-sage-400"></span>
						{/if}
					</button>
				{/each}
			</div>

			{#if activeQuery}
				<!-- Query Editor (editable, resizable height) -->
				<div class="flex shrink-0 flex-col border-b border-sand-200 bg-sand-50 overflow-hidden" style="height: {queryHeight}px;">
					<!-- Header -->
					<div class="flex shrink-0 items-center justify-between px-4 py-2">
						<div class="flex items-center gap-2">
							<h3 class="text-xs font-semibold text-sand-600">Query</h3>
							{#if activeQuery.error}
								<span class="rounded-full bg-red-100 px-2 py-0.5 text-[10px] font-medium text-red-700">Error</span>
							{:else if activeQuery.result}
								<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[10px] font-medium text-sage-700">
									{activeQuery.result.rows.length} of {activeQuery.result.totalRows.toLocaleString()} rows
								</span>
							{/if}
							<button
								onclick={() => scrollToMessage(activeQuery.messageRefId)}
								class="rounded bg-sand-100 px-1.5 py-0.5 text-[10px] font-medium text-sand-400 transition-colors hover:bg-sand-200 hover:text-sand-600"
								title="Jump to answer in chat"
							>
								ref:{activeQuery.messageRefId.slice(0, 6)}
							</button>
						</div>
						<div class="flex items-center gap-2">
							{#if activeQuery.queryTime > 0}
								<span class="text-[10px] text-sand-400">{activeQuery.queryTime.toFixed(0)}ms</span>
							{/if}
							<button
								onclick={handleRerun}
								disabled={analyst.streaming || !editSql.trim()}
								class="flex items-center gap-1 rounded-md bg-sage-600 px-2.5 py-1 text-[10px] font-medium text-white shadow-sm transition-all hover:bg-sage-700 disabled:opacity-40"
							>
								<svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
									<path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
								</svg>
								Run
							</button>
						</div>
					</div>
					<!-- Textarea -->
					<textarea
						bind:value={editSql}
						onkeydown={handleRerunKeydown}
						class="flex-1 min-h-0 w-full resize-none bg-sand-50 px-4 py-1 font-mono text-xs leading-relaxed text-sand-700 focus:outline-none focus:ring-1 focus:ring-inset focus:ring-sage-400"
					></textarea>
					<p class="shrink-0 px-4 pb-1.5 pt-0.5 text-[10px] text-sand-300">Ctrl+Enter to run</p>
				</div>

				<!-- Vertical Resize Handle -->
				<div
					class="group relative z-10 flex h-2 shrink-0 cursor-row-resize items-center justify-center bg-sand-200 transition-colors hover:bg-sage-400 {resizing === 'vertical' ? 'bg-sage-500' : ''}"
					onmousedown={startVerticalResize}
					role="separator"
					aria-orientation="horizontal"
				>
					<div class="w-8 h-0.5 rounded-full bg-sand-300 transition-colors group-hover:bg-white {resizing === 'vertical' ? 'bg-white' : ''}"></div>
				</div>

				<!-- Error Display -->
				{#if activeQuery.error}
					<div class="shrink-0 border-b border-sand-200 bg-red-50 px-4 py-3">
						<p class="text-sm text-red-700">{activeQuery.error}</p>
					</div>
				{/if}

				<!-- Table Viewer -->
				{#if activeQuery.result && activeQuery.result.columns.length > 0}
					<div class="flex-1 overflow-auto">
						<table class="w-full text-left text-sm">
							<thead class="sticky top-0 z-10">
								<tr class="border-b border-sand-200 bg-sand-50">
									{#each activeQuery.result.columns as col}
										<th class="whitespace-nowrap px-4 py-2.5 font-semibold text-sand-600">{col}</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each activeQuery.result.rows as row}
									<tr class="border-b border-sand-100 transition-colors hover:bg-sage-50/40">
										{#each activeQuery.result.columns as col}
											<td class="max-w-[300px] truncate px-4 py-2 text-sand-700"
												>{formatCell(row[col])}</td
											>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
						<div class="border-t border-sand-200 bg-sand-50 px-4 py-2 text-xs text-sand-400">
							{activeQuery.result.rows.length} of {activeQuery.result.totalRows.toLocaleString()} rows
						</div>
					</div>
				{:else if !activeQuery.error}
					<div class="flex flex-1 items-center justify-center">
						<div class="flex items-center gap-2 text-sand-300">
							<svg class="h-5 w-5 animate-spin" viewBox="0 0 24 24" fill="none">
								<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
								<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
							</svg>
							<span class="text-sm">Executing query...</span>
						</div>
					</div>
				{/if}
			{:else}
				<div class="flex flex-1 items-center justify-center">
					<p class="text-sm text-sand-400">Select a query tab to view results</p>
				</div>
			{/if}
		{:else}
			<div class="flex flex-1 flex-col items-center justify-center gap-3">
				<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-sand-100">
					<svg class="h-6 w-6 text-sand-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
						<path stroke-linecap="round" stroke-linejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H4.875c-.621 0-1.125.504-1.125 1.125v6.75M12 19.5h8.625c.621 0 1.125-.504 1.125-1.125v-6.75c0-.621-.504-1.125-1.125-1.125H13.125c-.621 0-1.125.504-1.125 1.125v6.75" />
					</svg>
				</div>
				<p class="text-sm text-sand-400">Query results will appear here</p>
				<p class="text-xs text-sand-300">Ask a question to get started</p>
			</div>
		{/if}
	</div>
</div>
