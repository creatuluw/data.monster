---
type: Decision
title: Skeleton card is the inline role-assignment surface — pick/create in-chart, drawer optional
description: Context
tags: [central-charts, page-editor, ux, skeleton]
status: accepted
timestamp: "2026-09-17T15:32:10.396Z"
---

# Skeleton card is the inline role-assignment surface — pick/create in-chart, drawer optional

## Context

The needsSetup gate decision ([[chart-blocks-start-empty-needssetup-gate]]) shipped skeleton cards whose "Add dimension / Add measure" buttons opened the config drawer. User asked (2026-09-17, PR #10) for an in-chart UX: pick or create dimensions/measures right on the chart, without leaving the canvas.

## Decision

The skeleton card itself is the assignment surface. `SkeletonSetup.svelte` renders inline grouped dropdowns per unmet role — ⭐ master items | source-table fields | linked-table fields | `✚ Create…` — plus a two-field inline create form (label + expression) that saves a master item via the host and wires the ref. The drawer remains for full editing, but the common path never opens it.

Verified by CDP e2e both directions: data renders only when all role minimums are met, and **removing** a role (deleting the measure row) re-engages the skeleton.

## Alternatives

- Keep drawer-only assignment — rejected: extra clicks for the 90% case.
- A floating popover from chart marks — rejected: more moving parts; the skeleton already occupies the same space.

## Consequences

- Pick/create UX on unconfigured blocks must stay on the card; don't route new affordances through the drawer by default.
- The pick-value codec is shared ([[pick-values-flow-through-one-codec-pickers-ts]]), so drawer and card cannot drift.
- The host (`/pages/[slug]` + PageGrid) owns master-item persistence via the `onCreateMasterItem` callback; SkeletonSetup stays persistence-free.

Related: [[chart-blocks-start-empty-needssetup-gate]], [[skeletonsetup-component]], [[adding-a-component-never-auto-opens-the-config]].
