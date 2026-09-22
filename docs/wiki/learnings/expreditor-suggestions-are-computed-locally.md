---
type: Learning
title: ExprEditor suggestions are computed locally
description: "While hunting the suspected "per-keystroke autocomplete invoke flood" (bug #4 of the /pages E2E report, 2026-09-22): **no such flood exists — don't chase it aga"
tags: [expreditor, performance, bug-hunt, central-charts]
timestamp: "2026-09-22T11:35:26.420Z"
---

# ExprEditor suggestions are computed locally

While hunting the suspected "per-keystroke autocomplete invoke flood" (bug #4 of the /pages E2E report, 2026-09-22): **no such flood exists — don't chase it again.**

- ExprEditor's suggestion list is computed **locally**: a `$derived` over the function catalog + bound-table fields + master items. Zero invokes per keystroke.
- The only async work per keystroke is the validation query — already debounced 500ms, serialized, and runId-guarded.
- The /query editor is likewise a plain `bind:textarea` with no per-keystroke traffic.

So an autocomplete-driven query storm is structurally impossible in this codebase. The report's #4 mechanism was wrong; the real app hang is the in-process DuckDB deadlock (see [[learnings/hard-reload-storms-deadlock-duckdb-in-process]]), now defended against in the frontend per [[decisions/timeout-and-retry-defend-against-hung-ipc]].
