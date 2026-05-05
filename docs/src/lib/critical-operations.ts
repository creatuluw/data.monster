/**
 * Critical Operation Utilities
 * 
 * Use these functions to mark database operations as critical.
 * Critical operations will:
 * - Show a loading overlay with a warning message
 * - Prevent the user from closing the app
 * - Protect against WAL corruption
 */

/**
 * Start a critical operation
 * @param operationName - Display name of the operation (e.g., "Uploading CSV", "Creating Table")
 */
export function startCriticalOperation(operationName: string): void {
	if (typeof window !== 'undefined') {
		window.dispatchEvent(new CustomEvent('start-critical-operation', {
			detail: { operation: operationName }
		}));
	}
}

/**
 * End a critical operation
 */
export function endCriticalOperation(): void {
	if (typeof window !== 'undefined') {
		window.dispatchEvent(new CustomEvent('end-critical-operation'));
	}
}

/**
 * Wrap an async function with critical operation tracking
 * @param operationName - Display name of the operation
 * @param fn - Async function to execute
 * @returns Promise with the result of the function
 */
export async function withCriticalOperation<T>(
	operationName: string,
	fn: () => Promise<T>
): Promise<T> {
	startCriticalOperation(operationName);
	try {
		return await fn();
	} finally {
		endCriticalOperation();
	}
}

/**
 * Example usage in a component:
 * 
 * import { withCriticalOperation } from '$lib/critical-operations';
 * 
 * async function uploadFile() {
 *   await withCriticalOperation('Uploading CSV File', async () => {
 *     const result = await invoke('load_csv_file', { filePath });
 *     // ... process result
 *   });
 * }
 * 
 * Or manually:
 * 
 * import { startCriticalOperation, endCriticalOperation } from '$lib/critical-operations';
 * 
 * async function complexOperation() {
 *   startCriticalOperation('Creating Data Model');
 *   try {
 *     // ... your database operations
 *   } finally {
 *     endCriticalOperation();
 *   }
 * }
 */

