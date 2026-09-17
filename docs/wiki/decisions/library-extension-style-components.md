---
type: Decision
title: Library components are self-contained extension-style packages — own definition, logic, and data
description: Context
tags: [library, components, architecture, extensions, spec-interview]
status: accepted
timestamp: "2026-09-17T07:04:02.525Z"
---

# Library components are self-contained extension-style packages — own definition, logic, and data

## Context

Fourth exchange in the `/library` spec interview (parent: [[library-central-component-library]]; Q2 locked display-only registry v1: [[library-q2-registry-display-only]]; Q3 locked master-detail layout: [[library-q3-master-detail-layout]]). The user stated the component model for the library:

- **Each component has its own card.**
- **Clicking a card shows a demo of the component**, with **dummy data** if the component needs data to render.
- **Each component carries its own definitions in code, logic, and data** — self-contained.
- **The goal: one way of wiring in components, like extensions** — so any dev can create a new component by importing assets into the defined data and code structures.

(Q4 — where the demo appears: detail route `/library/<id>` vs modal vs inline expansion — was asked, **unanswered**.)

## The choice (user-stated goal, 2026-09-17)

Library components are **self-contained extension-style packages**: definition + logic + (dummy) data travel with the component, plugged into one standard wiring mechanism. The demo-on-card behavior is part of the contract — a component must be renderable standalone with its own dummy data.

## Rationale

- One wiring path means adding a component is purely additive — no per-component plumbing in the app shell
- Self-containment makes components portable/testable in isolation and demos automatic
- Extension-style authoring opens component creation to any dev (and later agents) without touching app internals

## Alternatives considered

- Centrally-defined components (app owns each component's registration, data, and demo wiring) — rejected: N-th component costs N integrations; the user explicitly wants import-and-it-works

## Consequences

- The library registry defines a component contract: metadata + render + demo data source
- Demos must not depend on live workspace tables — dummy data ships with the component
- Tension to resolve: Q3 locked a master-detail (sidebar index + right pane) layout, while the user now describes a **card grid**; Q4's answer (route vs modal vs inline) will settle the demo surface
- Builds on the shared chart fundament ([labs-charts-reusable-fundament](../rules/labs-charts-reusable-fundament.md)) and the central-charts component set ([central-charts-component-system](../pages/entities/central-charts-component-system.md))

## Status

Accepted as the stated goal of the spec; mechanics of the wiring/contract still being defined in the interview.
