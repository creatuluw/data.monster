---
type: Entity
title: Agent docs system
description: The repo-authored documentation layer that teaches coding agents to operate the app by editing workspace files. Markdown sources live at `src-tauri/agent-docs/`
tags: [agents, docs, dm-tree, workspace, progressive-disclosure]
timestamp: "2026-09-22T15:30:09.094Z"
---

# Agent docs system

The repo-authored documentation layer that teaches coding agents to operate the app by editing workspace files. Markdown sources live at `src-tauri/agent-docs/`, are embedded into the Rust binary via `include_str!`, and are synced into the workspace as `dm/docs/` (plus the workspace-root `README.md`) when the app version changes (version marker: `dm/docs/.version`).

Structure is skill-style progressive disclosure, budgets enforced by tests:

- **L0** `README.md` (workspace root, ≤60 lines) — "this folder IS the app's state": the tree, the rules (INDEX-first, filename-is-identity, no secrets, atomic writes, invalid JSON is safe), version note
- **L1** `dm/docs/INDEX.md` (≤100 lines) — intent → file → doc routing
- **L2** `formats/*.md` (≤150 lines each) — one per content format (`page-doc`, `master-items`, `relationships`, `saved-queries`, `connections`), each with a `Read when:` header and one annotated example
- **L3** `reference/` (exhaustive, e.g. `page-doc-fields.md`) plus `recipes/common-tasks.md` and `concepts.md`

The secret policy is asserted by tests: the `connections` example shows env-var references only, while the `.env` sample intentionally shows a placeholder URL (see [the secret-policy learning](../../learnings/secret-policy-test-targets-the-connections-json-example-the-.md)).

## Details

- **Location**: `src-tauri/agent-docs/` (markdown sources) · `src-tauri/src/commands/agent_docs.rs` (embed + version-checked sync, 127 lines)
- **Interface**: sync runs on workspace open / version change → writes `dm/docs/**` + `README.md` into the active workspace
- **Configuration**: line budgets (60/100/150) and secret policy enforced by `#[cfg(test)]` tests in `agent_docs.rs`

## Relationships

- [decisions/workspace-files-are-canonical-agents-author-content-by-editi](./decisions-workspace-files-are-canonical-agents-author-conten.md) — the decision this docs layer serves: files are the interface agents author through
- [agent-prompts-page](./agent-prompts-page.md) — the human-facing copy-paste prompts that route agents into these docs
- [incoming-drop-folder](./incoming-drop-folder.md) — the no-docs-needed path: dropping a data file needs no reading at all
- [rules/secrets-never-live-in-workspace-content-files-env-references](./rules-secrets-never-live-in-workspace-content-files-env-refe.md) — the policy the docs teach and the tests assert

## Lifecycle

- First added: files-011, 2026-09-22 (commit `c122569`) — phase C of [pages/artifacts/workspace-file-first-spec-tasks](./workspace-file-first-spec-tasks.md)
