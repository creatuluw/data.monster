import { getDb, isOpfsSupported } from '$lib/duckdb';
import {
	previewFile,
	previewUrl,
	listTables,
	type PreviewData,
	type ColumnOverride
} from '$lib/db-helpers';

class AppState {
	dbReady = $state(false);
	opfsStatus = $state<'checking' | 'persistent' | 'memory'>('checking');
	tables = $state<string[]>([]);
	globalError = $state('');

	previewData = $state<PreviewData | null>(null);
	previewFileName = $state('');
	previewSourceType = $state<'file' | 'url'>('file');
	previewSourceName = $state('');
	columnOverrides = $state<ColumnOverride[]>([]);
	ingestTableName = $state('');

	pendingSql = $state('');
	pendingAutoRun = $state(false);

	async init() {
		try {
			await getDb();
			this.opfsStatus = isOpfsSupported() ? 'persistent' : 'memory';
			this.tables = await listTables();
			this.dbReady = true;
		} catch (e) {
			this.globalError = e instanceof Error ? e.message : 'Failed to initialize database';
		}
	}

	async refreshTables() {
		this.tables = await listTables();
	}

	async connectFile(file: File) {
		this.previewData = await previewFile(file);
		this.previewFileName = file.name;
		this.previewSourceType = 'file';
		this.previewSourceName = file.name;
		this.ingestTableName = file.name.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');
		this.columnOverrides = this.previewData.detectedTypes.map((col) => ({
			originalName: col.name,
			newName: col.name,
			detectedType: col.type,
			overrideType: null,
			enabled: true
		}));
	}

	async connectUrl(url: string) {
		this.previewData = await previewUrl(url);
		this.previewSourceType = 'url';
		this.previewSourceName = url;
		const fileName = url.split('/').pop()?.split('?')[0] ?? 'remote_data';
		this.previewFileName = fileName;
		this.ingestTableName = fileName.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');
		this.columnOverrides = this.previewData.detectedTypes.map((col) => ({
			originalName: col.name,
			newName: col.name,
			detectedType: col.type,
			overrideType: null,
			enabled: true
		}));
	}

	buildPreviewSql(): string {
		if (!this.previewData) return '';
		const enabledColumns = this.columnOverrides.filter((c) => c.enabled);
		const columns = enabledColumns.map((c) => {
			const typeChanged = c.overrideType !== null && c.overrideType !== c.detectedType;
			const name = typeChanged
				? `"${c.originalName}"::${c.overrideType}`
				: `"${c.originalName}"`;
			return `${name} AS "${c.newName}"`;
		}).join(',\n       ');

		const ext = this.previewFileName.split('.').pop()?.toLowerCase() ?? 'csv';
		const reader =
			ext === 'parquet'
				? `read_parquet('${this.previewFileName}')`
				: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
					? `read_json_auto('${this.previewFileName}')`
					: `read_csv_auto('${this.previewFileName}')`;

		this.ingestTableName = this.previewSourceName.replace(/\.[^.]+$/, '').replace(/[^a-zA-Z0-9_]/g, '_');

		return `SELECT\n       ${columns}\nFROM ${reader}`;
	}
}

export const app = new AppState();
