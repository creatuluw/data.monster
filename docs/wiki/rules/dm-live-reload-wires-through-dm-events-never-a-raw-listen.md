---
type: Rule
title: dm/ live-reload wires through dm-events — never a raw listen()
description: "**The rule**: any view or component that renders `dm/`-managed content subscribes to live changes through `src/lib/dm-events.ts` — `onDmChanged(kind, () => void"
tags: [dm, live-reload, tauri-events, svelte, workspace-file-first]
timestamp: "2026-09-22T14:56:22.952Z"
---

# dm/ live-reload wires through dm-events — never a raw listen()

**The rule**: any view or component that renders `dm/`-managed content subscribes to live changes through `src/lib/dm-events.ts` — `onDmChanged(kind, () => void refresh())` inside `$effect` (the returned `off()` is the effect's automatic cleanup), and errors flow through `initDmEvents(onError)` wired once in the root layout. Never import `listen` from `@tauri-apps/api/event` for dm events.

**When it applies**: every present and future dm/-backed surface — the six views wired in `files-008` (/pages list, /pages/[slug] editor, /query saved queries, ItemEditor, RelationshipEditor, /connect) and everything `files-009`/`files-010` (write-through) will touch.

**Rationale**: files-008 (2026-09-22, commit `d7f0fc7`) established one bus with a pure, unit-tested routing core (`tests/dm-events.test.ts`: routing matrix, malformed-payload defense, `off()` semantics). Hand-rolled listeners would bypass the malformed-payload defense, duplicate per-view unlisten bookkeeping, and fragment the single global error-banner surface. Views keep only their existing refresh functions — wiring is one line each.
