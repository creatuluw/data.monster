---
type: Decision
title: "Library Q3: /library layout is master-detail — left index + full-size live demo with schema alongside"
description: Context
tags: [library, components, layout, spec-interview, frontend]
status: accepted
timestamp: "2026-09-17T06:59:36.555Z"
---

# Library Q3: /library layout is master-detail — left index + full-size live demo with schema alongside

## Context

Third question in the `/library` spec interview (parent: [[library-central-component-library]]; Q2 locked display-only registry v1: [[library-q2-registry-display-only]]). With the first entry settled (the barchart as it renders on `/pages/smoke-test`), the open question was what the `/library` page itself looks like per component:

- **A)** `/ui`-style: one full-width live demo section per component + short spec header (name, id, purpose), long-scroll page
- **B)** `/labs`-style card grid: thumbnail mini-demos in a responsive grid, click-through to detail pages later
- **C)** Left index + right live demo: sidebar list of registered components; main pane shows the selected component full-size with its config schema alongside (mini editor preview)
- **D)** Other

## The choice (user answered "c", 2026-09-17)

**C — master-detail layout.** A left sidebar indexes the registered components; selecting one shows it rendered full-size in the main pane, with its config schema displayed alongside.

## Rationale

- Full-size live demos (the actual rendered component, not a thumbnail) at all times, without the endless scroll of A — the sidebar keeps the full component index reachable from every selection
- Scales better than long-scroll as component count grows
- The alongside config-schema display is the seed of the eventual editor wiring deferred by Q2 — it *shows* the schema read-only but does not edit, so it stays compatible with Q2's display-only v1

## Alternatives considered

- **A (`/ui`-style long scroll)** — rejected: full demos are good, but the index is only at the top; degrades with many components
- **B (`/labs` card grid)** — rejected: thumbnails can't show a chart at real fidelity; click-through detail pages add a second surface to build

## Consequences

- `/library` v1 is a two-pane page: left component index, right full-size demo + config schema display
- No per-component detail routes needed (unlike B); no long-scroll sections (unlike A)
- The schema pane is display-only in v1 — editing via it remains deferred per [[library-q2-registry-display-only]]
- Interview continues beyond Q3
