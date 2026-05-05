<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Database, Trash2, TestTube2, Link, Unlink, Loader2, AlertCircle } from 'lucide-svelte';
	import { 
		deletePostgresConnection, 
		attachPostgresDatabase, 
		detachPostgresDatabase,
		listPostgresConnections 
	} from '$lib/db-operations';

	interface PostgresConnection {
		connection_id: string;
		connection_name: string;
		connection_type: string;
		secret_name: string;
		is_attached: boolean;
		attach_mode: string;
		created_at: string;
		last_used_at: string | null;
	}

	interface Props {
		onSelectConnection?: (connectionId: string) => void;
		onNewConnection?: () => void;
		refreshTrigger?: number;
	}

	let { onSelectConnection, onNewConnection, refreshTrigger = 0 }: Props = $props();

	let connections = $state<PostgresConnection[]>([]);
	let loading = $state(true);
	let error = $state('');
	let testingConnection = $state<string | null>(null);
	let deletingConnection = $state<string | null>(null);
	let attachingConnection = $state<string | null>(null);

	onMount(() => {
		loadConnections();
	});

	$effect(() => {
		// Reload when refreshTrigger changes
		if (refreshTrigger > 0) {
			loadConnections();
		}
	});

	async function loadConnections() {
		loading = true;
		error = '';

		try {
			const rawConnections = await listPostgresConnections();
			
			// Handle both array and JSON string responses
			let connectionsArray: any[];
			if (typeof rawConnections === 'string') {
				connectionsArray = JSON.parse(rawConnections);
			} else if (Array.isArray(rawConnections)) {
				connectionsArray = rawConnections;
			} else {
				console.error('Unexpected connections format:', rawConnections);
				connectionsArray = [];
			}
			
			// Filter out any connections with undefined or null connection_id
			connections = connectionsArray.filter(conn => conn && conn.connection_id != null);
		} catch (err) {
			error = String(err);
			connections = [];
		} finally {
			loading = false;
		}
	}

	async function testConnection(connectionId: string) {
		testingConnection = connectionId;
		error = '';

		try {
			// Tauri automatically converts camelCase to snake_case
			const result = await invoke<string>('test_postgres_connection', { connectionId });
			alert(result);
			await loadConnections();
		} catch (err) {
			error = String(err);
		} finally {
			testingConnection = null;
		}
	}

	async function deleteConnection(connectionId: string, connectionName: string) {
		if (!confirm(`Are you sure you want to delete connection "${connectionName}"?`)) {
			return;
		}

		deletingConnection = connectionId;
		error = '';

		try {
			await deletePostgresConnection(connectionId);
			await loadConnections();
		} catch (err) {
			error = String(err);
		} finally {
			deletingConnection = null;
		}
	}

	async function toggleAttachment(connection: PostgresConnection) {
		attachingConnection = connection.connection_id;
		error = '';

		try {
			if (connection.is_attached) {
				await detachPostgresDatabase(connection.connection_id);
			} else {
				await attachPostgresDatabase(connection.connection_id);
			}
			await loadConnections();
		} catch (err) {
			error = String(err);
		} finally {
			attachingConnection = null;
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		try {
			const date = new Date(dateStr);
			return date.toLocaleString();
		} catch {
			return dateStr;
		}
	}
</script>

<div class="space-y-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
			PostgreSQL Connections
		</h2>
		{#if onNewConnection}
			<button
				onclick={onNewConnection}
				class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg flex items-center gap-2"
			>
				<Database class="w-4 h-4" />
				New Connection
			</button>
		{/if}
	</div>

	<!-- Error Message -->
	{#if error}
		<div class="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
			<AlertCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
			<p class="text-sm text-red-800 dark:text-red-300">{error}</p>
		</div>
	{/if}

	<!-- Loading State -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<Loader2 class="w-8 h-8 animate-spin text-blue-600" />
		</div>
	{:else if connections.length === 0}
		<!-- Empty State -->
		<div class="text-center py-12 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg">
			<div class="p-4 bg-slate-100 dark:bg-slate-700/50 rounded-full w-16 h-16 mx-auto mb-4 flex items-center justify-center">
				<Database class="w-8 h-8 text-slate-400" />
			</div>
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				No Connections Yet
			</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-4">
				Create your first PostgreSQL connection to get started.
			</p>
			{#if onNewConnection}
				<button
					onclick={onNewConnection}
					class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg inline-flex items-center gap-2"
				>
					<Database class="w-4 h-4" />
					New Connection
				</button>
			{/if}
		</div>
	{:else}
		<!-- Connections Grid -->
		<div class="grid grid-cols-1 gap-4">
			{#each connections as connection (connection.connection_id)}
				<div class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-blue-300 dark:hover:border-blue-700 transition-colors">
					<div class="flex items-start justify-between mb-4">
						<div class="flex items-start gap-3">
							<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
								<Database class="w-5 h-5 text-blue-600 dark:text-blue-400" />
							</div>
							<div>
								<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-1">
									{connection.connection_name}
								</h3>
								<div class="flex items-center gap-2 flex-wrap">
									<span class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 rounded">
										{connection.connection_type}
									</span>
									<span class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium {connection.is_attached ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400' : 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300'} rounded">
										{#if connection.is_attached}
											<Link class="w-3 h-3" />
											Attached
										{:else}
											<Unlink class="w-3 h-3" />
											Detached
										{/if}
									</span>
									<span class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 rounded capitalize">
										{connection.attach_mode} mode
									</span>
								</div>
							</div>
						</div>
					</div>

					<!-- Connection Details -->
					<div class="text-sm text-slate-600 dark:text-slate-400 space-y-1 mb-4">
						<p><span class="font-medium">Created:</span> {formatDate(connection.created_at)}</p>
						{#if connection.last_used_at}
							<p><span class="font-medium">Last Used:</span> {formatDate(connection.last_used_at)}</p>
						{/if}
					</div>

					<!-- Actions -->
					<div class="flex gap-2 flex-wrap">
						{#if onSelectConnection}
							<button
								onclick={() => onSelectConnection?.(connection.connection_id)}
								class="px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm flex items-center gap-2"
							>
								<Database class="w-4 h-4" />
								Browse Tables
							</button>
						{/if}

						<button
							onclick={() => toggleAttachment(connection)}
							disabled={attachingConnection === connection.connection_id}
							class="px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
						>
							{#if attachingConnection === connection.connection_id}
								<Loader2 class="w-4 h-4 animate-spin" />
							{:else if connection.is_attached}
								<Unlink class="w-4 h-4" />
							{:else}
								<Link class="w-4 h-4" />
							{/if}
							{connection.is_attached ? 'Detach' : 'Attach'}
						</button>

						<button
							onclick={() => testConnection(connection.connection_id)}
							disabled={testingConnection === connection.connection_id}
							class="px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
						>
							{#if testingConnection === connection.connection_id}
								<Loader2 class="w-4 h-4 animate-spin" />
							{:else}
								<TestTube2 class="w-4 h-4" />
							{/if}
							Test
						</button>

						<button
							onclick={() => deleteConnection(connection.connection_id, connection.connection_name)}
							disabled={deletingConnection === connection.connection_id}
							class="px-3 py-2 border border-red-300 dark:border-red-800 rounded-lg text-red-700 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
						>
							{#if deletingConnection === connection.connection_id}
								<Loader2 class="w-4 h-4 animate-spin" />
							{:else}
								<Trash2 class="w-4 h-4" />
							{/if}
							Delete
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

