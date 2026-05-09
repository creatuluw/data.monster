import { runPagedQuery, type PagedQueryResult, executeQuery, getSettings, generateChat, stopGeneration, loadModel, unloadModel } from '$lib/db-operations';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

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

export interface PlanOption {
	id: string;
	label: string;
	description: string;
}

export interface AnalysisPlan {
	messageId: string;
	options: PlanOption[];
	userQuestion: string;
}

class AnalystState {
	selectedTables = $state<string[]>([]);
	messages = $state<ChatMessage[]>([]);
	queries = $state<QueryEntry[]>([]);
	streaming = $state(false);
	streamingContent = $state('');
	activeQueryId = $state<string | null>(null);
	pendingPlan = $state<AnalysisPlan | null>(null);
	selectedPlanOptions = $state<Set<string>>(new Set());
	customPlanOption = $state('');

	private abortController: AbortController | null = null;
	private extractedIndex = 0;
	private pendingMessageId = '';

	apiUrl = $state('');
	apiKey = $state('');
	apiModel = $state('');
	inferenceMode = $state<'remote' | 'local'>('remote');
	localModelId = $state<string | null>(null);

	private initialized = false;
	private tokenUnlisten: UnlistenFn | null = null;
	private doneUnlisten: UnlistenFn | null = null;
	private errorUnlisten: UnlistenFn | null = null;
	private generationResolve: (() => void) | null = null;
	private lastGenerationTime = 0;
	private inactivityTimer: ReturnType<typeof setInterval> | null = null;

	async init() {
		if (this.initialized) return;
		this.initialized = true;
		try {
			const settings = await getSettings();
			if (settings.llmApiUrl) this.apiUrl = settings.llmApiUrl as string;
			else this.apiUrl = 'https://api.z.ai/api/coding/paas/v4/chat/completions';
			if (settings.llmApiKey) this.apiKey = settings.llmApiKey as string;
			if (settings.llmModel) this.apiModel = settings.llmModel as string;
			else this.apiModel = 'glm-5';
			this.inferenceMode = (settings.inferenceMode as string) === 'local' ? 'local' : 'remote';
			this.localModelId = (settings.localModelId as string) || null;

			if (this.inferenceMode === 'local' && this.localModelId) {
				try { await loadModel(this.localModelId); } catch { /* model may not be downloaded yet */ }
			}
		} catch {
			this.apiUrl = 'https://api.z.ai/api/coding/paas/v4/chat/completions';
			this.apiModel = 'glm-5';
		}

		this.startInactivityTimer();
	}

	private startInactivityTimer() {
		if (this.inactivityTimer) return;
		this.inactivityTimer = setInterval(() => {
			if (this.inferenceMode === 'local' && this.lastGenerationTime > 0) {
				const elapsed = Date.now() - this.lastGenerationTime;
				if (elapsed > 5 * 60 * 1000 && !this.streaming) {
					unloadModel().catch(() => {});
					this.lastGenerationTime = 0;
				}
			}
		}, 30000);
	}

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

	stop() {
		if (this.inferenceMode === 'local') {
			stopGeneration();
		}
		if (this.abortController) {
			this.abortController.abort();
		}
		this.cleanupListeners();
		if (this.generationResolve) {
			this.generationResolve();
			this.generationResolve = null;
		}
	}

	private cleanupListeners() {
		if (this.tokenUnlisten) { this.tokenUnlisten(); this.tokenUnlisten = null; }
		if (this.doneUnlisten) { this.doneUnlisten(); this.doneUnlisten = null; }
		if (this.errorUnlisten) { this.errorUnlisten(); this.errorUnlisten = null; }
	}

	togglePlanOption(optionId: string) {
		const next = new Set(this.selectedPlanOptions);
		if (next.has(optionId)) {
			next.delete(optionId);
		} else {
			next.add(optionId);
		}
		this.selectedPlanOptions = next;
	}

	selectAllPlanOptions() {
		if (!this.pendingPlan) return;
		this.selectedPlanOptions = new Set(this.pendingPlan.options.map((o) => o.id));
	}

	deselectAllPlanOptions() {
		this.selectedPlanOptions = new Set();
	}

	async proceedWithPlan() {
		if (!this.pendingPlan) return;

		const selected = this.pendingPlan.options
			.filter((o) => this.selectedPlanOptions.has(o.id))
			.map((o) => `- **${o.label}**: ${o.description}`);

		const custom = this.customPlanOption.trim();

		let instruction = `Proceed with the following analyses:\n${selected.join('\n')}`;
		if (custom) {
			instruction += `\n\nAdditionally, analyze this: ${custom}`;
		}
		instruction += `\n\nDo NOT present another plan. Go straight into the Question → Query → Analysis → Conclusion format for each selected analysis.`;

		this.pendingPlan = null;
		this.selectedPlanOptions = new Set();
		this.customPlanOption = '';
		await this.sendMessage(instruction);
	}

	private parseAndSetPlan(content: string, messageId: string) {
		const match = content.match(/<!-- plan:json\s*([\s\S]*?)\s*-->/);
		if (!match) return;

		try {
			const raw = JSON.parse(match[1]);
			if (!Array.isArray(raw) || raw.length < 3) return;

			const userMsg = [...this.messages].reverse().find((m) => m.role === 'user');
			const options: PlanOption[] = raw.map((item: { label?: string; description?: string }, i: number) => ({
				id: `plan-${i}`,
				label: item.label ?? `Option ${i + 1}`,
				description: item.description ?? ''
			}));

			this.pendingPlan = {
				messageId,
				options,
				userQuestion: userMsg?.content ?? ''
			};
			this.selectedPlanOptions = new Set(options.map((o) => o.id));
		} catch {
			// invalid JSON, ignore
		}
	}

	clear() {
		this.messages = [];
		this.queries = [];
		this.streamingContent = '';
		this.activeQueryId = null;
		this.extractedIndex = 0;
		this.pendingPlan = null;
		this.selectedPlanOptions = new Set();
		this.customPlanOption = '';
	}

	addUserMessage(content: string) {
		this.messages = [
			...this.messages,
			{ id: crypto.randomUUID(), role: 'user', content, timestamp: Date.now() }
		];
	}

	async getTableSchemas(): Promise<string> {
		const schemas: string[] = [];

		for (const tableName of this.selectedTables) {
			try {
				const descResult = await executeQuery(`DESCRIBE "${tableName}"`);
				const columns = 'data' in descResult
					? descResult.data.map((row: unknown[]) => `  ${row[0]} ${row[1]}`)
					: [];

				const countResult = await executeQuery(`SELECT COUNT(*) as cnt FROM "${tableName}"`);
				const rowCount = 'data' in countResult && countResult.data.length > 0
					? Number(countResult.data[0][0])
					: 0;

				const sampleResult = await executeQuery(`SELECT * FROM "${tableName}" LIMIT 3`);
				const sampleCols = 'columns' in sampleResult ? sampleResult.columns : [];
				const sampleRows = 'data' in sampleResult
					? sampleResult.data.map((row: unknown[]) =>
						row.map((v: unknown) => String(v ?? 'NULL')).join(' | ')
					)
					: [];

				let info = `Table "${tableName}" (${rowCount.toLocaleString()} rows):\n${columns.join('\n')}`;
				if (sampleRows.length > 0) {
					info += `\n\nSample data:\n  ${sampleCols.join(' | ')}\n  ${sampleRows.join('\n  ')}`;
				}
				schemas.push(info);
			} catch {
				schemas.push(`Table "${tableName}": (error reading schema)`);
			}
		}

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

### Question [QN]
Restate the specific question from the user that this step is answering. Use an incrementing number starting at 1 (e.g. "### Question [Q1]", "### Question [Q2]") so it matches the query tabs in the editor panel.

### Query
Present the SQL in a \`\`\`sql code block.

### Analysis
Describe what the query found and key observations from the results.

### Conclusion
Summarize the key insight or finding from this step.

**IMPORTANT — Planning for complex questions:**
If the user's question requires 3 or more distinct analyses/queries, you MUST first present an analysis plan using this exact format BEFORE writing any SQL:

<!-- plan:json [{"label": "Short label", "description": "What this analysis investigates"}, {"label": "Another label", "description": "What this other analysis investigates"}, ...] -->

After the plan block, briefly explain each option to the user in plain text. Do NOT include any SQL queries yet. The user will select which analyses to proceed with.

For questions that need only 1 or 2 queries, skip the plan and go directly into the Question → Query → Analysis → Conclusion format.

When running multiple queries, repeat this Question → Query → Analysis → Conclusion pattern for each one.
After all analysis steps, provide a brief overall summary of findings.
Be direct, insightful, and focus on what the data tells us.
Always generate at least one SQL query per user message.`;
	}

	async sendMessage(content: string) {
		if (this.streaming) {
			this.stop();
			await new Promise((r) => setTimeout(r, 100));
		}

		await this.init();

		this.addUserMessage(content);
		this.streaming = true;
		this.streamingContent = '';
		this.extractedIndex = 0;
		this.pendingMessageId = crypto.randomUUID();

		const schemas = await this.getTableSchemas();
		const systemPrompt = this.buildSystemPrompt(schemas);

		if (this.inferenceMode === 'local') {
			await this.sendLocal(systemPrompt);
		} else {
			await this.sendRemote(systemPrompt);
		}
	}

	private async sendRemote(systemPrompt: string) {
		const apiMessages = [
			{ role: 'system' as const, content: systemPrompt },
			...this.messages.map((m) => ({ role: m.role, content: m.content }))
		];

		this.abortController = new AbortController();

		try {
			const response = await fetch(this.apiUrl, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${this.apiKey}`
				},
				body: JSON.stringify({
					model: this.apiModel,
					messages: apiMessages,
					stream: true,
					temperature: 0.6
				}),
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
				this.parseAndSetPlan(this.streamingContent, this.pendingMessageId);
			}
		} catch (e) {
			if ((e as Error).name === 'AbortError') {
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
					this.parseAndSetPlan(this.streamingContent, this.pendingMessageId);
				}
			} else {
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

	private async sendLocal(systemPrompt: string) {
		this.cleanupListeners();
		this.lastGenerationTime = Date.now();

		const apiMessages = this.messages.map((m) => ({ role: m.role, content: m.content }));

		return new Promise<void>((resolve) => {
			this.generationResolve = resolve;

			listen<{ token: string }>('local-llm:token', (event) => {
				this.streamingContent += event.payload.token;
				this.extractAndExecuteSql();
			}).then((unlisten) => { this.tokenUnlisten = unlisten; });

			listen('local-llm:done', () => {
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
					this.parseAndSetPlan(this.streamingContent, this.pendingMessageId);
				}
				this.streaming = false;
				this.streamingContent = '';
			}).then((unlisten) => { this.doneUnlisten = unlisten; });

			listen<{ error: string }>('local-llm:error', (event) => {
				this.messages = [
					...this.messages,
					{
						id: this.pendingMessageId,
						role: 'assistant',
						content: `Error: ${event.payload.error}`,
						timestamp: Date.now()
					}
				];
				this.streaming = false;
				this.streamingContent = '';
			}).then((unlisten) => { this.errorUnlisten = unlisten; });

			generateChat(apiMessages, systemPrompt).catch((e) => {
				this.messages = [
					...this.messages,
					{
						id: this.pendingMessageId,
						role: 'assistant',
						content: `Error: ${e}`,
						timestamp: Date.now()
					}
				];
				this.streaming = false;
				this.streamingContent = '';
			});
		});
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
				const result = await runPagedQuery(sql, 1, 100);
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
			const result = await runPagedQuery(newSql, 1, 100);
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
