/**
 * Database Context Helper
 * 
 * Provides access to the global DuckDB initialization state
 * that is set up in the root +layout.svelte
 */

import { getContext } from 'svelte';

export interface DbContext {
	isInitialized: boolean;
	isTauriAvailable: boolean;
	error: string | null;
	isInitializing: boolean;
}

/**
 * Get the database context from the root layout
 * 
 * @returns The database context with initialization state
 */
export function getDbContext(): DbContext {
	const context = getContext<DbContext>('db');
	
	if (!context) {
		// Fallback for when context is not available (e.g., in non-Tauri environment)
		return {
			isInitialized: false,
			isTauriAvailable: false,
			error: 'Database context not available',
			isInitializing: false
		};
	}
	
	return context;
}

