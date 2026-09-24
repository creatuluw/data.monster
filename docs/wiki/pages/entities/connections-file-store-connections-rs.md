---
type: Entity
title: Connections file store (connections.rs)
description: The Rust content-command module that backs **saved PostgreSQL connections** — task `files-005` of the workspace-file-first build (commit `fb2f6fb`, 2026-09-22).
tags: [rust, backend, workspace-file-first, connections, secrets, dm-store]
timestamp: "2026-09-22T14:31:08.584Z"
---

# Connections file store (connections.rs)

The Rust content-command module that backs **saved PostgreSQL connections** — task `files-005` of the workspace-file-first build (commit `fb2f6fb`, 2026-09-22). Fifth of six Phase A content-type migrations. Unlike its siblings it is a **new invoke surface** (there was no saved-connections feature before) and the only one whose secret halves live *outside* the `dm/` tree.

## Why it matters

Lets a user save a PostgreSQL connection by name and reconnect with one click, while keeping the password-bearing connection-string URL out of the workspace's version-controllable files — the end-to-end enforcement of [rules/secrets-never-live-in-workspace-content-files](../../rules/secrets-never-live-in-workspace-content-files.md): an agent can read every `dm/` file and still never see a credential.

## Details

- **Location**: `src-tauri/src/commands/connections.rs` (built on [dm-store-command-module](./dm-store-command-module.md))
- **Doc format**: `dm/connections.json` holds `{connections: [{name, urlEnv}]}` — non-secret metadata only. A test asserts the doc never contains anything secret-shaped
- **Secret store**: the full connection URL (password inside) lives in the workspace `.env` (gitignored) as `DM_CONN_<SLUG>_URL` — slug of the connection name, uppercased, `-`→`_` (`env_var_for("Prod DB")` → `DM_CONN_PROD_DB_URL`)
- **`.env` writer** (`ws_env_set`): append-or-replace of exactly its own var, preserving every other line and comment; atomic write — never a partial `.env`. The `.env` is user-owned: delete removes only the `dm/connections.json` entry, never `.env` lines
- **Resolver** (`ws_env_value`): workspace `.env` first, then process env; a missing var errors naming the exact var + `.env` path ("DM_CONN_PROD_URL is not set — add it to …\.env (gitignored)")
- **Fail loud**: corrupt doc → `check_json_object` error, save refuses — [rules/single-doc-stores-fail-loud-never-clobber](../../rules/single-doc-stores-fail-loud-never-clobber.md); save is read-modify-write upsert-by-name
- **Invoke surface** (all new): `list_connections`, `save_connection(name, url)`, `delete_connection(name)` (idempotent), `resolve_connection(name) → {url}`
- **Frontend**: `/connect` Postgres tab gains a saved-connections select (Connect / Delete) + a "Save this connection as…" row; loads on tab activation; resolve fills the URL field and connects

## Lifecycle

- First added: 2026-09-22, task `files-005` (commit `fb2f6fb` on the workspace-file-first branch; suite 102/102, svelte-check clean)

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — path conventions, atomic writes, shape checks underneath
- [rules/secrets-never-live-in-workspace-content-files](../../rules/secrets-never-live-in-workspace-content-files.md) — the rule this module enforces end to end
- [saved-queries-file-store-saved-queries-rs](./saved-queries-file-store-saved-queries-rs.md) and [master-items-relationships-file-store-items-rs](./master-items-relationships-file-store-items-rs.md) — sibling Phase A stores; connections differs by storing secrets via `.env` indirection
- [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md) — `files-005` of the executable spec
- [decisions/workspace-files-are-canonical-agents-author](../../decisions/workspace-files-are-canonical-agents-author.md) — the architecture this implements
