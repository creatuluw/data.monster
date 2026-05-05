<script lang="ts">
	interface Props {
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		minHeight?: string;
		label?: string;
		helpText?: string;
	}
	
	let {
		value = $bindable(''),
		placeholder = 'SELECT * FROM ...',
		disabled = false,
		minHeight = '200px',
		label = '',
		helpText = ''
	}: Props = $props();
	
	let textarea: HTMLTextAreaElement;
	
	// SQL keywords for basic syntax highlighting
	const sqlKeywords = [
		'SELECT', 'FROM', 'WHERE', 'JOIN', 'LEFT', 'RIGHT', 'INNER', 'OUTER',
		'ON', 'AND', 'OR', 'NOT', 'IN', 'LIKE', 'BETWEEN', 'IS', 'NULL',
		'ORDER', 'BY', 'GROUP', 'HAVING', 'LIMIT', 'OFFSET', 'AS',
		'INSERT', 'INTO', 'VALUES', 'UPDATE', 'SET', 'DELETE',
		'CREATE', 'TABLE', 'DROP', 'ALTER', 'ADD', 'COLUMN',
		'DISTINCT', 'COUNT', 'SUM', 'AVG', 'MIN', 'MAX',
		'CASE', 'WHEN', 'THEN', 'ELSE', 'END', 'WITH', 'CTE'
	];
	
	// Simple syntax highlighting for display (read-only overlay)
	const highlightedCode = $derived(() => {
		if (!value) return '';
		
		let highlighted = value;
		
		// Highlight SQL keywords
		sqlKeywords.forEach(keyword => {
			const regex = new RegExp(`\\b(${keyword})\\b`, 'gi');
			highlighted = highlighted.replace(regex, '<span class="sql-keyword">$1</span>');
		});
		
		// Highlight strings
		highlighted = highlighted.replace(/('([^']*)')/g, '<span class="sql-string">$1</span>');
		
		// Highlight numbers
		highlighted = highlighted.replace(/\b(\d+)\b/g, '<span class="sql-number">$1</span>');
		
		// Highlight comments
		highlighted = highlighted.replace(/(--.*$)/gm, '<span class="sql-comment">$1</span>');
		
		return highlighted;
	});
	
	function handleTab(event: KeyboardEvent) {
		if (event.key === 'Tab') {
			event.preventDefault();
			const start = textarea.selectionStart;
			const end = textarea.selectionEnd;
			
			// Insert tab at cursor position
			value = value.substring(0, start) + '  ' + value.substring(end);
			
			// Move cursor after the tab
			setTimeout(() => {
				textarea.selectionStart = textarea.selectionEnd = start + 2;
			}, 0);
		}
	}
</script>

<div class="sql-editor-wrapper">
	{#if label}
		<label for="sql-editor" class="block text-sm font-medium text-slate-900 dark:text-slate-100 mb-2">
			{label}
		</label>
	{/if}
	
	<div class="relative rounded-lg border border-slate-300 dark:border-slate-600 overflow-hidden bg-slate-50 dark:bg-slate-900">
		<!-- Line numbers (optional) -->
		<div class="flex">
			<div class="line-numbers text-xs text-slate-400 dark:text-slate-500 py-3 px-2 bg-slate-100 dark:bg-slate-800 select-none">
				{#each value.split('\n') as _, i}
					<div class="text-right leading-6">{i + 1}</div>
				{/each}
			</div>
			
			<!-- Editable textarea -->
			<textarea
				id="sql-editor"
				bind:this={textarea}
				bind:value={value}
				{placeholder}
				{disabled}
				onkeydown={handleTab}
				class="flex-1 py-3 px-3 text-sm font-mono text-slate-900 dark:text-slate-100 bg-transparent resize-none focus:outline-none focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-400 focus:ring-inset leading-6"
				style="min-height: {minHeight};"
				spellcheck="false"
			></textarea>
		</div>
	</div>
	
	{#if helpText}
		<p class="text-xs text-slate-500 dark:text-slate-400 mt-2">{helpText}</p>
	{/if}
</div>

<style>
	.sql-editor-wrapper :global(.sql-keyword) {
		color: #2563eb; /* blue-600 */
		font-weight: 600;
	}
	
	.sql-editor-wrapper :global(.sql-string) {
		color: #16a34a; /* green-600 */
	}
	
	.sql-editor-wrapper :global(.sql-number) {
		color: #dc2626; /* red-600 */
	}
	
	.sql-editor-wrapper :global(.sql-comment) {
		color: #64748b; /* slate-500 */
		font-style: italic;
	}
	
	:global(.dark) .sql-editor-wrapper :global(.sql-keyword) {
		color: #60a5fa; /* blue-400 */
	}
	
	:global(.dark) .sql-editor-wrapper :global(.sql-string) {
		color: #4ade80; /* green-400 */
	}
	
	:global(.dark) .sql-editor-wrapper :global(.sql-number) {
		color: #f87171; /* red-400 */
	}
	
	:global(.dark) .sql-editor-wrapper :global(.sql-comment) {
		color: #94a3b8; /* slate-400 */
	}
	
	.line-numbers {
		min-width: 3rem;
	}
</style>

