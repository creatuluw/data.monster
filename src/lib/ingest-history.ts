import { getDb } from './duckdb';
import { DuckDBDataProtocol } from '@duckdb/duckdb-wasm';
import {
	getRecipe,
	getAllRecipes,
	clearAllRecipes
} from './recipes';

const INGEST_HISTORY_KEY = 'data-monster-ingest-history';

export interface IngestRecord {
	tableName: string;
	sql: string;
	createdAt: string;
}

function loadHistory(): IngestRecord[] {
	try {
		const raw = localStorage.getItem(INGEST_HISTORY_KEY);
		return raw ? JSON.parse(raw) : [];
	} catch {
		return [];
	}
}

function saveHistory(records: IngestRecord[]) {
	localStorage.setItem(INGEST_HISTORY_KEY, JSON.stringify(records));
}

export function addIngestRecord(record: IngestRecord) {
	const history = loadHistory();
	// Remove any existing record for the same table
	const filtered = history.filter((r) => r.tableName !== record.tableName);
	filtered.push(record);
	saveHistory(filtered);
}

export function removeIngestRecord(tableName: string) {
	saveHistory(loadHistory().filter((r) => r.tableName !== tableName));
}

export function clearIngestHistory() {
	localStorage.removeItem(INGEST_HISTORY_KEY);
}

// Re-play all ingest records to rebuild tables after reload.
// This is the fallback if OPFS persistence doesn't survive.
export async function replayIngestHistory(): Promise<string[]> {
	const db = await getDb();
	const conn = await db.connect();

	// Get existing tables
	const result = await conn.query(
		"SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name"
	);
	const existingTables = new Set(
		result.toArray().map((r: any) => r.table_name as string)
	);

	const history = loadHistory();
	const rebuilt: string[] = [];

	for (const record of history) {
		// Only re-create if the table doesn't already exist (OPFS may have it)
		if (!existingTables.has(record.tableName)) {
			try {
				await conn.query(`CREATE TABLE IF NOT EXISTS "${record.tableName}" AS ${record.sql}`);
				rebuilt.push(record.tableName);
			} catch (e) {
				console.warn(`Failed to rebuild table ${record.tableName}:`, e);
				// Remove broken record
				removeIngestRecord(record.tableName);
			}
		} else {
			rebuilt.push(record.tableName);
		}
	}

	if (rebuilt.length > 0) {
		await conn.query('CHECKPOINT');
	}

	conn.close();

	// Return all current tables
	const finalResult = await conn.query(
		"SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name"
	);
	return finalResult.toArray().map((r: any) => r.table_name as string);
}
