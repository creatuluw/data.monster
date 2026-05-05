<script lang="ts">
	interface Props {
		title?: string;
		subtitle?: string;
		children?: any;
	}
	
	let { title, subtitle, children }: Props = $props();
</script>

<main class="flex-grow">
	<div class="w-[90vw] mx-auto px-4 sm:px-6 lg:px-8">
		<!-- Page Header - Left Aligned -->
		<div class="pt-12 pb-6">
			{#if title}
				<h1 class="h1 font-inter text-slate-900 dark:text-slate-100 mb-2">{title}</h1>
				{#if subtitle}
					<p class="text-lg text-slate-600 dark:text-slate-400">{subtitle}</p>
				{/if}
			{/if}
		</div>
		
		<!-- Page Content -->
		<div class="pb-8">
			{@render children?.()}
		</div>
	</div>
</main>

