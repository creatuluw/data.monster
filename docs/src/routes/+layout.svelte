<script lang="ts">
	import '../app.css';
	import Header from '$lib/components/Header.svelte';
	import Breadcrumb from '$lib/components/Breadcrumb.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import SystemMessagesDrawer from '$lib/components/SystemMessagesDrawer.svelte';
	import UploadModal from '$lib/components/UploadModal.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import TabBar from '$lib/components/TabBar.svelte';
	import CriticalOperationOverlay from '$lib/components/CriticalOperationOverlay.svelte';
	import ToastContainer from '$lib/components/ToastContainer.svelte';
	import type { Snippet } from 'svelte';
	import { onMount, setContext } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { theme } from '$lib/stores/theme'; // Initialize theme store

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let isSidebarOpen = $state(false);
	let isMessagesDrawerOpen = $state(false);
	let isUploadModalOpen = $state(false);
	let isCommandPaletteOpen = $state(false);

	// DuckDB initialization state - shared across all pages and components
	let isDbInitialized = $state(false);
	let isTauriAvailable = $state(false);
	let dbInitError = $state<string | null>(null);
	let isInitializing = $state(true); // Track if we're still initializing
	
	// Critical operation state - prevents app closure during important operations
	let isCriticalOperation = $state(false);
	let criticalOperationName = $state('Processing');
	let criticalOperationCount = $state(0); // Track nested operations

	// Remote file progress state
	let remoteFileProgress = $state<{
		isDownloading: boolean;
		progressPercent: number;
		downloadedMb: number;
		totalMb: number;
		message: string;
		status: string;
	}>({
		isDownloading: false,
		progressPercent: 0,
		downloadedMb: 0,
		totalMb: 0,
		message: '',
		status: ''
	});

	// System messages state exposed via context
	interface SystemMessage {
		id: string;
		message: string;
		timestamp: Date;
		type: 'info' | 'success' | 'error' | 'warning';
	}
	
	let systemMessages = $state<SystemMessage[]>([]);

	function handleUploadClick() {
		isUploadModalOpen = true;
	}

	function handleCommandClick() {
		isCommandPaletteOpen = true;
	}

	// Make system messages available to child components via window events
	if (typeof window !== 'undefined') {
		window.addEventListener('add-system-message', ((e: CustomEvent) => {
			const { message, type = 'info' } = e.detail;
			systemMessages = [...systemMessages, {
				id: Date.now().toString() + Math.random(),
				message,
				timestamp: new Date(),
				type
			}];
		}) as EventListener);
	}

	// Helper function to add system messages
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}
	
	// Critical operation management
	function startCriticalOperation(operationName: string) {
		criticalOperationCount++;
		criticalOperationName = operationName;
		isCriticalOperation = true;
		console.log(`🔒 Started critical operation: ${operationName} (count: ${criticalOperationCount})`);
	}
	
	function endCriticalOperation() {
		criticalOperationCount = Math.max(0, criticalOperationCount - 1);
		if (criticalOperationCount === 0) {
			isCriticalOperation = false;
			console.log('✅ All critical operations completed');
		} else {
			console.log(`🔒 Critical operation ended, ${criticalOperationCount} remaining`);
		}
	}
	
	// Make critical operation functions available globally via window events
	if (typeof window !== 'undefined') {
		window.addEventListener('start-critical-operation', ((e: CustomEvent) => {
			const { operation } = e.detail;
			startCriticalOperation(operation);
		}) as EventListener);
		
		window.addEventListener('end-critical-operation', (() => {
			endCriticalOperation();
		}) as EventListener);
	}

	// Set context for child components to access DB initialization state
	setContext('db', {
		get isInitialized() { return isDbInitialized; },
		get isTauriAvailable() { return isTauriAvailable; },
		get error() { return dbInitError; },
		get isInitializing() { return isInitializing; },
		startCriticalOperation,
		endCriticalOperation
	});

	// Track header and footer heights dynamically
	let headerHeight = $state(0);
	let tabBarHeight = $state(0);
	let footerHeight = $state(0);

	// Initialize DuckDB once for the entire app
	onMount(async () => {
		// Prevent accidental closure during critical operations
		const handleBeforeUnload = (e: BeforeUnloadEvent) => {
			if (isCriticalOperation) {
				e.preventDefault();
				// Chrome requires returnValue to be set
				e.returnValue = 'A critical database operation is in progress. Closing now may corrupt your data. Are you sure?';
				return e.returnValue;
			}
		};
		
		window.addEventListener('beforeunload', handleBeforeUnload);
		
		// Cleanup on component unmount
		const cleanup = () => {
			window.removeEventListener('beforeunload', handleBeforeUnload);
		};
		
		// Calculate actual header and footer heights
		const updateHeights = () => {
			const headerEl = document.querySelector('[data-header-container]') as HTMLElement;
			const tabBarEl = document.querySelector('[data-tab-bar-container]') as HTMLElement;
			const footerEl = document.querySelector('[data-footer-container]') as HTMLElement;
			
			if (headerEl) {
				headerHeight = headerEl.offsetHeight;
			}
			if (tabBarEl) {
				tabBarHeight = tabBarEl.offsetHeight;
			}
			if (footerEl) {
				footerHeight = footerEl.offsetHeight;
			}

			// Set CSS custom properties for use in styles
			document.documentElement.style.setProperty('--header-height', `${headerHeight}px`);
			document.documentElement.style.setProperty('--tab-bar-height', `${tabBarHeight}px`);
			document.documentElement.style.setProperty('--footer-height', `${footerHeight}px`);
			
			console.log('📏 Heights updated:', { headerHeight, tabBarHeight, footerHeight });
		};

		// Wait for DOM to be fully rendered before calculating heights
		// Use requestAnimationFrame to ensure layout is complete
		const initializeHeights = () => {
			requestAnimationFrame(() => {
				setTimeout(() => {
					updateHeights();
					// Double-check after another frame to ensure accuracy
					requestAnimationFrame(() => {
						setTimeout(updateHeights, 50);
					});
				}, 0);
			});
		};

		// Initial calculation with delay
		initializeHeights();
		
		// Recalculate on window resize
		window.addEventListener('resize', updateHeights);
		
		// Use ResizeObserver for more accurate tracking
		const headerEl = document.querySelector('[data-header-container]');
		const tabBarEl = document.querySelector('[data-tab-bar-container]');
		const footerEl = document.querySelector('[data-footer-container]');
		
		if (headerEl || tabBarEl || footerEl) {
			const observer = new ResizeObserver(updateHeights);
			if (headerEl) observer.observe(headerEl);
			if (tabBarEl) observer.observe(tabBarEl);
			if (footerEl) observer.observe(footerEl);
		}
		
		// Debug logging
		console.log('═══════════════════════════════════════');
		console.log('🔍 LAYOUT INITIALIZATION STARTING');
		console.log('═══════════════════════════════════════');
		console.log('🔍 typeof window:', typeof window);
		console.log('🔍 window !== undefined:', typeof window !== 'undefined');
		console.log('🔍 window object:', window);
		console.log('🔍 __TAURI__ in window:', '__TAURI__' in window);
		console.log('🔍 window.__TAURI__:', (window as any).__TAURI__);
		console.log('🔍 typeof invoke:', typeof invoke);
		console.log('🔍 Object.keys(window) (first 20):', Object.keys(window).slice(0, 20));
		
		// Wait for Tauri API to be available (with retry logic for hot-reload)
		const waitForTauri = async (maxAttempts = 20, delayMs = 200): Promise<boolean> => {
			for (let attempt = 1; attempt <= maxAttempts; attempt++) {
				console.log(`🔄 Attempt ${attempt}/${maxAttempts} - Checking Tauri availability...`);
				console.log(`  - '__TAURI__' in window:`, '__TAURI__' in window);
				console.log(`  - typeof invoke:`, typeof invoke);
				console.log(`  - window.__TAURI__:`, (window as any).__TAURI__);
				
				// Check if invoke function is actually callable
				if (typeof window !== 'undefined' && typeof invoke === 'function') {
					console.log('✅ Tauri invoke function detected!');
					return true;
				}
				
				console.log(`⏳ Tauri not ready yet, waiting ${delayMs}ms...`);
				await new Promise(resolve => setTimeout(resolve, delayMs));
			}
			
			console.error(`❌ Tauri API not available after ${maxAttempts} attempts (${maxAttempts * delayMs}ms total)`);
			return false;
		};
		
		// Check if Tauri API is available
		try {
			console.log('🔍 Waiting for Tauri API to be ready...');
			const tauriAvailable = await waitForTauri();
			
			if (!tauriAvailable) {
				console.error('❌ TAURI NOT DETECTED!');
				console.error('❌ This is likely a hot-reload issue or you are in a browser');
				console.error('💡 TIP: Try fully restarting the app (Ctrl+C and npm run dev)');
				
				isTauriAvailable = false;
				dbInitError = 'Tauri API not available - try restarting the app';
				addSystemMessage('⚠️ Tauri API not available - Please restart the app (Ctrl+C and npm run dev)', 'error');
				isInitializing = false;
				return;
			}

			// Initialize DuckDB
			console.log('✅ Tauri API DETECTED!');
			console.log('🔄 Setting isTauriAvailable = true');
			isTauriAvailable = true;
			
			// Listen for remote file progress events
			const { listen } = await import('@tauri-apps/api/event');
			await listen('remote-file-progress', (event: any) => {
				const payload = event.payload as {
					status: string;
					progressPercent: number;
					downloadedMb: number;
					totalMb: number;
					message: string;
				};
				
				console.log('📡 Remote file progress:', payload);
				
				remoteFileProgress = {
					isDownloading: payload.status === 'downloading' || payload.status === 'connecting',
					progressPercent: payload.progressPercent,
					downloadedMb: payload.downloadedMb,
					totalMb: payload.totalMb,
					message: payload.message,
					status: payload.status
				};
				
				// Add system message for key status changes
				if (payload.status === 'connecting') {
					addSystemMessage('🌐 Downloading sample historical data (one-time only)...', 'info');
				} else if (payload.status === 'complete') {
					addSystemMessage('✅ Historical data download complete', 'success');
					remoteFileProgress.isDownloading = false;
				}
			});
			
			console.log('📢 Sending "Initializing database..." message');
			addSystemMessage('Initializing database...', 'info');
			
			// Mark as critical operation
			startCriticalOperation('Initializing Database');
			
			try {
				console.log('🔄 Calling invoke("initialize_duckdb")...');
				const result = await invoke<string>('initialize_duckdb');
				
				console.log('✅ invoke() returned successfully!');
				console.log('✅ Result:', result);
				console.log('🔄 Setting isDbInitialized = true');
				isDbInitialized = true;
				
				console.log('📢 Sending success message');
				addSystemMessage('Database initialized successfully', 'success');
				console.log('✅ DuckDB initialized globally:', result);
				console.log('═══════════════════════════════════════');
				console.log('✅ INITIALIZATION COMPLETE');
				console.log('═══════════════════════════════════════');
			} finally {
				// Always end critical operation, even if there's an error
				endCriticalOperation();
			}
		} catch (error) {
			console.error('═══════════════════════════════════════');
			console.error('❌ INITIALIZATION FAILED');
			console.error('═══════════════════════════════════════');
			console.error('❌ Error type:', typeof error);
			console.error('❌ Error value:', error);
			console.error('❌ Error message:', String(error));
			console.error('❌ Error stack:', (error as any)?.stack);
			
			const errorStr = String(error);
			dbInitError = errorStr;
			addSystemMessage(`Database initialization failed: ${error}`, 'error');
		} finally {
			console.log('🔄 Setting isInitializing = false');
			isInitializing = false;
			console.log('🔍 Final state:');
			console.log('  - isTauriAvailable:', isTauriAvailable);
			console.log('  - isDbInitialized:', isDbInitialized);
			console.log('  - dbInitError:', dbInitError);
			console.log('  - isInitializing:', isInitializing);
		}
	});
</script>

<div class="flex flex-col min-h-screen overflow-hidden">
	<!-- Fixed Header Area -->
	<div data-header-container class="fixed top-0 left-0 right-0 z-50 bg-white dark:bg-slate-950">
		<!-- Header Component -->
		<Header 
			onMenuClick={() => isSidebarOpen = true}
			onUploadClick={handleUploadClick}
			onMessagesClick={() => isMessagesDrawerOpen = true}
			onCommandClick={handleCommandClick}
			messageCount={systemMessages.length}
		/>

		<!-- Breadcrumb Component - Below Header -->
		<Breadcrumb />
	</div>

	<!-- System Messages Drawer -->
	<SystemMessagesDrawer bind:messages={systemMessages} bind:isOpen={isMessagesDrawerOpen} />

	<!-- Upload Modal -->
	<UploadModal bind:isOpen={isUploadModalOpen} />

	<!-- Command Palette -->
	<CommandPalette bind:isOpen={isCommandPaletteOpen} />

	<!-- Sidebar -->
	<Sidebar bind:isOpen={isSidebarOpen} />
	
	<!-- Critical Operation Overlay -->
	<CriticalOperationOverlay 
		isVisible={isCriticalOperation || remoteFileProgress.isDownloading} 
		operation={remoteFileProgress.isDownloading ? 'Downloading Sample Data' : criticalOperationName}
		showProgress={remoteFileProgress.isDownloading}
		progressPercent={remoteFileProgress.progressPercent}
		progressMessage={remoteFileProgress.message}
		downloadedMb={remoteFileProgress.downloadedMb}
		totalMb={remoteFileProgress.totalMb}
	/>

	<!-- Toast Notifications -->
	<ToastContainer />

	<!-- Main Content with Header Offset -->
	<main class="page-content" style="margin-top: var(--header-height, 73px); margin-bottom: var(--tab-bar-height, 52px);">
		{@render children()}
	</main>
	
	<!-- Fixed Tab Bar at Bottom -->
	<div data-tab-bar-container class="fixed bottom-0 left-0 right-0 z-40 bg-[#F9F9F9] dark:bg-slate-800">
		<TabBar />
	</div>
</div>

<style>
	:global(:root) {
		--header-height: 73px; /* Default fallback */
		--tab-bar-height: 52px; /* Tab bar at bottom */
		--footer-height: 0px;
	}

	.flex {
		display: flex;
	}

	.flex-col {
		flex-direction: column;
	}

	.min-h-screen {
		min-height: 100vh;
	}

	.overflow-hidden {
		overflow: hidden;
	}

	.fixed {
		position: fixed;
	}

	.top-0 {
		top: 0;
	}

	.left-0 {
		left: 0;
	}

	.right-0 {
		right: 0;
	}

	.z-50 {
		z-index: 50;
	}

	.flex-1 {
		flex: 1;
	}

	.min-h-0 {
		min-height: 0;
	}

	.page-content {
		width: 100%;
		height: calc(100vh - var(--header-height, 73px) - var(--tab-bar-height, 52px));
		overflow-y: auto;
	}
</style>
