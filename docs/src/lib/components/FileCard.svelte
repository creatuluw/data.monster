<script lang="ts">
	import { FileText, Folder, Database } from 'lucide-svelte';
	import { tabs } from '$lib/stores/tabs';

	interface Props {
		filename?: string;
		folder?: string;
		fileType?: string;
		sizeBytes?: number;
		href?: string;
		skeleton?: boolean;
		sourceFormat?: string;
	}

	let { 
		filename, 
		folder,
		fileType,
		sizeBytes,
		href,
		skeleton = false,
		sourceFormat
	}: Props = $props();

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
	}

	const typeConfig = {
		CSV: { 
			bg: 'bg-green-50 dark:bg-green-900/20',
			border: 'border-green-200 dark:border-green-800',
			text: 'text-green-700 dark:text-green-400',
			badge: 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
		},
		Parquet: { 
			bg: 'bg-purple-50 dark:bg-purple-900/20',
			border: 'border-purple-200 dark:border-purple-800',
			text: 'text-purple-700 dark:text-purple-400',
			badge: 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400'
		},
		JSON: { 
			bg: 'bg-blue-50 dark:bg-blue-900/20',
			border: 'border-blue-200 dark:border-blue-800',
			text: 'text-blue-700 dark:text-blue-400',
			badge: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
		},
		JSONL: { 
			bg: 'bg-blue-50 dark:bg-blue-900/20',
			border: 'border-blue-200 dark:border-blue-800',
			text: 'text-blue-700 dark:text-blue-400',
			badge: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
		}
	};

	const config = typeConfig[fileType as keyof typeof typeConfig] || { 
		bg: 'bg-slate-50 dark:bg-slate-800/20',
		border: 'border-slate-200 dark:border-slate-700',
		text: 'text-slate-700 dark:text-slate-400',
		badge: 'bg-slate-100 text-slate-700 dark:bg-slate-900/30 dark:text-slate-400'
	};

	// Handle opening in new tab
	function openInNewTab(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		
		if (!href) return;
		
		console.log('➕ Opening file in new tab:', filename);
		
		// Add new tab to the tab bar
		tabs.addTab(filename || 'Table View', href);
	}

	// Handle right-click to open in new tab
	function handleContextMenu(e: MouseEvent) {
		e.preventDefault();
		openInNewTab(e);
	}
</script>

{#if skeleton}
	<div class="group relative">
		<div class="rounded-xl border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/30 p-3 flex items-center justify-center min-h-[100px]">
			<div class="h-8 w-8 flex items-center justify-center border border-slate-200 dark:border-slate-600 rounded-lg bg-slate-50 dark:bg-slate-700/30 animate-pulse">
				<FileText class="w-4 h-4 text-slate-300 dark:text-slate-600 opacity-40" />
			</div>
		</div>
	</div>
{:else}
	<div class="group relative">
		<a 
			href={href}
			oncontextmenu={handleContextMenu}
			class="relative block overflow-visible rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 transition-all duration-300 ease-out hover:shadow-lg hover:shadow-slate-200/50 dark:hover:shadow-slate-950/50 hover:border-slate-300 dark:hover:border-slate-700 hover:-translate-y-0.5 min-h-[100px]"
		>

		<!-- Gradient overlay on hover -->
		<div class="absolute inset-0 bg-gradient-to-br from-slate-50/50 to-transparent dark:from-slate-800/30 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
		
		<!-- Content -->
		<div class="relative flex flex-col h-full p-3">
			<!-- Icon and badge in top-right corner -->
			<div class="absolute top-3 right-3 flex flex-col items-end gap-1">
				<div class="h-5 w-5 flex items-center justify-center border rounded transition-all duration-300 group-hover:scale-110 {config.bg} {config.border}">
					<FileText class="w-2.5 h-2.5 transition-colors duration-300 {config.text}" />
				</div>
				<div class="text-[9px] font-bold inline-flex items-center rounded px-1.5 py-0.5 {config.badge} uppercase tracking-wide">
					{fileType}
				</div>
			</div>

			<!-- File info at top -->
			<div class="space-y-1 pr-11">
				<h4 class="text-sm font-aspekta font-[650] text-slate-900 dark:text-slate-100 truncate" title={filename}>
					{filename}
				</h4>
				
				<!-- Metadata -->
				<div class="flex items-center gap-1.5 text-xs text-slate-600 dark:text-slate-400">
					<Folder class="w-3 h-3 shrink-0" />
					<span class="truncate">{folder}</span>
				</div>
				
				<!-- Source Format and Size -->
				<div class="flex items-center gap-2 text-xs">
					{#if sourceFormat}
						<div class="flex items-center gap-1 text-slate-600 dark:text-slate-400">
							{#if sourceFormat === 'mysql' || sourceFormat === 'postgres' || sourceFormat === 'sqlite' || sourceFormat === 'mssql' || sourceFormat === 'oracle' || sourceFormat === 'mongodb'}
								<Database class="w-3 h-3 shrink-0" />
							{/if}
							<span class="font-medium uppercase">{sourceFormat}</span>
						</div>
					{/if}
					<span class="text-slate-500 dark:text-slate-500">
						{formatBytes(sizeBytes || 0)}
					</span>
				</div>
			</div>
		</div>
		</a>
	</div>
{/if}

