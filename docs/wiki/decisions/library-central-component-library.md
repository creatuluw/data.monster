---
type: Decision
title: /library becomes the central component library (proposed — spec interview in progress)
description: Context
tags: [library, components, pages, frontend, spec-interview]
status: proposed
timestamp: "2026-09-17T06:54:06.275Z"
---

# /library becomes the central component library (proposed — spec interview in progress)

## Context

On 2026-09-17 the user kicked off a spec for a `/library` page (route `src/routes/library/+page.svelte`, currently a LabsPlaceholder stub, linked from the home page). Stated goal:

- `/library` shows **all components used in the UI** — one central library surface.
- **Component devs add components to the library**, and adding a chart there **makes it available to the full app** for use on `/pages` report pages.
- First task: add the barchart exactly as shown on `/pages/smoke-test` (the central-charts `barchart-component`).

An interview is in progress (one question at a time, lettered options per [interview-one-question-at-a-time](../rules/interview-one-question-at-a-time.md)).

## The choice (as stated by the user, not yet fully locked)

`/library` is the canonical component library: the registry where page-usable components live, surfaced for devs, and consumed by `/pages`.

## Open question — relation to `/labs` (Q1, asked, unanswered)

- **A)** `/library` is the new canonical home; `/labs` stays untouched for now, superseded/merged later
- **B)** `/library` is specifically the page-editor **block library** (what can be dropped on a `/pages` page); `/labs` stays the separate chart-type experimentation playground
- **C)** `/library` **replaces `/labs` immediately** — one catalog, one entry per component

Q2 (what "available to the full app" means mechanically) is **locked: B — registry v1 is display-only**, editor wiring deferred — see [library-q2-registry-display-only](library-q2-registry-display-only.md).

## Alternatives considered

- Keep `/labs` as the only catalog (status quo) — rejected: `/labs` is placeholder-first experimentation after theunspokenpitch.com, not a registry of app-usable components.

## Consequences

- The relationship to `/labs` (and its 32-card placeholder catalog) decides whether ~30 placeholder cards get superseded — see [labs-catalog-placeholder-first](labs-catalog-placeholder-first.md).
- Whatever lands, chart entries should build on the shared chart fundament ([labs-charts-reusable-fundament](../rules/labs-charts-reusable-fundament.md)) and the central-charts component set ([central-charts-component-system](../pages/entities/central-charts-component-system.md)).

## Status

**Proposed** — spec interview in progress; update/supersede when Q1+ are answered.
