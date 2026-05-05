<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { 
		SvelteFlow, 
		Controls, 
		Background, 
		MiniMap,
		type Node,
		type Edge
	} from '@xyflow/svelte';
	import '@xyflow/svelte/dist/style.css';
	import TableNode from '$lib/components/TableNode.svelte';
	import TablePreviewDrawer from '$lib/components/TablePreviewDrawer.svelte';
	import DropdownMenu from '$lib/components/DropdownMenu.svelte';
	import DropdownMenuItem from '$lib/components/DropdownMenuItem.svelte';
	import DropdownMenuSeparator from '$lib/components/DropdownMenuSeparator.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Plus, Maximize2, Minimize2, RefreshCw, Layout, ChevronDown } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import Button from '$lib/components/Button.svelte';
	import { parseQueryResult } from '$lib/db-utils';

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// Types
	interface TableMetadata {
		table_name: string;
		table_type: string;
		created_at: string;
	}

	interface ColumnInfo {
		column_name: string;
		data_type: string;
		is_nullable: string;
	}

	interface TableData extends Record<string, unknown> {
		table_name: string;
		table_type: 'source' | 'model';
		columns: ColumnInfo[];
	}

	interface RelationshipInfo {
		from_table: string;
		from_column: string;
		to_table: string;
		to_column: string;
		relationship_type: string;
	}

	// State
	let isLoading = $state(true);
	let isTauriAvailable = $state(false);
	let isFullscreen = $state(false);
	
	// Table preview drawer state
	let isPreviewDrawerOpen = $state(false);
	let previewTableName = $state<string | null>(null);

	// Svelte Flow state - using $state for Svelte 5
	let flowNodes = $state<Node[]>([]);
	let flowEdges = $state<Edge[]>([]);

	// Custom node types
	const nodeTypes = {
		table: TableNode as any
	};

	// Smart layout algorithm that considers relationships
	function calculateSmartLayout(
		nodes: Array<{ id: string; data: any }>,
		relationships: RelationshipInfo[]
	): Map<string, { x: number; y: number }> {
		const positions = new Map<string, { x: number; y: number }>();
		
		if (nodes.length === 0) return positions;
		
		// Build adjacency graph (who points to whom)
		const outgoing = new Map<string, Set<string>>(); // from_table -> [to_tables]
		const incoming = new Map<string, Set<string>>(); // to_table -> [from_tables]
		
		nodes.forEach(node => {
			outgoing.set(node.id, new Set());
			incoming.set(node.id, new Set());
		});
		
		relationships.forEach(rel => {
			if (outgoing.has(rel.from_table) && incoming.has(rel.to_table)) {
				outgoing.get(rel.from_table)!.add(rel.to_table);
				incoming.get(rel.to_table)!.add(rel.from_table);
			}
		});
		
		// Calculate hierarchy levels using topological sort
		const levels = new Map<string, number>();
		const visited = new Set<string>();
		
		// Find root nodes (no incoming edges or minimal incoming)
		const roots = nodes.filter(n => incoming.get(n.id)!.size === 0);
		
		// If no clear roots, use nodes with most outgoing and least incoming
		const startNodes = roots.length > 0 ? roots : 
			[...nodes].sort((a, b) => {
				const aScore = outgoing.get(a.id)!.size - incoming.get(a.id)!.size;
				const bScore = outgoing.get(b.id)!.size - incoming.get(b.id)!.size;
				return bScore - aScore;
			}).slice(0, Math.max(1, Math.ceil(nodes.length / 4)));
		
		// BFS to assign levels
		const queue: Array<{ id: string; level: number }> = startNodes.map(n => ({ id: n.id, level: 0 }));
		
		while (queue.length > 0) {
			const { id, level } = queue.shift()!;
			
			if (visited.has(id)) {
				// If revisiting, take the minimum level (closer to root)
				levels.set(id, Math.min(levels.get(id) ?? Infinity, level));
				continue;
			}
			
			visited.add(id);
			levels.set(id, level);
			
			// Add children to queue
			const children = outgoing.get(id);
			if (children) {
				children.forEach(childId => {
					queue.push({ id: childId, level: level + 1 });
				});
			}
		}
		
		// Assign levels to any unvisited nodes (disconnected components)
		nodes.forEach(node => {
			if (!visited.has(node.id)) {
				levels.set(node.id, 0);
			}
		});
		
		// Group nodes by level
		const nodesByLevel = new Map<number, string[]>();
		levels.forEach((level, nodeId) => {
			if (!nodesByLevel.has(level)) {
				nodesByLevel.set(level, []);
			}
			nodesByLevel.get(level)!.push(nodeId);
		});
		
		// Layout configuration
		const NODE_WIDTH = 300;
		const NODE_HEIGHT = 250;
		const HORIZONTAL_SPACING = 150;
		const VERTICAL_SPACING = 200;
		const START_X = 100;
		const START_Y = 100;
		
		// Calculate positions level by level
		const levelArray = Array.from(nodesByLevel.keys()).sort((a, b) => a - b);
		
		levelArray.forEach(level => {
			const nodesInLevel = nodesByLevel.get(level)!;
			
			// Try to order nodes within a level to minimize edge crossings
			// Sort by: 1) number of connections to previous level, 2) alphabetically
			const sortedNodes = nodesInLevel.sort((a, b) => {
				// Count connections to previous level
				const aConnections = incoming.get(a)?.size ?? 0;
				const bConnections = incoming.get(b)?.size ?? 0;
				
				if (aConnections !== bConnections) {
					return bConnections - aConnections;
				}
				
				return a.localeCompare(b);
			});
			
			// Calculate x positions to center the level
			const levelWidth = sortedNodes.length * NODE_WIDTH + (sortedNodes.length - 1) * HORIZONTAL_SPACING;
			const startXForLevel = START_X;
			
			sortedNodes.forEach((nodeId, index) => {
				const x = startXForLevel + index * (NODE_WIDTH + HORIZONTAL_SPACING);
				const y = START_Y + level * (NODE_HEIGHT + VERTICAL_SPACING);
				
				positions.set(nodeId, { x, y });
			});
		});
		
		// Apply force-directed adjustments to reduce edge crossings within levels
		// This is a simplified physics simulation
		for (let iteration = 0; iteration < 5; iteration++) {
			const adjustments = new Map<string, { dx: number; dy: number }>();
			
			relationships.forEach(rel => {
				const fromPos = positions.get(rel.from_table);
				const toPos = positions.get(rel.to_table);
				
				if (!fromPos || !toPos) return;
				
				const fromLevel = levels.get(rel.from_table) ?? 0;
				const toLevel = levels.get(rel.to_table) ?? 0;
				
				// Only adjust horizontally within the same level
				if (fromLevel === toLevel) {
					const dx = toPos.x - fromPos.x;
					
					// If too close or overlapping, push apart slightly
					if (Math.abs(dx) < NODE_WIDTH + HORIZONTAL_SPACING / 2) {
						const adjustment = (NODE_WIDTH + HORIZONTAL_SPACING / 2 - Math.abs(dx)) / 2;
						const direction = dx > 0 ? 1 : -1;
						
						if (!adjustments.has(rel.from_table)) {
							adjustments.set(rel.from_table, { dx: 0, dy: 0 });
						}
						if (!adjustments.has(rel.to_table)) {
							adjustments.set(rel.to_table, { dx: 0, dy: 0 });
						}
						
						adjustments.get(rel.from_table)!.dx -= direction * adjustment * 0.1;
						adjustments.get(rel.to_table)!.dx += direction * adjustment * 0.1;
					}
				}
			});
			
			// Apply adjustments
			adjustments.forEach((adj, nodeId) => {
				const pos = positions.get(nodeId);
				if (pos) {
					pos.x += adj.dx;
				}
			});
		}
		
		return positions;
	}

	// Load tables and their columns from DuckDB
	async function loadDataModel(silent = false) {
		try {
			if (!silent) {
				isLoading = true;
			}
			
		// Get only model tables (derived tables) - source tables are excluded from data model
		const query = `SELECT table_name, table_type, created_at 
			FROM _warphead_table_metadata 
			WHERE table_type = 'model' 
			ORDER BY created_at DESC`;
			const result = await invoke<string>('execute_query', { query });
			const tablesData: TableMetadata[] = parseQueryResult(result);
			
		// Load relationships first to use in layout calculation (only between model tables)
		const relQuery = `SELECT r.from_table, r.from_column, r.to_table, r.to_column, r.relationship_type 
			FROM _warphead_relationships r
			INNER JOIN _warphead_table_metadata m1 ON r.from_table = m1.table_name
			INNER JOIN _warphead_table_metadata m2 ON r.to_table = m2.table_name
			WHERE m1.table_type = 'model' AND m2.table_type = 'model'
			ORDER BY r.created_at DESC`;
		let relationships: RelationshipInfo[] = [];
		try {
			const relResult = await invoke<string>('execute_query', { query: relQuery });
			relationships = parseQueryResult(relResult);
		} catch (error) {
			console.error('Error loading relationships for layout:', error);
		}
			
			// Get columns for each table and create nodes
			const newNodes: Node[] = [];
			const basicNodeData: Array<{ id: string; data: any }> = [];
			
			for (let i = 0; i < tablesData.length; i++) {
				const table = tablesData[i];
				try {
					const columnsQuery = `SELECT column_name, data_type, is_nullable 
						FROM information_schema.columns 
						WHERE table_name = '${table.table_name}' 
						ORDER BY ordinal_position 
						LIMIT 10`;
					const columnsResult = await invoke<string>('execute_query', { query: columnsQuery });
					const columns: ColumnInfo[] = parseQueryResult(columnsResult);
					
					basicNodeData.push({
						id: table.table_name,
						data: {
							table_name: table.table_name,
							table_type: table.table_type as 'source' | 'model',
							columns: columns
						}
					});
				} catch (error) {
					console.error(`Error loading columns for ${table.table_name}:`, error);
				}
			}
			
			// Calculate smart layout positions
			const smartPositions = calculateSmartLayout(basicNodeData, relationships);
			
			// Create nodes with smart positions
			basicNodeData.forEach(nodeData => {
				// Check if user has manually positioned this node
				const existingNode = flowNodes.find(n => n.id === nodeData.id);
				const hasUserPosition = existingNode && (
					existingNode.position.x !== Math.round(existingNode.position.x) ||
					existingNode.position.y !== Math.round(existingNode.position.y)
				);
				
				const position = hasUserPosition 
					? existingNode.position 
					: (smartPositions.get(nodeData.id) ?? { x: 100, y: 100 });
				
				newNodes.push({
					id: nodeData.id,
					type: 'table',
					position,
					data: nodeData.data
				});
			});
			
			flowNodes = newNodes;
			
			// Load relationships and create edges
			await loadRelationships();
			
			if (!silent) {
				addSystemMessage(`Loaded ${newNodes.length} table(s) and ${flowEdges.length} relationship(s) on canvas`, 'success');
			}
		} catch (error) {
			console.error('Error loading data model:', error);
			if (!silent) {
				addSystemMessage(`Error loading data model: ${error}`, 'error');
			}
		} finally {
			if (!silent) {
				isLoading = false;
			}
		}
	}

	// Load relationships from database and create edges
	async function loadRelationships() {
		try {
			// Only load relationships where both tables are model tables (not source tables)
			const relQuery = `SELECT r.from_table, r.from_column, r.to_table, r.to_column, r.relationship_type 
				FROM _warphead_relationships r
				INNER JOIN _warphead_table_metadata m1 ON r.from_table = m1.table_name
				INNER JOIN _warphead_table_metadata m2 ON r.to_table = m2.table_name
				WHERE m1.table_type = 'model' AND m2.table_type = 'model'
				ORDER BY r.created_at DESC`;
			const relResult = await invoke<string>('execute_query', { query: relQuery });
			const relationships: RelationshipInfo[] = parseQueryResult(relResult);
			
			console.log('📊 Loaded relationships from DB:', relationships);
			console.log('📊 Number of relationships:', relationships.length);
			console.log('📊 Current flowNodes:', flowNodes.map(n => n.id));
			
			// Create edges from relationships with field-specific handles
			const newEdges: Edge[] = relationships.map((rel, idx) => {
				const edge: Edge = {
					id: `edge-${idx}-${rel.from_table}-${rel.from_column}-${rel.to_table}-${rel.to_column}`,
					source: rel.from_table,
					target: rel.to_table,
					// Use field-specific handles for precise connections
					sourceHandle: `${rel.from_table}-${rel.from_column}-source`,
					targetHandle: `${rel.to_table}-${rel.to_column}-target`,
					label: rel.from_column === rel.to_column ? rel.from_column : `${rel.from_column} → ${rel.to_column}`,
					type: 'smoothstep', // smoothstep automatically routes around nodes
					animated: rel.relationship_type === 'inferred',
					style: `stroke: ${rel.relationship_type === 'inferred' ? '#94a3b8' : '#3b82f6'}; stroke-width: 2;`,
					labelStyle: 'fill: #64748b; font-size: 11px; font-weight: 500; background: white; padding: 2px 4px; border-radius: 3px;',
					// Add arrow marker to help visualize direction
					markerEnd: {
						type: 'arrowclosed' as const,
						width: 20,
						height: 20,
						color: rel.relationship_type === 'inferred' ? '#94a3b8' : '#3b82f6'
					}
				};
				console.log(`🔗 Creating edge ${idx}:`, edge);
				return edge;
			});
			
			console.log('🔗 Total edges created for Svelte Flow:', newEdges.length);
			flowEdges = newEdges;
			console.log('🔗 flowEdges after assignment:', flowEdges);
		} catch (error) {
			console.error('❌ Error loading relationships:', error);
			flowEdges = [];
		}
	}

	// Re-apply smart layout
	async function reapplySmartLayout() {
		if (flowNodes.length === 0) return;
		
		addSystemMessage('Recalculating smart layout...', 'info');
		
		// Get current relationships (only between model tables)
		const relQuery = `SELECT r.from_table, r.from_column, r.to_table, r.to_column, r.relationship_type 
			FROM _warphead_relationships r
			INNER JOIN _warphead_table_metadata m1 ON r.from_table = m1.table_name
			INNER JOIN _warphead_table_metadata m2 ON r.to_table = m2.table_name
			WHERE m1.table_type = 'model' AND m2.table_type = 'model'
			ORDER BY r.created_at DESC`;
		
		let relationships: RelationshipInfo[] = [];
		try {
			const relResult = await invoke<string>('execute_query', { query: relQuery });
			relationships = parseQueryResult(relResult);
		} catch (error) {
			console.error('Error loading relationships for layout:', error);
		}
		
		// Prepare node data
		const basicNodeData = flowNodes.map(node => ({
			id: node.id,
			data: node.data
		}));
		
		// Calculate new positions
		const smartPositions = calculateSmartLayout(basicNodeData, relationships);
		
		// Update node positions
		flowNodes = flowNodes.map(node => ({
			...node,
			position: smartPositions.get(node.id) ?? node.position
		}));
		
		addSystemMessage('Smart layout applied successfully', 'success');
	}
	
	// Fit view to show all nodes
	function handleFitView() {
		// This will be called by the Controls component
		addSystemMessage('Fitting view to show all tables', 'info');
	}
	
	// Toggle fullscreen mode
	function toggleFullscreen() {
		isFullscreen = !isFullscreen;
		addSystemMessage(isFullscreen ? 'Canvas in fullscreen mode' : 'Canvas in normal mode', 'info');
	}

	// Listen for data model updates from other pages
	function handleDataModelUpdate(event: CustomEvent) {
		console.log('🔄 Data model update event received:', event.detail);
		loadDataModel(true); // Silent reload
	}
	
	// Handle table preview open event
	function handleTablePreviewOpen(event: CustomEvent<{ tableName: string }>) {
		previewTableName = event.detail.tableName;
		isPreviewDrawerOpen = true;
	}
	
	// Close table preview drawer
	function closePreviewDrawer() {
		isPreviewDrawerOpen = false;
		previewTableName = null;
	}

	onMount(() => {
		// Initialize async
		const init = async () => {
			try {
				await invoke('initialize_duckdb');
				isTauriAvailable = true;
				
				// First, scan for relationships to ensure they're detected
				console.log('🔍 Auto-scanning for relationships on page load...');
				try {
					const relationshipsJson = await invoke<string>('scan_all_relationships');
					const relationships: RelationshipInfo[] = JSON.parse(relationshipsJson);
					console.log(`✅ Auto-scan found ${relationships.length} relationship(s)`);
				} catch (scanError) {
					console.warn('Auto-scan failed, will load existing relationships:', scanError);
				}
				
				// Then load the data model with relationships
				await loadDataModel();
			} catch (error) {
				if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
					isTauriAvailable = false;
					addSystemMessage('Tauri API not available. Please run: npm run tauri:dev', 'error');
				} else {
					addSystemMessage('Database connection issue. Some features may not work.', 'warning');
					console.error('DuckDB initialization error:', error);
				}
				isLoading = false;
			}
		};
		
		init();
		
		// Listen for data model updates
		window.addEventListener('data-model-updated', handleDataModelUpdate as EventListener);
		
		// Listen for table preview open events
		window.addEventListener('open-table-preview', handleTablePreviewOpen as EventListener);
		
		// Cleanup on unmount
		return () => {
			window.removeEventListener('data-model-updated', handleDataModelUpdate as EventListener);
			window.removeEventListener('open-table-preview', handleTablePreviewOpen as EventListener);
		};
	});
</script>

<style>
	/* Hide Svelte Flow attribution */
	:global(.svelte-flow__attribution) {
		display: none !important;
	}
</style>

{#if isFullscreen}
	<!-- Fullscreen mode: maximize below header -->
	<div class="fixed left-0 right-0 bottom-0 z-40 bg-white dark:bg-slate-900" style="top: var(--header-height, 134px);">
		{#if isLoading}
			<div class="h-full flex items-center justify-center">
				<div class="text-center">
					<svg class="animate-spin h-8 w-8 mx-auto mb-4 text-green-600 dark:text-green-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<p class="text-sm text-slate-600 dark:text-slate-400">Loading data model canvas...</p>
				</div>
			</div>
		{:else if flowNodes.length === 0}
			<div class="h-full flex items-center justify-center">
				<div class="text-center max-w-md">
					<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center">
						<svg class="w-8 h-8 text-slate-400 dark:text-slate-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"></path>
						</svg>
					</div>
					<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">No tables yet</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
						Create your first table to start visualizing your data model on the canvas
					</p>
					<Button variant="primary" onclick={() => { isFullscreen = false; goto('/models/tables'); }}>
						<Plus class="w-4 h-4 mr-2" />
						Create Table
					</Button>
				</div>
			</div>
		{:else}
			<div class="relative h-full w-full">
				<!-- Canvas Actions Menu (floats in top-left corner) - Compact version -->
				<div class="absolute top-2 left-2 z-20">
					<DropdownMenu align="left">
						{#snippet trigger()}
							<span class="flex items-center gap-1.5 text-xs">
								Canvas Actions
								<ChevronDown class="w-3 h-3" />
							</span>
						{/snippet}
						
						{#snippet children()}
							<DropdownMenuItem 
								onclick={() => loadDataModel()} 
								disabled={isLoading}
								shortcut="⌘+R"
							>
								{#snippet icon()}
									<RefreshCw class="w-3.5 h-3.5" />
								{/snippet}
								<span class="text-xs">{isLoading ? 'Loading...' : 'Refresh Canvas'}</span>
							</DropdownMenuItem>
							
							<DropdownMenuItem 
								onclick={() => reapplySmartLayout()} 
								disabled={isLoading || flowNodes.length === 0}
								shortcut="⌘+L"
							>
								{#snippet icon()}
									<Layout class="w-3.5 h-3.5" />
								{/snippet}
								<span class="text-xs">Smart Layout</span>
							</DropdownMenuItem>
							
							<DropdownMenuSeparator />
							
							<DropdownMenuItem 
								onclick={() => goto('/models/tables')}
							>
								{#snippet icon()}
									<Plus class="w-3.5 h-3.5" />
								{/snippet}
								<span class="text-xs">Create Table</span>
							</DropdownMenuItem>
						{/snippet}
					</DropdownMenu>
				</div>
				
				<!-- Maximize/Minimize Button (floats in top-right corner) -->
				<button
					onclick={toggleFullscreen}
					class="absolute top-2 right-2 z-20 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-700 rounded p-1.5 shadow-md hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors"
					title="Exit fullscreen"
				>
					<Minimize2 class="w-4 h-4 text-slate-700 dark:text-slate-300" />
				</button>
				
				<!-- Svelte Flow Canvas -->
				<div class="h-full w-full">
					<SvelteFlow
						nodes={flowNodes}
						edges={flowEdges}
						{nodeTypes}
						fitView
						fitViewOptions={{ padding: 0.3, minZoom: 0.1, maxZoom: 0.75 }}
						minZoom={0.3}
						maxZoom={2}
						class="bg-slate-50 dark:bg-slate-950"
					>
						<Controls />
						<Background />
						<MiniMap 
							nodeColor={(node) => {
								const nodeData = node.data as TableData;
								return nodeData.table_type === 'source' ? '#22c55e' : '#3b82f6';
							}}
						/>
					</SvelteFlow>
				</div>
			</div>
		{/if}
	</div>
{:else}
	<!-- Normal mode: use PageLayout -->
	<PageLayout>
	<div class="pt-2 -ml-[calc((96vw-100%)/2)] w-[96vw]">
	{#if isLoading}
		<div class="flex items-center justify-center h-[600px]">
			<div class="text-center">
				<svg class="animate-spin h-8 w-8 mx-auto mb-4 text-green-600 dark:text-green-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
					<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
				</svg>
				<p class="text-sm text-slate-600 dark:text-slate-400">Loading data model canvas...</p>
			</div>
		</div>
	{:else if flowNodes.length === 0}
		<div class="flex items-center justify-center h-[600px] border-2 border-dashed border-slate-200 dark:border-slate-800 rounded-lg">
			<div class="text-center max-w-md">
				<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center">
					<svg class="w-8 h-8 text-slate-400 dark:text-slate-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"></path>
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">No tables yet</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
					Create your first table to start visualizing your data model on the canvas
				</p>
				<Button variant="primary" onclick={() => goto('/models/tables')}>
					<Plus class="w-4 h-4 mr-2" />
					Create Table
				</Button>
			</div>
		</div>
	{:else}
		<!-- Svelte Flow Canvas Wrapper -->
		<div class="relative">
			<!-- Canvas Actions Menu (floats in top-left corner) - Compact version -->
			<div class="absolute top-2 left-2 z-20">
				<DropdownMenu align="left">
					{#snippet trigger()}
						<span class="flex items-center gap-1.5 text-xs">
							Canvas Actions
							<ChevronDown class="w-3 h-3" />
						</span>
					{/snippet}
					
					{#snippet children()}
						<DropdownMenuItem 
							onclick={() => loadDataModel()} 
							disabled={isLoading}
							shortcut="⌘+R"
						>
							{#snippet icon()}
								<RefreshCw class="w-3.5 h-3.5" />
							{/snippet}
							<span class="text-xs">{isLoading ? 'Loading...' : 'Refresh Canvas'}</span>
						</DropdownMenuItem>
						
						<DropdownMenuItem 
							onclick={() => reapplySmartLayout()} 
							disabled={isLoading || flowNodes.length === 0}
							shortcut="⌘+L"
						>
							{#snippet icon()}
								<Layout class="w-3.5 h-3.5" />
							{/snippet}
							<span class="text-xs">Smart Layout</span>
						</DropdownMenuItem>
						
						<DropdownMenuSeparator />
						
						<DropdownMenuItem 
							onclick={() => goto('/models/tables')}
						>
							{#snippet icon()}
								<Plus class="w-3.5 h-3.5" />
							{/snippet}
							<span class="text-xs">Create Table</span>
						</DropdownMenuItem>
					{/snippet}
				</DropdownMenu>
			</div>
			
			<!-- Maximize/Minimize Button (floats in top-right corner) -->
			<button
				onclick={toggleFullscreen}
				class="absolute top-2 right-2 z-20 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-700 rounded p-1.5 shadow-md hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors"
				title="Enter fullscreen"
			>
				<Maximize2 class="w-4 h-4 text-slate-700 dark:text-slate-300" />
			</button>
			
			<!-- Svelte Flow Canvas -->
			<div 
				class="border-2 border-slate-200 dark:border-slate-800 rounded-lg overflow-hidden bg-white dark:bg-slate-900 min-h-[500px]"
				style="height: calc(100vh - var(--header-height, 134px) - var(--footer-height, 36px) - 240px);"
			>
				<SvelteFlow
					nodes={flowNodes}
					edges={flowEdges}
					{nodeTypes}
					fitView
					fitViewOptions={{ padding: 0.3, minZoom: 0.1, maxZoom: 0.75 }}
					minZoom={0.3}
					maxZoom={2}
					class="bg-slate-50 dark:bg-slate-950"
				>
					<Controls />
					<Background />
					<MiniMap 
						nodeColor={(node) => {
							const nodeData = node.data as TableData;
							return nodeData.table_type === 'source' ? '#22c55e' : '#3b82f6';
						}}
					/>
				</SvelteFlow>
			</div>
		</div>
		
		<!-- Legend -->
		<div class="mt-6 p-4 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg">
			<div class="flex items-center gap-6 text-xs flex-wrap">
				<div class="flex items-center gap-2">
					<div class="w-3 h-3 bg-green-500 dark:bg-green-400 rounded"></div>
					<span class="text-slate-600 dark:text-slate-400">Source Table</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="w-3 h-3 bg-blue-500 dark:bg-blue-400 rounded"></div>
					<span class="text-slate-600 dark:text-slate-400">Model Table</span>
				</div>
				<div class="flex items-center gap-2">
					<svg class="w-3 h-3 text-amber-500 dark:text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path>
					</svg>
					<span class="text-slate-600 dark:text-slate-400">ID Column</span>
				</div>
				<div class="flex items-center gap-2">
					<svg class="w-6 h-3" viewBox="0 0 24 12">
						<path d="M 2 6 L 22 6" stroke="#94a3b8" stroke-width="2" fill="none" stroke-dasharray="4 2" />
					</svg>
					<span class="text-slate-600 dark:text-slate-400">Inferred Relationship</span>
				</div>
				<div class="ml-auto text-slate-500 dark:text-slate-500 flex items-center gap-4">
					<span>🖱️ Drag tables to organize</span>
					<span>🔍 Scroll to zoom</span>
					<span>📍 Click to select</span>
				</div>
			</div>
		</div>
	{/if}
	</div>
	</PageLayout>
{/if}

<!-- Table Preview Drawer (rendered outside main layout) -->
<TablePreviewDrawer 
	bind:isOpen={isPreviewDrawerOpen} 
	bind:tableName={previewTableName}
	onClose={closePreviewDrawer}
/>
