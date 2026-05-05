<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Option {
		value: string;
		label: string;
		disabled?: boolean;
		icon?: any;
	}

	type Size = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

	interface Props {
		options: Option[];
		value: string;
		name: string;
		label?: string;
		helperText?: string;
		helperLink?: string;
		helperLinkText?: string;
		columns?: number;
		size?: Size;
		onchange?: (value: string) => void;
		disabled?: boolean;
	}

	let {
		options,
		value = $bindable(),
		name,
		label,
		helperText,
		helperLink,
		helperLinkText,
		columns = 3,
		size = 'md',
		onchange,
		disabled = false
	}: Props = $props();

	function handleChange(newValue: string) {
		value = newValue;
		if (onchange) {
			onchange(newValue);
		}
	}

	const sizeClasses = {
		xs: {
			padding: 'p-1.5',
			text: 'text-xs',
			icon: 'w-3 h-3',
			gap: 'gap-1.5',
			label: 'text-xs',
			helper: 'text-xs'
		},
		sm: {
			padding: 'p-2',
			text: 'text-xs',
			icon: 'w-3.5 h-3.5',
			gap: 'gap-2',
			label: 'text-sm',
			helper: 'text-xs'
		},
		md: {
			padding: 'p-3',
			text: 'text-sm',
			icon: 'w-4 h-4',
			gap: 'gap-3',
			label: 'text-sm',
			helper: 'text-xs'
		},
		lg: {
			padding: 'p-4',
			text: 'text-base',
			icon: 'w-5 h-5',
			gap: 'gap-3',
			label: 'text-base',
			helper: 'text-sm'
		},
		xl: {
			padding: 'p-5',
			text: 'text-lg',
			icon: 'w-6 h-6',
			gap: 'gap-4',
			label: 'text-lg',
			helper: 'text-base'
		}
	};

	const currentSize = $derived(sizeClasses[size]);
</script>

<fieldset aria-label={label || name} class="w-full" {disabled}>
	{#if label || helperText}
		<div class="flex items-center justify-between mb-2">
			{#if label}
				<div class="{currentSize.label} font-medium text-slate-900 dark:text-white">{label}</div>
			{/if}
			{#if helperLink && helperLinkText}
				<a 
					href={helperLink} 
					class="{currentSize.helper} font-medium text-blue-600 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300"
				>
					{helperLinkText}
				</a>
			{/if}
		</div>
	{/if}
	
	{#if helperText}
		<p class="{currentSize.helper} text-slate-600 dark:text-slate-400 mb-2">{helperText}</p>
	{/if}

	<div class="grid {currentSize.gap} grid-cols-{columns} sm:grid-cols-{Math.min(columns * 2, 6)}">
		{#each options as option}
			<label 
				aria-label={option.label}
				class="group relative flex items-center justify-center rounded-md border border-slate-300 bg-white {currentSize.padding} cursor-pointer transition-all
					has-[:checked]:border-blue-600 has-[:checked]:bg-blue-600 
					has-[:focus-visible]:outline has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-offset-2 has-[:focus-visible]:outline-blue-600
					has-[:disabled]:border-slate-400 has-[:disabled]:bg-slate-200 has-[:disabled]:opacity-50 has-[:disabled]:cursor-not-allowed
					dark:border-white/10 dark:bg-slate-800/50 
					dark:has-[:checked]:border-blue-500 dark:has-[:checked]:bg-blue-500
					dark:has-[:focus-visible]:outline-blue-500
					dark:has-[:disabled]:border-white/10 dark:has-[:disabled]:bg-slate-800
					hover:border-slate-400 dark:hover:border-white/20
					has-[:disabled]:hover:border-slate-400 dark:has-[:disabled]:hover:border-white/10"
			>
				<input
					type="radio"
					{name}
					value={option.value}
					checked={value === option.value}
					disabled={option.disabled || disabled}
					onchange={() => handleChange(option.value)}
					class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
				/>
				<span class="{currentSize.text} font-medium text-slate-900 uppercase group-has-[:checked]:text-white dark:text-white flex items-center gap-1.5">
					{#if option.icon}
						{@const Icon = option.icon}
						<Icon class={currentSize.icon} />
					{/if}
					{option.label}
				</span>
			</label>
		{/each}
	</div>
</fieldset>

