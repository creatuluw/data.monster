import {
	initializeDuckdb,
	getWorkspacePath,
	chooseWorkspaceFolder,
	executeQuery,
	listTables,
	getFileColumns,
	loadCsvFile,
	loadParquetFile,
	loadJsonFile,
	type PagedQueryResult,
	type ColumnInfo,
	isTauriAvailable
} from '$lib/db-operations';

class AppState {
	dbReady = $state(false);
	workspacePath = $state<string | null>(null);
	tables = $state<string[]>([]);
	globalError = $state('');

	pendingSql = $state('');
	pendingAutoRun = $state(false);

	async init() {
		if (!isTauriAvailable()) {
			this.globalError = 'This app requires the Tauri desktop runtime.';
			return;
		}

		try {
			const workspace = await getWorkspacePath();
			if (workspace) {
				this.workspacePath = workspace;
				await initializeDuckdb(workspace);
				this.tables = await listTables();
				this.dbReady = true;
			}
		} catch (e) {
			this.globalError = e instanceof Error ? e.message : 'Failed to initialize database';
		}
	}

	async selectWorkspace(): Promise<boolean> {
		try {
			const path = await chooseWorkspaceFolder();
			if (!path) return false;

			await initializeDuckdb(path);
			this.workspacePath = path;
			this.tables = await listTables();
			this.dbReady = true;
			return true;
		} catch (e) {
			this.globalError = e instanceof Error ? e.message : 'Failed to set workspace';
			return false;
		}
	}

	async refreshTables() {
		this.tables = await listTables();
	}

	async connectUrl(url: string) {
		const ext = url.split('.').pop()?.split('?')[0]?.toLowerCase() ?? 'csv';
		const reader =
			ext === 'parquet'
				? `read_parquet('${url}')`
				: ext === 'json' || ext === 'jsonl'
					? `read_json_auto('${url}')`
					: `read_csv_auto('${url}')`;

		const tableName = `url_${Date.now()}`;
		const createSql = `CREATE TABLE "${tableName}" AS SELECT * FROM ${reader}`;
		await executeQuery(createSql);
		this.tables = await listTables();
	}
}

export const app = new AppState();
