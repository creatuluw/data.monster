# How can we connect any LLM / coding agent / harness to data.monster and let it operate the app — ingest data, author content, run analysis — solidly and safely?

## Executive summary

**Ship an MCP server inside data.monster's Rust backend, served as Streamable HTTP on 127.0.0.1 from the already-running app, built on the official Tier-1 Rust SDK (rmcp).** MCP is the settled standard — every major harness (Claude Code, Cursor, VS Code, Codex, opencode, JetBrains, Gemini CLI) supports it as a client, the Rust SDK is conformance-complete, and the exact "agent connects to a running desktop app over loopback HTTP" pattern has production precedent (Figma Dev Mode MCP Server). data.monster is unusually well positioned: the Rust backend already owns every operation an agent needs (ingest, SQL, tables, saved queries, pages) as per-domain command modules — the MCP tool layer is a thin wrapper over code that exists. Everything else surveyed (OpenAI plugins, A2A, Copilot extensions, computer use) is either cloud-only, orthogonal, or a last resort. The real design work is the tool catalog and the safety model: read-only by default, opt-in write, elicitation confirmations for destructive ops, audit log, transactions, and checkpoints.

## Answer

### 1. The standard is MCP — bet on it, and bet on Streamable HTTP loopback, not stdio

MCP has consolidated as *the* agent↔app protocol (spec chain 2024-11-05 → 2025-03-26 → 2025-06-18 → 2025-11-25 → **2026-07-28 current**, all changelog-verified). ~500M SDK downloads/month; official registry frozen at v0.1; TS/Python/C#/Go/**Rust** SDKs are Tier 1. Every harness the team uses — Claude Code, Cursor, VS Code, Codex, opencode — consumes both stdio and HTTP MCP servers today.

For a **desktop app that is already running**, the transport decision is forced:

- **stdio requires the agent to spawn the server binary.** It cannot attach to a running app. stdio is only viable as a thin shim that proxies to the running app (the Obsidian Local-REST-API + wrapper pattern).
- **Streamable HTTP on 127.0.0.1 served in-process is the right architecture** — the Rust process that owns the DuckDB connection *is* the MCP server. Single process, no DB contention, agents connect to the live app. This is exactly what Figma's embedded Dev Mode MCP server does (`127.0.0.1:3845`).
- **A separate server process sharing the DuckDB file is blocked**: DuckDB allows a single read-write process per file (verified from the DuckDB FAQ).

rmcp (official Rust SDK) supports this directly: its Streamable-HTTP server is a **Tower service that nests into any axum router**, alongside `#[tool]`/`#[tool_handler]` macros. It targets spec revisions 2025-11-25 and 2026-07-28. Caveat: pin the version carefully — crates.io README examples lag the 3.x Tier-1 release (tension below).

There is also a community `tauri-plugin-mcp-bridge` (~50k downloads/mo) doing WebSocket bridging into a running Tauri app — but it's community-maintained with frequent breaking releases; rolling our own axum mount is small and avoids the dependency.

### 2. The architecture, concretely (what data.monster should build)

1. In the Tauri `setup` hook, start an axum server bound to **127.0.0.1** with the rmcp Streamable-HTTP service nested at `/mcp`.
2. **Auth**: loopback-only binding + a bearer token generated at startup (loopback is the OAuth 2.1 §1.5 carve-out; Obsidian's Local REST API uses API-key auth the same way). Write port+token to a well-known file (e.g. `%USERPROFILE%/.data-monster/mcp.json`) that agents' configs reference — dynamic ports without config drift.
3. **Tools are thin wrappers over existing Rust command modules** — `queries`, `files`, `tables`, `saved_queries`, `pages` (PageDoc storage), `postgres` — the same functions the Tauri invoke commands call. No second implementation, no drift.
4. **UI refresh**: after mutating tools, emit Tauri events so the frontend reloads tables/pages — the agent acts, the human sees it live. A `get_app_status`/`notify` tool covers long jobs.
5. Keeping everything in the one signed main exe avoids Windows code-signing a sidecar binary (medium-confidence reasoning from the Tauri sidecar model).

Config snippets for the big harnesses are one-liners (`{"mcpServers": {"data-monster": {"url": "http://127.0.0.1:<port>/mcp", "headers": {"Authorization": "Bearer …"}}}}`), which is the "any LLM / any harness" requirement satisfied by one server.

### 3. Tool catalog — precedent says split read/write and expose business content

Surveying the reference servers and vendor MCP servers (Postgres/SQLite reference, MotherDuck-DuckDB, Supabase, Databricks, dbt, Hex, Deepnote, Evidence):

- **Discovery (read-only, always on)**: `list_tables` (name, kind, rows, tags, source), `describe_table`, `list_saved_queries`, `list_pages` / `get_page` (PageDoc JSON), `search_workspace`.
- **Query**: `read_query` (SELECT/SHOW/DESCRIBE, enforced server-side in Rust) vs `write_query` (CTAS/DDL) — **split, not one SQL tool**: split lets harnesses gate writes at the protocol layer (Claude Code/opencode permission rules match on tool names). Return a preview with `rows_returned`/`total_rows`/`truncated` metadata (Hex pattern) and offer export-to-CSV for big results (Deepnote snapshot pattern). MotherDuck's caps (1024 rows / 50k chars) are a reasonable default.
- **Ingestion** (write-gated): `ingest` (path/URL, CSV/Parquet/JSON → table), `import_postgres`, `list_ingestions`.
- **Content** (write-gated): `save_query`, `create_page`/`update_page` (PageDoc spec), `set_table_tags`. Precedent: only notebook-app servers (Hex/Deepnote) and dbt expose business content — data.monster's Pages system is exactly this category, and the PageDoc JSON spec is already agent-authorable by design.
- **App-control**: `refresh_ui`, `notify`, `get_app_status`.

### 4. Safety model — layers, because no single layer holds

- **Read-only default; write opt-in** (MotherDuck `--read-write` / postgres-mcp `--access-mode=restricted` pattern). For an app-owned server: a mode toggle in Settings + per-session scope.
- **Annotate every tool** (`readOnlyHint`, `destructiveHint`, `idempotentHint`) — pessimistic defaults mean un-annotated tools are treated as destructive by strict clients. Annotations are advisory ("untrusted" per spec 2026-07-28) — necessary, not sufficient.
- **Elicitation** (stable spec, form mode, supported by Claude Code and Cursor) is the native in-band confirmation channel: use it for destructive SQL (DROP/DELETE/TRUNCATE) and for granting write scope. **Do not build on sampling — deprecated as of 2026-07-28.**
- **Transactions & checkpoints**: wrap agent write batches in `BEGIN … ROLLBACK`-able transactions (DuckDB is fully ACID); run `EXPORT DATABASE` before granting write scope as a filesystem-level undo point.
- **Audit**: log every tool call (agent, tool, args hash, row counts) to an append-only table in a *separate* DuckDB file.
- **Secrets stay in the app**: the Rust backend already proxies LLM keys; the MCP server holds tokens, never the agent. Server-side scoping alone can have holes (a restricted-mode bypass in Postgres MCP Pro was reported Sept 2026) — defense in depth is the answer.
- Design for the harness grammar (ask/allow/deny per tool + input patterns) — split tools make those rules expressible.

### 5. What's NOT the path (verified alternatives, and why)

- **OpenAI Apps SDK / ChatGPT plugins**: now MCP-based but cloud-first — requires a deployed public HTTPS endpoint and review. Poor fit for a local desktop app; nothing local-native remains.
- **Google A2A**: agent-to-agent task delegation, explicitly "not a replacement for MCP." Orthogonal — only relevant if data.monster later wants to *be* a delegable analysis agent.
- **Claude Desktop Extensions (.dxt → .mcpb)**: distribution packaging for MCP servers, not a protocol. Relevant later for one-click install of a companion shim, not now.
- **GitHub Copilot extensions**: GitHub/repo-centric; a local app reaches Copilot only *as an MCP server* anyway.
- **UI automation**: CDP/Playwright on WebView2 (already proven in this repo) is a great **e2e-test channel** but a bad agent API — pixels/DOM, no typed semantics, no transactions. Computer use is a last resort (CUA 38.1% vs human 72.4% on OSWorld; slow, token-heavy).

### 6. Tensions (unresolved, stated plainly)

- **Protocol era split**: the 2026-07-28 stateless redesign is large; clients and third-party docs are mixed between 2025-11-25 and 2026-07-28 behavior. rmcp targets both — rely on the SDK's era handling, and test against actual clients rather than docs.
- **rmcp version confusion**: crates.io README shows `0.16.0` while the Tier-1 promotion cites stable `3.0.1`. Pin what resolves today; read the 3.x migration notes before committing.
- **One SQL tool vs split**: vendors disagree (Supabase/MotherDuck one tool; SQLite reference split). Recommendation: split — it buys protocol-level write gating cheaply.
- **Annotations + restricted mode are both leaky**: spec says hints are untrusted; a restricted-mode bypass was reported. Hence the layered model above.
- **Windows-only CDP** (for the e2e channel) rests partly on third-party sources; fine for now — deployment is Windows-first.

## Open questions

- Live per-client matrix: which connected clients negotiate 2026-07-28 vs 2025-11-25, and which actually surface elicitation forms (test Claude Code, Cursor, opencode, pi directly).
- DuckDB time-travel-style undo (DuckLake / snapshots) as a cheaper agent-undo than EXPORT checkpoints.
- Whether to later ship a stdio shim exe + `.mcpb` bundle for one-click harness install (Windows signing cost).
- pi extension angle: a thin pi extension/skill wrapping the same MCP surface for this repo's own agents — redundant if pi's MCP client reaches localhost HTTP, which the harness here already supports natively (observed in-session, not web-verified).
- ARD (Agentic Resource Discovery) as an alternative to a hand-maintained tool catalog.

## How this was researched

Grounded fractal research, 1 root + 5 branches (protocol state; Tauri/Rust hosting; alternatives; tool-surface design; safety model), depth 1, fan-out 5, all leaves web-verified against primary sources (~45 URLs fetched; full search trails in `agents/d1-00*.md`). Synthesized bottom-up by the orchestrator. Run: 2026-09-15.

## Sources (key, per branch)

- Protocol: modelcontextprotocol.io changelogs (2025-03-26 → 2026-07-28), transports spec, registry repo; client docs (code.claude.com, cursor.com/docs, code.visualstudio.com, developers.openai.com/codex, opencode.ai, jetbrains.com).
- Hosting: github.com/modelcontextprotocol/rust-sdk + crates.io/docs.rs (rmcp), v2.tauri.app/develop/sidecar, duckdb.org/faq (single-writer lock), builder.io (Figma 127.0.0.1:3845), community.obsidian.md (Local REST API), lib.rs/crates.io (tauri-plugin-mcp-bridge, tauri-mcp).
- Alternatives: developers.openai.com/plugins (+ apps launch post), a2a-protocol.org, github.com/modelcontextprotocol/mcpb, docs.github.com/copilot MCP, platform.claude.com computer-use docs, openai.com CUA, playwright.dev/docs/webview2.
- Tool surface: servers-archived (postgres, sqlite READMEs), motherduckdb/mcp-server-motherduck, supabase.com/mcp, docs.databricks.com, dbt-labs/dbt-mcp, learn.hex.tech, deepnote.com/docs/deepnote-mcp, docs.evidence.dev/mcp.
- Safety: spec 2026-07-28 server/tools (annotations untrusted), 2025-11-25 elicitation + security best practices, 2026-07-28 client/sampling (deprecated), blog.modelcontextprotocol.io annotations post, code.claude.com/docs/en/permission-modes, opencode.ai/docs/permissions, github.com/crystaldba/postgres-mcp, duckdb.org ACID + EXPORT DATABASE docs.
