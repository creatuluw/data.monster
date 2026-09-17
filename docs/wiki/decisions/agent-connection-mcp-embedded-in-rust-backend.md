---
type: Decision
title: "Agent connection: MCP server embedded in the Rust backend"
description: Context
tags: [mcp, agents, architecture, rust, llm]
status: proposed
timestamp: "2026-09-15T15:43:47.201Z"
---

# Agent connection: MCP server embedded in the Rust backend

## Context

data.monster needs to be operable by external LLM agents and coding harnesses (Claude Code, Cursor, Codex, opencode, pi, VS Code): connect data, add content, run analysis. A fractal-research run on 2026-09-15 (~45 primary sources, all citations verified; report at `reports/2026-09-15-llm-agent-connection/` — see [[llm-agent-connection-research-report]]) surveyed how live apps expose themselves to agents (Figma, MotherDuck, Hex, Deepnote, dbt patterns) and how the major harnesses attach.

## The choice (proposed — not yet user-locked)

Ship an **MCP server inside the Rust backend**: Streamable HTTP bound to `127.0.0.1`, mounted in axum via the official Tier-1 Rust SDK (**rmcp**), hosted by the already-running Tauri app process.

- Streamable HTTP is the only transport that attaches an agent to a live app; a separate agent process is impossible anyway — DuckDB's **single-writer lock** means the workspace is pinned by the running app.
- One server = one-liner config for Claude Code, Cursor, Codex, opencode, pi, VS Code — the "any harness" requirement solved at once (the Figma pattern).

**Tool layer stays thin** — it wraps the existing command modules (`queries`, `files`, `tables`, `saved_queries`, PageDoc storage). Recommended catalog:

- discovery: `list_tables`, `get_page`, …
- split `read_query` / `write_query` with truncation metadata
- `ingest` / `import_postgres`
- content: `create_page` (PageDoc JSON is already agent-authorable)
- `refresh_ui` — emits Tauri events so the human watches the agent work live

**Safety is layered, never single**: read-only default with opt-in write; pessimistic MCP tool annotations; elicitation-based confirmation for destructive SQL (sampling is deprecated — avoid); transactions + `EXPORT DATABASE` checkpoints; append-only audit log in a separate DuckDB file; secrets stay in Rust.

## Alternatives considered and rejected

- **Separate agent process / direct DuckDB access** — impossible: single-writer lock held by the app.
- **OpenAI Apps SDK** — cloud-only.
- **A2A** — orthogonal (agent↔agent protocol, not app attachment).
- **Copilot extensions** — repo-centric.
- **Computer use / OS-level automation** — ~38% OSWorld success; last resort only.
- **CDP** — stays the e2e-test channel, not the agent API.

## Consequences

- Phase 1 (when the user says go): loopback MCP server + discovery/read tools, read-only.
- Rust gains an axum route + rmcp dependency; frontend gains a `refresh_ui` Tauri-event listener pattern.
- Every destructive op needs an elicitation gate before write tools ship.
