---
type: Learning
title: "SveltePlot internals: match datums by position, not identity; guard empty data"
description: "Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implementation. They apply t"
tags: [charts, labs, svelteplot]
timestamp: "2026-09-14T07:50:43.936Z"
---

# SveltePlot internals: match datums by position, not identity; guard empty data

Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implementation. They apply to **every chart-type component ported into Labs** (see [labs-per-chart-type](../decisions/labs-per-chart-type.md)).

## 1. svelteplot re-copies datum records each transform pass

Object identity does not survive a plot pass. Two consequences:

- **Selection can't use `===` on the datum** — svelteplot hands you a fresh copy of each record, so a `selected` object never matches what the mark callback receives. The proven pattern (from the kees reference): compute the selection *stroke* via a **per-datum function (`strokeOf`) that matches on grid position** (e.g. x/y value pair), never on object identity.
- **Direct DOM pokes get wiped** — don't select a cell's DOM node and style it; the next transform pass re-renders it. Encode per-datum state through mark props/functions instead.

## 2. Empty data poisons svelteplot scales with NaN

If zero-row data reaches `Plot`, scale transforms produce NaN and the plot breaks. Guard before rendering: check for empty data in the wrapper component and render a "No data" placeholder instead of a `Plot` with no marks.

## Related

- [[kees-reference-ports-cleanly]] — the reference implementations already encode both workarounds; port them verbatim.
- [[heatmap-component]] — first component using both patterns (`src/lib/components/Heatmap.svelte`).
