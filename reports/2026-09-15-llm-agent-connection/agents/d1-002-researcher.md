# Q: What is the current state (as of late 2026) of MCP as THE standard for connecting LLM agents to external applications — spec versions/revisions, transports, core primitives, auth, registries — and which major clients support which parts?

## Findings

### Spec versions & revision history (all verified from modelcontextprotocol.io changelogs, as of 2026-09-15)

- **Current spec: 2026-07-28**; docs are versioned (`/specification/2026-07-28/`), with a live draft accumulating post-release changes [verified: https://modelcontextprotocol.io/specification/2026-07-28/changelog] — confidence: high
- Revision chain: **2024-11-05** (initial, per 2025-03-26 changelog's "previous revision") → **2025-03-26** → **2025-06-18** → **2025-11-25** → **2026-07-28** [verified: each dated changelog page] — confidence: high. (A 2024-11-25 dot-release exists per community references — reported-only.)
- **2025-03-26**: added OAuth 2.1 authorization framework; **replaced HTTP+SSE transport with Streamable HTTP**; JSON-RPC batching; tool annotations [verified: .../specification/2025-03-26/changelog] — confidence: high
- **2025-06-18**: removed JSON-RPC batching; structured tool output; **MCP servers classified as OAuth Resource Servers** (RFC 9728 protected-resource metadata); clients **MUST** implement RFC 8707 Resource Indicators; **added elicitation**; resource links; `MCP-Protocol-Version` header [verified: .../specification/2025-06-18/changelog] — confidence: high
- **2025-11-25**: OIDC discovery for auth servers; incremental scope consent via `WWW-Authenticate`; URL-mode elicitation; enum/untitled/multi-select elicitation schemas; tool-calling inside sampling (`tools`/`toolChoice`); **OAuth Client ID Metadata Documents (CIMD)**; experimental tasks; formalized governance, Working Groups, SDK tiering (SEP-932/1302/1730) [verified: .../specification/2025-11-25/changelog] — confidence: high
- **2026-07-28 (largest change since launch)**: protocol made **stateless** — sessions/`Mcp-Session-Id` and `initialize` handshake removed (version+capabilities now per-request in `_meta`); new `server/discover` RPC; `subscriptions/listen` replaces HTTP GET stream + `resources/subscribe`; `ping`, `logging/setLevel`, `roots/list_changed` removed; **Multi-Round-Trip Requests (MRTR) replace server-initiated requests** (`roots/list`, `sampling/createMessage`, `elicitation/create` become `InputRequiredResult`/`inputResponses` on retry); tasks moved to official extension `io.modelcontextprotocol/tasks`; SSE resumability (`Last-Event-ID`) removed; list-result caching (`ttlMs`/`cacheScope`); RFC 9207 `iss`; DCR `application_type` required [verified: .../specification/2026-07-28/changelog] — confidence: high
- **Transports (2026-07-28)**: exactly two standard bindings — **stdio** and **Streamable HTTP** (POST to single endpoint; JSON or request-scoped SSE response stream); custom transports allowed, byte-stream ones SHOULD reuse stdio framing; in the new message patterns **servers no longer initiate JSON-RPC requests**; era-detection backward-compat matrix defined [verified: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports] — confidence: high
- Core primitives across revisions: tools, resources (+templates/links), prompts, sampling, elicitation, roots, completions, logging, tasks (now extension), plus new "MCP Apps" (interactive UI inside clients) [verified: changelogs + https://modelcontextprotocol.io/docs/2026-07-28/getting-started/intro] — confidence: high
- **Auth = OAuth 2.1 family**: resource-server model, RFC 8707, CIMD and DCR recommended registration paths [verified: 2025-06-18 + 2026-07-28 changelogs] — confidence: high
- **Official registry**: preview launched 2025-09-08; **API freeze v0.1 on 2025-10-24**; namespace model `io.github.<user>/...` and DNS/HTTP domain verification; Go service + publisher CLI; server v1.x releases shipped; Registry Working Group (Stacklok lead) [verified: https://github.com/modelcontextprotocol/registry] — confidence: high. Alternatives coexist: GitHub MCP Registry (2025-09, github.blog), Anthropic Directory (verified via Claude Code docs), Cursor Marketplace [verified: cursor.com/docs/mcp] — confidence: medium-high
- Adoption scale: ~500M SDK downloads/month across Tier 1 SDKs; TS+Python each >1B total (2026-07 blog) [verified: https://blog.modelcontextprotocol.io/] — confidence: high

### Client support (per official docs, as of 2026-09)

- **Claude Code**: transports http (=`streamable-http`), sse, ws; OAuth on http only; **elicitation supported** (dialog UI), **roots** (`roots/list` answered with launch dir + `--add-dir`); servers can push messages into session; sampling not documented [verified: https://code.claude.com/docs/en/mcp] — confidence: high
- **ChatGPT / Codex (OpenAI)**: desktop app + Codex CLI + IDE extension share `~/.codex/config.toml`; STDIO + Streamable HTTP; Bearer, OAuth incl CIMD + DCR; server `instructions` honored; ChatGPT web uses remote MCP via plugins/connectors only [verified: https://developers.openai.com/codex/mcp/ → learn.chatgpt.com/docs/extend/mcp] — confidence: high
- **Cursor**: **tools, prompts, resources, roots, elicitation, MCP Apps all "Supported"** (official feature table); transports stdio/SSE/Streamable HTTP; OAuth incl static-credential mode for non-DCR providers; sampling not listed [verified: https://cursor.com/docs/mcp] — confidence: high
- **VS Code**: MCP server gallery (`@mcp` in Extensions view), `.vscode/mcp.json` + user `mcp.json`, tools/resources/prompts/interactive apps; config forwarded to Agent Host, `~/.copilot/mcp-config.json` portable [verified: https://code.visualstudio.com/docs/copilot/customization/mcp-servers] — confidence: high
- **JetBrains**: AI Assistant plugin = MCP **client** (Settings > Tools > AI Assistant > MCP) and can expose IDE as MCP **server**; also ACP for external agents (doc dated 2026-08-05) [verified: https://www.jetbrains.com/help/webstorm/ai-assistant-in-jetbrains-ides.html] — confidence: high
- **opencode**: local (stdio) + remote (Streamable HTTP) servers, headers + OAuth config in opencode.jsonc [verified: https://opencode.ai/docs/mcp-servers/] — confidence: high
- **Gemini CLI**: MCP support for custom integrations (official README feature list) [verified: https://github.com/google-gemini/gemini-cli] — confidence: medium (README doesn't detail primitives)
- **Claude Desktop**: MCP connectors supported — appears in JetBrains' client auto-config list (Cursor, Claude App, Claude Code, Windsurf) [reported-only: youtrack.jetbrains.com IJPL-204939; Anthropic support page 404'd] — confidence: medium
- Community capability DB exists (apify/mcp-client-capabilities) because handshake negotiation underreports client features (elicitation/sampling/roots/completions per client) [verified: https://github.com/apify/mcp-client-capabilities] — confidence: high

## Tensions

- 2026-07-28's stateless/MRTR redesign is so large that much third-party content (and some clients' docs, e.g. Cursor still listing SSE transport; apify DB keyed on `initialize` handshake) describes the pre-2026 protocol; era-detection/backward-compat matters when quoting older material.
- Task question's premise of a "2024-11-25" major revision: changelogs show 2024-11-05 as the base; 2024-11-25 is referenced only in secondary sources — treat 2024-11-05 → 2025-03-26 as the canonical early chain.
- Sampling: no major client's docs verified above claim sampling support (Cursor's table omits it; Claude Code doc silent) even though it's a core spec primitive since 2024 — server authors cannot rely on it.

## Open questions

- Which clients implement the 2026-07-28 stateless protocol vs. still negotiating 2025-11-25 (per-client protocolVersion support matrix — apify DB or SDK release notes as of Sept 2026).
- Claude Desktop's exact primitive coverage (sampling? elicitation?) via a fetchable Anthropic page.
- Whether "MCP Apps" extension and `io.modelcontextprotocol/tasks` extension are implemented outside Cursor/VS Code.

## Search trail

- queries: "Model Context Protocol 
specification versions 2024-11-25 2025-06-18 revision streamable HTTP OAuth"; "MCP client support Claude Code Cursor Codex CLI VS Code Gemini CLI JetBrains sampling elicitation"; "modelcontextprotocol registry directory spec version changelog 2026"; "site:modelcontextprotocol.io specification changelog versions list"; "Cursor docs MCP servers model context protocol site:cursor.com"; "JetBrains IDE MCP client AI Assistant built-in support 2026"
- fetched: modelcontextprotocol.io/specification/{2025-03-26,2025-06-18,2025-11-25,2026-07-28}/changelog — full major/minor changes per revision
- fetched: modelcontextprotocol.io/specification/2026-07-28/basic/transports — stdio + Streamable HTTP bindings, custom transports, backward compat
- fetched: github.com/modelcontextprotocol/registry — registry status, API freeze v0.1 (2025-10-24), namespaces, auth
- fetched: blog.modelcontextprotocol.io — 2026-07-28 release, SDK download scale, roadmap
- fetched: code.claude.com/docs/en/mcp — Claude Code transports, elicitation/roots, OAuth
- fetched: developers.openai.com/codex/mcp/ — ChatGPT/Codex MCP features, CIMD/DCR, config sharing
- fetched: cursor.com/docs/mcp — Cursor capability table, transports, static OAuth
- fetched: code.visualstudio.com/docs/copilot/customization/mcp-servers — VS Code gallery, mcp.json
- fetched: jetbrains.com/help/webstorm/ai-assistant-in-jetbrains-ides.html — JetBrains MCP client+server, ACP
- fetched: opencode.ai/docs/mcp-servers/ + github.com/google-gemini/gemini-cli README + github.com/apify/mcp-client-capabilities — client configs / capability DB
- rejected: modelcontextprotocol.io/clients — redirects to 2026-07-28 intro; old support table gone
- rejected: spec.modelcontextprotocol.io — unreachable (spec merged into modelcontextprotocol.io)
- rejected: plugins.jetbrains.com/plugin/26071-mcp-server — page load error; support.anthropic.com configuring-mcp — 404; raw mcp-clients.json — 404 (data not at that path)
