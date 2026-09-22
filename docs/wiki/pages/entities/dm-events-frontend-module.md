---
type: Entity
title: dm-events frontend module
description: "`src/lib/dm-events.ts` — the frontend half of the dm/ live-reload loop (workspace-file-first task `files-008`, commit `d7f0fc7`): one Tauri event bus for `dm:ch"
tags: [dm, live-reload, tauri-events, svelte, workspace-file-first]
timestamp: "2026-09-22T14:56:22.952Z"
---

# dm-events frontend module

`src/lib/dm-events.ts` — the frontend half of the dm/ live-reload loop (workspace-file-first task `files-008`, commit `d7f0fc7`): one Tauri event bus for `dm:changed` / `dm:error`, a pure routing core, and per-kind subscriptions that views use to re-fetch. This is what closes the realtime agent→app loop: an agent writes `dm/pages/revenue.json` → the Rust watcher validates → this bus re-fetches every open view touching that content within ~300ms; parse errors surface in the global error banner + console instead of silently stale views.

## Why it matters

Without it, agent-authored workspace file changes only appear after a manual reload, and a broken JSON file fails invisibly. It makes "watch the agent build the page" possible and keeps the app honest about file-level validation errors.

## Details

- **Location**: `src/lib/dm-events.ts`; tests: `tests/dm-events.test.ts` (6 tests: routing matrix, malformed-payload defense, registry `off()` semantics)
- **Interface**:
  - Types: `DmChanged {kind, name, removed}`, `DmError {path, reason}`
  - `onDmChanged(kind, cb)` → returns `off()` (use as `$effect` cleanup)
  - `onDmError(cb)` → returns `off()`
  - `dispatchChanged(e)` / `dispatchError(e)` — internal dispatch
  - `routeDmEvent(handlers, event, payload)` — pure routing core; ignores malformed payloads (defensive; the watcher never sends them)
  - `initDmEvents(onError?)` — idempotent; called once from the root layout, wires the two `listen()` calls, `console.warn`s errors, and forwards them to the global error banner
- **Configuration**: none

## Relationships

- [dm-watch-command-module](./dm-watch-command-module.md) — the Rust producer: notify watcher → validate → emits `dm:changed`/`dm:error` that this bus consumes
- [dm-store-command-module](./dm-store-command-module.md) — writer-side echo suppression lives in its `atomic_write`; views re-fetch through dm_store-backed commands
- [workspace-file-first spec & tasks](../artifacts/workspace-file-first-spec-tasks.md) — the spec artifact; this module is task `files-008`

## Lifecycle

- First added: 2026-09-22 — `files-008` (commit `d7f0fc7`, branch `feature/workspace-file-first`)
- Wired views at birth: `/pages` list (`page`), `/pages/[slug]` editor (`page` + `measure` + `dimension` + `relationships`), `/query` saved queries (`saved-query`), `ItemEditor` (per-kind), `RelationshipEditor` (`relationships`), `/connect` connections (`connections`) — each one line: its existing refresh fn inside `$effect(() => onDmChanged(...))`
- Still pending in the phase: `files-009` write-through editor with the Reload/Keep-mine clobber banner, `files-010` write-through for all drawers

## Source

- `src/lib/dm-events.ts` — the module
- `tests/dm-events.test.ts` — routing-core tests
