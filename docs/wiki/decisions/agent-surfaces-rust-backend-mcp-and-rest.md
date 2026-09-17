---
type: Decision
title: "Agent surfaces: one Rust backend serves MCP and loopback REST; ship a dm skill+CLI alongside"
description: Context
tags: [agents, architecture, rest, skill, mcp, rust]
status: proposed
timestamp: "2026-09-15T16:06:34.733Z"
---

# Agent surfaces: one Rust backend serves MCP and loopback REST; ship a dm skill+CLI alongside

## Context

Follow-up to [[agent-connection-mcp-embedded-in-rust-backend]] (2026-09-15): the user asked whether the agent connection could also be done with **a skill + plain APIs** instead of MCP. Answer: yes — and it's not either/or. The lazy architecture serves both from one backend.

## The choice (proposed — not yet user-locked)

**One backend, two surfaces.** The same axum router mounts both:

```rust
let app = Router::new()
    .nest_service("/mcp", mcp_service)            // rmcp — typed tools, permission gating
    .route("/api/tables", get(rest::list_tables)) // plain JSON — curl, skills, scripts
    .route("/api/query", post(rest::read_query))
    .layer(bearer_auth);                          // same token, same audit, same read-only default
```

REST handlers are thin re-exports of the same Rust command modules the MCP tools wrap — one implementation, two doors.

**Plus a `dm` skill + CLI** for agents without an MCP client:

```
data-monster/
├── SKILL.md        # endpoints, token location (~/.data-monster/mcp.json), conventions, examples
└── scripts/dm      # tiny curl wrapper: dm tables | dm query "SELECT …" | dm page show <slug>
```

- Skills follow the Agent Skills standard (agentskills.io): pi loads `SKILL.md` from `~/.pi/agent/skills/` or `.pi/skills/` with progressive disclosure; **Claude Code and Codex read the same format** — one skill is near-portable across harnesses.
- The `dm` wrapper matters: "run `dm query`" is far more reliable for an agent than "curl this URL with these headers" — typed-ish invocation without MCP.

## Trade-offs (why both, not either)

- **Skill+API wins**: works with any agent that has a shell, zero protocol machinery, humans/CI reuse the same `dm` CLI, skills version in git.
- **MCP wins**: typed JSON-schema tools (function calling beats doc-following), per-tool permission gating + annotations, standard discovery.
- **Safety shifts server-side for REST**: with no MCP tool annotations, the read-only default, bearer token, and audit log must do the whole job. Threat model is the same either way (any local process can read the token file).
- Third pi-specific flavor: a pi extension (`pi.registerTool()`) calling the same REST API — typed tools inside pi without MCP config, but pi-only.

## Alternatives considered

- **Skill+API only** — fine for personal use, but you'd re-add typed gating the moment a second harness shows up. Skipped.
- **Separate REST server** — pointless duplication; same axum router serves both.

## Consequences

- When phase 1 ships, the loopback HTTP surface is built once (REST + `/mcp` side by side over shared handlers) — marginal cost over MCP-only is small.
- Ship the MCP server for MCP-capable harnesses; ship the `dm` skill+CLI for everything else. The skill documents the API; the API backs the MCP tools.
