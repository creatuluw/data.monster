---
type: Decision
title: "Typography settles: Inter everywhere (display + body), Geist Mono for data detail"
description: Context
tags: [design, typography, tokens, inter, geist-mono, fonts]
status: accepted
supersedes: "["typography-host-grotesk-headings-geist-body"]"
timestamp: "2026-09-16T19:02:41.641Z"
---

# Typography settles: Inter everywhere (display + body), Geist Mono for data detail

## Context

2026-09-16 was a full day of display-face churn: Inter → Squada One/Libre Baskerville → Calluna → Figtree (×2) → Poppins → Bricolage → Space Grotesk → Syne → Geist → Host Grotesk + Geist body. After seeing Host Grotesk headings in the app, the user asked to **change body and headings back to Inter**.

## Decision

Full circle back to the original [[typography-inter-mono-for-data]] system, now settled:

- `--font-display: 'Inter', sans-serif` (headings, bold 700)
- `--font-body: 'Inter', sans-serif` (all other UI text)
- `--font-mono: 'Geist Mono', ...` unchanged — reserved for data detail (table cells/headers, chart ticks/legends, tags, IDs)
- `app.html` Google Fonts link reduced to **Inter + Geist Mono** (one css2 request); Host Grotesk and Geist (sans) dropped entirely.

Enforced by the rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] (two-font system).

## Alternatives considered

- Keep Host Grotesk/Geist — rejected: user reverted after seeing it live.
- Inter display + Geist body hybrid — rejected: one family everywhere is simpler and reads more neutral/professional for a data app.

## Rationale

A single neutral family (Inter) for all UI chrome with mono marking "machine data" is the calm, professional baseline the exploration kept circling back to. Two families loaded total, one font request.

## Consequences

- The eleven-way display experiment is closed; `--font-display` stays Inter unless the user explicitly reopens it.
- Tour HTML captures embed the Google-Fonts @import — existing captures still reference older font sets until recaptured ([[tour-html-captures-embed-google-fonts-import]]).
