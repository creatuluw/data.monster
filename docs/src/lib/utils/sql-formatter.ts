/**
 * SQL Formatter Utility
 * Formats SQL queries with proper indentation, structure, and comments
 * like a professional SQL developer would write them
 */

/**
 * Formats a SQL query with proper indentation and structure
 * @param query - The SQL query to format
 * @returns The formatted SQL query
 */
export function formatSQLQuery(query: string): string {
	// Remove existing formatting and normalize whitespace
	let sql = query.trim().replace(/\s+/g, ' ');
	
	// Don't format if it's already well-formatted (has multiple lines with proper indentation)
	const lines = query.split('\n');
	if (lines.length > 3 && lines.some(line => line.match(/^\s{2,}/))) {
		return query; // Already formatted, return as-is
	}
	
	// Remove CREATE TABLE ... AS prefix if present to just format the SELECT
	const createTablePattern = /^CREATE\s+TABLE\s+\w+\s+AS\s+/i;
	const hasCreateTable = createTablePattern.test(sql);
	if (hasCreateTable) {
		sql = sql.replace(createTablePattern, '');
	}
	
	// Add line breaks after major SQL keywords
	sql = sql
		// Main clauses
		.replace(/\s+(SELECT|FROM|WHERE|GROUP BY|HAVING|ORDER BY|LIMIT|OFFSET)\s+/gi, '\n$1\n  ')
		// JOIN clauses
		.replace(/\s+(LEFT JOIN|RIGHT JOIN|INNER JOIN|OUTER JOIN|FULL JOIN|CROSS JOIN|JOIN)\s+/gi, '\n$1\n  ')
		// Boolean operators in WHERE clause
		.replace(/\s+(AND|OR)\s+/gi, '\n  $1 ')
		// Set operations
		.replace(/\s+(UNION|UNION ALL|INTERSECT|EXCEPT)\s+/gi, '\n$1\n')
		// Window functions
		.replace(/\s+(OVER\s*\()/gi, '\n  $1')
		// CASE statements
		.replace(/\s+(CASE)\s+/gi, '\n  $1 ')
		.replace(/\s+(WHEN)\s+/gi, '\n    $1 ')
		.replace(/\s+(THEN)\s+/gi, ' $1 ')
		.replace(/\s+(ELSE)\s+/gi, '\n    $1 ')
		.replace(/\s+(END)\s+/gi, '\n  $1 ');
	
	// Clean up extra spaces
	sql = sql
		.replace(/\n\s*\n\s*\n/g, '\n\n') // Remove triple line breaks
		.replace(/\n +\n/g, '\n') // Remove lines with only spaces
		.replace(/,\s+/g, ',\n  '); // Format comma-separated items
	
	// Split into lines for processing
	let formattedLines = sql.split('\n').map(line => line.trim());
	
	// Apply indentation
	let indentLevel = 0;
	const indentedLines: string[] = [];
	
	for (let i = 0; i < formattedLines.length; i++) {
		const line = formattedLines[i];
		if (!line) continue;
		
		const upperLine = line.toUpperCase();
		
		// Decrease indent for closing keywords
		if (upperLine.match(/^(END|ELSE)/)) {
			indentLevel = Math.max(0, indentLevel - 1);
		}
		
		// Add the line with current indentation
		const indent = '  '.repeat(indentLevel);
		indentedLines.push(indent + line);
		
		// Increase indent for opening keywords
		if (upperLine.match(/^(CASE|WHEN|WHERE|FROM|JOIN|LEFT JOIN|RIGHT JOIN|INNER JOIN)/)) {
			// Don't increase indent, these manage their own
		} else if (upperLine.includes('(') && !upperLine.includes(')')) {
			indentLevel++;
		} else if (upperLine.match(/^(SELECT|FROM)/)) {
			// After SELECT or FROM, indent the next items
			if (i + 1 < formattedLines.length && !formattedLines[i + 1].toUpperCase().match(/^(FROM|WHERE|GROUP|HAVING|ORDER|LIMIT)/)) {
				indentLevel++;
			}
		}
		
		// Decrease indent for closing parentheses
		if (upperLine.includes(')') && !upperLine.includes('(')) {
			indentLevel = Math.max(0, indentLevel - 1);
		}
	}
	
	let formatted = indentedLines.join('\n');
	
	// Final cleanup
	formatted = formatted
		.replace(/\n{3,}/g, '\n\n') // Max 2 consecutive line breaks
		.trim();
	
	return formatted;
}

/**
 * Formats a SELECT query from column metadata
 * @param tableName - The name of the table
 * @param columns - Array of column information
 * @param isOriginal - Whether this is the original creation query
 * @returns A formatted SQL query string with comments
 */
export function formatSelectQuery(
	tableName: string,
	columns: Array<{ name: string; type?: string }>,
	isOriginal: boolean = false
): string {
	const lines: string[] = [
		`SELECT`
	];
	
	// Add columns with proper formatting
	columns.forEach((col, idx) => {
		const isLast = idx === columns.length - 1;
		const comma = isLast ? '' : ',';
		const typeComment = col.type ? `  -- ${col.type}` : '';
		lines.push(`  "${col.name}"${comma}${typeComment}`);
	});
	
	lines.push(`FROM ${tableName}`);
	lines.push(`WHERE 1=1  -- Add your filters here`);
	lines.push(`-- ORDER BY column_name ASC`);
	lines.push(`LIMIT 100`);
	
	return lines.join('\n');
}

/**
 * Formats a query with a descriptive comment header
 * @param query - The SQL query to format
 * @param tableName - The table name
 * @param metadata - Additional metadata for the header
 * @returns Formatted query with comment header
 */
export function formatQueryWithHeader(
	query: string,
	tableName: string,
	metadata?: {
		type?: 'original' | 'generated' | 'file';
		source?: string;
		columnCount?: number;
		description?: string;
	}
): string {
	const timestamp = new Date().toISOString().split('T')[0];
	const type = metadata?.type || 'generated';
	
	const header: string[] = [
		`-- ================================================`,
		`-- Table: ${tableName}`,
	];
	
	if (metadata?.source) {
		header.push(`-- Source: ${metadata.source}`);
	}
	
	if (type === 'original') {
		header.push(`-- Type: Original creation query`);
	} else if (type === 'file') {
		header.push(`-- Type: Loaded from file`);
	} else {
		header.push(`-- Type: Generated from table structure`);
	}
	
	if (metadata?.columnCount) {
		header.push(`-- Columns: ${metadata.columnCount}`);
	}
	
	if (metadata?.description) {
		header.push(`-- ${metadata.description}`);
	}
	
	header.push(`-- Date: ${timestamp}`);
	header.push(`-- ================================================`);
	header.push(``);
	
	// Format the actual query
	const formattedQuery = formatSQLQuery(query);
	
	return header.join('\n') + formattedQuery;
}

/**
 * Extracts just the SELECT portion from a CREATE TABLE AS statement
 * @param createTableQuery - The full CREATE TABLE AS query
 * @returns The SELECT portion only
 */
export function extractSelectFromCreateTable(createTableQuery: string): string {
	const createTablePattern = /^CREATE\s+TABLE\s+\w+\s+AS\s+/i;
	if (createTablePattern.test(createTableQuery)) {
		return createTableQuery.replace(createTablePattern, '').trim();
	}
	return createTableQuery;
}

