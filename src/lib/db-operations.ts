import { invoke } from '@tauri-apps/api/core';

export function extractErrorMessage(e: unknown, fallback: string = 'Query failed'): string {
	if (e instanceof Error) return e.message;
	if (typeof e === 'string') return e;
	if (e && typeof e === 'object' && 'message' in e) return String((e as { message: unknown }).message);
	return fallback;
}

export interface PreviewData {
	columns: string[];
	rows: Record<string, unknown>[];
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

export interface QueryResult {
	columns: string[];
	data: unknown[][];
	rowCount: number;
}

export interface DmlResult {
	type: 'dml';
	affectedRows: number;
}

export interface PagedQueryResult {
	columns: string[];
	rows: Record<string, unknown>[];
	totalRows: number;
	page: number;
	pageSize: number;
	totalPages: number;
	isMutation: boolean;
	hasUnknownTotal?: boolean;
}

export interface ColumnInfo {
	name: string;
	type: string;
}

export interface FileLoadResult {
	tableName: string;
	rowCount: number;
}

export interface SavedQuery {
	slug: string;
	name: string;
	sql: string;
	description: string | null;
	tags: string | null;
	createdAt: string | null;
	updatedAt: string | null;
}

export interface TableLabels {
	tableName: string;
	tags: string[];
	group: string | null;
}

export async function initializeDuckdb(workspacePath: string): Promise<string> {
	return invoke<string>('initialize_duckdb', { workspacePath });
}

export async function shutdownDuckdb(): Promise<void> {
	return invoke<void>('shutdown_duckdb');
}

export async function chooseWorkspaceFolder(): Promise<string | null> {
	const result = await invoke<string | null>('choose_workspace_folder');
	return result;
}

export async function getWorkspacePath(): Promise<string | null> {
	return invoke<string | null>('get_workspace_path');
}

export async function setWorkspacePath(path: string): Promise<void> {
	return invoke<void>('set_workspace_path', { path });
}

export async function executeQuery(sql: string): Promise<QueryResult | DmlResult> {
	return invoke<QueryResult | DmlResult>('execute_query', { sql });
}

export async function cancelQuery(): Promise<void> {
	return invoke<void>('cancel_query');
}

export async function listTables(): Promise<string[]> {
	const result = await invoke<{ tables: { name: string }[] }>('list_tables');
	return result.tables.map((t) => t.name);
}

export async function dropTable(tableName: string): Promise<void> {
	return invoke<void>('drop_table', { tableName });
}

export async function createTableFromQuery(
	tableName: string,
	sql: string
): Promise<void> {
	return invoke<void>('create_table_from_query', { tableName, sql });
}

export async function renameTable(oldName: string, newName: string): Promise<void> {
	return invoke<void>('rename_table', { oldName, newName });
}

export interface TableSource {
	creationQuery: string | null;
	sourcePath: string | null;
	sourceType: string | null;
	originalSource: string | null;
}

export async function saveTableSource(
	tableName: string,
	creationQuery: string,
	sourcePath: string,
	sourceType?: string | null,
	originalSource?: string | null
): Promise<void> {
	return invoke<void>('save_table_source', { tableName, creationQuery, sourcePath, sourceType: sourceType ?? null, originalSource: originalSource ?? null });
}

export async function getTableSource(tableName: string): Promise<TableSource> {
	return invoke<TableSource>('get_table_source', { tableName });
}

export async function refreshTableFromSource(tableName: string): Promise<void> {
	return invoke<void>('refresh_table_from_source', { tableName });
}

export interface TableTypeEntry {
	tableName: string;
	tableType: string;
}

export async function getTableTypes(): Promise<TableTypeEntry[]> {
	const result = await invoke<{ types: TableTypeEntry[] }>('get_table_types');
	return result.types;
}

export async function loadCsvFile(
	path: string,
	tableName: string
): Promise<FileLoadResult> {
	return invoke<FileLoadResult>('load_csv_file', { path, tableName });
}

export async function loadParquetFile(
	path: string,
	tableName: string
): Promise<FileLoadResult> {
	return invoke<FileLoadResult>('load_parquet_file', { path, tableName });
}

export async function loadJsonFile(
	path: string,
	tableName: string
): Promise<FileLoadResult> {
	return invoke<FileLoadResult>('load_json_file', { path, tableName });
}

export async function getFileColumns(
	path: string
): Promise<{ columns: ColumnInfo[] }> {
	return invoke<{ columns: ColumnInfo[] }>('get_file_columns', { path });
}

export async function getFileSize(path: string): Promise<number> {
	return invoke<number>('get_file_size', { path });
}

export async function downloadUrlToWorkspace(url: string): Promise<{ path: string }> {
	return invoke<{ path: string }>('download_url_to_workspace', { url });
}

export interface FilePreviewResult {
	columns: string[];
	rows: Record<string, unknown>[];
	totalRows: number;
	columnTypes: { name: string; type: string }[];
	workspacePath: string;
}

export async function previewFile(
	path: string,
	limit: number = 100
): Promise<FilePreviewResult> {
	return invoke<FilePreviewResult>('preview_file', { path, limit });
}

export async function saveTableLabels(
	tableName: string,
	tags: string,
	group: string | null
): Promise<void> {
	return invoke<void>('save_table_labels', { tableName, tags, group });
}

export async function getTableLabels(
	tableName: string
): Promise<TableLabels> {
	return invoke<TableLabels>('get_table_labels', { tableName });
}

export async function getAllTags(): Promise<string[]> {
	return invoke<string[]>('get_all_tags');
}

export async function getAllGroups(): Promise<string[]> {
	return invoke<string[]>('get_all_groups');
}

export async function listSavedQueries(): Promise<SavedQuery[]> {
	const result = await invoke<{ queries: SavedQuery[] }>('list_saved_queries');
	return result.queries;
}

export async function saveQuery(
	name: string,
	sql: string,
	description?: string,
	tags?: string
): Promise<void> {
	return invoke<void>('save_query', { name, sql, description: description ?? null, tags: tags ?? null });
}

export async function updateSavedQuery(
	slug: string,
	name?: string,
	sql?: string,
	description?: string,
	tags?: string
): Promise<void> {
	return invoke<void>('update_saved_query', {
		slug,
		name: name ?? null,
		sql: sql ?? null,
		description: description ?? null,
		tags: tags ?? null
	});
}

export async function deleteSavedQuery(slug: string): Promise<void> {
	return invoke<void>('delete_saved_query', { slug });
}

export async function getSettings(): Promise<Record<string, unknown>> {
	return invoke<Record<string, unknown>>('get_settings');
}

export async function saveSettings(settings: Record<string, unknown>): Promise<void> {
	return invoke<void>('save_settings', { settings });
}

export interface InternalTable {
	name: string;
	rowCount: number;
	columns: { name: string; type: string }[];
	isInternal: boolean;
}

export interface InternalTableData {
	columns: string[];
	rows: Record<string, unknown>[];
	totalRows: number;
	page: number;
	pageSize: number;
}

export async function listInternalTables(): Promise<InternalTable[]> {
	const result = await invoke<{ tables: InternalTable[] }>('list_internal_tables');
	return result.tables;
}

export async function queryInternalTable(
	tableName: string,
	page: number,
	pageSize: number
): Promise<InternalTableData> {
	return invoke<InternalTableData>('query_internal_table', { tableName, page, pageSize });
}

export async function updateInternalRow(
	tableName: string,
	pkColumn: string,
	pkValue: string,
	updates: Record<string, string>
): Promise<void> {
	return invoke<void>('update_internal_row', { tableName, pkColumn, pkValue, updates });
}

export async function deleteInternalRow(
	tableName: string,
	pkColumn: string,
	pkValue: string
): Promise<void> {
	return invoke<void>('delete_internal_row', { tableName, pkColumn, pkValue });
}

export async function initializeDataFolders(
	workspacePath: string
): Promise<void> {
	return invoke<void>('initialize_data_folders', { workspacePath });
}

export async function connectPostgres(url: string): Promise<string[]> {
	const result = await invoke<{ schemas: string[] }>('connect_postgres', { url });
	return result.schemas;
}

export async function listPostgresTables(schema: string): Promise<string[]> {
	const result = await invoke<{ tables: string[] }>('list_postgres_tables', { schema });
	return result.tables;
}

export interface PgIngestStatement {
	tableName: string;
	sql: string;
	sourcePath: string;
}

export async function generatePgInestSql(
	url: string,
	schema: string,
	tableNames: string[]
): Promise<PgIngestStatement[]> {
	const result = await invoke<{ statements: PgIngestStatement[] }>('generate_pg_ingest_sql', { url, schema, tableNames });
	return result.statements;
}

export function isTauriAvailable(): boolean {
	return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function runPagedQuery(
	sql: string,
	page: number = 1,
	pageSize: number = 100
): Promise<PagedQueryResult> {
	const trimmed = sql.trim().replace(/;+\s*$/, '');
	const lower = trimmed.toLowerCase();
	const isMutation =
		lower.startsWith('insert') ||
		lower.startsWith('update') ||
		lower.startsWith('delete') ||
		lower.startsWith('alter') ||
		lower.startsWith('drop') ||
		lower.startsWith('create');

	if (isMutation) {
		const result = await executeQuery(trimmed);
		if ('type' in result && result.type === 'dml') {
			return {
				columns: [],
				rows: [],
				totalRows: result.affectedRows,
				page: 1,
				pageSize,
				totalPages: 1,
				isMutation: true
			};
		}
		throw new Error('Unexpected result from DML query');
	}

	const usesFileFn = /read_csv_auto|read_parquet|read_json_auto/i.test(trimmed);
	const safePageSize = Math.min(Math.max(pageSize, 1), 50000);
	const offset = (page - 1) * safePageSize;

	if (usesFileFn) {
		const pagedSql = `${trimmed} LIMIT ${safePageSize} OFFSET ${offset}`;
		const result = await executeQuery(pagedSql);
		if (!('columns' in result)) {
			throw new Error('Unexpected result from SELECT query');
		}
		const columns = result.columns;
		const rows: Record<string, unknown>[] = result.data.map((row: unknown[]) => {
			const obj: Record<string, unknown> = {};
			columns.forEach((col, i) => {
				obj[col] = row[i];
			});
			return obj;
		});
		const totalRows = rows.length < safePageSize ? offset + rows.length : -1;
		const totalPages = totalRows >= 0 ? Math.max(1, Math.ceil(totalRows / safePageSize)) : -1;
		return { columns, rows, totalRows, page, pageSize: safePageSize, totalPages, isMutation: false };
	}

	const countSql = `SELECT COUNT(*) as cnt FROM (${trimmed}) as _q`;
	let totalRows = 0;
	try {
		const countResult = await executeQuery(countSql);
		if ('data' in countResult && countResult.data.length > 0) {
			totalRows = Number(countResult.data[0][0]);
		}
	} catch {
	}

	const pagedSql = `SELECT * FROM (${trimmed}) as _paged LIMIT ${safePageSize} OFFSET ${offset}`;

	const result = await executeQuery(pagedSql);
	if (!('columns' in result)) {
		throw new Error('Unexpected result from SELECT query');
	}

	const columns = result.columns;
	const rows: Record<string, unknown>[] = result.data.map((row: unknown[]) => {
		const obj: Record<string, unknown> = {};
		columns.forEach((col, i) => {
			obj[col] = row[i];
		});
		return obj;
	});

	const totalPages = Math.max(1, Math.ceil(totalRows / safePageSize));
	return { columns, rows, totalRows, page, pageSize: safePageSize, totalPages, isMutation: false };
}

export interface TableMeta {
	name: string;
	columnCount: number;
	rowCount: number;
	columns: ColumnInfo[];
}

export async function getTableMeta(tableName: string): Promise<TableMeta> {
	const schemaResult = await executeQuery(
		`SELECT column_name, column_type FROM (DESCRIBE SELECT * FROM "${tableName}")`
	);
	const columns: ColumnInfo[] =
		'data' in schemaResult
			? schemaResult.data.map((row: unknown[]) => ({
					name: String(row[0]),
					type: String(row[1])
				}))
			: [];

	const countResult = await executeQuery(
		`SELECT COUNT(*) as cnt FROM "${tableName}"`
	);
	const rowCount =
		'data' in countResult && countResult.data.length > 0
			? Number(countResult.data[0][0])
			: 0;

	return { name: tableName, columnCount: columns.length, rowCount, columns };
}

export async function getAllTableMeta(): Promise<TableMeta[]> {
	const tableNames = await listTables();
	const metas: TableMeta[] = [];
	for (const name of tableNames) {
		metas.push(await getTableMeta(name));
	}
	return metas;
}
