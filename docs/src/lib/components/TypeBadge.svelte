<script lang="ts">
	import { getTypeColor, formatTypeName, getTypeIcon } from '$lib/utils/type-detection';
	import * as Icons from 'lucide-svelte';

	interface Props {
		type: string;
		showIcon?: boolean;
		size?: 'sm' | 'md' | 'lg';
	}

	let { type, showIcon = true, size = 'sm' }: Props = $props();

	// Get the icon component dynamically
	function getIcon(iconName: string) {
		return (Icons as any)[iconName] || Icons.HelpCircle;
	}

	const sizeClasses = {
		sm: 'text-xs px-2 py-0.5',
		md: 'text-sm px-3 py-1',
		lg: 'text-base px-4 py-2'
	};

	const iconSizes = {
		sm: 'w-3 h-3',
		md: 'w-4 h-4',
		lg: 'w-5 h-5'
	};
</script>

<span
	class="inline-flex items-center gap-1.5 rounded-full font-medium {getTypeColor(
		type
	)} {sizeClasses[size]}"
>
	{#if showIcon}
		{@const IconComponent = getIcon(getTypeIcon(type))}
		<IconComponent class={iconSizes[size]} />
	{/if}
	{formatTypeName(type)}
</span>

