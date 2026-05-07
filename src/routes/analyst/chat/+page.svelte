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
	let leftWidth = $state(35);
	let resizing: 'horizontal' | 'vertical' | null = $state(null);
	let expandedAccordions = $state<Set<string>>(new Set());

	function toggleAccordion(id: string) {
		if (expandedAccordions.has(id)) {
			expandedAccordions.delete(id);
		} else {
			expandedAccordions.add(id);
		}
		expandedAccordions = new Set(expandedAccordions);
	}

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

	function stripPlanBlock(content: string): string {
		return content.replace(/<!-- plan:json\s*[\s\S]*?\s*-->\n?/g, '');
	}

	function renderParts(content: string): RenderPart[] {
		const cleaned = stripPlanBlock(content);
		const parts: RenderPart[] = [];
		const regex = /```sql\s*\n([\s\S]*?)```/g;
		let lastIndex = 0;
		let match;

		while ((match = regex.exec(cleaned)) !== null) {
			if (match.index > lastIndex) {
				const text = cleaned.slice(lastIndex, match.index).trim();
				if (text) parts.push({ type: 'text', content: text });
			}
			parts.push({ type: 'sql', content: match[1].trim(), partial: false });
			lastIndex = match.index + match[0].length;
		}

		if (lastIndex < cleaned.length) {
			const remaining = cleaned.slice(lastIndex);
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

	function renderMarkdown(text: string, messageId?: string): string {
		const html = marked.parse(text, { async: false }) as string;
		return html.replace(/\[Q(\d+)\]/g, (_match, num) => {
			const queries = messageId
				? analyst.queries.filter((q) => q.messageRefId === messageId)
				: analyst.queries;
			const idx = parseInt(num, 10) - 1;
			const query = queries[idx];
			if (query) {
				return `<button class="q-ref" data-qid="${query.id}">[Q${num}]</button>`;
			}
			return `<span class="q-ref">[Q${num}]</span>`;
		});
	}

	function handleProseClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (target.classList.contains('q-ref') && target.dataset.qid) {
			analyst.setActiveQuery(target.dataset.qid);
		}
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
		const editorPanel = rightPanel.querySelector('.query-editor-panel') as HTMLElement;

		function onMove(ev: MouseEvent) {
			if (!rightPanel || !tabBar || !editorPanel) return;
			const tabBarBottom = tabBar.getBoundingClientRect().bottom;
			const y = ev.clientY - tabBarBottom;
			editorPanel.style.flex = '0 0 auto';
			editorPanel.style.height = y + 'px';
		}

		function onUp() {
			resizing = null;
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		}

		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
	}

	onMount(() => {
		if (analyst.selectedTables.length === 0) {
			goto('/analyst');
			return;
		}
		inputEl?.focus();
	});
</script>

<svelte:head>
	<title>Data Analyst — Data Monster</title>
</svelte:head>

<div class="chat-layout {resizing ? 'is-resizing' : ''}">
	<div class="chat-left" style="width: {leftWidth}%;">
		<div class="chat-header">
			<div class="chat-header-left">
				<div class="chat-avatar">
					<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
						<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714a2.25 2.25 0 0 1-.659 1.591L15 14.5" />
					</svg>
				</div>
				<div>
					<h2 class="chat-header-title">Data Analyst</h2>
					<p class="chat-header-meta">{analyst.selectedTables.length} table{analyst.selectedTables.length !== 1 ? 's' : ''} in context</p>
				</div>
			</div>
			<a href="/analyst" class="chat-settings-link" title="Change tables">
				<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
					<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
				</svg>
			</a>
		</div>

		<div bind:this={messagesEl} class="chat-messages">
			{#if analyst.messages.length === 0 && !analyst.streaming}
				<div class="welcome">
					<div class="welcome-icon">
						<svg width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
							<path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456ZM16.894 20.567 16.5 21.75l-.394-1.183a2.25 2.25 0 0 0-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 0 0 1.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 0 0 1.423 1.423l1.183.394-1.183.394a2.25 2.25 0 0 0-1.423 1.423Z" />
						</svg>
					</div>
					<h3 class="welcome-title">Ask anything about your data</h3>
					<p class="welcome-desc">I'll analyze your {analyst.selectedTables.length} {analyst.selectedTables.length !== 1 ? 'tables' : 'table'} using SQL. Queries run automatically and results appear on the right.</p>

					<div class="welcome-tables">
						{#each analyst.selectedTables as tableName}
							<span class="welcome-table-chip">{tableName}</span>
						{/each}
					</div>

					<div class="suggestions">
						<span class="suggestions-label">Try asking</span>
						<div class="suggestions-grid">
							{#each [
								'Give me an overview of the data',
								'What are the key patterns and trends?',
								'Show me summary statistics',
								'What are the outliers?'
							] as suggestion}
								<button
									onclick={() => handleSuggestion(suggestion)}
									class="suggestion-btn"
								>
									{suggestion}
								</button>
							{/each}
						</div>
					</div>
				</div>
			{/if}

			{#each analyst.messages as message (message.id)}
				<div id="msg-{message.id}" class="message">
					{#if message.role === 'user'}
						<div class="message-user">
							<span class="message-user-label">You</span>
							<div class="prose-chat" onclick={handleProseClick}>{@html renderMarkdown(message.content)}</div>
						</div>
					{:else}
						<div class="message-assistant">
							{#each renderParts(message.content) as part, idx}
								{#if part.type === 'text'}
									<div class="prose-chat" onclick={handleProseClick}>{@html renderMarkdown(part.content, message.id)}</div>
								{:else}
									{@const accordionId = `${message.id}-sql-${idx}`}
									<div class="sql-accordion">
										<button 
											onclick={() => toggleAccordion(accordionId)}
											class="sql-accordion-header"
											aria-expanded={expandedAccordions.has(accordionId)}
										>
											<div class="sql-accordion-title">
												<svg class="sql-accordion-icon" class:is-open={expandedAccordions.has(accordionId)} width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
													<path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
												</svg>
												<span class="sql-accordion-label">
													<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
														<path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 3.75v3.75c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125v-3.75" />
													</svg>
													SQL Query
												</span>
											</div>
											<div class="sql-accordion-meta">
												<span class="sql-accordion-lines">{part.content.split('\n').length} line{part.content.split('\n').length !== 1 ? 's' : ''}</span>
												{#if part.partial}
													<span class="sql-accordion-status">
														<svg class="spinner-sm" viewBox="0 0 24 24" fill="none">
															<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
															<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
														</svg>
														Writing...
													</span>
												{/if}
											</div>
										</button>
										<div 
											class="sql-accordion-content"
											class:is-open={expandedAccordions.has(accordionId)}
										>
											<pre class="sql-block-code"><code>{part.content}</code></pre>
										</div>
									</div>
								{/if}
							{/each}
						</div>
					{/if}
				</div>
			{/each}

			{#if analyst.pendingPlan}
				<div class="plan-panel">
					<div class="plan-header">
						<span class="plan-header-icon">
							<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
								<path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09Z" />
							</svg>
						</span>
						<span class="plan-header-text">Choose analyses to proceed with</span>
						<div class="plan-header-actions">
							<button class="plan-action-link" onclick={() => analyst.selectAllPlanOptions()}>Select all</button>
							<button class="plan-action-link" onclick={() => analyst.deselectAllPlanOptions()}>Deselect all</button>
						</div>
					</div>
					<div class="plan-options">
						{#each analyst.pendingPlan.options as option}
							<button
								class="plan-chip {analyst.selectedPlanOptions.has(option.id) ? 'is-selected' : ''}"
								onclick={() => analyst.togglePlanOption(option.id)}
							>
								<span class="plan-chip-check">
									{#if analyst.selectedPlanOptions.has(option.id)}
										<svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="2">
											<path d="M2.5 6l2.5 2.5 4.5-5" stroke-linecap="round" stroke-linejoin="round" />
										</svg>
									{/if}
								</span>
								<span class="plan-chip-label">{option.label}</span>
								<span class="plan-chip-desc">{option.description}</span>
							</button>
						{/each}
					</div>
					<div class="plan-custom">
						<input
							type="text"
							class="plan-custom-input"
							bind:value={analyst.customPlanOption}
							placeholder="Add your own analysis question..."
							onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); analyst.proceedWithPlan(); } }}
						/>
					</div>
					<div class="plan-footer">
						<span class="plan-footer-count">{analyst.selectedPlanOptions.size} selected</span>
						<button
							class="plan-proceed-btn"
							onclick={() => analyst.proceedWithPlan()}
							disabled={analyst.selectedPlanOptions.size === 0 && !analyst.customPlanOption.trim()}
						>
							Proceed with selected
							<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
								<path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" />
							</svg>
						</button>
					</div>
				</div>
			{/if}

			{#if analyst.streaming && analyst.streamingContent}
				<div class="message">
					<div class="message-assistant">
						{#each renderParts(analyst.streamingContent) as part, idx}
							{#if part.type === 'text'}
								<div class="prose-chat" onclick={handleProseClick}>{@html renderMarkdown(part.content)}</div>
							{:else}
								{@const accordionId = `streaming-sql-${idx}`}
								<div class="sql-accordion">
									<button 
										onclick={() => toggleAccordion(accordionId)}
										class="sql-accordion-header"
										aria-expanded={expandedAccordions.has(accordionId)}
									>
										<div class="sql-accordion-title">
											<svg class="sql-accordion-icon" class:is-open={expandedAccordions.has(accordionId)} width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
												<path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
											</svg>
											<span class="sql-accordion-label">
												<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
													<path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 3.75v3.75c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125v-3.75" />
												</svg>
												SQL Query
											</span>
										</div>
										<div class="sql-accordion-meta">
											<span class="sql-accordion-lines">{part.content.split('\n').length} line{part.content.split('\n').length !== 1 ? 's' : ''}</span>
											{#if part.partial}
												<span class="sql-accordion-status">
													<svg class="spinner-sm" viewBox="0 0 24 24" fill="none">
														<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
														<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
													</svg>
													Writing...
												</span>
											{/if}
										</div>
									</button>
									<div 
										class="sql-accordion-content"
										class:is-open={expandedAccordions.has(accordionId)}
									>
										<pre class="sql-block-code"><code>{part.content}{#if part.partial}<span class="cursor">|</span>{/if}</code></pre>
									</div>
								</div>
							{/if}
						{/each}
						<span class="streaming-cursor"></span>
					</div>
				</div>
			{:else if analyst.streaming}
				<div class="message">
					<div class="thinking">
						<svg class="spinner" viewBox="0 0 24 24" fill="none" style="width: 16px; height: 16px;">
							<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
							<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
						</svg>
						<span class="thinking-text">Thinking...</span>
					</div>
				</div>
			{/if}
		</div>

		<div class="chat-input-area">
			<div class="chat-input-row">
				<textarea
					bind:this={inputEl}
					bind:value={inputText}
					onkeydown={handleKeydown}
					placeholder="Ask about your data..."
					rows="1"
					disabled={analyst.streaming || !!analyst.pendingPlan}
					class="chat-input"
					oninput={() => {
						if (inputEl) {
							inputEl.style.height = 'auto';
							inputEl.style.height = Math.min(inputEl.scrollHeight, 96) + 'px';
						}
					}}
				></textarea>
				{#if analyst.streaming}
					<button
						onclick={() => analyst.stop()}
						aria-label="Stop generating"
						class="stop-btn"
					>
						<svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
							<rect width="12" height="12" rx="2" />
						</svg>
					</button>
				{:else}
					<button
						onclick={handleSend}
						disabled={!inputText.trim() || !!analyst.pendingPlan}
						aria-label="Send message"
						class="send-btn"
					>
						<svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
							<path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0 1 21.485 12 59.77 59.77 0 0 1 3.27 20.876L5.999 12Zm0 0h7.5" />
						</svg>
					</button>
				{/if}
			</div>
			<span class="chat-input-hint">Enter to send · Shift+Enter for new line</span>
		</div>
	</div>

	<div
		class="resize-handle-h"
		class:is-active={resizing === 'horizontal'}
		onmousedown={startHorizontalResize}
		role="separator"
		aria-orientation="vertical"
	>
		<div class="resize-grip-h"></div>
	</div>

	<div id="right-panel" class="chat-right">
		{#if analyst.queries.length > 0}
			<div data-tab-bar class="tab-bar">
				{#each analyst.queries as query, i}
					{@const label = getQueryLabel(i, query)}
					<button
						onclick={() => analyst.setActiveQuery(query.id)}
						class="tab {query.id === analyst.activeQueryId ? 'is-active' : ''}"
					>
						<span>{label}</span>
						{#if query.error}
							<span class="tab-dot tab-dot-error"></span>
						{:else if query.result}
							<span class="tab-dot tab-dot-ok"></span>
						{/if}
					</button>
				{/each}
			</div>

			{#if activeQuery}
				<div class="query-editor-panel">
					<div class="query-editor-header">
						<div class="query-editor-header-left">
							<span class="query-label">Query</span>
							{#if activeQuery.error}
								<span class="tag tag-danger">Error</span>
							{:else if activeQuery.result}
								<span class="tag tag-accent">{activeQuery.result.rows.length} of {activeQuery.result.totalRows.toLocaleString()} rows</span>
							{/if}
							<button
								onclick={() => scrollToMessage(activeQuery.messageRefId)}
								class="ref-btn"
								title="Jump to answer in chat"
							>
								ref:{activeQuery.messageRefId.slice(0, 6)}
							</button>
						</div>
						<div class="query-editor-header-right">
							{#if activeQuery.queryTime > 0}
								<span class="query-time">{activeQuery.queryTime.toFixed(0)}ms</span>
							{/if}
							<button
								onclick={handleRerun}
								disabled={analyst.streaming || !editSql.trim()}
								class="btn btn-primary btn-sm"
							>
								<svg width="10" height="10" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
									<path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
								</svg>
								Run
							</button>
						</div>
					</div>
					<textarea
						bind:value={editSql}
						onkeydown={handleRerunKeydown}
						class="query-textarea"
					></textarea>
					<span class="query-hint">Ctrl+Enter to run</span>
				</div>

				<div
					class="resize-handle-v"
					class:is-active={resizing === 'vertical'}
					onmousedown={startVerticalResize}
					role="separator"
					aria-orientation="horizontal"
				>
					<div class="resize-grip-v"></div>
				</div>

				{#if activeQuery.error}
					<div class="query-error">
						{activeQuery.error}
					</div>
				{/if}

				{#if activeQuery.result && activeQuery.result.columns.length > 0}
					<div class="query-results">
						<table class="data-table">
							<thead>
								<tr>
									{#each activeQuery.result.columns as col}
										<th>{col}</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each activeQuery.result.rows as row}
									<tr>
										{#each activeQuery.result.columns as col}
											<td>{formatCell(row[col])}</td>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
						<div class="query-results-footer">
							{activeQuery.result.rows.length} of {activeQuery.result.totalRows.toLocaleString()} rows
						</div>
					</div>
				{:else if !activeQuery.error}
					<div class="query-loading">
						<svg class="spinner" viewBox="0 0 24 24" fill="none" style="width: 20px; height: 20px; color: var(--color-text-tertiary);">
							<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
							<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
						</svg>
						<span class="query-loading-text">Executing query...</span>
					</div>
				{/if}
			{:else}
				<div class="query-placeholder">
					<span class="placeholder-text">Select a query tab to view results</span>
				</div>
			{/if}
		{:else}
			<div class="query-placeholder">
				<div class="placeholder-empty">
					<svg width="40" height="40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1" opacity="0.3">
						<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
					</svg>
					<span class="placeholder-title">Query results</span>
					<span class="placeholder-desc">Ask a question to start exploring your data</span>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.chat-layout {
		display: flex;
		overflow: hidden;
		height: 100%;
	}

	.chat-layout.is-resizing {
		user-select: none;
	}

	.chat-left {
		display: flex;
		flex-direction: column;
		min-width: 0;
		height: 100%;
		border-right: 1px solid var(--color-border);
		background: var(--color-surface-raised);
	}

	.chat-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		background: var(--color-surface);
		flex-shrink: 0;
	}

	.chat-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.chat-avatar {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		border: 1px solid oklch(0.82 0.03 41);
		flex-shrink: 0;
	}

	.chat-header-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.chat-header-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.chat-settings-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.chat-settings-link:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.chat-messages {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-3) var(--space-4);
		min-height: 0;
	}

	.welcome {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		padding: var(--space-8) var(--space-4);
		gap: var(--space-4);
	}

	.welcome-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-lg);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		border: 1px solid oklch(0.82 0.03 41);
	}

	.welcome-title {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 700;
		color: var(--color-text);
	}

	.welcome-desc {
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 28rem;
	}

	.welcome-tables {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: var(--space-1);
	}

	.welcome-table-chip {
		display: inline-flex;
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text-secondary);
		letter-spacing: 0.02em;
	}

	.suggestions {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		width: 100%;
		max-width: 28rem;
		margin-top: var(--space-2);
	}

	.suggestions-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		text-align: left;
	}

	.suggestions-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.suggestion-btn {
		display: block;
		width: 100%;
		text-align: left;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		cursor: pointer;
		border-radius: var(--radius-sm);
		transition: border-color var(--duration-fast) ease, background var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.suggestion-btn:hover {
		border-color: var(--color-accent);
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
	}

	.message {
		margin-bottom: var(--space-4);
	}

	.message-user {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		background: var(--color-accent-muted);
		padding: var(--space-3);
		border-radius: var(--radius-md);
		border: 1px solid oklch(0.82 0.03 41);
	}

	.message-user-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-accent);
	}

	.message-user :global(.prose-chat) {
		color: var(--color-accent-dark);
	}

	.message-assistant {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.plan-panel {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		margin: var(--space-2) 0;
	}

	.plan-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.plan-header-icon {
		color: var(--color-accent);
		display: flex;
	}

	.plan-header-text {
		font-family: var(--font-display);
		font-weight: 700;
		font-size: var(--text-sm);
		color: var(--color-text);
		flex: 1;
	}

	.plan-header-actions {
		display: flex;
		gap: var(--space-2);
	}

	.plan-action-link {
		background: none;
		border: none;
		color: var(--color-accent);
		font-size: var(--text-xs);
		cursor: pointer;
		font-family: var(--font-mono);
		padding: 0;
	}

	.plan-action-link:hover {
		text-decoration: underline;
	}

	.plan-options {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.plan-chip {
		display: flex;
		align-items: flex-start;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface-sunken);
		cursor: pointer;
		text-align: left;
		transition: all var(--duration-fast) ease;
	}

	.plan-chip:hover {
		border-color: var(--color-accent);
	}

	.plan-chip.is-selected {
		border-color: var(--color-accent);
		background: color-mix(in srgb, var(--color-accent) 8%, var(--color-surface-sunken));
	}

	.plan-chip-check {
		width: 16px;
		height: 16px;
		border: 1.5px solid var(--color-border);
		border-radius: var(--radius-xs);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		margin-top: 2px;
		color: white;
		transition: all var(--duration-fast) ease;
	}

	.plan-chip.is-selected .plan-chip-check {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}

	.plan-chip-label {
		font-weight: 600;
		font-size: var(--text-sm);
		color: var(--color-text);
		white-space: nowrap;
	}

	.plan-chip-desc {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}

	.plan-custom {
		display: flex;
	}

	.plan-custom-input {
		flex: 1;
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text);
		outline: none;
		transition: border-color var(--duration-fast) ease;
	}

	.plan-custom-input::placeholder {
		color: var(--color-text-tertiary);
	}

	.plan-custom-input:focus {
		border-color: var(--color-accent);
	}

	.plan-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.plan-footer-count {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.plan-proceed-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		background: var(--color-accent);
		color: var(--color-text-on-accent);
		border: none;
		border-radius: var(--radius-sm);
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-display);
		font-weight: 600;
		font-size: var(--text-sm);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.plan-proceed-btn:hover:not(:disabled) {
		background: var(--color-accent-dark);
	}

	.plan-proceed-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.prose-chat {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
	}

	.prose-chat :global(h1) {
		font-family: var(--font-display);
		font-weight: 800;
		color: var(--color-text);
		font-size: 1.75rem;
		margin-top: 2rem;
		margin-bottom: 1rem;
		line-height: 1.2;
		letter-spacing: -0.03em;
		border-bottom: 2px solid var(--color-accent);
		padding-bottom: 0.5rem;
	}

	.prose-chat :global(h1):first-child {
		margin-top: 0;
	}

	.prose-chat :global(h2) {
		font-family: var(--font-display);
		font-weight: 700;
		color: var(--color-text);
		font-size: 1.375rem;
		margin-top: 1.75rem;
		margin-bottom: 0.75rem;
		line-height: 1.3;
		letter-spacing: -0.02em;
	}

	.prose-chat :global(h3) {
		font-family: var(--font-display);
		font-weight: 700;
		color: var(--color-text);
		font-size: 1.125rem;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
		line-height: 1.4;
		letter-spacing: -0.01em;
	}

	.prose-chat :global(.q-ref) {
		color: var(--color-accent);
		font-weight: 800;
	}

	.prose-chat :global(button.q-ref) {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		font: inherit;
		color: var(--color-accent);
		font-weight: 800;
	}

	.prose-chat :global(button.q-ref:hover) {
		text-decoration: underline;
	}

	.prose-chat :global(h4) {
		font-family: var(--font-display);
		font-weight: 700;
		color: var(--color-text);
		font-size: 1rem;
		margin-top: 1.25rem;
		margin-bottom: 0.5rem;
		line-height: 1.4;
	}

	.prose-chat :global(p) {
		margin: 0 0 var(--space-2);
	}

	.prose-chat :global(p:last-child) {
		margin-bottom: 0;
	}

	.prose-chat :global(ul),
	.prose-chat :global(ol) {
		margin: 0 0 var(--space-2);
		padding-left: var(--space-4);
	}

	.prose-chat :global(li) {
		margin-bottom: var(--space-1);
	}

	.prose-chat :global(strong) {
		font-weight: 600;
		color: var(--color-text);
	}

	.prose-chat :global(code) {
		font-family: var(--font-mono);
		font-size: 0.85em;
		background: var(--color-surface-raised);
		padding: 1px 4px;
		border-radius: var(--radius-xs);
		color: var(--color-accent-dark);
	}

	.prose-chat :global(pre) {
		background: var(--color-surface-raised);
		padding: var(--space-3);
		border-radius: var(--radius-sm);
		overflow-x: auto;
		margin: var(--space-2) 0;
	}

	.prose-chat :global(pre code) {
		background: none;
		padding: 0;
		color: var(--color-text-secondary);
	}

	.prose-chat :global(blockquote) {
		border-left: 3px solid var(--color-accent);
		padding-left: var(--space-3);
		margin: var(--space-2) 0;
		color: var(--color-text-secondary);
		font-style: italic;
	}

	.prose-chat :global(hr) {
		border: none;
		border-top: 1px solid var(--color-border);
		margin: var(--space-3) 0;
	}

	.prose-chat :global(table) {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		font-weight: 400;
		border-collapse: collapse;
		margin: var(--space-2) 0;
		width: 100%;
	}

	.prose-chat :global(table th),
	.prose-chat :global(table td) {
		padding: var(--space-1) var(--space-2);
		border: 1px solid var(--color-border);
		text-align: left;
	}

	.prose-chat :global(table th) {
		font-weight: 600;
		color: var(--color-text-secondary);
		background: var(--color-surface-raised);
	}

	.sql-accordion {
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface);
		overflow: hidden;
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.sql-accordion:hover {
		border-color: var(--color-border-strong);
	}

	.sql-accordion-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: var(--color-surface);
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background var(--duration-fast) ease;
	}

	.sql-accordion-header:hover {
		background: var(--color-surface-raised);
	}

	.sql-accordion-title {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.sql-accordion-icon {
		width: 14px;
		height: 14px;
		color: var(--color-text-tertiary);
		transition: transform var(--duration-fast) ease;
	}

	.sql-accordion-icon.is-open {
		transform: rotate(180deg);
	}

	.sql-accordion-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-secondary);
	}

	.sql-accordion-label svg {
		color: var(--color-accent);
	}

	.sql-accordion-meta {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.sql-accordion-lines {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.sql-accordion-status {
		display: flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-accent);
		letter-spacing: 0.04em;
	}

	.spinner-sm {
		width: 10px;
		height: 10px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.sql-accordion-content {
		max-height: 0;
		overflow: hidden;
		transition: max-height var(--duration-base) var(--ease-out-expo);
	}

	.sql-accordion-content.is-open {
		max-height: 600px;
	}

	.sql-block-code {
		padding: var(--space-3);
		overflow-x: auto;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		margin: 0;
	}

	.sql-block {
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface);
		overflow: hidden;
	}

	.sql-block-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-1) var(--space-3);
		background: var(--color-surface-raised);
		border-bottom: 1px solid var(--color-border);
	}

	.sql-block-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.sql-block-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.sql-block-writing {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-accent);
		letter-spacing: 0.04em;
		animation: pulse-cursor 1s ease infinite;
	}

	.sql-block-code {
		overflow-x: auto;
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		margin: 0;
	}

	.cursor {
		animation: pulse-cursor 1s ease infinite;
		color: var(--color-accent);
	}

	@keyframes pulse-cursor {
		0%, 100% { opacity: 1; }
		50% { opacity: 0; }
	}

	.streaming-cursor {
		display: inline-block;
		width: 4px;
		height: 16px;
		background: var(--color-accent);
		border-radius: 1px;
		animation: pulse-cursor 1s ease infinite;
	}

	.thinking {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) 0;
		color: var(--color-text-tertiary);
	}

	.thinking-text {
		font-size: var(--text-xs);
	}

	.chat-input-area {
		flex-shrink: 0;
		border-top: 1px solid var(--color-border);
		background: var(--color-surface);
		padding: var(--space-3) var(--space-4);
	}

	.chat-input-row {
		display: flex;
		align-items: flex-end;
		gap: var(--space-2);
	}

	.chat-input {
		flex: 1;
		max-height: 96px;
		min-height: 36px;
		resize: none;
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		background: var(--color-surface);
		border-radius: var(--radius-sm);
		font-family: var(--font-body);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.chat-input::placeholder {
		color: var(--color-text-tertiary);
	}

	.chat-input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.chat-input:disabled {
		opacity: 0.5;
	}

	.send-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		flex-shrink: 0;
		background: var(--color-accent);
		color: var(--color-text-on-accent);
		border: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.send-btn:hover:not(:disabled) {
		background: var(--color-accent-dark);
	}

	.send-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.stop-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		flex-shrink: 0;
		background: var(--color-surface-raised);
		color: var(--color-text-secondary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.stop-btn:hover {
		background: var(--color-danger);
		border-color: var(--color-danger);
		color: white;
	}

	.chat-input-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
		display: block;
		margin-top: var(--space-1);
	}

	.resize-handle-h {
		position: relative;
		z-index: 10;
		width: 6px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: col-resize;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-h::before {
		content: '';
		position: absolute;
		inset: 0;
		background: var(--color-border);
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-h:hover::before,
	.resize-handle-h.is-active::before {
		background: var(--color-accent);
	}

	.resize-grip-h {
		height: 24px;
		width: 2px;
		border-radius: 1px;
		background: var(--color-border-strong);
		position: relative;
		z-index: 1;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-h:hover .resize-grip-h,
	.resize-handle-h.is-active .resize-grip-h {
		background: white;
	}

	.resize-handle-v {
		position: relative;
		z-index: 10;
		height: 6px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: row-resize;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-v::before {
		content: '';
		position: absolute;
		inset: 0;
		background: var(--color-border);
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-v:hover::before,
	.resize-handle-v.is-active::before {
		background: var(--color-accent);
	}

	.resize-grip-v {
		width: 24px;
		height: 2px;
		border-radius: 1px;
		background: var(--color-border-strong);
		position: relative;
		z-index: 1;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-v:hover .resize-grip-v,
	.resize-handle-v.is-active .resize-grip-v {
		background: white;
	}

	.chat-right {
		flex: 1;
		min-width: 0;
		height: 100%;
		display: flex;
		flex-direction: column;
		background: var(--color-surface);
	}

	.tab-bar {
		display: flex;
		overflow-x: auto;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-surface-raised);
		flex-shrink: 0;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: var(--space-2) var(--space-3);
		border: none;
		border-bottom: 2px solid transparent;
		background: none;
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 500;
		color: var(--color-text-tertiary);
		cursor: pointer;
		white-space: nowrap;
		flex-shrink: 0;
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.tab:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
	}

	.tab.is-active {
		border-bottom-color: var(--color-accent);
		background: var(--color-surface);
		color: var(--color-accent-dark);
		font-weight: 600;
	}

	.tab-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.tab-dot-error {
		background: var(--color-danger);
	}

	.tab-dot-ok {
		background: var(--color-accent);
	}

	.query-editor-panel {
		flex: 2 1 0;
		display: flex;
		flex-direction: column;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-surface-raised);
		overflow: hidden;
		min-height: 80px;
	}

	.query-editor-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-4);
		flex-shrink: 0;
	}

	.query-editor-header-left,
	.query-editor-header-right {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.query-label {
		font-family: var(--font-display);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-secondary);
	}

	.ref-btn {
		padding: 1px 6px;
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.ref-btn:hover {
		background: var(--color-surface-raised);
		color: var(--color-text-secondary);
	}

	.query-time {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.query-textarea {
		flex: 1;
		min-height: 0;
		width: 100%;
		resize: none;
		padding: var(--space-1) var(--space-4);
		background: var(--color-surface-raised);
		border: none;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		outline: none;
	}

	.query-textarea:focus {
		box-shadow: inset 0 0 0 1px var(--color-accent);
	}

	.query-hint {
		flex-shrink: 0;
		padding: var(--space-1) var(--space-4);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.resize-handle-v {
		position: relative;
		z-index: 10;
		height: 6px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: row-resize;
	}

	.resize-handle-v::before {
		content: '';
		position: absolute;
		inset: 0;
		background: var(--color-border);
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-v:hover::before,
	.resize-handle-v.is-active::before {
		background: var(--color-accent);
	}

	.resize-grip-v {
		width: 24px;
		height: 2px;
		border-radius: 1px;
		background: var(--color-border-strong);
		position: relative;
		z-index: 1;
		transition: background var(--duration-fast) ease;
	}

	.resize-handle-v:hover .resize-grip-v,
	.resize-handle-v.is-active .resize-grip-v {
		background: white;
	}

	.query-error {
		flex-shrink: 0;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		background: oklch(0.95 0.03 22);
		font-size: var(--text-sm);
		color: oklch(0.38 0.12 22);
	}

	.query-results {
		flex: 3 1 0;
		display: flex;
		flex-direction: column;
		overflow: auto;
		min-height: 80px;
	}

	.query-results-footer {
		padding: var(--space-2) var(--space-4);
		border-top: 1px solid var(--color-border);
		background: var(--color-surface-raised);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.query-loading {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
	}

	.query-loading-text {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.query-placeholder {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.placeholder-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-3);
		text-align: center;
	}

	.placeholder-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text-tertiary);
	}

	.placeholder-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.placeholder-text {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}
</style>