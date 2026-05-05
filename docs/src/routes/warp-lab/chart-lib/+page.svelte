<script lang="ts">
	import ChartBuilder from '$lib/viz/core/ChartBuilder.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Database, Calculator, Tag as TagIcon } from 'lucide-svelte';
	import { parseQueryResult } from '$lib/db-utils';

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	interface Metric {
		slug: string;
		metric_name: string;
		formula: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	interface Dimension {
		slug: string;
		dimension_name: string;
		field_name: string;
		source_table: string;
		description: string | null;
		tags: string | null;
	}

	let sampleData = $state<any[]>([]);
	let isLoading = $state(true);
	let metrics = $state<Metric[]>([]);
	let dimensions = $state<Dimension[]>([]);
	let availableMetrics = $state<Metric[]>([]);
	let availableDimensions = $state<Dimension[]>([]);
	let isLoadingMetadata = $state(false);

	// Load metrics and dimensions
	async function loadMetricsAndDimensions() {
		try {
			isLoadingMetadata = true;
			
			// First, scan for relationships to ensure JOINs will work
			try {
				console.log('🔍 Scanning for relationships...');
				const relationshipsResult = await invoke<string>('scan_all_relationships');
				const relationships = JSON.parse(relationshipsResult);
				console.log('✅ Relationships scanned successfully. Found:', relationships.length);
				console.log('📋 Relationships:', relationships);
			} catch (error) {
				console.warn('⚠️ Could not scan relationships:', error);
				// Continue anyway - relationships might already exist
			}
			
			// Load metrics
			try {
				const metricsResult = await invoke<string>('list_metrics');
				metrics = parseQueryResult(metricsResult);
			} catch (error) {
				console.error('Error loading metrics:', error);
				metrics = [];
			}

			// Load dimensions
			try {
				const dimensionsResult = await invoke<string>('list_dimensions');
				dimensions = parseQueryResult(dimensionsResult);
			} catch (error) {
				console.error('Error loading dimensions:', error);
				dimensions = [];
			}

			availableMetrics = metrics;
			availableDimensions = dimensions;
			
			if (metrics.length > 0 || dimensions.length > 0) {
				addSystemMessage(`Loaded ${metrics.length} metrics and ${dimensions.length} dimensions`, 'success');
			}
		} catch (error) {
			console.error('Error loading metadata:', error);
			addSystemMessage(`Error loading metrics/dimensions: ${error}`, 'error');
		} finally {
			isLoadingMetadata = false;
		}
	}

	// Load sample data from database
	onMount(async () => {
		try {
			await invoke('initialize_duckdb');
			
			// Load metrics and dimensions first
			await loadMetricsAndDimensions();
			
			// Try to load sample data
			try {
				const result = await invoke<string>('execute_query', {
					query: 'SELECT * FROM barchart LIMIT 100'
				});
				const parsed = JSON.parse(result);

				if (parsed.columns && parsed.data) {
					if (parsed.data.length > 0) {
						sampleData = parsed.data;
					}
				} else if (parsed.length > 0) {
					sampleData = parsed;
				}
			} catch (err) {
				console.error('Error loading sample data:', err);
				sampleData = [];
			}
		} catch (err) {
			console.error('Error initializing:', err);
			addSystemMessage('Error initializing database', 'error');
		} finally {
			isLoading = false;
		}
	});

	function handleChartCreate(config: any) {
		console.log('Chart created:', config);
	}
</script>

<div class="page-container">
	<div class="builder-wrapper">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading chart library...</p>
			</div>
		{:else}
			<ChartBuilder 
				initialData={sampleData} 
				metrics={availableMetrics}
				dimensions={availableDimensions}
				onChartCreate={handleChartCreate} 
			/>
		{/if}
	</div>
</div>

<style>
	.page-container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: rgb(249, 250, 251);
	}

	:global(.dark) .page-container {
		background: rgb(17, 24, 39);
	}

	.builder-wrapper {
		flex: 1;
		overflow: hidden;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 1rem;
		color: rgb(107, 114, 128);
	}

	:global(.dark) .loading-state {
		color: rgb(156, 163, 175);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgb(229, 231, 235);
		border-top-color: rgb(99, 102, 241);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	:global(.dark) .spinner {
		border-color: rgb(55, 65, 81);
		border-top-color: rgb(99, 102, 241);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

