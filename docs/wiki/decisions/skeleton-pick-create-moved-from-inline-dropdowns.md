---
type: Decision
title: Skeleton pick/create moved from inline dropdowns to card buttons opening a modal (searchahead + New)
description: "Skeleton role-assignment: card buttons open a pick/create modal (searchahead + New)"
tags: [central-charts, page-editor, ux, skeleton, modal]
status: accepted
supersedes: "["skeleton-card-is-the-inline-role-assignment"]"
timestamp: "2026-09-17T16:05:15.482Z"
---

# Skeleton pick/create moved from inline dropdowns to card buttons opening a modal (searchahead + New)

# Skeleton role-assignment: card buttons open a pick/create modal (searchahead + New)

## Context

[[skeleton-card-is-the-inline-role-assignment]] shipped `SkeletonSetup.svelte` with **inline grouped dropdowns** per unmet role (⭐ master items | source fields | linked fields | Create…). User asked (2026-09-17) for buttons again instead — each "Add dimension / Add measure" button opens a **modal**, matching the new-page modal pattern, containing a **searchahead dropdown select** over the pick options plus a **+ New** button for creating a master item inline.

## Decision

- The skeleton card keeps the assignment surface (card-not-drawer still holds), but each unmet role renders as a **button**; clicking opens a **modal built on the new-page modal's exact overlay pattern**.
- Inside the modal: a **searchahead** (filter-as-you-type dropdown over grouped pick options) and a **+ New** button (label + expression → master item).
- The searchahead is **built inline in the modal**, not extracted from `SearchAhead.svelte` — that file is a hardcoded `/ui` showcase demo, not prop-driven ([[searchahead-svelte-is-a-ui-showcase-demo-not-prop-driven]]). Extract a real prop-driven component only when a second consumer appears.
- Pick values still flow through the shared codec ([[pick-values-flow-through-one-codec-src-lib-charts]]); the host (`/pages/[slug]` + PageGrid) still owns master-item persistence via `onCreateMasterItem`.

## Alternatives

- Keep inline grouped dropdowns — rejected by user: too dense/cluttered on the card; buttons keep the skeleton visually quiet until needed.
- Prop-driven refactor of `SearchAhead.svelte` — rejected: YAGNI with a single consumer; the inline version is small.

## Consequences

- Modal ≠ drawer: the drawer remains for full editing; the modal is the quick pick/create path from the card.
- Any future modal in the app should reuse the new-page modal overlay pattern rather than invent a new one (user explicitly asked for "as with the new page modal").
- CDP e2e continues to gate this flow (needsSetup gate unchanged).
