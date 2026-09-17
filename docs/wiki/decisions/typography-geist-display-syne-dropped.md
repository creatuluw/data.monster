---
type: Decision
title: "Typography: Geist display — Syne dropped"
description: Context
tags: [design, typography, frontend]
status: accepted
supersedes: typography-space-grotesk-display-bricolage-dropped
timestamp: "2026-09-16T18:55:43.079Z"
---

# Typography: Geist display — Syne dropped

## Context

The display face churned through Inter → Squada One/Libre Baskerville → Calluna → Figtree → Poppins → Bricolage Grotesque → Space Grotesk ([[typography-space-grotesk-display-bricolage-dropped]]), then an interim, unrecorded swap to **Syne**. The user then said: **"use geist."** Only the display face changed — body and mono roles untouched.

## Choice

- `--font-display: 'Geist', 'Inter', sans-serif` in `src/app.css`; Syne removed.
- **Geist variable weights 100–900** loaded from Google Fonts in `src/app.html` — the single css2 request now carries Geist + Geist Mono + Inter.
- Headings keep `font-weight: 700`; body **Inter** and **Geist Mono** data role unchanged.

## Alternatives considered

- **Space Grotesk** — worked, but capped at 700 and superseded by user preference.
- **Syne** (interim, unrecorded) — dropped by user preference.

## Rationale

User preference. Geist is on Google Fonts as a true variable font covering 100–900 — no weight ceiling, no synthetic-bold trap, still one font request. Pairing Geist display with Geist Mono data means UI hierarchy and machine-data texture now come from one typographic superfamily.

## Consequences

- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated to Geist display / Inter body / Geist Mono data.
- Tour HTML captures embed the Google-Fonts @import → all 8 tours recaptured after this swap (see [[tour-html-captures-embed-google-fonts-import]]).
