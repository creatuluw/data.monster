<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import Card from '$lib/components/Card.svelte';
	import Label from '$lib/components/Label.svelte';
	import KPITile from '$lib/components/KPITile.svelte';
	import RadioButtonGroup from '$lib/components/RadioButtonGroup.svelte';
	import Select from '$lib/components/Select.svelte';
	import { 
		Download, ArrowRight, Trash2, Plus, Database, 
		Network, LineChart, Search, BookOpen, Code,
		Palette, CheckCircle, AlertCircle, Info
	} from 'lucide-svelte';
	
	let isLoading = $state(false);
	let formData = $state({
		name: '',
		email: '',
		message: ''
	});
	
	// Radio button demo state
	let memoryOption = $state('16gb');
	let strategyOption = $state('manual');
	let themeOption = $state('light');
	
	// Select demo state
	let countryXs = $state('canada');
	let countrySm = $state('canada');
	let countryMd = $state('canada');
	let countryLg = $state('canada');
	let countryXl = $state('canada');
	
	async function handleAsync() {
		isLoading = true;
		await new Promise(resolve => setTimeout(resolve, 2000));
		isLoading = false;
	}
	
	function handleSubmit(e: Event) {
		e.preventDefault();
		console.log('Form submitted', formData);
	}
</script>

<PageLayout>
	<div class="max-w-7xl mx-auto space-y-12 scale-90 origin-top">
		
		<!-- Hero Section -->
		<section class="text-center py-12">
			<div class="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-green-500 to-purple-600 rounded-2xl mb-6">
				<Palette class="w-10 h-10 text-white" />
			</div>
			<h1 class="text-5xl md:text-6xl font-aspekta font-[650] text-slate-900 mb-4">
				Warphead Design System
			</h1>
			<p class="text-xl text-slate-600 max-w-3xl mx-auto mb-8">
				A comprehensive collection of reusable components, patterns, and guidelines for building beautiful, accessible interfaces.
			</p>
			<div class="flex gap-4 justify-center">
				<Button href="/docs/design-system" variant="primary" size="lg">
					<BookOpen class="w-5 h-5 mr-2" />
					View Documentation
				</Button>
				<Button href="/color-demo" variant="secondary" size="lg">
					<Palette class="w-5 h-5 mr-2" />
					Color Demo
				</Button>
			</div>
		</section>

		<!-- Quick Links -->
		<section class="bg-purple-50 border border-purple-200 rounded-xl p-8">
			<h2 class="text-2xl font-aspekta font-[650] text-slate-900 mb-6">Quick Navigation</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<a href="#foundations" class="flex items-center gap-3 p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<Palette class="w-5 h-5 text-purple-600" />
					<span class="font-medium">Foundations</span>
				</a>
				<a href="#components" class="flex items-center gap-3 p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<Code class="w-5 h-5 text-purple-600" />
					<span class="font-medium">Components</span>
				</a>
				<a href="#patterns" class="flex items-center gap-3 p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<Database class="w-5 h-5 text-purple-600" />
					<span class="font-medium">Patterns</span>
				</a>
			</div>
		</section>

		<!-- Foundations -->
		<section id="foundations" class="scroll-mt-20">
			<h2 class="text-3xl font-aspekta font-[650] text-slate-900 mb-2">Foundations</h2>
			<p class="text-slate-600 mb-8">Core design elements that form the basis of all interfaces</p>
			
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
				<!-- Colors -->
				<div class="rounded-xl border border-slate-200 p-6 bg-white">
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Color Palette</h3>
					<p class="text-sm text-slate-600 mb-6">
						Green primary, purple accent system with semantic colors
					</p>
					
					<div class="space-y-4">
						<div>
							<p class="text-xs font-medium text-slate-500 mb-2 uppercase">Primary Green</p>
							<div class="flex gap-2">
								<div class="flex-1">
									<div class="h-16 bg-green-600 rounded-lg"></div>
									<p class="text-xs text-center mt-1 font-mono">green-600</p>
								</div>
								<div class="flex-1">
									<div class="h-16 bg-green-700 rounded-lg"></div>
									<p class="text-xs text-center mt-1 font-mono">green-700</p>
								</div>
								<div class="flex-1">
									<div class="h-16 bg-green-100 rounded-lg border border-slate-200"></div>
									<p class="text-xs text-center mt-1 font-mono">green-100</p>
								</div>
							</div>
						</div>
						
						<div>
							<p class="text-xs font-medium text-slate-500 mb-2 uppercase">Accent Purple</p>
							<div class="flex gap-2">
								<div class="flex-1">
									<div class="h-16 bg-purple-600 rounded-lg"></div>
									<p class="text-xs text-center mt-1 font-mono">purple-600</p>
								</div>
								<div class="flex-1">
									<div class="h-16 bg-purple-700 rounded-lg"></div>
									<p class="text-xs text-center mt-1 font-mono">purple-700</p>
								</div>
								<div class="flex-1">
									<div class="h-16 bg-purple-50 rounded-lg border border-slate-200"></div>
									<p class="text-xs text-center mt-1 font-mono">purple-50</p>
								</div>
							</div>
						</div>
					</div>
					
					<div class="mt-6">
						<Button href="/color-demo" variant="ghost" size="sm" fullWidth>
							View Full Palette →
						</Button>
					</div>
				</div>
				
				<!-- Typography -->
				<div class="rounded-xl border border-slate-200 p-6 bg-white">
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Typography</h3>
					<p class="text-sm text-slate-600 mb-6">
						Three font families for different contexts
					</p>
					
					<div class="space-y-6">
						<div>
							<p class="text-xs font-medium text-slate-500 mb-2 uppercase">Aspekta (Headings)</p>
							<p class="text-3xl font-aspekta font-[650] text-slate-900">Quick Brown Fox</p>
							<p class="text-xs text-slate-500 mt-1 font-mono">font-aspekta font-[650]</p>
						</div>
						
						<div>
							<p class="text-xs font-medium text-slate-500 mb-2 uppercase">Inter (Body)</p>
							<p class="text-base font-inter text-slate-900">The quick brown fox jumps over the lazy dog</p>
							<p class="text-xs text-slate-500 mt-1 font-mono">font-inter</p>
						</div>
						
						<div>
							<p class="text-xs font-medium text-slate-500 mb-2 uppercase">PT Mono (Code)</p>
							<p class="text-sm font-pt-mono text-slate-900">SELECT * FROM users;</p>
							<p class="text-xs text-slate-500 mt-1 font-mono">font-pt-mono</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- Components -->
		<section id="components" class="scroll-mt-20">
			<h2 class="text-3xl font-aspekta font-[650] text-slate-900 mb-2">Components</h2>
			<p class="text-slate-600 mb-8">Reusable UI elements ready to use in your interfaces</p>
			
			<!-- Buttons -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white mb-6">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Buttons</h3>
				<p class="text-sm text-slate-600 mb-6">
					Five variants, multiple sizes, with support for icons and states
				</p>
				
				<div class="space-y-6">
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Variants</p>
						<div class="flex flex-wrap gap-3">
							<Button variant="primary">Primary</Button>
							<Button variant="secondary">Secondary</Button>
							<Button variant="accent">Accent</Button>
							<Button variant="ghost">Ghost</Button>
							<Button variant="danger">Danger</Button>
						</div>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">With Icons</p>
						<div class="flex flex-wrap gap-3">
							<Button variant="primary">
								<Plus class="w-4 h-4 mr-2" />
								Add Item
							</Button>
							<Button variant="secondary">
								<Download class="w-4 h-4 mr-2" />
								Download
							</Button>
							<Button variant="danger">
								<Trash2 class="w-4 h-4 mr-2" />
								Delete
							</Button>
						</div>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Sizes</p>
						<div class="flex flex-wrap items-center gap-3">
							<Button size="xs">Extra Small</Button>
							<Button size="sm">Small</Button>
							<Button size="md">Medium</Button>
							<Button size="lg">Large</Button>
							<Button size="xl">Extra Large</Button>
						</div>
					</div>
				</div>
				
				<div class="mt-6 pt-6 border-t border-slate-200">
					<details class="group">
						<summary class="cursor-pointer text-sm font-medium text-slate-700 hover:text-slate-900 flex items-center gap-2">
							<span>View Code Example</span>
							<ArrowRight class="w-4 h-4 transition-transform group-open:rotate-90" />
						</summary>
						<div class="mt-4 bg-slate-900 rounded-lg p-4">
							<pre class="text-sm text-slate-300 overflow-x-auto"><code>{`<Button variant="primary">
  <Plus class="w-4 h-4 mr-2" />
  Add Item
</Button>`}</code></pre>
						</div>
					</details>
				</div>
			</div>
			
			<!-- Cards -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white mb-6">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Cards</h3>
				<p class="text-sm text-slate-600 mb-6">
					Interactive content containers with icons, badges, and hover effects
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-stretch">
					<Card
						title="Data"
						description="Upload and manage your datasets"
						href="/data"
						icon={Database}
						badge="New"
						badgeColor="green"
					/>
					<Card
						title="Models"
						description="Create reusable data models"
						href="/models"
						icon={Network}
						badgeColor="purple"
					/>
					<Card
						title="Analysis"
						description="Analyze and visualize data"
						href="/analysis"
						icon={LineChart}
						badgeColor="blue"
					/>
				</div>
				
				<div class="mt-6 pt-6 border-t border-slate-200">
					<details class="group">
						<summary class="cursor-pointer text-sm font-medium text-slate-700 hover:text-slate-900 flex items-center gap-2">
							<span>View Code Example</span>
							<ArrowRight class="w-4 h-4 transition-transform group-open:rotate-90" />
						</summary>
						<div class="mt-4 bg-slate-900 rounded-lg p-4">
							<pre class="text-sm text-slate-300 overflow-x-auto"><code>{`<Card
  title="Data"
  description="Upload and manage datasets"
  href="/data"
  icon={Database}
  badge="New"
  badgeColor="green"
/>`}</code></pre>
						</div>
					</details>
				</div>
			</div>
			
			<!-- KPI Tiles -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white mb-6">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">KPI Tiles</h3>
				<p class="text-sm text-slate-600 mb-6">
					Display metrics with optional trend indicators
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
					<KPITile 
						title="Total Users" 
						value="1,234"
						trend="up"
						trendValue="+12%"
					/>
					<KPITile 
						title="Revenue" 
						value="$45,678"
						trend="up"
						trendValue="+8%"
					/>
					<KPITile 
						title="Queries" 
						value="9,876"
						trend="down"
						trendValue="-3%"
					/>
					<KPITile 
						title="Storage" 
						value="45 GB"
						subtitle="of 100 GB"
					/>
				</div>
			</div>
			
			<!-- Forms -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Form Elements</h3>
				<p class="text-sm text-slate-600 mb-6">
					Consistent form controls with purple accent theme
				</p>
				
				<form onsubmit={handleSubmit} class="max-w-xl space-y-6">
					<div>
						<Label for="name">Full Name</Label>
						<input
							id="name"
							type="text"
							bind:value={formData.name}
							class="input-primary w-full px-4 py-2.5 rounded-lg"
							placeholder="Enter your name"
						/>
					</div>
					
					<div>
						<Label for="email">Email Address</Label>
						<input
							id="email"
							type="email"
							bind:value={formData.email}
							class="input-primary w-full px-4 py-2.5 rounded-lg"
							placeholder="you@example.com"
						/>
						<p class="text-xs text-slate-500 mt-1">We'll never share your email</p>
					</div>
					
					<div>
						<Label for="message">Message</Label>
						<textarea
							id="message"
							bind:value={formData.message}
							class="input-primary w-full px-4 py-2.5 rounded-lg"
							rows="4"
							placeholder="Your message here..."
						></textarea>
					</div>
					
					<div class="flex gap-3">
						<Button type="submit" variant="accent">Submit Form</Button>
						<Button type="button" variant="ghost">Reset</Button>
					</div>
				</form>
			</div>
			
			<!-- Radio Button Groups -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white mt-6">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Radio Button Groups</h3>
				<p class="text-sm text-slate-600 mb-6">
					Visually enhanced radio button groups with icon support and multiple sizes
				</p>
				
				<div class="space-y-8">
					<!-- Size Examples -->
					<div>
						<p class="text-xs font-medium text-slate-500 mb-4 uppercase">Size: Extra Small (xs)</p>
						<RadioButtonGroup
							bind:value={strategyOption}
							name="strategy-xs"
							options={[
								{ value: 'manual', label: 'Manual', icon: Download },
								{ value: 'auto', label: 'Auto', icon: CheckCircle },
								{ value: 'scheduled', label: 'Scheduled', icon: AlertCircle }
							]}
							size="xs"
							columns={3}
						/>
						<p class="text-xs text-slate-500 mt-2">Selected: {strategyOption}</p>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-4 uppercase">Size: Small (sm)</p>
						<RadioButtonGroup
							bind:value={themeOption}
							name="theme-sm"
							label="Theme"
							options={[
								{ value: 'light', label: 'Light' },
								{ value: 'dark', label: 'Dark' },
								{ value: 'auto', label: 'Auto' }
							]}
							size="sm"
							columns={3}
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-4 uppercase">Size: Medium (md) - Default</p>
						<RadioButtonGroup
							bind:value={memoryOption}
							name="memory-md"
							label="RAM"
							helperText="Choose your memory configuration"
							options={[
								{ value: '4gb', label: '4 GB' },
								{ value: '8gb', label: '8 GB' },
								{ value: '16gb', label: '16 GB' },
								{ value: '32gb', label: '32 GB' },
								{ value: '64gb', label: '64 GB' },
								{ value: '128gb', label: '128 GB', disabled: true }
							]}
							size="md"
							columns={3}
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-4 uppercase">Size: Large (lg)</p>
						<RadioButtonGroup
							bind:value={memoryOption}
							name="memory-lg"
							label="RAM Configuration"
							options={[
								{ value: '8gb', label: '8 GB' },
								{ value: '16gb', label: '16 GB' },
								{ value: '32gb', label: '32 GB' }
							]}
							size="lg"
							columns={3}
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-4 uppercase">Size: Extra Large (xl)</p>
						<RadioButtonGroup
							bind:value={themeOption}
							name="theme-xl"
							label="Choose Theme"
							options={[
								{ value: 'light', label: 'Light', icon: Database },
								{ value: 'dark', label: 'Dark', icon: Network }
							]}
							size="xl"
							columns={2}
						/>
					</div>
				</div>
				
				<div class="mt-6 pt-6 border-t border-slate-200">
					<details class="group">
						<summary class="cursor-pointer text-sm font-medium text-slate-700 hover:text-slate-900 flex items-center gap-2">
							<span>View Code Example</span>
							<ArrowRight class="w-4 h-4 transition-transform group-open:rotate-90" />
						</summary>
						<div class="mt-4 bg-slate-900 rounded-lg p-4">
							<pre class="text-sm text-slate-300 overflow-x-auto"><code>{`<RadioButtonGroup
  bind:value={selectedValue}
  name="strategy"
  label="Ingestion Strategy"
  helperText="Choose how data should be loaded"
  options={[
    { value: 'manual', label: 'Manual', icon: Download },
    { value: 'auto', label: 'Auto', icon: CheckCircle },
    { value: 'scheduled', label: 'Scheduled', icon: AlertCircle }
  ]}
  size="md"
  columns={3}
  onchange={(value) => console.log('Selected:', value)}
/>`}</code></pre>
						</div>
					</details>
				</div>
			</div>
			
			<!-- Select Dropdowns -->
			<div class="rounded-xl border border-slate-200 p-6 bg-white mt-6">
				<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Select Dropdowns</h3>
				<p class="text-sm text-slate-600 mb-6">
					Styled select dropdowns with custom chevron icon and multiple sizes
				</p>
				
				<div class="space-y-8 max-w-2xl">
					<!-- Size Examples -->
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Size: Extra Small (xs)</p>
						<Select
							bind:value={countryXs}
							name="country-xs"
							label="Location"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' },
								{ value: 'uk', label: 'United Kingdom' }
							]}
							size="xs"
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Size: Small (sm)</p>
						<Select
							bind:value={countrySm}
							name="country-sm"
							label="Location"
							helperText="Select your country"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' },
								{ value: 'uk', label: 'United Kingdom' }
							]}
							size="sm"
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Size: Medium (md) - Default</p>
						<Select
							bind:value={countryMd}
							name="country-md"
							label="Location"
							helperText="Choose your primary location"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' },
								{ value: 'uk', label: 'United Kingdom' },
								{ value: 'germany', label: 'Germany' },
								{ value: 'france', label: 'France' }
							]}
							size="md"
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Size: Large (lg)</p>
						<Select
							bind:value={countryLg}
							name="country-lg"
							label="Location"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' }
							]}
							size="lg"
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Size: Extra Large (xl)</p>
						<Select
							bind:value={countryXl}
							name="country-xl"
							label="Choose Your Country"
							helperText="This will be your primary billing location"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' }
							]}
							size="xl"
						/>
					</div>
					
					<!-- With Placeholder and Error States -->
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">With Placeholder</p>
						<Select
							value=""
							name="country-placeholder"
							label="Location"
							placeholder="Select a country..."
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' }
							]}
							size="md"
						/>
					</div>
					
					<div>
						<p class="text-xs font-medium text-slate-500 mb-3 uppercase">Error State</p>
						<Select
							value=""
							name="country-error"
							label="Location"
							required
							error="Please select a location"
							options={[
								{ value: 'usa', label: 'United States' },
								{ value: 'canada', label: 'Canada' },
								{ value: 'mexico', label: 'Mexico' }
							]}
							size="md"
						/>
					</div>
				</div>
				
				<div class="mt-6 pt-6 border-t border-slate-200">
					<details class="group">
						<summary class="cursor-pointer text-sm font-medium text-slate-700 hover:text-slate-900 flex items-center gap-2">
							<span>View Code Example</span>
							<ArrowRight class="w-4 h-4 transition-transform group-open:rotate-90" />
						</summary>
						<div class="mt-4 bg-slate-900 rounded-lg p-4">
							<pre class="text-sm text-slate-300 overflow-x-auto"><code>{`<Select
  bind:value={selectedCountry}
  name="country"
  label="Location"
  helperText="Select your country"
  placeholder="Choose a country..."
  options={[
    { value: 'usa', label: 'United States' },
    { value: 'canada', label: 'Canada' },
    { value: 'mexico', label: 'Mexico' }
  ]}
  size="md"
  required
  onchange={(value) => console.log('Selected:', value)}
/>`}</code></pre>
						</div>
					</details>
				</div>
			</div>
		</section>

		<!-- Patterns -->
		<section id="patterns" class="scroll-mt-20">
			<h2 class="text-3xl font-aspekta font-[650] text-slate-900 mb-2">Common Patterns</h2>
			<p class="text-slate-600 mb-8">Pre-built layouts and UI patterns for common scenarios</p>
			
			<div class="space-y-6">
				<!-- Search Pattern -->
				<div class="rounded-xl border border-slate-200 p-6 bg-white">
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Search Bar Pattern</h3>
					<div class="relative max-w-md">
						<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
						<input
							type="search"
							placeholder="Search..."
							class="w-full pl-10 pr-4 py-2.5 border border-slate-200 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200"
						/>
					</div>
				</div>
				
				<!-- Alert Patterns -->
				<div class="rounded-xl border border-slate-200 p-6 bg-white">
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Alert Messages</h3>
					<div class="space-y-4 max-w-2xl">
						<!-- Success -->
						<div class="bg-green-50 border border-green-200 rounded-lg p-4 flex items-start gap-3">
							<CheckCircle class="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
							<div class="flex-1">
								<h4 class="font-semibold text-green-900 mb-1">Success!</h4>
								<p class="text-sm text-green-700">Your changes have been saved successfully.</p>
							</div>
						</div>
						
						<!-- Error -->
						<div class="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3">
							<AlertCircle class="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
							<div class="flex-1">
								<h4 class="font-semibold text-red-900 mb-1">Error</h4>
								<p class="text-sm text-red-700">Unable to process request. Please try again.</p>
							</div>
						</div>
						
						<!-- Info -->
						<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 flex items-start gap-3">
							<Info class="w-5 h-5 text-blue-600 flex-shrink-0 mt-0.5" />
							<div class="flex-1">
								<h4 class="font-semibold text-blue-900 mb-1">Information</h4>
								<p class="text-sm text-blue-700">New features are available in this update.</p>
							</div>
						</div>
					</div>
				</div>
				
				<!-- Empty State -->
				<div class="rounded-xl border border-slate-200 p-6 bg-white">
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 mb-4">Empty State Pattern</h3>
					<div class="text-center py-12 max-w-md mx-auto">
						<div class="w-16 h-16 bg-slate-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<Database class="w-8 h-8 text-slate-400" />
						</div>
						<h3 class="text-lg font-semibold text-slate-900 mb-2">No data yet</h3>
						<p class="text-sm text-slate-600 mb-4">
							Get started by uploading your first file
						</p>
						<Button variant="primary">
							<Plus class="w-4 h-4 mr-2" />
							Upload File
						</Button>
					</div>
				</div>
			</div>
		</section>

		<!-- Resources -->
		<section class="bg-gradient-to-br from-green-50 to-purple-50 border border-green-200 rounded-xl p-8">
			<h2 class="text-2xl font-aspekta font-[650] text-slate-900 mb-4">Resources & Documentation</h2>
			<p class="text-slate-600 mb-6">
				Complete guides, code examples, and best practices
			</p>
			
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				<a href="/docs/design-system" class="block p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<BookOpen class="w-6 h-6 text-green-600 mb-2" />
					<h3 class="font-semibold text-slate-900 mb-1">Full Documentation</h3>
					<p class="text-sm text-slate-600">Complete design system docs</p>
				</a>
				
				<a href="/color-demo" class="block p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<Palette class="w-6 h-6 text-purple-600 mb-2" />
					<h3 class="font-semibold text-slate-900 mb-1">Color Demo</h3>
					<p class="text-sm text-slate-600">Interactive color palette</p>
				</a>
				
				<a href="/" class="block p-4 bg-white rounded-lg hover:shadow-md transition-shadow">
					<Code class="w-6 h-6 text-blue-600 mb-2" />
					<h3 class="font-semibold text-slate-900 mb-1">Live Examples</h3>
					<p class="text-sm text-slate-600">See components in action</p>
				</a>
			</div>
		</section>

		<!-- Getting Started -->
		<section class="rounded-xl border-2 border-green-500 bg-green-50 p-8">
			<h2 class="text-2xl font-aspekta font-[650] text-slate-900 mb-4">Ready to Build?</h2>
			<p class="text-slate-700 mb-6 max-w-2xl">
				Start using the Warphead design system in your interfaces. All components are TypeScript-ready, accessible, and fully documented.
			</p>
			
			<div class="bg-slate-900 rounded-lg p-4 mb-6 max-w-3xl">
				<pre class="text-sm text-slate-300 overflow-x-auto"><code>{`// Import components
import Button from '$lib/components/Button.svelte';
import Card from '$lib/components/Card.svelte';

// Use in your interface
<Card title="Feature" description="..." href="/feature" />
<Button variant="primary">Get Started</Button>`}</code></pre>
			</div>
			
			<div class="flex gap-4">
				<Button href="/docs/design-system" variant="primary" size="lg">
					Read the Docs
				</Button>
				<Button href="https://github.com/your-repo" variant="secondary" size="lg">
					View on GitHub
				</Button>
			</div>
		</section>

	</div>
</PageLayout>
