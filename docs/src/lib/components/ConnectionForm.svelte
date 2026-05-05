<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Database, Loader2, CheckCircle2, XCircle } from 'lucide-svelte';
	import { createPostgresConnection, deletePostgresConnection } from '$lib/db-operations';

	interface Props {
		onSuccess?: () => void;
		onCancel?: () => void;
	}

	let { onSuccess, onCancel }: Props = $props();

	// Form state
	let connectionName = $state('');
	let host = $state('localhost');
	let port = $state(5432);
	let database = $state('');
	let username = $state('postgres');
	let password = $state('');
	let sslMode = $state('prefer');

	// UI state
	let testing = $state(false);
	let saving = $state(false);
	let testResult = $state<{ success: boolean; message: string } | null>(null);
	let error = $state('');

	async function testConnection() {
		if (!validateForm()) return;

		testing = true;
		testResult = null;
		error = '';

		try {
			// Create a temporary connection to test
			const connectionId = await createPostgresConnection({
				name: `test_${Date.now()}`,
				host,
				port,
				database,
				username,
				password,
				sslMode
			});

			// Delete the test connection
			await deletePostgresConnection(connectionId);

			testResult = { success: true, message: 'Connection successful!' };
		} catch (err) {
			testResult = { success: false, message: String(err) };
		} finally {
			testing = false;
		}
	}

	async function saveConnection() {
		if (!validateForm()) return;

		saving = true;
		error = '';

		try {
			await createPostgresConnection({
				name: connectionName,
				host,
				port,
				database,
				username,
				password,
				sslMode
			});

			if (onSuccess) onSuccess();
		} catch (err) {
			error = String(err);
		} finally {
			saving = false;
		}
	}

	function validateForm(): boolean {
		if (!connectionName.trim()) {
			error = 'Connection name is required';
			return false;
		}
		if (!host.trim()) {
			error = 'Host is required';
			return false;
		}
		if (!database.trim()) {
			error = 'Database name is required';
			return false;
		}
		if (!username.trim()) {
			error = 'Username is required';
			return false;
		}
		return true;
	}
</script>

<div class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-6">
	<div class="flex items-center gap-3 mb-6">
		<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
			<Database class="w-6 h-6 text-blue-600 dark:text-blue-400" />
		</div>
		<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
			New PostgreSQL Connection
		</h2>
	</div>

	<form onsubmit={(e) => { e.preventDefault(); saveConnection(); }} class="space-y-4">
		<!-- Connection Name -->
		<div>
			<label for="connection-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
				Connection Name
			</label>
			<input
				id="connection-name"
				type="text"
				bind:value={connectionName}
				placeholder="My PostgreSQL Database"
				class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				required
			/>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<!-- Host -->
			<div>
				<label for="host" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Host
				</label>
				<input
					id="host"
					type="text"
					bind:value={host}
					placeholder="localhost"
					class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					required
				/>
			</div>

			<!-- Port -->
			<div>
				<label for="port" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Port
				</label>
				<input
					id="port"
					type="number"
					bind:value={port}
					placeholder="5432"
					class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					required
				/>
			</div>
		</div>

		<!-- Database -->
		<div>
			<label for="database" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
				Database Name
			</label>
			<input
				id="database"
				type="text"
				bind:value={database}
				placeholder="mydb"
				class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				required
			/>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<!-- Username -->
			<div>
				<label for="username" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Username
				</label>
				<input
					id="username"
					type="text"
					bind:value={username}
					placeholder="postgres"
					class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					required
				/>
			</div>

			<!-- Password -->
			<div>
				<label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Password
				</label>
				<input
					id="password"
					type="password"
					bind:value={password}
					placeholder="••••••••"
					class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				/>
			</div>
		</div>

		<!-- SSL Mode -->
		<div>
			<label for="ssl-mode" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
				SSL Mode
			</label>
			<select
				id="ssl-mode"
				bind:value={sslMode}
				class="form-select w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
			>
				<option value="disable">Disable</option>
				<option value="allow">Allow</option>
				<option value="prefer">Prefer (Default)</option>
				<option value="require">Require</option>
			</select>
			<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
				'Prefer' will use SSL if available, 'Require' will fail if SSL is not available.
			</p>
		</div>

		<!-- Test Result -->
		{#if testResult}
			<div class="flex items-start gap-3 p-4 rounded-lg {testResult.success ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800' : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'}">
				{#if testResult.success}
					<CheckCircle2 class="w-5 h-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
					<p class="text-sm text-green-800 dark:text-green-300">{testResult.message}</p>
				{:else}
					<XCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
					<p class="text-sm text-red-800 dark:text-red-300">{testResult.message}</p>
				{/if}
			</div>
		{/if}

		<!-- Error Message -->
		{#if error}
			<div class="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
				<XCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
				<p class="text-sm text-red-800 dark:text-red-300">{error}</p>
			</div>
		{/if}

		<!-- Action Buttons -->
		<div class="flex gap-3 pt-4">
			<button
				type="button"
				onclick={testConnection}
				disabled={testing || saving}
				class="px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
			>
				{#if testing}
					<Loader2 class="w-4 h-4 animate-spin" />
					Testing...
				{:else}
					Test Connection
				{/if}
			</button>

			<button
				type="submit"
				disabled={testing || saving}
				class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
			>
				{#if saving}
					<Loader2 class="w-4 h-4 animate-spin" />
					Saving...
				{:else}
					Save Connection
				{/if}
			</button>

			{#if onCancel}
				<button
					type="button"
					onclick={onCancel}
					disabled={testing || saving}
					class="px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					Cancel
				</button>
			{/if}
		</div>
	</form>
</div>

