---
type: Decision
title: "Live-reload mechanics: notify watcher → validate → dm:changed/dm:error events"
description: Shipped as files-007 (2026-09-22): notify watcher + 300ms debounce + writer-fed echo suppression emits dm:changed/dm:error; frontend listeners pending files-008.
tags: [agents, workspace, live-reload, file-watcher, tauri-events, watcher-shipped]
status: accepted
timestamp: "2026-09-22T13:21:28.681Z"
---

# Live-reload mechanics: notify watcher → validate → dm:changed/dm:error events

## Context

Q3 of the agent-authored-workspace interview (2026-09-22), after conventions locked. With [[decisions/agent-authors-app-content-by-editing-workspace-files-the-wor]] the agent writes workspace files (e.g. `dm/pages/revenue.json`) and the app must react — this is what makes it feel alive ("watch the agent build the page") vs. batch import. All the pieces already exist: Tauri's event system, the `notify` crate in Rust, and the earlier `refresh_ui` idea from the MCP research — delivered here by filesystem instead of API.

## The flow (as shipped in files-007)

```
agent writes dm/pages/revenue.json
  → notify watcher fires (debounced ~300ms — files settle across multiple writes)
  → Rust validates: parse + validatePageDoc (validator never throws)
  → valid:   emit Tauri event "dm:changed" {kind, slug} → open views reload that content live
  → invalid: emit Tauri event "dm:error" {path, errors} → problems surface in-app, nothing breaks
```

## Two traps designed around (apply regardless of watcher depth)

- **Echo suppression** — the app writes the same files (pages editor auto-saves every 60s, silently — see [[learnings/pages-editor-auto-saves-silently-every-60s-no-ui]]). The watcher must ignore the app's own writes or every save triggers a reload loop. Standard fix: Rust tracks "paths I just wrote" with a timestamp window.
- **The clobber** — agent rewrites a file the user has open with unsaved edits. Silent last-write-wins would eat someone's work. VS Code's answer: a "file changed on disk — Reload / Keep mine" banner on the affected editor. Explicit, cheap, no merge machinery.

## Alternatives (Q3 — A chosen, shipped)

- **A. Full watcher** (lean) — `notify` crate, live reload of open views, error events for bad files. ~80 lines of Rust + one small dep; the difference between "the agent did stuff" and "watch the agent do stuff".
- **B. Reload on focus** — no watcher; content re-reads on window focus/refresh. Zero deps, but no live view. The ponytail fallback if the dep is deferred.
- **C. Hybrid** — watcher for pages only (where live-building matters), focus-reload for master items/saved queries.

## Shipped (2026-09-22, files-007)

Approach A landed as `files-007` (commit `72bb92e`): notify recursive watch on `dm/` → 300ms debounced flusher (removals win over modifies in-window) → echo check → pure `classify` → `dm:changed {kind, name, removed}` / `dm:error {path, reason}`. Echo suppression shipped with the writer, not the watcher: `dm_store::atomic_write` marks destination paths (600ms registry window ≥ 400ms write debounce) — see [dm_watch command module](../pages/entities/dm-watch-command-module.md). Remaining: the frontend half — `dm:changed`/`dm:error` listeners, store invalidation, toasts and the clobber banner — is `files-008`.
## Consequences

- Whatever depth lands, echo suppression and the clobber banner are required either way.
- Status locks when the user answers Q3; parent interview decisions remain [[decisions/agent-authors-app-content-by-editing-workspace-files-the-wor]] (proposed).
