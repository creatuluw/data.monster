<script lang="ts">
	type Size = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

	interface Option {
		value: string;
		label: string;
		disabled?: boolean;
	}

	interface Props {
		options: Option[];
		value: string;
		name?: string;
		id?: string;
		label?: string;
		helperText?: string;
		placeholder?: string;
		size?: Size;
		disabled?: boolean;
		required?: boolean;
		error?: string;
		onchange?: (value: string) => void;
	}

	let {
		options,
		value = $bindable(),
		name,
		id,
		label,
		helperText,
		placeholder,
		size = 'md',
		disabled = false,
		required = false,
		error,
		onchange
	}: Props = $props();

	function handleChange(event: Event) {
		const target = event.currentTarget as HTMLSelectElement;
		value = target.value;
		if (onchange) {
			onchange(value);
		}
	}

	const sizeClasses = {
		xs: {
			select: 'py-0.5 pr-6 pl-2 text-xs min-w-[100px]',
			icon: 'size-3 mr-1.5',
			label: 'text-xs',
			helper: 'text-xs'
		},
		sm: {
			select: 'py-1 pr-7 pl-2.5 text-xs min-w-[120px]',
			icon: 'size-3.5 mr-1.5',
			label: 'text-sm',
			helper: 'text-xs'
		},
		md: {
			select: 'py-1.5 pr-8 pl-3 text-sm min-w-[140px]',
			icon: 'size-4 mr-2',
			label: 'text-sm',
			helper: 'text-xs'
		},
		lg: {
			select: 'py-2 pr-10 pl-4 text-base min-w-[180px]',
			icon: 'size-5 mr-2',
			label: 'text-base',
			helper: 'text-sm'
		},
		xl: {
			select: 'py-2.5 pr-12 pl-5 text-lg min-w-[220px]',
			icon: 'size-6 mr-2.5',
			label: 'text-lg',
			helper: 'text-base'
		}
	};

	const currentSize = $derived(sizeClasses[size]);
	const uniqueId = $derived(id || name || `select-${Math.random().toString(36).substr(2, 9)}`);
	const hasError = $derived(!!error);
</script>

{#if label}
	<label for={uniqueId} class="block {currentSize.label} font-medium text-slate-900 dark:text-white mb-2">
		{label}
		{#if required}
			<span class="text-red-500">*</span>
		{/if}
	</label>
{/if}

<div class="inline-grid grid-cols-1 relative">
	<select
		{...{ id: uniqueId, name, disabled, required }}
		{value}
		onchange={handleChange}
		style="background-image: none;"
		class="col-start-1 row-start-1 appearance-none rounded-md bg-white dark:bg-white/5 {currentSize.select} text-slate-900 dark:text-white outline-1 -outline-offset-1 {hasError ? 'outline-red-500 dark:outline-red-400' : 'outline-slate-300 dark:outline-white/10'} focus-visible:outline-2 focus-visible:-outline-offset-2 {hasError ? 'focus-visible:outline-red-600 dark:focus-visible:outline-red-500' : 'focus-visible:outline-blue-600 dark:focus-visible:outline-blue-500'} dark:*:bg-slate-800 disabled:opacity-50 disabled:cursor-not-allowed [&::-ms-expand]:hidden"
	>
		{#if placeholder}
			<option value="" disabled selected={!value}>{placeholder}</option>
		{/if}
		{#each options as option}
			<option value={option.value} disabled={option.disabled} selected={value === option.value}>
				{option.label}
			</option>
		{/each}
	</select>
	<svg 
		viewBox="0 0 16 16" 
		fill="currentColor" 
		aria-hidden="true" 
		class="pointer-events-none col-start-1 row-start-1 {currentSize.icon} self-center justify-self-end {hasError ? 'text-red-500 dark:text-red-400' : 'text-slate-500 dark:text-slate-400'}"
	>
		<path fill-rule="evenodd" d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.75.75 0 1 1 1.06 1.06l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd" />
	</svg>
</div>

{#if error}
	<p class="mt-1 {currentSize.helper} text-red-600 dark:text-red-400">
		{error}
	</p>
{:else if helperText}
	<p class="mt-1 {currentSize.helper} text-slate-500 dark:text-slate-400">
		{helperText}
	</p>
{/if}

