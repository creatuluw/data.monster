<script lang="ts">
	import { onMount } from 'svelte';
	import { CheckCircle, XCircle, AlertCircle, Info, X } from 'lucide-svelte';
	
	interface Props {
		message: string;
		type?: 'success' | 'error' | 'warning' | 'info';
		duration?: number;
		onClose?: () => void;
	}
	
	let { 
		message, 
		type = 'info', 
		duration = 3000,
		onClose = () => {}
	}: Props = $props();
	
	let visible = $state(true);
	let isExiting = $state(false);
	
	onMount(() => {
		const timer = setTimeout(() => {
			handleClose();
		}, duration);
		
		return () => clearTimeout(timer);
	});
	
	function handleClose() {
		isExiting = true;
		setTimeout(() => {
			visible = false;
			onClose();
		}, 300); // Match animation duration
	}
	
	const icons = {
		success: CheckCircle,
		error: XCircle,
		warning: AlertCircle,
		info: Info
	};
	
	const Icon = icons[type];
	
	const colorClasses = {
		success: 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800 text-green-800 dark:text-green-200',
		error: 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800 text-red-800 dark:text-red-200',
		warning: 'bg-amber-50 dark:bg-amber-900/20 border-amber-200 dark:border-amber-800 text-amber-800 dark:text-amber-200',
		info: 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800 text-blue-800 dark:text-blue-200'
	};
	
	const iconColorClasses = {
		success: 'text-green-600 dark:text-green-400',
		error: 'text-red-600 dark:text-red-400',
		warning: 'text-amber-600 dark:text-amber-400',
		info: 'text-blue-600 dark:text-blue-400'
	};
</script>

{#if visible}
	<div 
		class="toast-wrapper fixed top-20 left-1/2 -translate-x-1/2 z-[100] {isExiting ? 'toast-exit' : 'toast-enter'}"
	>
		<div class="flex items-center gap-3 px-4 py-3 rounded-lg border shadow-lg {colorClasses[type]} min-w-[300px] max-w-[500px]">
			<Icon class="w-5 h-5 flex-shrink-0 {iconColorClasses[type]}" />
			<span class="flex-1 text-sm font-medium">{message}</span>
			<button
				onclick={handleClose}
				class="flex-shrink-0 hover:opacity-70 transition-opacity"
				aria-label="Close notification"
			>
				<X class="w-4 h-4" />
			</button>
		</div>
	</div>
{/if}

<style>
	.toast-wrapper {
		animation: slideDown 0.3s ease-out;
	}
	
	.toast-enter {
		animation: slideDown 0.3s ease-out;
	}
	
	.toast-exit {
		animation: slideUp 0.3s ease-in forwards;
	}
	
	@keyframes slideDown {
		from {
			transform: translateY(-100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}
	
	@keyframes slideUp {
		from {
			transform: translateY(0);
			opacity: 1;
		}
		to {
			transform: translateY(-100%);
			opacity: 0;
		}
	}
</style>

