<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { getMonitorScreenshot, getScreenshotableMonitors } from 'tauri-plugin-screenshots-api';
	import { onMount } from 'svelte';

	interface PageInfo {
		name: string;
		path: string;
		title: string;
	}

	let allRoutes = $state<string[]>([]);
	let pages = $state<PageInfo[]>([]);
	let isLoadingRoutes = $state(true);
	let routeLoadError = $state<string | null>(null);

	let isCapturing = $state(false);
	let currentIndex = $state(0);
	let currentPage = $state('');
	let log = $state<string[]>([]);
	let outputFolder = $state('');
	let isComplete = $state(false);
	let monitorId = $state<number | null>(null);

	// Function to convert route path to readable title
	function routeToTitle(route: string): string {
		if (route === '/') return 'Home';
		
		// Remove leading slash and split by /
		const parts = route.substring(1).split('/');
		
		// Capitalize each part and join with space
		return parts
			.map(part => part.charAt(0).toUpperCase() + part.slice(1))
			.join(' - ');
	}

	// Function to convert route path to filename
	function routeToFilename(route: string, index: number): string {
		if (route === '/') return `${String(index).padStart(2, '0')}-home`;
		
		// Remove leading slash and replace / with -
		const filename = route.substring(1).replace(/\//g, '-');
		return `${String(index).padStart(2, '0')}-${filename}`;
	}

	// Load all routes from the file system
	async function loadRoutes() {
		try {
			isLoadingRoutes = true;
			routeLoadError = null;
			
			addLog('Scanning for routes...');
			
			// Get all routes from Tauri command
			const routes = await invoke<string[]>('get_all_routes');
			allRoutes = routes;
			
			// Filter out the screenshot-tool route itself
			const filteredRoutes = routes.filter(r => r !== '/screenshot-tool');
			
			// Convert routes to page info objects
			pages = filteredRoutes.map((route, index) => ({
				name: routeToFilename(route, index + 1),
				path: route,
				title: routeToTitle(route)
			}));
			
			addLog(`Found ${pages.length} routes to capture`);
			
		} catch (error) {
			console.error('Error loading routes:', error);
			routeLoadError = `Failed to load routes: ${error}`;
			addLog(`Error: ${error}`);
		} finally {
			isLoadingRoutes = false;
		}
	}

	// Check if we just completed a capture (from sessionStorage)
	if (typeof window !== 'undefined') {
		const completedData = sessionStorage.getItem('screenshot-completed');
		if (completedData) {
			const data = JSON.parse(completedData);
			isComplete = true;
			outputFolder = data.outputFolder;
			log = data.log || [];
			sessionStorage.removeItem('screenshot-completed');
		}
	}

	// Function to add log entry
	function addLog(message: string) {
		log = [...log, `[${new Date().toLocaleTimeString()}] ${message}`];
	}

	// Function to capture screenshot and save to screenshots folder
	async function captureScreenshot(filename: string, destFolder: string): Promise<string> {
		try {
			if (monitorId === null) {
				throw new Error('Monitor ID not set');
			}

			// Capture the monitor screenshot
			addLog(`📸 Capturing screenshot...`);
			const tempScreenshotPath = await getMonitorScreenshot(monitorId);
			addLog(`✓ Captured: ${filename}`);

			// Copy to our screenshots folder
			const finalPath = await invoke<string>('save_screenshot_file', {
				sourcePath: tempScreenshotPath,
				destFolder: destFolder,
				filename: `${filename}.png`
			});
			
			addLog(`✓ Saved to screenshots folder`);
			return finalPath;
		} catch (error) {
			addLog(`✗ Failed to capture ${filename}: ${error}`);
			console.error('Capture error:', error);
			throw error;
		}
	}

	// Function to wait
	function wait(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	// Start the screenshot capture process
	async function startCapture() {
		isCapturing = true;
		isComplete = false;
		currentIndex = 0;
		log = [];

		addLog('Starting screenshot capture...');

		try {
			// Create timestamped screenshots folder
			const screenshotsFolder = await invoke<string>('create_screenshots_folder');
			outputFolder = screenshotsFolder;
			addLog(`Created folder: ${screenshotsFolder}`);

			// Get available monitors
			const monitors = await getScreenshotableMonitors();
			addLog(`Found ${monitors.length} monitor(s)`);
			console.log('Available monitors:', monitors);

			if (monitors.length === 0) {
				throw new Error('No monitors found for screenshots');
			}

			// Use the first monitor (primary monitor)
			monitorId = monitors[0].id;
			addLog(`Using monitor: ${monitors[0].name} (ID: ${monitorId})`);

			// Capture screenshots for all pages
			addLog(`Starting to capture ${pages.length} pages...`);
			console.log('About to start page loop');
			
			for (let i = 0; i < pages.length; i++) {
				const page = pages[i];
				currentIndex = i + 1;
				currentPage = page.title;

				addLog(`[${currentIndex}/${pages.length}] Navigating to ${page.title}...`);
				console.log(`Processing page ${i + 1}/${pages.length}:`, page);

				// Navigate to the page
				await goto(page.path);
				addLog(`✓ Navigated to ${page.path}`);

				// Wait for page to load and render
				addLog(`⏳ Waiting for page to render...`);
				await wait(2500);

				// Take screenshot and save to folder
				addLog(`📸 Taking screenshot of ${page.name}...`);
				await captureScreenshot(page.name, screenshotsFolder);

				// Small delay between screenshots
				await wait(500);
			}
			
			console.log('Finished page loop');

			addLog('Screenshot capture complete!');
			addLog(`All ${pages.length} screenshots saved to: ${outputFolder}`);
			console.log('Setting isComplete to true');
			
			// Save completion state to sessionStorage before navigating
			sessionStorage.setItem('screenshot-completed', JSON.stringify({
				outputFolder,
				log
			}));
			
			// Navigate back to screenshot tool page
			await goto('/screenshot-tool');
			
		} catch (error) {
			addLog(`Error during capture: ${error}`);
			console.error('Screenshot capture error:', error);
		} finally {
			console.log('Finally block - setting isCapturing to false');
			// First set capturing to false
			isCapturing = false;
			// Wait a moment for UI to update
			await wait(200);
			// Then set complete to true
			console.log('Setting isComplete to true in finally');
			isComplete = true;
			console.log('States after finally: isCapturing=', isCapturing, 'isComplete=', isComplete);
		}
	}

	// Function to cancel capture
	function cancelCapture() {
		isCapturing = false;
		addLog('Screenshot capture cancelled by user');
	}

	// Function to open folder in file explorer
	async function openFolder() {
		try {
			await invoke('open_folder_in_explorer', { folderPath: outputFolder });
		} catch (error) {
			addLog(`Error opening folder: ${error}`);
		}
	}

	// Load routes on mount
	onMount(() => {
		loadRoutes();
	});
</script>

<div class="min-h-screen bg-gray-950 text-gray-100 p-8">
	<div class="max-w-4xl mx-auto">
		<div class="mb-8">
			<h1 class="text-3xl font-bold mb-2">📸 Tauri App Screenshot Tool</h1>
			<p class="text-gray-400">
				Automatically capture screenshots of all pages in the Tauri desktop app
			</p>
			<!-- Debug info -->
			<p class="text-xs text-gray-600 mt-2">
				Debug: isCapturing={isCapturing}, isComplete={isComplete}, outputFolder={outputFolder ? 'set' : 'empty'}, routes={pages.length}
			</p>
		</div>

		{#if isLoadingRoutes}
			<div class="bg-gray-900 border border-gray-800 rounded-lg p-6 mb-6">
				<div class="flex items-center gap-3">
					<svg class="animate-spin h-5 w-5 text-blue-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<span class="text-gray-300">Loading routes from file system...</span>
				</div>
			</div>
		{:else if routeLoadError}
			<div class="bg-red-900/20 border border-red-600 rounded-lg p-6 mb-6">
				<h2 class="text-xl font-semibold mb-2 text-red-400">❌ Error Loading Routes</h2>
				<p class="text-gray-300 mb-4">{routeLoadError}</p>
				<button
					onclick={loadRoutes}
					class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors"
				>
					Retry
				</button>
			</div>
		{:else if pages.length === 0}
			<div class="bg-yellow-900/20 border border-yellow-600 rounded-lg p-6 mb-6">
				<h2 class="text-xl font-semibold mb-2 text-yellow-400">⚠️ No Routes Found</h2>
				<p class="text-gray-300 mb-4">No routes were detected in the application.</p>
			</div>
		{:else if !isCapturing && !isComplete}
			<div class="bg-gray-900 border border-gray-800 rounded-lg p-6 mb-6">
				<h2 class="text-xl font-semibold mb-4">Ready to Capture</h2>
				<p class="text-gray-400 mb-4">
					This tool will navigate through all {pages.length} pages and capture screenshots automatically.
					Screenshots will be saved to a timestamped folder in the screenshots directory.
				</p>
				
				<!-- Show detected routes -->
				<details class="mb-4">
					<summary class="cursor-pointer text-blue-400 hover:text-blue-300 text-sm mb-2">
						View all {pages.length} detected routes
					</summary>
					<div class="mt-2 max-h-60 overflow-y-auto bg-gray-950 rounded p-3 text-xs">
						<ul class="space-y-1">
							{#each pages as page}
								<li class="text-gray-400">
									<span class="text-gray-600">{page.name}</span>
									<span class="text-gray-500 mx-2">→</span>
									<span class="text-blue-400">{page.path}</span>
									<span class="text-gray-500 mx-2">—</span>
									<span class="text-gray-300">{page.title}</span>
								</li>
							{/each}
						</ul>
					</div>
				</details>
				
				<button
					onclick={startCapture}
					class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors"
				>
					Start Screenshot Capture
				</button>
			</div>
		{/if}

		{#if isCapturing}
			<div class="bg-gray-900 border border-blue-600 rounded-lg p-6 mb-6">
				<h2 class="text-xl font-semibold mb-4">Capturing Screenshots...</h2>
				<div class="mb-4">
					<div class="flex justify-between text-sm text-gray-400 mb-2">
						<span>Progress: {currentIndex}/{pages.length}</span>
						<span>{Math.round((currentIndex / pages.length) * 100)}%</span>
					</div>
					<div class="w-full bg-gray-800 rounded-full h-2 overflow-hidden">
						<div
							class="bg-blue-600 h-full transition-all duration-300"
							style="width: {(currentIndex / pages.length) * 100}%"
						></div>
					</div>
				</div>
				<p class="text-gray-300 mb-4">
					Current: <span class="font-semibold">{currentPage}</span>
				</p>
				<button
					onclick={cancelCapture}
					class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white font-semibold rounded-lg transition-colors"
				>
					Cancel
				</button>
			</div>
		{/if}

		{#if isComplete}
			<div class="bg-green-900/20 border border-green-600 rounded-lg p-6 mb-6">
				<h2 class="text-xl font-semibold mb-2">✅ Screenshot Capture Complete!</h2>
				<p class="text-gray-300 mb-4">
					All {pages.length} screenshots have been captured and saved to:
				</p>
				<code class="block bg-gray-900 px-4 py-2 rounded text-sm text-green-400 mb-4 break-all">
					{outputFolder}
				</code>
				<div class="flex gap-3">
					<button
						onclick={openFolder}
						class="px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg transition-colors flex items-center gap-2"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
						</svg>
						Open Folder
					</button>
					<button
						onclick={() => {
							isComplete = false;
							log = [];
						}}
						class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors"
					>
						Capture Again
					</button>
				</div>
			</div>
		{/if}

		<div class="bg-gray-900 border border-gray-800 rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-4">Log</h2>
			<div class="bg-black rounded p-4 max-h-96 overflow-y-auto font-mono text-sm">
				{#if log.length === 0}
					<p class="text-gray-500">No log entries yet...</p>
				{:else}
					{#each log as entry}
						<div class="text-gray-300 mb-1">{entry}</div>
					{/each}
				{/if}
			</div>
		</div>

		<div class="mt-6 text-center">
			<a href="/" class="text-blue-400 hover:text-blue-300 underline">← Back to Home</a>
		</div>
	</div>
</div>
