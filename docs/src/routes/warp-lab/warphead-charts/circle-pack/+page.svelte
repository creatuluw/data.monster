<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Circle } from 'lucide-svelte';
	import { hierarchy, pack } from 'd3-hierarchy';
	import type { HierarchyNode } from 'd3-hierarchy';

	// Helper function to generate hierarchical data
	function generateHierarchy(levels: number, branchesPerLevel: number[], startValue: number): any {
		if (levels === 0) {
			return { name: `Leaf ${Math.random().toString(36).substr(2, 5)}`, value: startValue * (0.5 + Math.random() * 0.5) };
		}
		const children: any[] = [];
		const branchCount = branchesPerLevel[0] || 5;
		const childValue = startValue / branchCount;
		for (let i = 0; i < branchCount; i++) {
			children.push(generateHierarchy(levels - 1, branchesPerLevel.slice(1), childValue));
		}
		return {
			name: `Level ${levels}`,
			value: startValue,
			children
		};
	}

	// Hierarchical data structures for circle packing
	// Basic hierarchical data - 50+ nodes
	const basicHierarchyData = {
		name: 'Root',
		value: 10000,
		children: Array.from({ length: 10 }, (_, i) => ({
			name: `Category ${String.fromCharCode(65 + i)}`,
			value: 800 + Math.random() * 400 + Math.sin(i * 0.5) * 200
		}))
	};

	// Multi-level hierarchy - 200+ nodes
	const departments = ['Sales', 'Marketing', 'Support', 'Operations', 'Development', 'HR', 'Finance', 'Legal', 'IT', 'Research'];
	const multiLevelHierarchyData = {
		name: 'Company',
		value: 50000,
		children: departments.map((dept, dIdx) => ({
			name: dept,
			value: 4000 + Math.random() * 2000,
			children: Array.from({ length: 5 }, (_, qIdx) => ({
				name: `Q${qIdx + 1}`,
				value: 600 + Math.random() * 400 + Math.sin(dIdx * 0.3) * 200,
				children: Array.from({ length: 4 }, (_, tIdx) => ({
					name: `Team ${tIdx + 1}`,
					value: 100 + Math.random() * 150 + Math.sin(qIdx * 0.2) * 50
				}))
			}))
		}))
	};

	// Category-based hierarchy - 300+ nodes
	const mainCategories = ['Electronics', 'Clothing', 'Home', 'Sports', 'Books', 'Toys', 'Health', 'Beauty', 'Automotive', 'Garden'];
	const categoryHierarchyData = {
		name: 'Products',
		value: 100000,
		children: mainCategories.map((category, cIdx) => ({
			name: category,
			value: 8000 + Math.random() * 2000,
			children: Array.from({ length: 6 }, (_, sIdx) => ({
				name: `Subcategory ${sIdx + 1}`,
				value: 1000 + Math.random() * 800 + Math.sin(cIdx * 0.2) * 300,
				children: Array.from({ length: 5 }, (_, pIdx) => ({
					name: `Product ${pIdx + 1}`,
					value: 150 + Math.random() * 200 + Math.sin(sIdx * 0.3) * 80
				}))
			}))
		}))
	};

	// Deep hierarchy - 400+ nodes (4 levels)
	const regions = ['Region 1', 'Region 2', 'Region 3', 'Region 4', 'Region 5'];
	const deepHierarchyData = {
		name: 'Organization',
		value: 100000,
		children: regions.map((region, rIdx) => ({
			name: region,
			value: 15000 + Math.random() * 5000,
			children: Array.from({ length: 8 }, (_, dIdx) => ({
				name: `Department ${String.fromCharCode(65 + dIdx)}`,
				value: 1500 + Math.random() * 800 + Math.sin(rIdx * 0.2) * 300,
				children: Array.from({ length: 6 }, (_, tIdx) => ({
					name: `Team ${tIdx + 1}`,
					value: 200 + Math.random() * 150 + Math.sin(dIdx * 0.3) * 80,
					children: Array.from({ length: 3 }, (_, mIdx) => ({
						name: `Member ${mIdx + 1}`,
						value: 50 + Math.random() * 50 + Math.sin(tIdx * 0.4) * 20
					}))
				}))
			}))
		}))
	};

	// Simple flat hierarchy (for comparison) - 100 items
	const flatHierarchyData = {
		name: 'Root',
		value: 50000,
		children: Array.from({ length: 100 }, (_, i) => ({
			name: `Item ${i + 1}`,
			value: 300 + Math.random() * 200 + Math.sin(i * 0.1) * 100
		}))
	};

	// Complex mixed hierarchy - 500+ nodes
	const divisions = ['Division 1', 'Division 2', 'Division 3', 'Division 4', 'Division 5'];
	const complexHierarchyData = {
		name: 'Enterprise',
		value: 200000,
		children: divisions.map((division, divIdx) => ({
			name: division,
			value: 30000 + Math.random() * 10000,
			children: Array.from({ length: 8 }, (_, uIdx) => {
				const hasChildren = Math.random() > 0.3;
				return {
					name: `Unit ${String.fromCharCode(65 + uIdx)}`,
					value: 2000 + Math.random() * 1500 + Math.sin(divIdx * 0.2) * 500,
					children: hasChildren ? Array.from({ length: 6 }, (_, pIdx) => ({
						name: `${uIdx % 2 === 0 ? 'Product' : 'Service'} ${pIdx + 1}`,
						value: 200 + Math.random() * 300 + Math.sin(uIdx * 0.3) * 100,
						children: Math.random() > 0.5 ? Array.from({ length: 3 }, (_, sIdx) => ({
							name: `Sub-item ${sIdx + 1}`,
							value: 50 + Math.random() * 100 + Math.sin(pIdx * 0.4) * 30
						})) : undefined
					})).filter(c => c) : undefined
				};
			})
		}))
	};

	interface PackedNode extends HierarchyNode<any> {
		x: number;
		y: number;
		r: number;
	}

	// Function to compute circle pack layout
	function computePackLayout(data: any, width: number, height: number, padding: number = 3): PackedNode[] {
		const root = hierarchy(data)
			.sum((d: any) => d.value || 0)
			.sort((a, b) => (b.value || 0) - (a.value || 0));

		const packLayout = pack()
			.size([width, height])
			.padding(padding);

		const packed = packLayout(root);
		
		// Extract all nodes (excluding root)
		const nodes: PackedNode[] = [];
		packed.descendants().forEach((node, i) => {
			if (i > 0) { // Skip root node
				nodes.push(node as PackedNode);
			}
		});
		
		return nodes;
	}

	// Color scale function
	function getColor(d: PackedNode, index: number): string {
		const colors = [
			'#6366F1', // indigo
			'#10B981', // emerald
			'#F59E0B', // amber
			'#EF4444', // red
			'#8B5CF6', // violet
			'#06B6D4', // cyan
			'#EC4899', // pink
			'#84CC16'  // lime
		];
		
		// Use depth for color variation
		if (d.depth === 1) return colors[0];
		if (d.depth === 2) return colors[1];
		if (d.depth === 3) return colors[2];
		return colors[index % colors.length];
	}

	let containerWidth1 = $state(0);
	let containerWidth2 = $state(0);
	let containerWidth3 = $state(0);
	let containerWidth4 = $state(0);
	let containerWidth5 = $state(0);
	let containerWidth6 = $state(0);

	let containerEl1 = $state<HTMLDivElement>();
	let containerEl2 = $state<HTMLDivElement>();
	let containerEl3 = $state<HTMLDivElement>();
	let containerEl4 = $state<HTMLDivElement>();
	let containerEl5 = $state<HTMLDivElement>();
	let containerEl6 = $state<HTMLDivElement>();

	function setupResizeObserver(el: HTMLDivElement | undefined, setWidth: (w: number) => void) {
		if (!el) return;
		
		const updateSize = () => {
			if (el) {
				const width = el.clientWidth || el.offsetWidth;
				if (width > 0) {
					setWidth(width);
				}
			}
		};
		
		// Initial measurement
		updateSize();
		
		// Use requestAnimationFrame to ensure DOM is ready
		requestAnimationFrame(() => {
			updateSize();
			const resizeObserver = new ResizeObserver(updateSize);
			resizeObserver.observe(el);
		});
	}

	$effect(() => {
		setupResizeObserver(containerEl1, (w) => containerWidth1 = w);
		setupResizeObserver(containerEl2, (w) => containerWidth2 = w);
		setupResizeObserver(containerEl3, (w) => containerWidth3 = w);
		setupResizeObserver(containerEl4, (w) => containerWidth4 = w);
		setupResizeObserver(containerEl5, (w) => containerWidth5 = w);
		setupResizeObserver(containerEl6, (w) => containerWidth6 = w);
	});

	// Computed packed nodes for each chart
	const packedNodes1 = $derived(
		containerWidth1 > 0 ? computePackLayout(basicHierarchyData, containerWidth1 - 40, 360, 3) : []
	);
	const packedNodes2 = $derived(
		containerWidth2 > 0 ? computePackLayout(multiLevelHierarchyData, containerWidth2 - 40, 360, 3) : []
	);
	const packedNodes3 = $derived(
		containerWidth3 > 0 ? computePackLayout(categoryHierarchyData, containerWidth3 - 40, 360, 3) : []
	);
	const packedNodes4 = $derived(
		containerWidth4 > 0 ? computePackLayout(deepHierarchyData, containerWidth4 - 40, 360, 3) : []
	);
	const packedNodes5 = $derived(
		containerWidth5 > 0 ? computePackLayout(flatHierarchyData, containerWidth5 - 40, 360, 3) : []
	);
	const packedNodes6 = $derived(
		containerWidth6 > 0 ? computePackLayout(complexHierarchyData, containerWidth6 - 40, 360, 3) : []
	);
</script>

<PageLayout>
	<div class="max-w-full mx-auto">
		<!-- Header -->
		<div class="flex items-center gap-4 mb-6">
			<div>
				<h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100">Circle Pack Chart</h1>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- 1. Basic Circle Pack -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">1. Basic Circle Pack</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl1}>
					{#if containerWidth1 > 0 && packedNodes1.length > 0}
						<svg width={containerWidth1} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes1 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: {node.children ? 0.3 : 0.6}; stroke: #fff; stroke-width: {node.children ? 2 : 1};"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 20 && !node.children}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 12px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Uses D3's <code class="bg-slate-100 dark:bg-slate-700 px-2 py-1 rounded">pack()</code> layout for hierarchical circle packing</p>
				</div>
			</div>

			<!-- 2. Multi-Level Hierarchy -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">2. Multi-Level Hierarchy</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl2}>
					{#if containerWidth2 > 0 && packedNodes2.length > 0}
						<svg width={containerWidth2} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes2 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: {node.children ? 0.25 : 0.6}; stroke: #fff; stroke-width: {node.children ? 2 : 1};"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 25 && !node.children}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 11px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Shows nested circles representing hierarchical relationships</p>
				</div>
			</div>

			<!-- 3. Category-Based Hierarchy -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">3. Category-Based Hierarchy</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl3}>
					{#if containerWidth3 > 0 && packedNodes3.length > 0}
						<svg width={containerWidth3} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes3 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: {node.children ? 0.3 : 0.65}; stroke: #fff; stroke-width: {node.children ? 2.5 : 1};"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 22 && !node.children}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 11px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Product categories with nested subcategories</p>
				</div>
			</div>

			<!-- 4. Deep Hierarchy -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">4. Deep Hierarchy</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl4}>
					{#if containerWidth4 > 0 && packedNodes4.length > 0}
						<svg width={containerWidth4} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes4 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: {node.children ? 0.25 : 0.6}; stroke: #fff; stroke-width: {node.children ? 2 : 1};"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 20 && !node.children}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 10px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Multiple levels of nesting showing organizational structure</p>
				</div>
			</div>

			<!-- 5. Flat Hierarchy -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">5. Flat Hierarchy</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl5}>
					{#if containerWidth5 > 0 && packedNodes5.length > 0}
						<svg width={containerWidth5} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes5 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: 0.6; stroke: #fff; stroke-width: 1;"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 18}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 10px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Simple two-level hierarchy with root and children</p>
				</div>
			</div>

			<!-- 6. Complex Mixed Hierarchy -->
			<div class="bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700 p-6 flex flex-col">
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-4">6. Complex Mixed Hierarchy</h3>
				<div class="flex-1 min-h-[400px]" bind:this={containerEl6}>
					{#if containerWidth6 > 0 && packedNodes6.length > 0}
						<svg width={containerWidth6} height={400} class="overflow-visible">
							<g transform="translate(20, 20)">
								{#each packedNodes6 as node, i}
									<circle
										cx={node.x}
										cy={node.y}
										r={node.r}
										fill={getColor(node, i)}
										style="fill-opacity: {node.children ? 0.25 : 0.6}; stroke: #fff; stroke-width: {node.children ? 2 : 1};"
										class="hover:opacity-80 transition-opacity"
									/>
									{#if node.r > 24 && !node.children}
										<text
											x={node.x}
											y={node.y}
											style="text-anchor: middle; dominant-baseline: middle; fill: #1e293b; font-size: 11px; font-weight: 500;"
											class="dark:fill-slate-100 pointer-events-none"
										>
											{node.data.name}
										</text>
									{/if}
								{/each}
							</g>
						</svg>
					{/if}
				</div>
				<div class="mt-4 text-xs text-slate-500 dark:text-slate-400">
					<p>Complex enterprise structure with varying hierarchy depths</p>
				</div>
			</div>
		</div>

		<!-- Help Section -->
		<div class="mt-8 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-6">
			<h3 class="text-sm font-semibold text-purple-900 dark:text-purple-300 mb-3">About Circle Pack Charts</h3>
			<div class="text-sm text-purple-800 dark:text-purple-200 space-y-2">
				<p>
					<strong>Circle Pack</strong> visualizations use D3's hierarchical packing algorithm to display nested data structures.
					Each circle represents a node in the hierarchy, with parent circles containing their children.
					Circle size is proportional to the node's value, and the layout automatically arranges circles to minimize wasted space.
				</p>
				<p>
					These examples use D3's <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">d3.pack()</code> layout
					from the <code class="bg-purple-100 dark:bg-purple-900/50 px-1.5 py-0.5 rounded">d3-hierarchy</code> module to compute
					circle positions, then render them using SVG in Svelte with reactive updates.
				</p>
				<p>
					📚 <a href="https://observablehq.com/@d3/pack" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">View D3 Circle Pack Example</a> | 
					<a href="https://github.com/d3/d3-hierarchy#pack" target="_blank" rel="noopener noreferrer" class="underline hover:no-underline">D3 Pack Documentation</a>
				</p>
			</div>
		</div>
	</div>
</PageLayout>
