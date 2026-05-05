<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	
	let pushNotification = $state(true);
	let fullName = $state('Timothy Cain');
	let password = $state('••••••••');
</script>

<PageLayout>
	<div class="grid lg:grid-cols-2 gap-8">
				<!-- Left Column: Color Swatches -->
				<div>
					<h2 class="text-lg font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-6">Color Presets</h2>
			
			<!-- Green Palette -->
			<div class="mb-8 bg-white rounded-lg shadow-md p-6">
				<h3 class="text-lg font-semibold text-slate-800 mb-4">Primary Green</h3>
				<div class="flex gap-2">
					<div class="flex-1">
						<div class="h-20 bg-green-700 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">700</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-green-600 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">600</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-green-500 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">500</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-green-200 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">200</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-green-100 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">100</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-green-50 rounded-t-lg border border-slate-200"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">50</div>
					</div>
				</div>
			</div>

			<!-- Purple Palette -->
			<div class="mb-8 bg-white rounded-lg shadow-md p-6">
				<h3 class="text-lg font-semibold text-slate-800 mb-4">Accent Purple</h3>
				<div class="flex gap-2">
					<div class="flex-1">
						<div class="h-20 bg-purple-700 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">700</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-purple-600 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">600</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-purple-500 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">500</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-purple-200 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">200</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-purple-100 rounded-t-lg"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">100</div>
					</div>
					<div class="flex-1">
						<div class="h-20 bg-purple-50 rounded-t-lg border border-slate-200"></div>
						<div class="bg-slate-50 p-2 text-center text-xs font-mono rounded-b-lg">50</div>
					</div>
				</div>
			</div>

			<!-- Button Examples -->
			<div class="bg-white rounded-lg shadow-md p-6">
				<h3 class="text-lg font-semibold text-slate-800 mb-4">Button Styles</h3>
				
				<div class="space-y-4">
					<div>
						<p class="text-sm text-slate-600 mb-2">Primary (Green)</p>
						<Button variant="primary">
							View Profile
						</Button>
					</div>
					
					<div>
						<p class="text-sm text-slate-600 mb-2">Secondary (Light Green)</p>
						<Button variant="secondary">
							Change Status
						</Button>
					</div>
					
					<div>
						<p class="text-sm text-slate-600 mb-2">Accent (Purple)</p>
						<Button variant="accent" fullWidth>
							Save Changes
						</Button>
					</div>
				</div>
			</div>
		</div>

				<!-- Right Column: Live UI Kit Demo -->
				<div>
					<h2 class="text-lg font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-6">⚡ Live UI Kit Demo</h2>
					<p class="text-sm text-slate-500 dark:text-slate-400 mb-6">Interactive component preview</p>
			
			<div class="bg-white rounded-lg shadow-lg p-8">
				<!-- Profile Header -->
				<div class="flex items-center gap-4 mb-6">
					<div class="relative">
						<div class="w-20 h-20 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white text-2xl font-bold">
							AB
						</div>
						<div class="absolute bottom-0 right-0 w-4 h-4 bg-green-500 rounded-full border-2 border-white"></div>
					</div>
					<div>
						<h3 class="text-xl font-semibold text-slate-900">Anna Black</h3>
						<p class="text-slate-600">Sr. Customer Manager</p>
					</div>
				</div>

				<!-- Action Buttons -->
				<div class="flex gap-3 mb-6">
					<Button variant="primary" class="flex-1">
						View profile
					</Button>
					<Button variant="secondary" class="flex-1">
						Change Status
					</Button>
				</div>

				<!-- Save Button -->
				<Button variant="accent" fullWidth class="mb-6">
					Save Changes
				</Button>

				<!-- Form Fields -->
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div>
						<label for="fullName" class="block text-xs font-medium text-slate-500 mb-2 uppercase tracking-wide">
							Full Name
						</label>
						<input
							id="fullName"
							type="text"
							bind:value={fullName}
							class="input-primary w-full px-4 py-2.5 rounded-lg text-slate-700 font-medium focus:outline-none focus:ring-2"
						/>
					</div>
					<div>
						<label for="password" class="block text-xs font-medium text-slate-500 mb-2 uppercase tracking-wide">
							Password
						</label>
						<input
							id="password"
							type="password"
							bind:value={password}
							class="input-primary w-full px-4 py-2.5 rounded-lg text-slate-700 font-medium focus:outline-none focus:ring-2"
						/>
					</div>
				</div>

				<!-- Icon Buttons -->
				<div class="flex gap-4 mb-6 justify-center">
					<button aria-label="Messages" class="w-12 h-12 rounded-full bg-green-100 text-green-700 hover:bg-green-200 transition-colors flex items-center justify-center">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path>
						</svg>
					</button>
					<button aria-label="Location" class="w-12 h-12 rounded-full bg-purple-100 text-purple-700 hover:bg-purple-200 transition-colors flex items-center justify-center">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
						</svg>
					</button>
					<button aria-label="Email" class="w-12 h-12 rounded-full bg-green-100 text-green-700 hover:bg-green-200 transition-colors flex items-center justify-center">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
						</svg>
					</button>
					<button aria-label="Security" class="w-12 h-12 rounded-full bg-purple-100 text-purple-700 hover:bg-purple-200 transition-colors flex items-center justify-center">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
						</svg>
					</button>
					<button aria-label="Calendar" class="w-12 h-12 rounded-full bg-orange-100 text-orange-700 hover:bg-orange-200 transition-colors flex items-center justify-center">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
						</svg>
					</button>
				</div>

				<!-- Toggle Switch -->
				<div class="flex items-center justify-between py-4 border-t border-slate-200">
					<div>
						<p class="font-medium text-slate-900">Push Notification</p>
						<p class="text-sm text-slate-500">Commits data and history</p>
					</div>
					<label class="relative inline-flex items-center cursor-pointer">
						<input
							type="checkbox"
							bind:checked={pushNotification}
							class="sr-only peer"
						/>
						<div class="w-14 h-8 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-purple-300 rounded-full peer peer-checked:after:translate-x-6 rtl:peer-checked:after:-translate-x-6 peer-checked:after:border-white after:content-[''] after:absolute after:top-1 after:start-1 after:bg-white after:border-slate-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-purple-600"></div>
					</label>
				</div>

				<!-- Sign In Link -->
				<div class="text-center mt-6 pt-4 border-t border-slate-200">
					<p class="text-sm text-slate-500">
						Already have an account? <a href="/signin" class="text-green-600 font-medium hover:text-green-700">Sign In</a>
					</p>
				</div>
			</div>

			<!-- Usage Instructions -->
			<div class="mt-6 bg-purple-50 border border-purple-200 rounded-lg p-6">
				<h3 class="text-lg font-semibold text-purple-900 mb-3">💡 How to Use</h3>
				<div class="space-y-2 text-sm text-purple-800">
					<p><strong>Primary buttons:</strong> Use <code class="px-2 py-1 bg-purple-100 rounded text-xs">&lt;Button variant="primary"&gt;</code> for main actions</p>
					<p><strong>Secondary buttons:</strong> Use <code class="px-2 py-1 bg-purple-100 rounded text-xs">&lt;Button variant="secondary"&gt;</code> for alternative actions</p>
					<p><strong>Accent buttons:</strong> Use <code class="px-2 py-1 bg-purple-100 rounded text-xs">&lt;Button variant="accent"&gt;</code> for important CTAs</p>
					<p><strong>Form inputs:</strong> Use <code class="px-2 py-1 bg-purple-100 rounded text-xs">input-primary</code> for styled inputs</p>
				</div>
			</div>
		</div>
	</div>

			<!-- Color Reference -->
			<div class="mt-12 bg-slate-50 dark:bg-slate-900 rounded-lg p-8">
				<h2 class="text-lg font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-6">Color Reference</h2>
		<div class="grid md:grid-cols-2 gap-6">
			<div>
				<h3 class="font-semibold text-slate-800 mb-3">Green Palette</h3>
				<div class="space-y-2 font-mono text-sm">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-green-50 border border-slate-300 rounded"></div>
						<span>green-50: #e6ffe6</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-green-100 rounded"></div>
						<span>green-100: #c0f0c0</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-green-500 rounded"></div>
						<span>green-500: #00b300</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-green-600 rounded"></div>
						<span>green-600: #009900</span>
					</div>
				</div>
			</div>
			<div>
				<h3 class="font-semibold text-slate-800 mb-3">Purple Palette</h3>
				<div class="space-y-2 font-mono text-sm">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-purple-50 border border-slate-300 rounded"></div>
						<span>purple-50: #f3e6ff</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-purple-100 rounded"></div>
						<span>purple-100: #e8d8ff</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-purple-500 rounded"></div>
						<span>purple-500: #9b00ff</span>
					</div>
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-purple-600 rounded"></div>
						<span>purple-600: #8b00ff</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</PageLayout>

