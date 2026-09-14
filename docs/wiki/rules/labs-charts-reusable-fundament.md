---
type: Rule
title: All /labs charts are built on the shared reusable-chart fundament
description: All /labs charts are built on the shared reusable-chart fundament
tags: [charts, labs, architecture, components]
timestamp: "2026-09-14T07:56:13.066Z"
---

# All /labs charts are built on the shared reusable-chart fundament

# All /labs charts are built on the shared reusable-chart fundament

## Guideline

Every chart added to `/labs` (per the [labs-per-chart-type decision](../decisions/labs-per-chart-type.md)) is not a throwaway playground — it is a **candidate report-authoring component**. Build it on the shared fundament so it can graduate into the app's chart library without rework:

- **Accessor props API** — component takes data + accessors (not a bespoke shape), so any query result maps onto it
- **Bindable selection** — selected marks are a bindable prop, not internal-only state
- **Tooltip snippet pattern** — one reusable tooltip snippet, shared across chart types
- **DS theming + Geist Mono for data detail** — design-system tokens, mono for numeric/data labels (per typography rule)
- **Empty-data guard** — render a graceful empty state, never crash on zero rows
- **`svelte-check` clean** before the chart ships via PR

## When it applies

Any new chart-type card in `/labs` (heatmap was the first, bar chart the second) and any change to an existing labs chart component.

## Rationale

User directive (2026-09-14, bar-chart task): labs charts "need an architecture to easily convert to a reusable chart for our app in the future — as for all charts we build into /labs" and a "strong fundament and architecture for re-usable chart components we can use for later report authoring." Labs is the proving ground; report authoring is the destination. Related: [SveltePlot datum-identity/empty-guard learning](../learnings/svelteplot-datum-identity-empty-guard.md).

## Source

- Established during the heatmap port and formalized in the bar-chart TODO (`TODO-b476a398`, 2026-09-14): components live in `src/lib/components/`, playground routes at `/labs/<type>` on synthetic data.
