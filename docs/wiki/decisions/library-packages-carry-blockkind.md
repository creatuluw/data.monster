---
type: Decision
title: Library packages carry blockKind — table/text are built-in blocks, not chart types
description: Context
tags: [library, blockKind, page-editor, central-charts]
status: accepted
timestamp: "2026-09-17T08:34:47.888Z"
---

# Library packages carry blockKind — table/text are built-in blocks, not chart types

## Context

`/pages` renders report pages from four block kinds: the chart types **bar** and **heatmap** (registry-driven), plus the built-in **table** and **text** blocks that `PageGrid` renders directly. The mandate was to surface *every* page component in `/library` — but the library registry v1 only knew about chart types.

## Choice

Table and text ship as regular `/library` packages carrying a new **`blockKind`** field:

- `'chart'` entries feed the chart registry exactly as before (they are renderable chart types).
- `'table'` / `'text'` are marked as built-in block kinds: they appear in the `/library` grid with real demos (their detail pages render the actual `TableRenderer` / text card), but they are **not** registered into the chart registry.

## Alternatives considered

- **Register table/text as chart types** — rejected: `PageGrid` renders these two kinds directly as built-ins; pushing them through the chart-type registry would corrupt block-spec validation.
- **Exclude table/text from `/library`** — rejected: breaks the "every page component is visible in the library" invariant.

## Consequences

- The library grid now shows 4 components (+ dev card); page builders can discover all block kinds in one place.
- Anything that iterates library packages must branch on `blockKind` before assuming a chart renderer exists.
- Future built-in block kinds follow the same pattern: package + `blockKind`, never a fake chart registration.
