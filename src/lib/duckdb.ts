import duckdb_wasm_eh from '@duckdb/duckdb-wasm/dist/duckdb-eh.wasm?url';
import eh_worker from '@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url';
import duckdb_wasm_mvp from '@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url';
import mvp_worker from '@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url';
import type { AsyncDuckDB, DuckDBBundles } from '@duckdb/duckdb-wasm';
import { browser } from '$app/environment';
import { saveTable, loadTable, listSavedTables, deleteSavedTable as deletePersistedTable, clearAllSavedTables } from './persistence';

let db: AsyncDuckDB | null = null;
let initPromise: Promise<AsyncDuckDB> | null = null;
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

export async function getDb(): Promise<AsyncDuckDB> {
	if (db) return db;
	if (initPromise) return initPromise;

	initPromise = (async () => {
		if (!browser) throw new Error('DuckDB only available in browser');

		const duckdb = await import('@duckdb/duckdb-wasm');

		const BUNDLES: DuckDBBundles = {
			eh: {
				mainModule: duckdb_wasm_eh,
				mainWorker: eh_worker
			},
			mvp: {
				mainModule: duckdb_wasm_mvp,
				mainWorker: mvp_worker
			}
		};

		const bundle = await duckdb.selectBundle(BUNDLES);
		const worker = new Worker(bundle.mainWorker!);
		const logger = new duckdb.VoidLogger();
		db = new duckdb.AsyncDuckDB(logger, worker);
		await db.instantiate(bundle.mainModule, bundle.pthreadWorker);

		await db.open({
			path: ':memory:',
			accessMode: duckdb.DuckDBAccessMode.READ_WRITE
		});

		// Restore all saved tables from IndexedDB (non-fatal)
		try {
			const savedTables = await listSavedTables();
			console.log(`[restore] Found ${savedTables.length} saved tables in IndexedDB`);
			if (savedTables.length > 0) {
				const conn = await db.connect();
				for (const tableName of savedTables) {
					const buffer = await loadTable(tableName);
					if (buffer && buffer.byteLength > 0) {
						try {
							const fileName = `__restore_${tableName}.parquet`;
							await db.registerFileBuffer(fileName, new Uint8Array(buffer));
							await conn.query(
								`CREATE TABLE IF NOT EXISTS "${tableName}" AS SELECT * FROM read_parquet('${fileName}')`
							);
							console.log(`[restore] Restored table "${tableName}" (${buffer.byteLength} bytes)`);
						} catch (e) {
							console.error(`[restore] FAILED for table "${tableName}":`, e);
							await deletePersistedTable(tableName);
						}
					}
				}
				conn.close();
			}
		} catch (e) {
			console.error('[restore] IndexedDB restore failed, starting fresh:', e);
		}

		return db;
	})();

	return initPromise;
}

// Persist a single table to IndexedDB as Parquet
export async function persistTable(tableName: string): Promise<void> {
	if (!db) return;
	try {
		const conn = await db.connect();

		// Export to parquet in DuckDB's virtual filesystem
		const exportPath = `__persist_${tableName}.parquet`;
		await conn.query(`COPY "${tableName}" TO '${exportPath}' (FORMAT PARQUET)`);
		conn.close();

		// Extract the buffer and save to IndexedDB
		const buffer = await db.copyFileToBuffer(exportPath);
		console.log(`[persist] Saved table "${tableName}" to IndexedDB (${buffer.byteLength} bytes)`);
		await saveTable(tableName, new Uint8Array(buffer));
	} catch (e) {
		console.error(`[persist] FAILED for table "${tableName}":`, e);
	}
}

// Persist all current tables
export async function persistAllTables(): Promise<void> {
	if (!db) return;
	try {
		const conn = await db.connect();
		const result = await conn.query(
			"SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name"
		);
		conn.close();

		const tables = result.toArray().map((r: any) => r.table_name as string);
		await Promise.all(tables.map((t) => persistTable(t)));
	} catch (e) {
		console.warn('Failed to persist all tables:', e);
	}
}

// Debounced persist — coalesces multiple calls within 500ms
let pendingPersistTables = new Set<string>();
let persistAll = false;

export function schedulePersist(tableName?: string): void {
	if (tableName) {
		pendingPersistTables.add(tableName);
	} else {
		persistAll = true;
	}
	if (saveTimeout) clearTimeout(saveTimeout);
	saveTimeout = setTimeout(async () => {
		if (persistAll) {
			await persistAllTables();
		} else {
			for (const t of pendingPersistTables) {
				await persistTable(t);
			}
		}
		pendingPersistTables.clear();
		persistAll = false;
	}, 500);
}

export function isOpfsSupported(): boolean {
	if (!browser) return false;
	return 'storage' in navigator && typeof navigator.storage.getDirectory === 'function';
}

export async function clearAllData(): Promise<void> {
	if (db) {
		await db.terminate();
		db = null;
		initPromise = null;
	}
	await clearAllSavedTables();
}
