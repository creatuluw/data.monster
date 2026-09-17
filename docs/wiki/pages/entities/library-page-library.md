---
type: Entity
title: Library page (/library)
description: "A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs regist"
tags: [library, components, frontend, route]
timestamp: "2026-09-17T06:54:06.276Z"
---

# Library page (/library)

A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs register components — and adding a chart to the library makes it available app-wide for use on `/pages` report pages. Currently a placeholder; the barchart from `/pages/smoke-test` is slated as the first entry.

## Why it matters

It is the discovery/registration surface between component development and the page editor — the intended single place a component goes from "built" to "usable everywhere."

## Details

- **Location**: `src/routes/library/+page.svelte`
- **Current state**: placeholder — renders `<LabsPlaceholder title="Library" section="" />` (so the head title is `Library — Data Monster`)
- **Reachable from**: the home page's bottom link row (`/labs →` `/library →`), added 2026-09-17
- **Configuration**: none yet

## Relationships

- Placeholder shell: [labsplaceholder-component](./labsplaceholder-component.md) — first consumer outside `/labs`
- First planned entry: [barchart-component](./barchart-component.md) (as used on the smoke-test page)
- Intended consumer: [central-charts-component-system](./central-charts-component-system.md) page editor
- Governing direction (proposed, interview in progress): [library-central-component-library](../../decisions/library-central-component-library.md)

## Lifecycle

- First added: 2026-09-17 — placeholder route + home link; spec interview for its real shape started the same day

## Source

- `src/routes/library/+page.svelte`
- `src/routes/+page.svelte` — home-page link
