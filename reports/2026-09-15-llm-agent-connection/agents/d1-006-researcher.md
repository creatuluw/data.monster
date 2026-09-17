# Q: What safety and permission models exist (as of late 2026) for letting agents operate apps that can WRITE data and content?

## Findings

**MCP tool annotations**
- `ToolAnnotations` = `readOnlyHint` (default false), `destructiveHint` (default true), `idempotentHint` (default false), `openWorldHint` (default true); shipped in spec 2025-03-26. Defaults are deliberately pessimistic: un-annotated tool is assumed non-read-only, destructive, non-idempotent, open-world [verified: https://blog.modelcontextprotocol.io/posts/2026-03-16-tool-annotations/] — confidence: high
- Spec 2026-07-28: "clients MUST consider tool annotations to be untrusted unless they come from trusted servers" — hints are advisory, not verified behavior; client enforcement varies ("clients vary in how strictly they honor the pessimistic defaults") [verified: https://modelcontextprotocol.io/specification/2026-07-28/server/tools + annotations blog] — confidence: high
- First three hints ≈ preflight "ask for confirmation?"; `openWorldHint` is about untrusted-content reach (post-call), not confirmation [verified: annotations blog] — confidence: high
- Ecosystem coverage is uneven: many servers ship without annotations (ChatGPT dev mode displays annotation-less tools as "write" tools — reported-only via nickyt.co search snippet; filesystem server issue #3402 flags missing hints) — confidence: medium
- Active SEPs (as of 2026-03): #1913 Trust/Sensitivity annotations (GitHub+OpenAI co-authored from production gaps), #1984 governance annotations, #1561 `unsafeOutputHint`, #1560 `secretHint`, #1487 `trustedHint` — annotation model is actively evolving; don't hard-depend on exact set [verified: annotations blog] — confidence: high

**Elicitation & sampling**
- Elicitation (server→client user-input requests, `elicitation/create`) is stable spec: clients MUST declare `elicitation` capability, MUST support ≥1 mode (`form` or `url`); empty caps object = form-only back-compat; 2025-11-25 added `url` mode (SEP-1036) for OAuth flows where server binds tokens to user identity [verified: https://modelcontextprotocol.io/specification/2025-11-25/client/elicitation] — confidence: high
- **Sampling is DEPRECATED as of spec 2026-07-28 (SEP-2577)**: new implementations SHOULD NOT adopt it; remains in spec ≥12 months; migrate to direct LLM-provider API integration [verified: https://modelcontextprotocol.io/specification/2026-07-28/client/sampling] — confidence: high
- Sampling security notes (while it lasts): clients SHOULD implement user approval controls, rate limiting, iteration limits for tool loops [verified: same page] — confidence: high

**Harness HITL patterns**
- Claude Code (as of 2026-09): modes `default` (ask), `plan` (no edits until plan approved), `auto` (classifier-based, default on Pro/Max/Team), `dontAsk` (exact `--allowedTools` allowlist for CI), `bypassPermissions`/`--dangerously-skip-permissions` (requires container/VM/sandbox + non-root user; orgs can disable). Plus a separate Bash sandbox (`/sandbox` auto-allow) that composes with modes; deny rules always apply even in bypass [verified: https://code.claude.com/docs/en/permission-modes] — confidence: high
- opencode: `permission` config resolves every action to `allow`/`ask`/`deny`, per-tool and per-input pattern rules (`"bash": {"git *": "allow", "rm *": "deny"}`), last-matching-rule-wins, wildcard `*`/`?`, `external_directory` scoping for paths outside cwd, `--auto` flag auto-approves non-denied, org-managed settings (MDM) users can't override [verified: https://opencode.ai/docs/permissions/ + https://opencode.ai/docs/config/] — confidence: high
- Cursor: agent asks clarifying questions mid-task while continuing read-only work; automatic checkpoints before significant changes with one-click restore (local, separate from Git) [verified: https://cursor.com/docs/agent/overview] — confidence: medium (approval UX details not on fetched page)

**Data-safety patterns**
- postgres-mcp (Postgres MCP Pro): `--access-mode=restricted` = read-only transactions + execution-time limits, "suitable for production"; `unrestricted` = full read/write for dev [verified: https://github.com/crystaldba/postgres-mcp] — confidence: high
- Reported (Sept 2026, not fetched): a restricted-mode bypass in Postgres MCP Pro was disclosed — server-side scoping alone can have holes; defense-in-depth needed [reported-only: forkast.news search snippet] — confidence: low
- DuckDB is fully ACID (MVCC, WAL, crash-recovery verified by TPC-H suite); `BEGIN TRANSACTION … ROLLBACK` gives atomic multi-statement undo — wrap agent write batches in explicit transactions [verified: https://duckdb.org/2024/09/25/changing-data-with-confidence-and-acid] — confidence: high
- DuckDB backup/restore: `EXPORT DATABASE 'dir' (FORMAT parquet, …)` + `IMPORT DATABASE` — filesystem-level snapshot as rollback path; no built-in time travel (unverified absence; not found in fetched docs) [verified: https://duckdb.org/docs/stable/sql/statements/export] — confidence: medium
- No DB MCP server found with first-class DROP/DELETE confirmation built in — destructive-SQL gating is typically client-side (Claude Code deny rules, opencode patterns) or via read-only mode

**Secrets & local auth**
- MCP security best practices: require HTTPS for OAuth URLs except loopback (`localhost`, `127.0.0.1`, `::1`) per OAuth 2.1 §1.5; clients SHOULD block private/reserved IPs (loopback excepted for dev), beware DNS rebinding, cloud-metadata SSRF, redirect chains; don't hand-roll IP validation [verified: https://modelcontextprotocol.io/specification/2025-11-25/basic/security_best_practices] — confidence: high
- URL-mode elicitation pattern for third-party auth: server acts as OAuth client, stores third-party tokens bound to user identity — i.e., the server (app), not the agent, holds credentials [verified: elicitation spec] — confidence: high

**Actionable for a Tauri app exposing DuckDB writes**
- Serve MCP on loopback only; app (Rust) owns all API keys/tokens; never pass secrets through tool args/results
- Expose separate tool sets: read-only query tools (annotated `readOnlyHint:true`, idempotent) vs. write tools (`destructiveHint` where true); pessimistic defaults mean un-annotated = worst-case in strict clients
- Use `elicitation/create` (form mode) for in-band confirmation of destructive SQL (DROP/DELETE/TRUNCATE) and for granting write scope — it's the spec-native HITL channel and stable
- Don't build on sampling — deprecated 2026-07-28
- Default session to read-only; write mode opt-in per session (postgres-mcp restricted-mode pattern); budget query execution time
- Log every tool call (agent, tool, args hash, rowcounts) to an append-only audit table in a separate DuckDB file; wrap writes in BEGIN/ROLLBACK; run `EXPORT DATABASE` checkpoint before granting write scope
- Design for Claude Code-style modes (ask/allow/deny per tool + input patterns), because that's the dominant harness grammar

## Tensions
- Spec says annotations are untrusted hints; clients vary in enforcement — annotating tools correctly is necessary but not sufficient for safety
- `restricted` mode is the community "production" answer, yet a bypass in Postgres MCP Pro was reported (Sept 2026) — server-side scoping alone is not a guarantee
- Elicitation is stable but client adoption is partial (Kiro URL-mode support was still a feature request Jan 2026); sampling is deprecated — mid-flow server-initiated LLM use is now out-of-protocol

## Open questions
- Actual elicitation client-support matrix (Claude Code, Cursor, opencode) as of 2026-09 — each harness's MCP capability declaration should be tested directly
- DuckDB time travel / COW snapshots via extensions (ducklake, community forks) as cheap agent-undo — worth a dedicated branch
- Status of SEP-1913 trust/sensitivity annotations and whether any client enforces them yet

## Search trail
- queries: "MCP tool annotations readOnlyHint destructiveHint idempotentHint openWorldHint client behavior"; "Model Context Protocol elicitation sampling specification 2025 client support"; "Claude Code permission modes acceptEdits bypassPermissions dangerously-skip-permissions"; "postgres-mcp access-mode restricted DuckDB ACID backup EXPORT DATABASE agent MCP"; "duckdb.org docs concurrency single writer multiple readers ACID transactions"; '"Changing Data with Confidence and ACID" duckdb blog'
- fetched: https://modelcontextprotocol.io/specification/2026-07-28/server/tools - annotations untrusted-MUST, tool definition fields
- fetched: https://modelcontextprotocol.io/specification/2025-11-25/client/elicitation - elicitation/create, form/url modes, capability rules, OAuth binding pattern
- fetched: https://modelcontextprotocol.io/specification/2026-07-28/client/sampling - DEPRECATED (SEP-2577), security notes
- fetched: https://modelcontextprotocol.io/specification/2025-11-25/basic/security_best_practices - HTTPS/loopback, private-IP blocking, DNS rebinding, metadata SSRF
- fetched: https://blog.modelcontextprotocol.io/posts/2026-03-16-tool-annotations/ - hint defaults, history, open SEPs
- fetched: https://code.claude.com/docs/en/permission-modes - 5 modes, allowlists, sandbox, isolation requirements
- fetched: https://github.com/crystaldba/postgres-mcp - access modes, transport, client config
- fetched: https://opencode.ai/docs/permissions/ + /docs/config/ - allow/ask/deny patterns, --auto, managed settings
- fetched: https://cursor.com/docs/agent/overview - checkpoints, clarifying questions
- fetched: https://duckdb.org/2024/09/25/changing-data-with-confidence-and-acid - ACID/MVCC/WAL/ROLLBACK
- fetched: https://duckdb.org/docs/stable/sql/statements/export - EXPORT/IMPORT DATABASE
- rejected: https://duckdb.org/docs/stable/guides/concurrency (404); https://cursor.com/docs/agent/terminal (empty stub, 97 chars); https://duckdb.org/2024/09/25/acid.html variants (404 - correct slug found via search)
