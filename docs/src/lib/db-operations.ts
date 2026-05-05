/**
 * Database Operations with Critical Operation Protection
 * 
 * This module wraps all database operations with critical operation tracking.
 * Import these functions instead of calling invoke() directly for database operations.
 */

import { invoke } from '@tauri-apps/api/core';
import { withCriticalOperation } from './critical-operations';

// ============================================================================
// FILE OPERATIONS (CSV, Parquet, JSON)
// ============================================================================

export async function loadCSVFile(filePath: string, tableName: string): Promise<string> {
	return withCriticalOperation('Loading CSV File', async () => {
		return await invoke<string>('load_csv_file', { filePath, tableName });
	});
}

export async function loadParquetFile(filePath: string, tableName: string): Promise<string> {
	return withCriticalOperation('Loading Parquet File', async () => {
		return await invoke<string>('load_parquet_file', { filePath, tableName });
	});
}

export async function loadJSONFile(filePath: string, tableName: string): Promise<string> {
	return withCriticalOperation('Loading JSON File', async () => {
		return await invoke<string>('load_json_file', { filePath, tableName });
	});
}

// ============================================================================
// TABLE OPERATIONS
// ============================================================================

export async function createTableFromQuery(tableName: string, query: string): Promise<string> {
	return withCriticalOperation('Creating Table', async () => {
		return await invoke<string>('create_table_from_query', { tableName, query });
	});
}

export async function dropTable(tableName: string): Promise<string> {
	return withCriticalOperation('Deleting Table', async () => {
		return await invoke<string>('drop_table', { tableName });
	});
}

export async function getTableSize(tableName: string): Promise<number> {
	// Read-only operation, no critical protection needed
	return await invoke<number>('get_table_size', { tableName });
}

export async function listTables(): Promise<any[]> {
	// Read-only operation, no critical protection needed
	return await invoke<any[]>('list_tables');
}

export async function getSavedTables(): Promise<any[]> {
	// Read-only operation, no critical protection needed
	return await invoke<any[]>('get_saved_tables');
}

export async function getTableCreationQuery(tableName: string): Promise<string> {
	// Read-only operation, no critical protection needed
	return await invoke<string>('get_table_creation_query', { tableName });
}

// ============================================================================
// QUERY EXECUTION
// ============================================================================

export async function executeQuery(query: string): Promise<string> {
	// Check if it's a write operation (INSERT, UPDATE, DELETE, CREATE, DROP, ALTER)
	const writeOperations = ['INSERT', 'UPDATE', 'DELETE', 'CREATE', 'DROP', 'ALTER', 'TRUNCATE', 'VACUUM'];
	const isWriteOperation = writeOperations.some(op => 
		query.trim().toUpperCase().startsWith(op)
	);
	
	if (isWriteOperation) {
		return withCriticalOperation('Executing Query', async () => {
			return await invoke<string>('execute_query', { query });
		});
	} else {
		// Read-only query, no protection needed
		return await invoke<string>('execute_query', { query });
	}
}

// ============================================================================
// POSTGRES OPERATIONS
// ============================================================================

export async function createPostgresConnection(params: {
	name: string;
	host: string;
	port: number;
	database: string;
	username: string;
	password: string;
	sslMode?: string;
}): Promise<string> {
	return withCriticalOperation('Creating PostgreSQL Connection', async () => {
		// Tauri expects camelCase but Rust function uses snake_case
		// Tauri handles the conversion automatically
		const tauriParams = {
			connectionName: params.name,
			host: params.host,
			port: params.port,
			database: params.database,
			username: params.username,
			password: params.password,
			sslMode: params.sslMode || 'prefer'
		};
		
		console.log('🔍 Sending to Tauri (camelCase):', tauriParams);
		
		return await invoke<string>('create_postgres_connection', tauriParams);
	});
}

export async function deletePostgresConnection(connectionId: string): Promise<string> {
	return withCriticalOperation('Deleting PostgreSQL Connection', async () => {
		return await invoke<string>('delete_postgres_connection', { connectionId });
	});
}

export async function attachPostgresDatabase(connectionId: string): Promise<string> {
	return withCriticalOperation('Attaching PostgreSQL Database', async () => {
		return await invoke<string>('attach_postgres_database', { connectionId });
	});
}

export async function detachPostgresDatabase(connectionId: string): Promise<string> {
	return withCriticalOperation('Detaching PostgreSQL Database', async () => {
		return await invoke<string>('detach_postgres_database', { connectionId });
	});
}

export async function listPostgresConnections(): Promise<any[] | string> {
	// Read-only operation
	// Note: Tauri may return JSON string or parsed array depending on configuration
	return await invoke<any[]>('list_postgres_connections');
}

export async function getPostgresConnection(connectionId: string): Promise<any> {
	// Read-only operation
	// Tauri automatically converts camelCase to snake_case
	return await invoke<any>('get_postgres_connection', { connectionId });
}

// ============================================================================
// UDF (USER-DEFINED FUNCTIONS) OPERATIONS
// ============================================================================

export async function createUDF(params: {
	functionName: string;
	functionType: string;
	parameters: string;
	returnType: string;
	language: string;
	functionBody: string;
	description?: string;
}): Promise<string> {
	return withCriticalOperation('Creating Function', async () => {
		return await invoke<string>('create_udf', params);
	});
}

export async function deleteUDF(functionName: string): Promise<string> {
	return withCriticalOperation('Deleting Function', async () => {
		return await invoke<string>('delete_udf', { functionName });
	});
}

export async function listUDFs(): Promise<any[]> {
	// Read-only operation
	return await invoke<any[]>('list_udfs');
}

export async function getUDFInfo(functionName: string): Promise<any> {
	// Read-only operation
	return await invoke<any>('get_udf_info', { functionName });
}

// ============================================================================
// METRICS OPERATIONS
// ============================================================================

export async function saveMetric(params: {
	metricName: string;
	formula: string;
	sourceTable: string;
	description?: string;
	tags?: string;
}): Promise<string> {
	return withCriticalOperation('Saving Metric', async () => {
		return await invoke<string>('save_metric', params);
	});
}

export async function deleteMetric(metricName: string): Promise<string> {
	return withCriticalOperation('Deleting Metric', async () => {
		return await invoke<string>('delete_metric', { metricName });
	});
}

export async function listMetrics(): Promise<any[]> {
	// Read-only operation
	return await invoke<any[]>('list_metrics');
}

export async function getMetric(metricName: string): Promise<any> {
	// Read-only operation
	return await invoke<any>('get_metric', { metricName });
}

// ============================================================================
// DIMENSIONS OPERATIONS
// ============================================================================

export async function saveDimension(params: {
	dimensionName: string;
	sourceTable: string;
	sourceColumn: string;
	description?: string;
	tags?: string;
}): Promise<string> {
	return withCriticalOperation('Saving Dimension', async () => {
		return await invoke<string>('save_dimension', params);
	});
}

export async function deleteDimension(dimensionName: string): Promise<string> {
	return withCriticalOperation('Deleting Dimension', async () => {
		return await invoke<string>('delete_dimension', { dimensionName });
	});
}

export async function listDimensions(): Promise<any[]> {
	// Read-only operation
	return await invoke<any[]>('list_dimensions');
}

// ============================================================================
// RELATIONSHIP OPERATIONS
// ============================================================================

export async function detectTableRelationships(tableName: string): Promise<string> {
	return withCriticalOperation('Detecting Relationships', async () => {
		return await invoke<string>('detect_table_relationships', { tableName });
	});
}

export async function scanAllRelationships(): Promise<string> {
	return withCriticalOperation('Scanning All Relationships', async () => {
		return await invoke<string>('scan_all_relationships');
	});
}

export async function getTableRelationships(tableName?: string): Promise<any[]> {
	// Read-only operation
	return await invoke<any[]>('get_table_relationships', tableName ? { tableName } : {});
}

// ============================================================================
// FILE MANAGEMENT
// ============================================================================

export async function saveFileToFolder(sourcePath: string, folderName: string): Promise<string> {
	return withCriticalOperation('Saving File to Folder', async () => {
		return await invoke<string>('save_file_to_folder', { sourcePath, folderName });
	});
}

export async function createFolder(folderName: string): Promise<string> {
	return withCriticalOperation('Creating Folder', async () => {
		return await invoke<string>('create_folder', { folderName });
	});
}

export async function listFolders(): Promise<string[]> {
	// Read-only operation
	return await invoke<string[]>('list_folders');
}

export async function listFilesInFolder(folderName: string): Promise<any[]> {
	// Read-only operation
	return await invoke<any[]>('list_files_in_folder', { folderName });
}

export async function getFileMetadata(folderName: string, fileName: string): Promise<any> {
	// Read-only operation
	return await invoke<any>('get_file_metadata', { folderName, fileName });
}

// ============================================================================
// READ-ONLY OPERATIONS (No Protection Needed)
// ============================================================================

export async function getFileColumns(filePath: string, fileType: string): Promise<any[]> {
	return await invoke<any[]>('get_file_columns', { filePath, fileType });
}

/**
 * Example Usage:
 * 
 * Before (direct invoke):
 *   const result = await invoke('load_csv_file', { filePath, tableName });
 * 
 * After (with protection):
 *   import { loadCSVFile } from '$lib/db-operations';
 *   const result = await loadCSVFile(filePath, tableName);
 * 
 * The loading overlay will automatically appear and the app will be protected from closure.
 */

