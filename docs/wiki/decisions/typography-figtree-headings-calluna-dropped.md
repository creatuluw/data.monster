---
type: Decision
title: "Typography: Figtree headings — Calluna dropped"
description: Context
tags: [design, typography, frontend]
status: superseded
supersedes: "["typography-calluna-headings-inter-body"]"
timestamp: "2026-09-16T18:50:30.043Z"
---

*Superseded by [[typography-poppins-headings-figtree-dropped]] (2026-09-16). Duplicate of [[typography-figtree-bold-display-inter-body]] — parallel recaps recorded the same Figtree state twice.*

# Typography: Figtree headings — Calluna dropped

## Context

On 2026-09-16 the app was retyped to Calluna (display) + Inter (body) + Geist Mono (data). Two problems surfaced immediately: Calluna is **not on Google Fonts** — `css2` returns 200 but silently drops it, so headings fell back to Inter (see [[calluna-not-on-google-fonts-css2-drops-silently]]) — and a Source Sans Pro interim was loaded to cover it. The user then asked for **Figtree** headings.

## Choice

- `--font-display: 'Figtree', 'Inter', sans-serif` in `src/app.css`.
- Figtree **variable 300–900 (+ italic)** loaded from Google Fonts in `src/app.html`; Source Sans Pro removed from the link.
- Headings keep weight **700** from the previous pass.
- Body **Inter** and **Geist Mono** data role unchanged — the three-role split survives, only the display face changed.

## Alternatives considered

- **Calluna** (previous decision) — would require self-hosting since Google Fonts silently drops it.
- **Source Sans Pro** — same-day interim, dropped in this change.
- **Squada One / Libre Baskerville** — superseded earlier the same day (single-weight Squada caused browser-synthesized bold).

## Rationale

Figtree is on Google Fonts as a true variable font (300–900): real bold weights at every size, no self-hosting, no synthetic-bold trap. Keeps the font link to one request.

## Consequences

- Tour HTML captures embed the Google-Fonts @import → **tours must be recaptured** after this change (see [[tour-html-captures-embed-google-fonts-import]]).
- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated: Figtree display / Inter body / Geist Mono data.
