/**
 * Type Detection Utility for DuckDB
 * 
 * Provides automatic type detection for data before it reaches the database.
 * Integrates with Tauri commands for backend type inference.
 */

import { invoke } from '@tauri-apps/api/core';

export interface ColumnTypeInfo {
	column_name: string;
	detected_type: string; // DuckDB type (VARCHAR, INTEGER, DOUBLE, TIMESTAMP, etc.)
	nullable: boolean;
	sample_values: string[];
	distinct_count: number;
	null_count: number;
	confidence: number; // 0.0 to 1.0
}

/**
 * Infer column types from sample data (JSON array of objects)
 * @param data - Array of objects representing rows
 * @returns Array of column type information
 */
export async function inferColumnTypes(data: any[]): Promise<ColumnTypeInfo[]> {
	try {
		const result = await invoke<string>('infer_column_types', {
			data: JSON.stringify(data)
		});
		return JSON.parse(result);
	} catch (error) {
		console.error('Error inferring column types:', error);
		throw error;
	}
}

/**
 * Infer column types from a file (CSV, JSON, Parquet)
 * @param filePath - Path to the file (relative to app data directory)
 * @param sampleSize - Number of rows to sample (default: 100)
 * @returns Array of column type information
 */
export async function inferFileTypes(
	filePath: string,
	sampleSize: number = 100
): Promise<ColumnTypeInfo[]> {
	try {
		const result = await invoke<string>('infer_file_types', {
			filePath,
			sampleSize
		});
		return JSON.parse(result);
	} catch (error) {
		console.error('Error inferring file types:', error);
		throw error;
	}
}

/**
 * Detect type from a single value (client-side only, for preview)
 * @param value - The value to check
 * @returns Detected type string
 */
export function detectValueType(value: any): string {
	if (value === null || value === undefined) {
		return 'NULL';
	}

	if (typeof value === 'boolean') {
		return 'BOOLEAN';
	}

	if (typeof value === 'number') {
		return Number.isInteger(value) ? 'BIGINT' : 'DOUBLE';
	}

	if (typeof value === 'string') {
		const trimmed = value.trim();

		// Boolean strings
		if (
			trimmed.toLowerCase() === 'true' ||
			trimmed.toLowerCase() === 'false'
		) {
			return 'BOOLEAN';
		}

		// Numbers
		if (!isNaN(Number(trimmed))) {
			return trimmed.includes('.') ? 'DOUBLE' : 'BIGINT';
		}

		// Date (YYYY-MM-DD)
		if (/^\d{4}-\d{2}-\d{2}$/.test(trimmed)) {
			return 'DATE';
		}

		// Timestamp (ISO 8601)
		if (/^\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}/.test(trimmed)) {
			return 'TIMESTAMP';
		}

		// Time
		if (/^\d{2}:\d{2}(:\d{2})?(\.\d+)?$/.test(trimmed)) {
			return 'TIME';
		}

		// JSON
		if (
			(trimmed.startsWith('{') && trimmed.endsWith('}')) ||
			(trimmed.startsWith('[') && trimmed.endsWith(']'))
		) {
			try {
				JSON.parse(trimmed);
				return 'JSON';
			} catch {
				// Not valid JSON
			}
		}

		return 'VARCHAR';
	}

	if (typeof value === 'object') {
		return 'JSON';
	}

	return 'VARCHAR';
}

/**
 * Format a type name for display
 * @param type - DuckDB type name
 * @returns Formatted display name
 */
export function formatTypeName(type: string): string {
	const typeMap: Record<string, string> = {
		VARCHAR: 'Text',
		BIGINT: 'Integer',
		DOUBLE: 'Decimal',
		BOOLEAN: 'Boolean',
		DATE: 'Date',
		TIMESTAMP: 'Timestamp',
		TIME: 'Time',
		JSON: 'JSON',
		BLOB: 'Binary',
		NULL: 'Null'
	};

	return typeMap[type.toUpperCase()] || type;
}

/**
 * Get an icon name for a type (for use with lucide-svelte icons)
 * @param type - DuckDB type name
 * @returns Icon component name
 */
export function getTypeIcon(type: string): string {
	const iconMap: Record<string, string> = {
		VARCHAR: 'Type',
		BIGINT: 'Hash',
		DOUBLE: 'SquareFunction',
		BOOLEAN: 'ToggleLeft',
		DATE: 'Calendar',
		TIMESTAMP: 'Clock',
		TIME: 'Clock',
		JSON: 'Braces',
		BLOB: 'FileCode',
		NULL: 'CircleOff'
	};

	return iconMap[type.toUpperCase()] || 'HelpCircle';
}

/**
 * Get color class for a type (Tailwind CSS classes)
 * @param type - DuckDB type name
 * @returns Tailwind color classes
 */
export function getTypeColor(type: string): string {
	const colorMap: Record<string, string> = {
		VARCHAR: 'text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20',
		BIGINT: 'text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20',
		DOUBLE: 'text-emerald-600 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-900/20',
		BOOLEAN:
			'text-purple-600 dark:text-purple-400 bg-purple-50 dark:bg-purple-900/20',
		DATE: 'text-orange-600 dark:text-orange-400 bg-orange-50 dark:bg-orange-900/20',
		TIMESTAMP:
			'text-amber-600 dark:text-amber-400 bg-amber-50 dark:bg-amber-900/20',
		TIME: 'text-yellow-600 dark:text-yellow-400 bg-yellow-50 dark:bg-yellow-900/20',
		JSON: 'text-pink-600 dark:text-pink-400 bg-pink-50 dark:bg-pink-900/20',
		BLOB: 'text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-gray-900/20',
		NULL: 'text-slate-500 dark:text-slate-400 bg-slate-50 dark:bg-slate-900/20'
	};

	return (
		colorMap[type.toUpperCase()] ||
		'text-slate-600 dark:text-slate-400 bg-slate-50 dark:bg-slate-900/20'
	);
}

/**
 * Generate CREATE TABLE SQL with proper types
 * @param tableName - Name of the table to create
 * @param columns - Column type information
 * @returns SQL CREATE TABLE statement
 */
export function generateCreateTableSQL(
	tableName: string,
	columns: ColumnTypeInfo[]
): string {
	const columnDefs = columns
		.map((col) => {
			const nullable = col.nullable ? '' : ' NOT NULL';
			return `  ${col.column_name} ${col.detected_type}${nullable}`;
		})
		.join(',\n');

	return `CREATE TABLE ${tableName} (\n${columnDefs}\n);`;
}

/**
 * Validate if a value matches a type
 * @param value - Value to validate
 * @param expectedType - Expected DuckDB type
 * @returns True if value matches type
 */
export function validateType(value: any, expectedType: string): boolean {
	const detectedType = detectValueType(value);

	// Exact match
	if (detectedType === expectedType) {
		return true;
	}

	// Compatible types
	if (expectedType === 'DOUBLE' && detectedType === 'BIGINT') {
		return true; // Integers can be stored as doubles
	}

	if (expectedType === 'VARCHAR') {
		return true; // Everything can be stored as VARCHAR
	}

	return false;
}

