# Q: Besides MCP, what other agent↔app integration approaches exist as of late 2026, and when is each a better fit than MCP for a LOCAL desktop data app?

## Findings

### 1. OpenAI Apps SDK / ChatGPT apps & connectors

- Apps SDK launched 2025-10-06 in preview; apps render interactive UI in ChatGPT; "open standard built on the Model Context Protocol (MCP)"; apps connect to developer backends so users can log in (OAuth); first-connect consent prompt; app review pipeline before directory listing [verified: https://openai.com/index/introducing-apps-in-chatgpt/] — confidence: high
- As of 2026-09, `developers.openai.com/apps-sdk` redirects to `developers.openai.com/plugins`: ChatGPT and Codex share one "universal plugin directory"; a plugin = skills + an MCP server + optional UI; the quickstart registers a *deployed HTTPS MCP server URL* (Settings → Developer mode → Plugins) [verified: https://developers.openai.com/plugins and https://developers.openai.com/plugins/quickstart] — confidence: high
- **Fit for local desktop DuckDB app: poor as primary path.** Cloud-first: requires public HTTPS endpoint (tunnel for a local app), OpenAI review, ChatGPT-hosted UI. The Actions/connectors OpenAPI-manifest lineage (chatgpt.com/actions) has been folded into this MCP-based plugin model — nothing local-native remains.

### 2. Google A2A (Agent2Agent)

- A2A is an open standard for *agent-to-agent* interop: discovery (Agent Cards), task delegation, result streaming across frameworks (LangGraph, CrewAI, custom); agents stay "opaque" — no shared memory/tools [verified: https://a2a-protocol.org/] — confidence: high
- Official positioning: "MCP is for agent-to-tool communication… A2A is for agent-to-agent"; explicitly "not a sub-agent or tool-call protocol" and "not a replacement for MCP" [verified: https://a2a-protocol.org/] — confidence: high
- Developed by Google, donated to Linux Foundation; TSC includes AWS, Microsoft, Salesforce, SAP, ServiceNow; Apache 2.0; SDKs incl. Rust/Go/C# [verified: https://a2a-protocol.org/] — confidence: high
- **Fit: orthogonal, not an alternative.** Only a better fit than MCP when data.monster should appear as a delegable *agent* (e.g., "analysis agent" other agents discover and hand tasks to), not when an agent needs to operate the app's tools/data directly.

### 3. Claude Desktop Extensions (.dxt → .mcpb)

- June 2025: DXT renamed to MCPB; `.dxt` → `.mcpb`, `dxt` CLI → `mcpb`; old `.dxt` files keep working [verified: https://github.com/modelcontextprotocol/mcpb (rename notice); rename date also reported at anthropic.com] — confidence: high
- `.mcpb` = zip archive containing a local MCP server + `manifest.json`; "spiritually similar to" .crx/.vsix; one-click local install; bundles runtimes/node_modules; supports Node, Python (uv), and native binary servers (static linking preferred) [verified: https://github.com/modelcontextprotocol/mcpb] — confidence: high
- Solves *distribution*, not protocol: auto-updates, user-friendly config of server variables, curated directory; spec/toolchain open-sourced with the stated hope other AI desktop apps adopt the format [verified: https://github.com/modelcontextprotocol/mcpb] — confidence: high
- **Fit: strong — but it's MCP-family.** It answers "how does a user install the data.monster MCP server safely in one click" (or: data.monster could *host* .mcpb bundles itself, as the spec is open), not "what replaces MCP."

### 4. GitHub Copilot extensions / agents

- MCP works "across all major Copilot surfaces" — IDE, Copilot CLI, Copilot app, cloud agent/code review; repo-level MCP config; enterprise `MCP servers in Copilot` policy (default disabled); GitHub MCP Registry in public preview; "agent finder" implements the open Agentic Resource Discovery (ARD) spec for runtime discovery [verified: https://docs.github.com/en/copilot/concepts/context/mcp] — confidence: high
- Copilot coding agent GA; GitHub hosts third-party agents (incl. Claude) assignable to issues [reported-only: https://github.com/features/copilot/agents] — confidence: medium
- **Fit: poor directly.** Copilot's surface is GitHub/repo-centric; a local desktop app reaches it only as an MCP server consumed by Copilot clients — i.e., back to MCP.

### 5. UI automation (CDP/Playwright, computer use/CUA)

- Playwright automates Edge WebView2 (Chromium) via `connectOverCDP()` after starting the control with `--remote-debugging-port`; documented recipe = set `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` env var [verified: https://playwright.dev/docs/webview2] — confidence: high
- Tauri v2 on Windows uses WebView2 (Chromium-based) [reported-only: https://v2.tauri.app webview docs]; on Linux/macOS it uses WebKitGTK/WKWebView which lack CDP — no Playwright attach there [reported-only: lib.rs/crates.io tauri-plugin-playwright, GitHub issue] — confidence: medium
- Anthropic computer use (`computer_toolset_20260801`): 17 screenshot/mouse/keyboard member tools, executed client-side in your own environment; documented limitations: latency "too slow," coordinate/tool-selection hallucination, lower reliability on niche apps, ~4,500-token toolset overhead + 1,000–1,800 tokens/screenshot; prompt-injection classifiers can force user confirmation [verified: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool] — confidence: high
- OpenAI CUA (powers Operator/ChatGPT agent): 38.1% success on OSWorld full-computer-use (human 72.4%); 58.1% WebArena; explicitly unreliable on unfamiliar UIs (3–4/10 in examples) and imprecise at text editing [verified: https://openai.com/index/computer-using-agent/] — confidence: high
- **Fit: CDP/Playwright = good *e2e-test* channel for Tauri-on-Windows, bad as the agent API (pixels/DOM, no typed semantics, no transactional guarantees); computer use = last resort only — too slow/fragile for data work (bulk ingest, SQL).**

### 6. Others

- OpenAI function/tool calling (JSON-schema functions; Responses API; `tool_search` for many tools) — the lowest common denominator every harness supports; an app-local HTTP/RPC API exposed as N function definitions works with *any* model without MCP, but you own discovery/auth/versioning [verified: https://developers.openai.com/api/docs/guides/function-calling] — confidence: high
- LangChain/LlamaIndex tool wrappers: thin per-framework adapters over the same idea; no new capability — [reported-only, not fetched] — confidence: low
- LSP-style: no verified evidence of an analogous open "app operations" protocol beyond MCP/A2A/ARD as of 2026-09 — [unverified] — confidence: low

## Tensions
- Tauri+CDP: works on Windows only per fetched docs (WebView2); Linux/macOS claims come from third-party sources, not official Tauri docs — verify before relying on cross-platform UI automation.
- Copilot "extensions" vs "MCP servers" naming shifted during 2025–2026; GitHub docs now route extension pages to MCP concepts.

## Open questions
- Does OpenAI's plugin directory allow local/loopback servers or require public endpoints? (Developer-mode personal plugins accept any URL; published-directory policy unclear.)
- ARD (Agentic Resource Discovery) spec details — could it replace a hand-written tool catalog for data.monster?
- Exact WebKitGTK automation story for Tauri on Linux (Driver.js/accessibility-tree alternatives to CDP).

## Search trail
- queries: OpenAI Apps SDK ChatGPT apps connectors chatgpt.com/actions manifest; Google A2A agent2agent protocol vs MCP 2026; Claude Desktop Extensions .dxt .mcpb packaging MCP servers distribution; GitHub Copilot extensions agent experience Copilot coding agent 2026; Anthropic computer use API OpenAI Operator CUA agent desktop automation limitations; Tauri WebView2 Chrome DevTools Protocol attach Playwright automate webview
- fetched: https://developers.openai.com/apps-sdk/ — redirects to Plugins overview (Apps SDK unified into plugin system); https://developers.openai.com/plugins/quickstart — plugin = skills + MCP server URL + optional UI, shared ChatGPT/Codex directory; https://openai.com/index/introducing-apps-in-chatgpt/ — Apps SDK launch, MCP-based, OAuth/backend login, review pipeline; https://a2a-protocol.org/ — A2A purpose, MCP complementarity, Linux Foundation governance, SDK list; https://github.com/modelcontextprotocol/mcpb — .mcpb format, dxt-to-mcpb rename, manifest spec, runtime bundling incl. binaries; https://docs.github.com/en/copilot/concepts/context/mcp — Copilot MCP surfaces, policies, registry, agent finder/ARD; https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool — toolset mechanics, limitations, token costs, injection classifiers; https://openai.com/index/computer-using-agent/ — CUA benchmark numbers and failure modes; https://playwright.dev/docs/webview2 — connectOverCDP WebView2 recipe; https://developers.openai.com/api/docs/guides/function-calling — function/tool calling as base mechanism
- rejected: https://docs.github.com/en/copilot/how-tos/build-extensions — 404/redirect; help.openai.com, stytch.com, mcp.so, workos.com — secondary sources, skipped in favor of primary docs
