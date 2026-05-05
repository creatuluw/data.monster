import { getDb, schedulePersist } from './duckdb';
import { DuckDBDataProtocol } from '@duckdb/duckdb-wasm';

export interface QueryResult {
	columns: string[];
	rows: Record<string, unknown>[];
	totalRows?: number;
}

export interface PreviewData extends QueryResult {
	detectedTypes: { name: string; type: string }[];
	totalRows: number;
	sourceName?: string;
}

export interface ColumnOverride {
	originalName: string;
	newName: string;
	detectedType: string;
	overrideType: string | null;
	enabled: boolean;
}

// ── Preview a file (read 10 rows, detect types) ──

export async function previewFile(
	file: File
): Promise<PreviewData> {
	const db = await getDb();

	// Register the file with DuckDB
	await db.registerFileHandle(file.name, file, DuckDBDataProtocol.BROWSER_FILEREADER, false);

	const conn = await db.connect();

	// Detect format
	const ext = file.name.split('.').pop()?.toLowerCase() ?? 'csv';
	const reader =
		ext === 'parquet'
			? `read_parquet('${file.name}')`
			: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
				? `read_json_auto('${file.name}')`
				: `read_csv_auto('${file.name}')`;

	// Get detected column types
	const schemaResult = await conn.query(
		`SELECT column_name, column_type FROM (DESCRIBE SELECT * FROM ${reader}) `
	);
	const detectedTypes = schemaResult.toArray().map((row: any) => ({
		name: row.column_name as string,
		type: row.column_type as string
	}));

	// Get total row count
	const countResult = await conn.query(`SELECT COUNT(*) as cnt FROM ${reader}`);
	const totalRows = Number(countResult.toArray()[0].cnt);

	// Get first 10 rows
	const rowsResult = await conn.query(`SELECT * FROM ${reader} LIMIT 10`);
	const columns = rowsResult.schema.fields.map((f: any) => f.name);
	const rows = rowsResult.toArray().map((row: any) =>
		Object.fromEntries(columns.map((col: string) => [col, row[col]]))
	);

	conn.close();
	return { columns, rows, detectedTypes, totalRows };
}

// ── Preview a URL ──

export async function previewUrl(url: string): Promise<PreviewData> {
	const db = await getDb();
	const conn = await db.connect();

	// Detect format from URL extension
	const ext = url.split('.').pop()?.split('?')[0]?.toLowerCase() ?? 'csv';
	const reader =
		ext === 'parquet'
			? `read_parquet('${url}')`
			: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
				? `read_json_auto('${url}')`
				: `read_csv_auto('${url}')`;

	const tableName = `__preview_${Date.now()}`;

	const schemaResult = await conn.query(
		`SELECT column_name, column_type FROM (DESCRIBE SELECT * FROM ${reader}) `
	);
	const detectedTypes = schemaResult.toArray().map((row: any) => ({
		name: row.column_name as string,
		type: row.column_type as string
	}));

	const countResult = await conn.query(`SELECT COUNT(*) as cnt FROM ${reader}`);
	const totalRows = Number(countResult.toArray()[0].cnt);

	const rowsResult = await conn.query(`SELECT * FROM ${reader} LIMIT 10`);
	const columns = rowsResult.schema.fields.map((f: any) => f.name);
	const rows = rowsResult.toArray().map((row: any) =>
		Object.fromEntries(columns.map((col: string) => [col, row[col]]))
	);

	conn.close();
	return { columns, rows, detectedTypes, totalRows };
}

// ── Ingest a file as a table ──

export async function ingestFileAsTable(
	file: File,
	tableName: string,
	columnOverrides?: { name: string; newName: string; type: string }[]
): Promise<void> {
	const db = await getDb();
	await db.registerFileHandle(file.name, file, DuckDBDataProtocol.BROWSER_FILEREADER, false);
	const conn = await db.connect();

	const ext = file.name.split('.').pop()?.toLowerCase() ?? 'csv';
	const reader =
		ext === 'parquet'
			? `read_parquet('${file.name}')`
			: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
				? `read_json_auto('${file.name}')`
				: `read_csv_auto('${file.name}')`;

	// Build column selections with type casts and renames
	let selectExpr = '*';
	if (columnOverrides && columnOverrides.length > 0) {
		selectExpr = columnOverrides
			.map((c) => {
				const alias = c.newName !== c.name ? ` AS "${c.newName}"` : '';
				return `CAST("${c.name}" AS ${c.type})${alias}`;
			})
			.join(', ');
	}

	await conn.query(`CREATE OR REPLACE TABLE "${tableName}" AS SELECT ${selectExpr} FROM ${reader}`);
	await conn.query('CHECKPOINT');
		schedulePersist(tableName);
	conn.close();
}

// ── Run arbitrary SQL (with pagination for SELECT queries) ──

export interface PagedQueryResult extends QueryResult {
	totalRows: number;
	page: number;
	pageSize: number;
	totalPages: number;
	isMutation: boolean;
}

export async function runQuery(sql: string, page: number = 1, pageSize: number = 10): Promise<PagedQueryResult> {
	const db = await getDb();
	const conn = await db.connect();

	const trimmed = sql.trim();
	const lower = trimmed.toLowerCase();
	const isMutation =
		lower.startsWith('insert') ||
		lower.startsWith('update') ||
		lower.startsWith('delete') ||
		lower.startsWith('alter') ||
		lower.startsWith('drop') ||
		lower.startsWith('create');

	if (isMutation) {
		const result = await conn.query(trimmed);
		const columns = result.schema.fields.map((f: any) => f.name);
		const rows = result.toArray().map((row: any) =>
			Object.fromEntries(columns.map((col: string) => [col, row[col]]))
		);
		await conn.query('CHECKPOINT');
		schedulePersist(); // persist all - mutation may affect unknown tables
		conn.close();
		return { columns, rows, totalRows: rows.length, page: 1, pageSize, totalPages: 1, isMutation: true };
	}

	// SELECT query — get total count, then paginate
	const countSql = `SELECT COUNT(*) as cnt FROM (${trimmed}) as _q`;
	let totalRows = 0;
	try {
		const countResult = await conn.query(countSql);
		totalRows = Number(countResult.toArray()[0].cnt);
	} catch {
		// If count fails (e.g. non-standard SQL), just run without pagination
	}

	const safePageSize = Math.min(Math.max(pageSize, 1), 100);
	const offset = (page - 1) * safePageSize;
	const pagedSql = `${trimmed} LIMIT ${safePageSize} OFFSET ${offset}`;

	const result = await conn.query(pagedSql);
	const columns = result.schema.fields.map((f: any) => f.name);
	const rows = result.toArray().map((row: any) =>
		Object.fromEntries(columns.map((col: string) => [col, row[col]]))
	);

	conn.close();
	const totalPages = Math.max(1, Math.ceil(totalRows / safePageSize));
	return { columns, rows, totalRows, page, pageSize: safePageSize, totalPages, isMutation: false };
}

// ── List tables ──

export async function listTables(): Promise<string[]> {
	const db = await getDb();
	const conn = await db.connect();
	const result = await conn.query(
		"SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name"
	);
	const tables = result.toArray().map((r: any) => r.table_name as string);
	conn.close();
	return tables;
}

// ── Get table data (paginated) ──

export async function getTableData(
	tableName: string,
	page: number = 1,
	pageSize: number = 100
): Promise<QueryResult & { totalRows: number }> {
	const db = await getDb();
	const conn = await db.connect();

	const countResult = await conn.query(`SELECT COUNT(*) as cnt FROM "${tableName}"`);
	const totalRows = Number(countResult.toArray()[0].cnt);

	const offset = (page - 1) * pageSize;
	const result = await conn.query(
		`SELECT * FROM "${tableName}" LIMIT ${pageSize} OFFSET ${offset}`
	);
	const columns = result.schema.fields.map((f: any) => f.name);
	const rows = result.toArray().map((row: any) =>
		Object.fromEntries(columns.map((col: string) => [col, row[col]]))
	);

	conn.close();
	return { columns, rows, totalRows };
}

// ── Drop a table ──

export async function dropTable(tableName: string): Promise<void> {
	const db = await getDb();
	const conn = await db.connect();
	await conn.query(`DROP TABLE IF EXISTS "${tableName}"`);
	await conn.query('CHECKPOINT');
		schedulePersist(); // persist all - removes dropped table from IndexedDB
	conn.close();
}

// ── Save a query result as a table ──

export async function saveQueryAsTable(sql: string, tableName: string): Promise<void> {
	const db = await getDb();
	const conn = await db.connect();
	await conn.query(`CREATE OR REPLACE TABLE "${tableName}" AS ${sql}`);
	await conn.query('CHECKPOINT');
		schedulePersist(tableName);
	conn.close();
}

// ── Clear all data (re-export from duckdb) ──

export { clearAllData } from './duckdb';
