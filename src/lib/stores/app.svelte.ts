import {
	initializeDuckdb,
	shutdownDuckdb,
	getWorkspacePath,
	chooseWorkspaceFolder,
	listTables,
	isTauriAvailable,
	extractErrorMessage
} from '$lib/db-operations';

class AppState {
	dbReady = $state(false);
	workspacePath = $state<string | null>(null);
	tables = $state<string[]>([]);
	globalError = $state('');

	pendingSql = $state('');

	pendingFile = $state<{ path: string; tableName: string; originalSource?: string; sourceType?: string } | null>(null);
	pendingPreviewData = $state<{ tableName: string; columns: string[]; sourcePath: string; sourceType?: string; originalSource?: string } | null>(null);
	pendingBatchIngest = $state<{ tableName: string; sql: string; sourcePath: string; sourceType?: string; originalSource?: string }[] | null>(null);

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
			this.globalError = extractErrorMessage(e, 'Failed to initialize database');
		}
	}

	async shutdown() {
		if (!this.dbReady) return;
		try {
			await shutdownDuckdb();
		} catch (e) {
			console.warn('Shutdown warning:', extractErrorMessage(e, ''));
		}
		this.dbReady = false;
		this.tables = [];
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
			this.globalError = extractErrorMessage(e, 'Failed to set workspace');
			return false;
		}
	}

	async refreshTables() {
		this.tables = await listTables();
	}
}

export const app = new AppState();
