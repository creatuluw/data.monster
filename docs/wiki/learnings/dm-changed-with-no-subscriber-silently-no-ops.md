---
type: Learning
title: "dm:changed with no subscriber silently no-ops — incoming ingest's kind:"table" had zero listeners"
description: "Found by the workspace-file-first e2e pass (2026-09-22): the `data/incoming/` ingest correctly emitted `dm:changed {kind:"table"}` via the [[dm-watch-command-mo"
tags: [dm-events, live-reload, tauri-events, e2e]
timestamp: "2026-09-22T15:47:07.718Z"
---

# dm:changed with no subscriber silently no-ops — incoming ingest's kind:"table" had zero listeners

Found by the workspace-file-first e2e pass (2026-09-22): the `data/incoming/` ingest correctly emitted `dm:changed {kind:"table"}` via the [[dm-watch-command-module]] watcher, but **no frontend view subscribed to that kind** — the header table count stayed stale until a manual reload. Fixed in commit `58d713a`: the layout now refreshes `app.tables` on that event (6→7→8 live across two drops).

**The invariant**: a `dm:changed` kind with zero subscribers is a silent no-op — no error, no warning, the event just evaporates. Emitting a new kind (or adding a UI surface that displays dm-derived state) must include wiring its frontend listener through [[dm-events-frontend-module]] in the same change. This is the mirror of the emit-side rule ("dm/ live-reload wires through dm-events — never a raw listen()"): the bus is only as complete as its subscribers.
