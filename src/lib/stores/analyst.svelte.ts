import { getDb } from '$lib/duckdb';
import { runQuery, type PagedQueryResult } from '$lib/db-helpers';

export interface ChatMessage {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	timestamp: number;
}

export interface QueryEntry {
	id: string;
	sql: string;
	result: PagedQueryResult | null;
	error: string | null;
	queryTime: number;
	timestamp: number;
	messageRefId: string;
}

class AnalystState {
	selectedTables = $state<string[]>([]);
	messages = $state<ChatMessage[]>([]);
	queries = $state<QueryEntry[]>([]);
	streaming = $state(false);
	streamingContent = $state('');
	activeQueryId = $state<string | null>(null);

	private abortController: AbortController | null = null;
	private extractedIndex = 0;
	private pendingMessageId = '';

	get currentQuery(): QueryEntry | null {
		if (this.activeQueryId) {
			return this.queries.find((q) => q.id === this.activeQueryId) ?? null;
		}
		if (this.queries.length > 0) {
			return this.queries[this.queries.length - 1];
		}
		return null;
	}

	setActiveQuery(id: string) {
		this.activeQueryId = id;
	}

	clear() {
		this.messages = [];
		this.queries = [];
		this.streamingContent = '';
		this.activeQueryId = null;
		this.extractedIndex = 0;
	}

	addUserMessage(content: string) {
		this.messages = [
			...this.messages,
			{ id: crypto.randomUUID(), role: 'user', content, timestamp: Date.now() }
		];
	}

	async getTableSchemas(): Promise<string> {
		const db = await getDb();
		const conn = await db.connect();
		const schemas: string[] = [];

		for (const tableName of this.selectedTables) {
			try {
				const descResult = await conn.query(`DESCRIBE "${tableName}"`);
				const columns = descResult
					.toArray()
					.map((row: any) => `  ${row.column_name} ${row.column_type}`);

				const countResult = await conn.query(`SELECT COUNT(*) as cnt FROM "${tableName}"`);
				const rowCount = Number(countResult.toArray()[0].cnt);

				const sampleResult = await conn.query(`SELECT * FROM "${tableName}" LIMIT 3`);
				const sampleCols = sampleResult.schema.fields.map((f: any) => f.name);
				const sampleRows = sampleResult
					.toArray()
					.map((row: any) =>
						sampleCols.map((col: string) => String(row[col] ?? 'NULL')).join(' | ')
					);

				let info = `Table "${tableName}" (${rowCount.toLocaleString()} rows):\n${columns.join('\n')}`;
				if (sampleRows.length > 0) {
					info += `\n\nSample data:\n  ${sampleCols.join(' | ')}\n  ${sampleRows.join('\n  ')}`;
				}
				schemas.push(info);
			} catch {
				schemas.push(`Table "${tableName}": (error reading schema)`);
			}
		}

		conn.close();
		return schemas.join('\n\n');
	}

	buildSystemPrompt(schemas: string): string {
		return `You are a Flue-powered data analyst agent. You analyze data using SQL queries against a DuckDB in-memory database.

Available tables:

${schemas}

Instructions:
- Analyze the user's questions by writing SQL queries against the available tables
- ALWAYS wrap SQL queries in \`\`\`sql code blocks — they will be auto-executed against DuckDB
- Use DuckDB-compatible SQL syntax
- If a query fails, try to fix it and re-run
- Do NOT end SQL statements with a semicolon

Response format — for EACH analysis step, follow this exact structure:

### Query
Present the SQL in a \`\`\`sql code block.

### Analysis
Describe what the query found and key observations from the results.

### Conclusion
Summarize the key insight or finding from this step.

When running multiple queries, repeat this Query → Analysis → Conclusion pattern for each one.
After all analysis steps, provide a brief overall summary of findings.
Be direct, insightful, and focus on what the data tells us.
Always generate at least one SQL query per user message.`;
	}

	async sendMessage(content: string) {
		if (this.streaming && this.abortController) {
			this.abortController.abort();
		}

		this.addUserMessage(content);
		this.streaming = true;
		this.streamingContent = '';
		this.extractedIndex = 0;
		this.pendingMessageId = crypto.randomUUID();

		const schemas = await this.getTableSchemas();
		const systemPrompt = this.buildSystemPrompt(schemas);

		const apiMessages = [
			{ role: 'system' as const, content: systemPrompt },
			...this.messages.map((m) => ({ role: m.role, content: m.content }))
		];

		this.abortController = new AbortController();

		try {
			const response = await fetch('/api/analyst', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ messages: apiMessages }),
				signal: this.abortController.signal
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(errorText || 'API request failed');
			}

			if (!response.body) throw new Error('No response body');

			const reader = response.body.getReader();
			const decoder = new TextDecoder();
			let buffer = '';

			while (true) {
				const { done, value } = await reader.read();
				if (done) break;

				buffer += decoder.decode(value, { stream: true });
				const lines = buffer.split('\n');
				buffer = lines.pop() || '';

				for (const line of lines) {
					const trimmed = line.trim();
					if (!trimmed || !trimmed.startsWith('data: ')) continue;
					const data = trimmed.slice(6);
					if (data === '[DONE]') break;

					try {
						const parsed = JSON.parse(data);
						const delta = parsed.choices?.[0]?.delta?.content || '';
						if (delta) {
							this.streamingContent += delta;
							await this.extractAndExecuteSql();
						}
					} catch {
						// skip malformed chunks
					}
				}
			}

			if (this.streamingContent) {
				this.messages = [
					...this.messages,
					{
						id: this.pendingMessageId,
						role: 'assistant',
						content: this.streamingContent,
						timestamp: Date.now()
					}
				];
			}
		} catch (e) {
			if ((e as Error).name !== 'AbortError') {
				this.messages = [
					...this.messages,
					{
						id: this.pendingMessageId,
						role: 'assistant',
						content: `Error: ${(e as Error).message}`,
						timestamp: Date.now()
					}
				];
			}
		} finally {
			this.streaming = false;
			this.streamingContent = '';
			this.abortController = null;
		}
	}

	private async extractAndExecuteSql() {
		const content = this.streamingContent;
		const regex = /```sql\s*\n([\s\S]*?)```/g;
		regex.lastIndex = this.extractedIndex;

		let match;
		while ((match = regex.exec(content)) !== null) {
			const sql = match[1].trim();
			this.extractedIndex = regex.lastIndex;

			const queryId = crypto.randomUUID();
			const t0 = performance.now();
			try {
				const result = await runQuery(sql, 1, 100);
				const queryTime = performance.now() - t0;
				const entry: QueryEntry = {
					id: queryId,
					sql,
					result,
					error: null,
					queryTime,
					timestamp: Date.now(),
					messageRefId: this.pendingMessageId
				};
				this.queries = [...this.queries, entry];
				this.activeQueryId = queryId;
			} catch (e) {
				const queryTime = performance.now() - t0;
				const entry: QueryEntry = {
					id: queryId,
					sql,
					result: null,
					error: (e as Error).message,
					queryTime,
					timestamp: Date.now(),
					messageRefId: this.pendingMessageId
				};
				this.queries = [...this.queries, entry];
				this.activeQueryId = queryId;
			}
		}
	}

	async rerunQuery(queryId: string, newSql: string) {
		const idx = this.queries.findIndex((q) => q.id === queryId);
		if (idx === -1) return;

		const t0 = performance.now();
		try {
			const result = await runQuery(newSql, 1, 100);
			const queryTime = performance.now() - t0;
			const updated: QueryEntry = {
				...this.queries[idx],
				sql: newSql,
				result,
				error: null,
				queryTime,
				timestamp: Date.now()
			};
			const newQueries = [...this.queries];
			newQueries[idx] = updated;
			this.queries = newQueries;
		} catch (e) {
			const queryTime = performance.now() - t0;
			const updated: QueryEntry = {
				...this.queries[idx],
				sql: newSql,
				result: null,
				error: (e as Error).message,
				queryTime,
				timestamp: Date.now()
			};
			const newQueries = [...this.queries];
			newQueries[idx] = updated;
			this.queries = newQueries;
		}
	}
}

export const analyst = new AnalystState();
