---
type: Decision
title: Library demos render the real components fed dummy query-shaped data — no demo-only clones
description: Context
tags: [library, components, demo, reuse, spec-interview, frontend]
status: accepted
timestamp: "2026-09-17T07:09:06.554Z"
---

# Library demos render the real components fed dummy query-shaped data — no demo-only clones

## Context

Exchange in the `/library` spec interview (parent: [[library-central-component-library]]; prior locks: Q2 display-only registry [[library-q2-registry-display-only]], Q3 master-detail [[library-q3-master-detail-layout]], Q4 dedicated tabbed views [[library-q4-dedicated-tabbed-views]], self-contained extension-style components [[library-extension-style-components]]). The open question: do library demos render a demo-only clone of each component, or the real component the app itself uses? The user answered "what makes sense?" — delegating the call — and the recommendation was taken as locked (2026-09-17).

## The choice

**A — reuse. Library demos render the real production components**, fed dummy **query-result-shaped** data. No demo-only clones. One renderer, two data sources: live workspace queries in `/pages` production, component-shipped dummy fixtures in the library demo.

## Rationale

- The point of `/library` is showing **the real components the app uses** — a demo-only clone would drift from the real one; the demo would lie.
- One renderer, two data sources = less code. The chart core already takes props; feeding it dummy query-result-shaped data is trivial.
- Extension authors write **one** component (definition + logic + data), exactly matching the "own definitions in code, logic and data" requirement — the same registry entry serves demo and production.

## Alternatives considered

- **Demo-only clone per component** — rejected: drifts from the real component (demo lies), doubles the code to maintain, and forces extension authors to author two things (real + demo).

## Consequences

- Dummy demo data must be shaped like real query results so the unchanged component consumes it — the demo-data contract is "query-result-shaped fixture."
- Strengthens the extension contract from [[library-extension-style-components]]: one entry = demo + production; a component that renders in the library is by construction the component `/pages` uses.
- Q8 (registry's home) asked, **unanswered**: A) TypeScript registry module under `src/lib/library/` (types + `register()` + entries), each component a self-contained folder under `src/lib/library/components/<name>/`; B) user has a specific structure in mind. This was flagged as the last question before writing the spec + todo and building.
- Chart entries build on the shared chart fundament ([labs-charts-reusable-fundament](../rules/labs-charts-reusable-fundament.md)) and the central-charts set ([central-charts-component-system](../pages/entities/central-charts-component-system.md)).
