<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		initialSize?: number; // Initial size as percentage (0-100)
		minSize?: number; // Minimum size as percentage
		maxSize?: number; // Maximum size as percentage
		orientation?: 'horizontal' | 'vertical'; // horizontal = left/right, vertical = top/bottom
		leftPanel?: Snippet;
		rightPanel?: Snippet;
		topPanel?: Snippet;
		bottomPanel?: Snippet;
	}

	let {
		initialSize = 50,
		minSize = 20,
		maxSize = 80,
		orientation = 'horizontal',
		leftPanel,
		rightPanel,
		topPanel,
		bottomPanel
	}: Props = $props();

	let isDragging = $state(false);
	let splitSize = $state(initialSize);
	let containerRef = $state<HTMLDivElement>();

	function handleMouseDown(e: MouseEvent) {
		e.preventDefault();
		isDragging = true;
		document.body.style.cursor = orientation === 'horizontal' ? 'col-resize' : 'row-resize';
		document.body.style.userSelect = 'none';
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isDragging || !containerRef) return;

		const rect = containerRef.getBoundingClientRect();
		let newSize: number;

		if (orientation === 'horizontal') {
			const mouseX = e.clientX - rect.left;
			newSize = (mouseX / rect.width) * 100;
		} else {
			const mouseY = e.clientY - rect.top;
			newSize = (mouseY / rect.height) * 100;
		}

		// Clamp between min and max
		splitSize = Math.max(minSize, Math.min(maxSize, newSize));
	}

	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
			document.body.style.cursor = '';
			document.body.style.userSelect = '';
		}
	}

	$effect(() => {
		if (isDragging) {
			document.addEventListener('mousemove', handleMouseMove);
			document.addEventListener('mouseup', handleMouseUp);

			return () => {
				document.removeEventListener('mousemove', handleMouseMove);
				document.removeEventListener('mouseup', handleMouseUp);
			};
		}
	});
</script>

<div
	bind:this={containerRef}
	class="flex {orientation === 'horizontal' ? 'flex-row' : 'flex-col'} h-full w-full overflow-hidden"
>
	<!-- First Panel -->
	<div
		style="{orientation === 'horizontal' ? 'width' : 'height'}: {splitSize}%;"
		class="overflow-hidden flex flex-col"
	>
		{#if orientation === 'horizontal' && leftPanel}
			{@render leftPanel()}
		{:else if orientation === 'vertical' && topPanel}
			{@render topPanel()}
		{/if}
	</div>

	<!-- Resize Handle -->
	<button
		type="button"
		aria-label="{orientation === 'horizontal' ? 'Resize panels horizontally' : 'Resize panels vertically'}"
		onmousedown={handleMouseDown}
		onkeydown={(e) => {
			if (e.key === 'ArrowLeft' && orientation === 'horizontal') {
				splitSize = Math.max(minSize, splitSize - 1);
			} else if (e.key === 'ArrowRight' && orientation === 'horizontal') {
				splitSize = Math.min(maxSize, splitSize + 1);
			} else if (e.key === 'ArrowUp' && orientation === 'vertical') {
				splitSize = Math.max(minSize, splitSize - 1);
			} else if (e.key === 'ArrowDown' && orientation === 'vertical') {
				splitSize = Math.min(maxSize, splitSize + 1);
			}
		}}
		class="relative flex-shrink-0 {orientation === 'horizontal'
			? 'w-1 cursor-col-resize hover:w-2'
			: 'h-1 cursor-row-resize hover:h-2'} bg-slate-200 dark:bg-slate-700 hover:bg-indigo-500 transition-all group z-10"
	>
		<!-- Visual indicator -->
		<div
			class="absolute inset-0 bg-indigo-500 opacity-0 group-hover:opacity-100 transition-opacity"
		></div>
	</button>

	<!-- Second Panel -->
	<div class="flex-1 overflow-hidden flex flex-col">
		{#if orientation === 'horizontal' && rightPanel}
			{@render rightPanel()}
		{:else if orientation === 'vertical' && bottomPanel}
			{@render bottomPanel()}
		{/if}
	</div>
</div>

