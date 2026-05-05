<script lang="ts">
	import { TrendingUp, TrendingDown, Minus } from 'lucide-svelte';

	interface Props {
		title: string;
		value: string | number;
		formatString?: string;
		subtitle?: string;
		trend?: 'up' | 'down' | 'neutral';
		trendValue?: string;
		loading?: boolean;
		error?: string;
	}

	let {
		title,
		value,
		formatString = '',
		subtitle = '',
		trend,
		trendValue,
		loading = false,
		error = ''
	}: Props = $props();

	function formatValue(val: string | number, format: string): string {
		if (typeof val === 'string') return val;
		
		if (!format) return val.toString();
		
		// Simple format handling
		if (format.includes('%')) {
			return `${(val * 100).toFixed(2)}%`;
		}
		if (format.includes('$')) {
			return `$${val.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
		}
		if (format.includes(',')) {
			return val.toLocaleString('en-US');
		}
		
		return val.toString();
	}

	const trendConfig = {
		up: {
			icon: TrendingUp,
			color: 'text-green-600 dark:text-green-400',
			bg: 'bg-green-50 dark:bg-green-900/20'
		},
		down: {
			icon: TrendingDown,
			color: 'text-red-600 dark:text-red-400',
			bg: 'bg-red-50 dark:bg-red-900/20'
		},
		neutral: {
			icon: Minus,
			color: 'text-slate-600 dark:text-slate-400',
			bg: 'bg-slate-50 dark:bg-slate-800/50'
		}
	};

	const config = trend ? trendConfig[trend] : null;
</script>

<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6 hover:shadow-md transition-shadow">
	<!-- Title -->
	<div class="text-sm font-medium text-slate-600 dark:text-slate-400 mb-2">
		{title}
	</div>

	{#if loading}
		<!-- Loading State -->
		<div class="space-y-3">
			<div class="h-10 bg-slate-200 dark:bg-slate-700 rounded animate-pulse"></div>
			{#if subtitle}
				<div class="h-4 bg-slate-200 dark:bg-slate-700 rounded animate-pulse w-2/3"></div>
			{/if}
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="text-sm text-red-600 dark:text-red-400 py-2">
			{error}
		</div>
	{:else}
		<!-- Value -->
		<div class="text-3xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-1">
			{formatValue(value, formatString)}
		</div>

		<!-- Subtitle or Trend -->
		<div class="flex items-center justify-between">
			{#if subtitle}
				<span class="text-sm text-slate-500 dark:text-slate-400">
					{subtitle}
				</span>
			{:else}
				<div></div>
			{/if}

			{#if config && trendValue}
				{@const IconComponent = config.icon}
				<div class="flex items-center gap-1 px-2 py-1 rounded {config.bg}">
					<IconComponent class="w-4 h-4 {config.color}" />
					<span class="text-sm font-medium {config.color}">
						{trendValue}
					</span>
				</div>
			{/if}
		</div>
	{/if}
</div>

