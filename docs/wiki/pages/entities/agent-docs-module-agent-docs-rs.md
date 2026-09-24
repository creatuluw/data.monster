---
type: Entity
title: Agent docs module (agent_docs.rs)
description: Agent docs module (agent_docs.rs)
tags: [rust, backend, workspace-file-first, agent-docs, agents]
timestamp: "2026-09-22T15:29:40.284Z"
---

# Agent docs module (agent_docs.rs)

# Agent docs module (agent_docs.rs)

`src-tauri/src/commands/agent_docs.rs` plus its doc sources under `src-tauri/agent-docs/` — the app's documentation **for coding agents**, compiled into the binary with `include_str!` and synced into the workspace as a root `README.md` plus `dm/docs/` (workspace-file-first FR-11, 2026-09-22). This is the zero-config onboarding surface: an agent holding only file tools reads the docs that ship inside the workspace and can author valid content with no MCP, tokens, or install.

## Why it matters

It operationalizes [Workspace files are canonical (decision)](../../decisions/workspace-files-are-canonical-agents-author.md): agents author content by editing `dm/` files, and this module is what teaches them the file formats. Progressive disclosure (L0 README always-read → L1 INDEX routes intent → L2 self-contained format docs, one annotated example each → L3 exhaustive reference) mirrors this repo's own pi-skills/OKF tooling so token cost stays bounded; line budgets are enforced by `tests/agent-docs.test.ts`.

## Details

- **Location**: `src-tauri/src/commands/agent_docs.rs` (sync logic); docs sources in `src-tauri/agent-docs/` (`README.md`, `INDEX.md`, `concepts.md`, `formats/{page-doc,master-items,saved-queries,connections,relationships}.md`, `recipes/common-tasks.md`, `reference/page-doc-fields.md`)
- **Sync**: `sync_agent_docs(ws)` writes everything when `dm/docs/.version` is missing or ≠ the app version, otherwise no-ops; writes via [dm-store-command-module](./dm-store-command-module.md) `atomic_write`
- **README at workspace root is seed-only** — never overwritten once it exists (user customizations win between versions); the canonical copy always lives in `dm/docs/`
- The [dm-watch-command-module](./dm-watch-command-module.md) watcher deliberately ignores `dm/docs/`, `dm/drafts/`, and the root `README.md` — app-owned docs never trigger reload events

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — path conventions + atomic writes underneath
- [dm-watch-command-module](./dm-watch-command-module.md) — ignores the docs tree so app-owned writes don't echo
- [agent-prompts-page](./agent-prompts-page.md) — the /agent page that routes users' agents to these docs
- [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md) — FR-11 of the executable spec

## Lifecycle

- First added: 2026-09-22, FR-11 of the workspace-file-first build
- Docs regenerate automatically whenever the app version changes and `dm/docs/.version` differs
