---
type: Artifact
title: LLM agent connection research report
description: Fractal-research report on how to connect any LLM / coding agent / harness to data.monster and let it operate the app — add data & content, run analysis. Produc
tags: [mcp, agents, research, llm]
timestamp: "2026-09-15T15:43:57.143Z"
---

# LLM agent connection research report

Fractal-research report on how to connect any LLM / coding agent / harness to data.monster and let it operate the app — add data & content, run analysis. Produced 2026-09-15 from 5 parallel grounded research branches (~45 primary sources fetched, all citations verified).

Its headline recommendation — embed an MCP server (Streamable HTTP + rmcp) in the Rust backend with a thin tool layer over the existing command modules and layered safety — is recorded as [agent-connection-mcp-embedded-in-rust-backend](../../decisions/agent-connection-mcp-embedded-in-rust-backend.md) (status: proposed).

## What it documents

- [agent-connection-mcp-embedded-in-rust-backend](../../decisions/agent-connection-mcp-embedded-in-rust-backend.md) — the proposed architecture, the tool catalog (discovery, read/write query split, ingest, create_page, refresh_ui), and the alternatives rejected (OpenAI Apps SDK, A2A, Copilot extensions, computer use, CDP)
- Tool-catalog patterns surveyed from MotherDuck / Hex / Deepnote / dbt agent surfaces
- Safety model: read-only default, elicitation-based confirmation (not deprecated sampling), audit log in a separate DuckDB file, `EXPORT DATABASE` checkpoints

## Details

- **Location**: `reports/2026-09-15-llm-agent-connection/`
- **Format**: self-contained `report.html` (fractal-research template) + audit trail — `research.log`, `metrics.json`, branch files under `agents/`
- **Generated from**: fractal-research run, root decomposed into 5 branches (q-002..q-006), fanned out in parallel; 6,978 synthesized words

## Relationships

- [agent-connection-mcp-embedded-in-rust-backend](../../decisions/agent-connection-mcp-embedded-in-rust-backend.md) — the decision this report proposes
- [llm-sensitive-data-privacy-research](./llm-sensitive-data-privacy-research.md) — sibling LLM research artifact; the agent connection inherits its layered-safety posture (secrets stay in Rust, no data sent to cloud)
- [remote-chat-command](../entities/remote-chat-command.md) — the existing in-app LLM path (app calls LLM); the MCP server is the inverse direction (agent drives the app)
