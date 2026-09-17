---
type: Decision
title: "Library Q4: component demos get dedicated views, split into tabs — Preview is the default tab"
description: Context
tags: [library, components, tabs, detail-view, spec-interview, frontend]
status: accepted
timestamp: "2026-09-17T07:05:40.051Z"
---

# Library Q4: component demos get dedicated views, split into tabs — Preview is the default tab

## Context

Fifth exchange in the `/library` spec interview (parent: [[library-central-component-library]]; Q2 locked display-only registry v1: [[library-q2-registry-display-only]]; Q3 locked master-detail layout: [[library-q3-master-detail-layout]]; last exchange locked self-contained extension-style components: [[library-extension-style-components]]). Q4 — where the component demo appears: detail route `/library/<id>` vs modal vs inline expansion — had been asked and left unanswered.

## The choice (user answered "a", 2026-09-17)

**Q4 = A — a dedicated view per component** (not a modal, not inline expansion), and the user added the structure: **all dedicated views are split into tabs, with Preview (the live demo) as the default/first tab.**

Confirmed in the interview: "Preview (live demo, default) is confirmed." Q5 — which tabs exist beyond Preview (Schema / Code / Docs) — is open.

## Rationale

- A full dedicated view gives the live demo room to render at real fidelity — modals and inline expansions constrain size
- Tabs separate concerns (demo vs schema vs source vs docs) without long-scroll or second surfaces
- Preview-first ordering matches the library's purpose: see the component working before anything else

## Alternatives considered

- **Modal demo** — rejected: constrains the live demo's size and interactivity
- **Inline expansion in the index** — rejected: same size constraint, and the component index stays cluttered by open demos

## Consequences

- Each component gets a dedicated view (detail route) whose body is a tab strip with Preview active by default
- The tab set beyond Preview is Q5, still open
- Relation to Q3's master-detail layout needs settling: whether the left index remains and the right pane becomes the dedicated view, or dedicated views replace the right pane entirely — unresolved
- Demos in dedicated views use the component's own dummy data per [[library-extension-style-components]]
- Interview continues (Q5: tab contents)
