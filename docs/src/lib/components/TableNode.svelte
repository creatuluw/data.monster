<script lang="ts">
	import { Table as TableIcon, Key, SquarePen, Eye } from 'lucide-svelte';
	import { Handle, Position, type NodeProps, useNodeConnections } from '@xyflow/svelte';
	import { goto } from '$app/navigation';
	import Tooltip from './Tooltip.svelte';

	// Define the data structure for table nodes
	interface TableData extends Record<string, unknown> {
		table_name: string;
		table_type: 'source' | 'model';
		columns: Array<{
			column_name: string;
			data_type: string;
			is_nullable: string;
		}>;
	}

	// Get props from Svelte Flow
	type Props = NodeProps & { data: TableData };
	let { data, selected = $bindable(false), id }: Props = $props();
	
	// Get all connections for this node to determine which handles have connections
	const connections = useNodeConnections();
	
	// Helper function to check if a specific handle has connections
	function hasConnection(columnName: string, type: 'source' | 'target'): boolean {
		const handleId = `${data.table_name}-${columnName}-${type}`;
		return connections.current.some((conn: any) => 
			(type === 'source' && conn.sourceHandle === handleId) ||
			(type === 'target' && conn.targetHandle === handleId)
		);
	}
	
	// Handle wheel event to prevent canvas zoom when scrolling inside table node
	function handleWheel(event: WheelEvent) {
		// Stop the event from propagating to the canvas (which would trigger zoom)
		event.stopPropagation();
	}
	
	// Navigate to tables page to edit this table
	function handleEdit(event: MouseEvent) {
		event.stopPropagation(); // Prevent node selection when clicking edit icon
		goto(`/models/tables?edit=${encodeURIComponent(data.table_name)}`);
	}
	
	// Dispatch event to open table preview drawer
	function handlePreview(event: MouseEvent) {
		event.stopPropagation(); // Prevent node selection when clicking preview icon
		window.dispatchEvent(new CustomEvent('open-table-preview', {
			detail: { tableName: data.table_name }
		}));
	}
</script>

<div
	class="table-node bg-white dark:bg-slate-900 border-2 rounded-lg shadow-lg transition-all duration-200"
	class:border-green-500={data.table_type === 'source'}
	class:dark:border-green-400={data.table_type === 'source'}
	class:border-blue-500={data.table_type === 'model'}
	class:dark:border-blue-400={data.table_type === 'model'}
	class:shadow-2xl={selected}
	class:ring-4={selected}
	class:ring-green-200={selected}
	class:dark:ring-green-800={selected}
	style="width: 220px;"
>
	<!-- Table Header -->
	<div 
		role="group"
		class="px-4 py-3 border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 rounded-t-lg"
	>
		<div class="flex items-center gap-2">
			<TableIcon 
				class="w-5 h-5 flex-shrink-0 {data.table_type === 'source' ? 'text-green-500 dark:text-green-400' : 'text-blue-500 dark:text-blue-400'}"
			/>
			<div class="flex-1 min-w-0">
				<h3 class="font-semibold text-sm text-slate-900 dark:text-slate-100 truncate">
					{data.table_name}
				</h3>
			</div>
			<Tooltip text="Preview table data" position="bottom" size="sm">
				<button
					onclick={handlePreview}
					class="flex-shrink-0 p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded transition-colors"
				>
					<Eye class="w-4 h-4 text-slate-600 dark:text-slate-400" />
				</button>
			</Tooltip>
			<Tooltip text="Edit table query" position="bottom" size="sm">
				<button
					onclick={handleEdit}
					class="flex-shrink-0 p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded transition-colors"
				>
					<SquarePen class="w-4 h-4 text-slate-600 dark:text-slate-400" />
				</button>
			</Tooltip>
		</div>
	</div>
	
	<!-- Columns List -->
	<div class="p-2 max-h-64 overflow-y-auto overflow-x-hidden" onwheel={handleWheel}>
		{#if data.columns.length === 0}
			<div class="px-2 py-4 text-center text-xs text-slate-500 dark:text-slate-400">
				No columns
			</div>
		{:else}
			<div class="space-y-0.5">
				{#each data.columns as column}
					{@const hasTargetConn = hasConnection(column.column_name, 'target')}
					{@const hasSourceConn = hasConnection(column.column_name, 'source')}
					<div class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors relative">
						<!-- Connection handle on the left (as target) -->
						<Handle 
							type="target" 
							position={Position.Left} 
							id={`${data.table_name}-${column.column_name}-target`}
							class={`!w-2 !h-2 !bg-blue-500 !border-white !border-2 transition-opacity !-left-3 ${hasTargetConn ? 'opacity-100' : 'opacity-0 hover:opacity-100'}`}
						/>
						
						{#if column.column_name.toLowerCase().includes('id')}
							<Key class="w-3 h-3 text-amber-500 dark:text-amber-400 flex-shrink-0" />
						{:else}
							<div class="w-3 h-3 flex-shrink-0"></div>
						{/if}
						<span class="flex-1 font-mono text-slate-700 dark:text-slate-300 truncate">
							{column.column_name}
						</span>
						<span class="text-slate-500 dark:text-slate-500 text-[10px]">
							{column.data_type}
						</span>
						
						<!-- Connection handle on the right (as source) -->
						<Handle 
							type="source" 
							position={Position.Right} 
							id={`${data.table_name}-${column.column_name}-source`}
							class={`!w-2 !h-2 !bg-blue-500 !border-white !border-2 transition-opacity !-right-3 ${hasSourceConn ? 'opacity-100' : 'opacity-0 hover:opacity-100'}`}
						/>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.table-node {
		cursor: grab;
	}
	
	.table-node:active {
		cursor: grabbing;
	}
</style>

