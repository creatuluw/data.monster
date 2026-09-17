# Q: How do you host an MCP server from inside a Tauri v2 desktop app's Rust backend? (architectures a/b/c, rmcp SDK state, desktop prior art)

## Findings

**Official Rust SDK (rmcp)**

- rmcp is the official MCP SDK (`An official Rust Model Context Protocol SDK implementation with tokio async runtime`), ~3.8k GitHub stars, Apache-2.0, 16.09M all-time crates.io downloads, 50 versions published [verified: https://github.com/modelcontextprotocol/rust-sdk + https://crates.io/crates/rmcp] - confidence: high
- rmcp was promoted Tier 2 to Tier 1 on 2026-08-21 (PR #3287, issue #3179): 67/67 server + 50/50 client conformance vs suite 0.2.0-alpha.11, stable rmcp 3.0.1, same-day spec tracking, published VERSIONING/DEPENDENCY_POLICY/ROADMAP. As of 2026-09, 5 of 10 official SDKs are Tier 1 (TS, Python, C#, Go, Rust) [verified: https://www.digitalapplied.com/blog/mcp-sdk-conformance-tiers-what-tier-1-means] - confidence: high
- rmcp targets spec revisions 2025-11-25 and 2026-07-28 (server discovery, subscriptions, long-running tasks, caching, standard HTTP headers) [verified: https://github.com/modelcontextprotocol/rust-sdk] - confidence: high
- rmcp transports today: stdio (transport-io, client+server), client-side child-process (transport-child-process), Streamable HTTP server (transport-streamable-http-server) exposed as a Tower service mountable on any axum/hyper router (axum Router nest_service "/mcp"), Streamable HTTP client (reqwest), worker/in-process. Legacy HTTP+SSE (2024-11-05) is a deliberate non-goal; there is an auth feature flag (OAuth) [verified: https://github.com/modelcontextprotocol/rust-sdk + https://docs.rs/rmcp] - confidence: high
- Server authoring via #[tool]/#[tool_router]/#[tool_handler] macros; StreamableHttpServerConfig supports stateless mode [verified: https://github.com/modelcontextprotocol/rust-sdk] - confidence: high

**Architecture comparison**

- (a) stdio requires the *agent* to spawn the server binary (TokioChildProcess is client-side). A stdio MCP endpoint cannot attach to an already-running app unless it is a thin shim that proxies to the app over IPC/HTTP [verified: https://github.com/modelcontextprotocol/rust-sdk transport table] - confidence: high
- (b) Streamable HTTP on 127.0.0.1 served in-process from the Rust backend (axum + tokio) is the only architecture where the running app that owns the DuckDB connection is directly the MCP server; single process, no DB contention. rmcp Tower-service design drops straight into axum [verified: https://github.com/modelcontextprotocol/rust-sdk] - confidence: high
- (c) separate process sharing the DuckDB file is blocked by DuckDB concurrency: one database file, single read-write process (file lock); multiple processes only concurrent in read-only mode [verified: https://duckdb.org/faq (How does DuckDB handle concurrency? Can multiple processes write to DuckDB?)] - confidence: high
- Tauri sidecars are spawned *by the app* (externalBin in tauri.conf.json, target-triple-suffixed binary, app.shell().sidecar() via tauri_plugin_shell) - they do not help when the *agent* must spawn something [verified: https://v2.tauri.app/develop/sidecar/] - confidence: high
- Port-conflict handling, loopback-only binding, and a per-session auth token are the (b) design burdens: localhost HTTP is reachable by any local process; Obsidian equivalent ships API-key auth + self-signed HTTPS on 127.0.0.1:27124 [verified: https://community.obsidian.md/plugins/obsidian-local-rest-api] - confidence: high
- Windows packaging: a second sidecar binary must be bundled and code-signed alongside the main exe (signing follows the bundle externalBin list); keeping everything in the one signed main exe avoids this - reasoning from Tauri sidecar model [reported-only, consistent with https://v2.tauri.app/develop/sidecar/] - confidence: medium

**Prior art**

- Figma Dev Mode MCP Server: embedded in the Figma *desktop app*, enabled in Preferences, then serves locally at http://127.0.0.1:3845/sse (older endpoint; transport since moved toward streamable HTTP); requires the running desktop app - the canonical agent-connects-to-running-app-over-localhost-HTTP [verified: https://www.builder.io/blog/claude-code-figma-mcp-server] - confidence: high
- Obsidian (community pattern): Local REST API plugin serves HTTPS 127.0.0.1:27124 with API key + local CA cert; separate stdio MCP servers (obsidian-local-rest-api-mcp, mcp-obsidian-via-rest) just proxy to it - i.e. the hybrid (stdio shim -> HTTP to running app) [verified: https://community.obsidian.md/plugins/obsidian-local-rest-api; listings at mcpservers.org and github.com/oleksandrkucherenko/mcp-obsidian-via-rest surfaced in search] - confidence: high
- Tauri-specific: tauri-plugin-mcp-bridge (v0.13.0, 2026-08-28, 27 releases, ~50k downloads/mo) - plugin inside the Tauri app; the MCP server binary talks to it via WebSocket (bridge to running app). Also tauri-mcp v0.1.4: stdio MCP server for *testing* Tauri v2 apps (launch_app, screenshots, input simulation) - spawns the app, dev-oriented [verified: https://lib.rs/crates/tauri-plugin-mcp-bridge + https://crates.io/crates/tauri-mcp] - confidence: high
- VS Code consumes both stdio and type:"http" MCP servers in mcp.json, so an http-endpoint architecture works with major harnesses [verified: https://code.visualstudio.com/docs/copilot/customization/mcp-servers] - confidence: high

## Tensions

- rmcp version: crates.io README example shows rmcp = "0.16.0" while the Tier-1 promotion (2026-08-21) cites "stable rmcp 3.0.1" and the GitHub README links a 3.x migration guide - README example likely lags; pin whatever crates.io resolves in 2026-09 and read migration discussion #969 before committing.
- Figma endpoint: builder.io (2025-era) documents /sse; MCP deprecated SSE transport in favor of streamable HTTP - copy the enabled-in-app / fixed-127.0.0.1-port pattern, not the /sse shape.
- tauri-plugin-mcp-bridge is community-maintained (not Tauri-official) with 12 breaking releases in 27 - verify maintenance cadence before depending on it vs rolling your own axum mount.

## Open questions

- Auth/session model for the loopback server: bearer token in config file vs per-connection handshake vs OAuth (auth feature) - needs a dedicated branch.
- Whether the app should ALSO ship a tiny stdio shim exe (spawned by agents) that proxies to the running app HTTP port - the Obsidian pattern formalized; Windows packaging/signing cost worth costing out.
- rmcp 3.x breaking changes (discussion #969) and stateful vs stateless StreamableHttpServerConfig trade-offs for multi-agent concurrent sessions.
- Named-pipe transport on Windows (stdio-equivalent without TCP ports) as alternative to HTTP loopback - not offered by rmcp today (unverified beyond transport table).

## Search trail

- queries: rmcp modelcontextprotocol rust-sdk MCP Rust SDK transports maturity; Tauri desktop app host MCP server axum localhost connect to running application; embedded MCP server desktop app connect to running app VS Code Obsidian Figma how it works localhost; VS Code as MCP server command line agent control running instance obsidian local rest api mcp
- fetched: https://github.com/modelcontextprotocol/rust-sdk - transports table, spec revisions 2025-11-25/2026-07-28, Tower/axum streamable-HTTP example, macros, no legacy SSE
- fetched: https://crates.io/crates/rmcp - 0.16.0 README example, 16.09M downloads, 50 versions, Apache-2.0
- fetched: https://docs.rs/rmcp - feature flags incl. transport-streamable-http-server, auth
- fetched: https://www.digitalapplied.com/blog/mcp-sdk-conformance-tiers-what-tier-1-means - Tier-1 promotion 2026-08-21, rmcp 3.0.1, 67/67 + 50/50 conformance
- fetched: https://v2.tauri.app/develop/sidecar/ - externalBin, target-triple suffix, app-spawned sidecar
- fetched: https://duckdb.org/faq - single file, single read-write process, concurrency doc pointer
- fetched: https://www.builder.io/blog/claude-code-figma-mcp-server - Figma desktop MCP at 127.0.0.1:3845/sse, enabled in Preferences
- fetched: https://community.obsidian.md/plugins/obsidian-local-rest-api - HTTPS 127.0.0.1:27124, API key, local CA
- fetched: https://lib.rs/crates/tauri-plugin-mcp-bridge - v0.13.0 2026-08-28, WebSocket bridge into running Tauri app
- fetched: https://crates.io/crates/tauri-mcp - v0.1.4 stdio test server that launches Tauri apps
- fetched: https://code.visualstudio.com/docs/copilot/customization/mcp-servers - VS Code consumes stdio + http MCP servers
- rejected: https://help.figma.com/hc/en-us/articles/34832987333527 - fetch failed twice (page render error); used builder.io instead
- rejected: https://github.com/figma/figma-developer-mcp - 404 (repo not found at that path)
- rejected: https://forum.obsidian.md/t/obsidian-mcp-servers-experiences-and-recommendations/121142 - fetch failed; community plugin page used instead
- rejected: duckdb.org/docs/stable/operations_manual/concurrency and /guides/concurrency/overview - 404s; FAQ page carries the concurrency fact
