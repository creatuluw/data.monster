<script lang="ts">
	import { page } from '$app/stores';
	
	// Generate breadcrumbs from the current path
	const breadcrumbs = $derived(() => {
		const currentPage = $page.url.pathname;
		const segments = currentPage.split('/').filter(Boolean);
		
		const crumbs = [{ label: 'Home', href: '/' }];
		
		let path = '';
		segments.forEach((segment) => {
			path += `/${segment}`;
			// Convert kebab-case to Title Case
			const label = segment
				.split('-')
				.map(word => word.charAt(0).toUpperCase() + word.slice(1))
				.join(' ');
			crumbs.push({ label, href: path });
		});
		
		return crumbs;
	});
</script>

<!-- Breadcrumb Bar - Part of header container flow -->
<div class="bg-[#F9F9F9] border-b border-slate-200 dark:border-slate-700 dark:bg-slate-800">
	<div class="px-4 sm:px-6 lg:px-8 py-2">
		<nav aria-label="Breadcrumb">
			<ol class="flex items-center space-x-2 text-xs">
				{#each breadcrumbs() as crumb, index}
					{#if index > 0}
						<li class="text-slate-400 dark:text-slate-500">
							<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
							</svg>
						</li>
					{/if}
					<li>
						{#if index === breadcrumbs().length - 1}
							<span class="font-bold text-slate-900 dark:text-slate-100">{crumb.label}</span>
						{:else}
							<a 
								href={crumb.href} 
								class="text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
							>
								{crumb.label}
							</a>
						{/if}
					</li>
				{/each}
			</ol>
		</nav>
	</div>
</div>

