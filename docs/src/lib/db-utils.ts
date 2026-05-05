/**
 * Database utility functions for handling DuckDB query results
 */

/**
 * Response format from execute_query command
 * New format: { columns: string[], data: any[] }
 * Old format: any[] (array of objects)
 */
interface QueryResponse {
	columns?: string[];
	data?: any[];
}

/**
 * Parse the execute_query result and handle both old and new response formats
 * 
 * @param result - JSON string from execute_query Tauri command
 * @returns Array of result objects
 * 
 * @example
 * ```typescript
 * const result = await invoke<string>('execute_query', { query: 'SELECT * FROM users' });
 * const users = parseQueryResult(result);
 * ```
 */
export function parseQueryResult<T = any>(result: string): T[] {
	const parsed: QueryResponse | T[] = JSON.parse(result);
	
	// Check if it's the new format with columns and data properties
	if (parsed && typeof parsed === 'object' && 'columns' in parsed && 'data' in parsed) {
		return (parsed as QueryResponse).data as T[];
	}
	
	// Fallback to old format (direct array)
	return parsed as T[];
}

/**
 * Parse the execute_query result and return both columns and data separately
 * Useful when you need to preserve column order
 * 
 * @param result - JSON string from execute_query Tauri command
 * @returns Object with columns array and data array
 * 
 * @example
 * ```typescript
 * const result = await invoke<string>('execute_query', { query: 'SELECT name, age FROM users' });
 * const { columns, data } = parseQueryResultWithColumns(result);
 * // columns = ['name', 'age']
 * // data = [{ name: 'Alice', age: 30 }, ...]
 * ```
 */
export function parseQueryResultWithColumns<T = any>(result: string): { columns: string[]; data: T[] } {
	const parsed: QueryResponse | T[] = JSON.parse(result);
	
	// Check if it's the new format with columns and data properties
	if (parsed && typeof parsed === 'object' && 'columns' in parsed && 'data' in parsed) {
		return {
			columns: (parsed as QueryResponse).columns || [],
			data: (parsed as QueryResponse).data as T[]
		};
	}
	
	// Fallback to old format - extract columns from first row
	const dataArray = parsed as T[];
	const columns = dataArray.length > 0 ? Object.keys(dataArray[0]) : [];
	
	return {
		columns,
		data: dataArray
	};
}

