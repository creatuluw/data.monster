---
type: Entity
title: Connections command module (connections.rs)
description: What is it?
tags: [rust, backend, workspace-file-first, connections, dm-store, secrets]
timestamp: "2026-09-22T14:32:21.838Z"
---

# Connections command module (connections.rs)

## What is it?

`src-tauri/src/commands/connections.rs` — the workspace-file-first store for **saved PostgreSQL connections**: connection names live as non-secret metadata in the doc `dm/connections.json`, and the connection-string URLs (which contain passwords) live only in the workspace's gitignored `.env` file under referenced variables. Task `files-005` (FR-5) of the workspace-file-first build; fifth of the Phase A content-type migrations after saved queries (`files-004`).

## Why it matters

Lets a user save a PostgreSQL connection once and reconnect by name from the /connect page — while honoring the secrets rule: workspaces may be git-version-controlled, so `dm/connections.json` never contains a password, only `{name, urlEnv}` references. Agents can read/edit the connections doc with plain file tools.

## Details

- **Location**: `src-tauri/src/commands/connections.rs`; frontend dropdown in `src/routes/connect/+page.svelte`
- **Doc shape**: `dm/connections.json` → `{connections: [{name, urlEnv}]}`; built on [dm-store-command-module](./dm-store-command-module.md) (path conventions, `check_json_object` shape check, atomic write)
- **Invoke commands**: `list_connections`, `save_connection(name, url)`, `delete_connection(name)`, `resolve_connection(name)` — save writes the URL into `.env` under `DM_CONN_<SLUG>_URL` and stores only the name + var reference
- **Env resolution**: `ws_env_value` reads the workspace `.env` first (simple `KEY=VALUE` parse, `#` comments skipped), falls back to process env; `ws_env_set` appends/replaces one line preserving all others, atomic write — never a partial `.env`
- **Fail loud**: corrupt/unshapeable `dm/connections.json` refuses saves (clobber guard, per [single-doc-stores-fail-loud-never-clobber-corrupt-file-list-](../../rules/single-doc-stores-fail-loud-never-clobber-corrupt-file-list-.md))
- **UI**: /connect shows a "Saved connections" select + Connect/Delete buttons once the list is non-empty

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — path conventions, atomic writes, shape checks underneath
- [saved-queries-file-store-saved-queries-rs](./saved-queries-file-store-saved-queries-rs.md) — sibling Phase A store (`files-004`)
- [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md) — `files-005` of the executable spec
- [secrets-never-live-in-workspace-content-files](../../rules/secrets-never-live-in-workspace-content-files-env-references.md) — the rule this store implements
- [workspace-files-are-canonical](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — the architecture this implements

## Lifecycle

- First added: 2026-09-22, task `files-005` on the `feature/workspace-file-first` branch (commits `4b722d9`→`fb2f6fb` range)
