---
type: Decision
title: "Library Q2: registry v1 is display-only — editor wiring deferred"
description: Context
tags: [library, components, registry, pages, scope, spec-interview]
status: accepted
timestamp: "2026-09-17T06:55:23.690Z"
---

# Library Q2: registry v1 is display-only — editor wiring deferred

## Context

Follow-up question in the `/library` spec interview (parent: [[library-central-component-library]]). The stated goal — "adding a chart to the library makes it available to the full app" — needed a mechanical meaning. Question 2 offered:

- **A)** Registry drives the page editor: components self-register (id, label, demo, config schema) in one central registry module; the `/pages` editor's add-block picker auto-lists everything registered — library page and editor read the same source
- **B)** Registry v1 is display-only: `/library` just shows the components with live demos; wiring it into the page editor's block picker is a separate later task
- **C)** Registry drives the editor picker AND each entry declares its config-panel schema, so the editor gets full configuration for free — bigger scope now

## The choice (user answered "b", 2026-09-17)

**B — registry v1 is display-only.** `/library` in v1 is a demo/gallery surface showing registered components with live demos. Making those components appear in the page editor's block picker is explicitly a **separate later task**, not part of v1.

## Rationale

Smallest thing that delivers value: a place to browse and demo components now, with the editor integration deferred until the registry shape proves itself. Mirrors how central-charts v1 scoped down ([[central-charts-v1-scope-bar-heatmap-table]]) rather than building the full integration surface up front.

## Alternatives considered

- **A (registry drives editor picker)** — rejected for v1: couples the library page build to the page-editor block picker before either's shape is settled
- **C (A + config schema in registry)** — rejected for v1: biggest scope; config schemas per entry duplicate/anticipate what the editor's own option panels ([[q9-chart-option-panels-schema-driven]]) will need

## Consequences

- Adding a component to `/library` in v1 does **not** make it appear in the `/pages` editor — expect this; it is intentional, not a bug
- A/C remain the growth path: when editor wiring lands, the registry module gains id/label/demo (+ config schema) and the picker reads it
- First v1 entry is still the barchart as shown on `/pages/smoke-test` ([[barchart-component]])
- Q1 (relation to `/labs`) was still open at the parent decision's last update

## Status

Accepted (interview answer, 2026-09-17).
