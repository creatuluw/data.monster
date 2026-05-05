<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Star, Search, TableIcon, FileText, Bookmark } from 'lucide-svelte';
	import Card from '$lib/components/Card.svelte';
</script>

<PageLayout>
	<!-- Bookmark Categories Section -->
			<section class="mb-12">
				<h2 class="font-aspekta text-xl font-[650] mb-6">Quick Access</h2>
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
					<!-- Queries Card - Placeholder -->
					<div class="relative block overflow-hidden rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 opacity-60">
						<div class="relative flex flex-col h-full p-6">
							<div class="absolute top-3 right-3">
								<div class="h-8 w-8 flex items-center justify-center border rounded-lg bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-800">
									<Search class="w-3.5 h-3.5 text-blue-600 dark:text-blue-400" />
								</div>
							</div>
							<div class="absolute bottom-3 right-3">
								<div class="text-xs font-semibold inline-flex items-center rounded-full px-2.5 py-1 bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400">
									Coming Soon
								</div>
							</div>
							<div class="grow space-y-2">
								<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 tracking-tight">Queries</h3>
								<p class="text-sm leading-relaxed text-slate-600 dark:text-slate-400">Save your favorite SQL queries</p>
							</div>
						</div>
					</div>

					<!-- Tables Card - Placeholder -->
					<div class="relative block overflow-hidden rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 opacity-60">
						<div class="relative flex flex-col h-full p-6">
							<div class="absolute top-3 right-3">
								<div class="h-8 w-8 flex items-center justify-center border rounded-lg bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-800">
									<TableIcon class="w-3.5 h-3.5 text-green-600 dark:text-green-400" />
								</div>
							</div>
							<div class="absolute bottom-3 right-3">
								<div class="text-xs font-semibold inline-flex items-center rounded-full px-2.5 py-1 bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400">
									Coming Soon
								</div>
							</div>
							<div class="grow space-y-2">
								<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 tracking-tight">Tables</h3>
								<p class="text-sm leading-relaxed text-slate-600 dark:text-slate-400">Quick access to frequently viewed tables</p>
							</div>
						</div>
					</div>

					<!-- Views Card - Placeholder -->
					<div class="relative block overflow-hidden rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 opacity-60">
						<div class="relative flex flex-col h-full p-6">
							<div class="absolute top-3 right-3">
								<div class="h-8 w-8 flex items-center justify-center border rounded-lg bg-purple-50 border-purple-200 dark:bg-purple-900/20 dark:border-purple-800">
									<FileText class="w-3.5 h-3.5 text-purple-600 dark:text-purple-400" />
								</div>
							</div>
							<div class="absolute bottom-3 right-3">
								<div class="text-xs font-semibold inline-flex items-center rounded-full px-2.5 py-1 bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400">
									Coming Soon
								</div>
							</div>
							<div class="grow space-y-2">
								<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 tracking-tight">Views</h3>
								<p class="text-sm leading-relaxed text-slate-600 dark:text-slate-400">Saved analysis views and dashboards</p>
							</div>
						</div>
					</div>

					<!-- Collections Card - Placeholder -->
					<div class="relative block overflow-hidden rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 opacity-60">
						<div class="relative flex flex-col h-full p-6">
							<div class="absolute top-3 right-3">
								<div class="h-8 w-8 flex items-center justify-center border rounded-lg bg-orange-50 border-orange-200 dark:bg-orange-900/20 dark:border-orange-800">
									<Bookmark class="w-3.5 h-3.5 text-orange-600 dark:text-orange-400" />
								</div>
							</div>
							<div class="absolute bottom-3 right-3">
								<div class="text-xs font-semibold inline-flex items-center rounded-full px-2.5 py-1 bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400">
									Coming Soon
								</div>
							</div>
							<div class="grow space-y-2">
								<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 tracking-tight">Collections</h3>
								<p class="text-sm leading-relaxed text-slate-600 dark:text-slate-400">Organize bookmarks into collections</p>
							</div>
						</div>
					</div>
				</div>
			</section>

			<!-- Recent Bookmarks Section -->
			<section class="mb-12">
				<h2 class="font-aspekta text-xl font-[650] mb-6">Recent Bookmarks</h2>
				<div class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 p-12 text-center">
					<div class="flex justify-center mb-6">
						<Star class="w-24 h-24 text-slate-300 dark:text-slate-700" strokeWidth={1.5} />
					</div>
					<h3 class="text-2xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-3">
						No Bookmarks Yet
					</h3>
					<p class="text-slate-600 dark:text-slate-400 max-w-md mx-auto">
						Start bookmarking your favorite queries, tables, and views. They'll appear here for quick access.
					</p>
				</div>
			</section>

			<!-- Bookmark Features Section -->
			<section class="mt-12">
				<h2 class="font-aspekta text-xl font-[650] mb-6">Features Coming Soon</h2>
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
					<Card
						title="Smart Tags"
						description="Automatically organize bookmarks with AI-powered tagging"
						href="#tags"
						icon={Bookmark}
						badge="Coming Soon"
						badgeColor="blue"
					/>
					<Card
						title="Quick Search"
						description="Find your bookmarks instantly with powerful search"
						href="#search"
						icon={Search}
						badge="Coming Soon"
						badgeColor="green"
					/>
					<Card
						title="Share Collections"
						description="Share bookmark collections with your team"
						href="#share"
						icon={FileText}
						badge="Coming Soon"
						badgeColor="purple"
					/>
					<Card
						title="Sync & Backup"
						description="Keep your bookmarks synced across devices"
						href="#sync"
						icon={Star}
						badge="Coming Soon"
						badgeColor="orange"
					/>
			</div>
		</section>
</PageLayout>

