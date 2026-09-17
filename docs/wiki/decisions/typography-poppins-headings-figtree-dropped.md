---
type: Decision
title: "Typography: Poppins headings — Figtree dropped"
description: Context
tags: [design, typography, frontend]
status: accepted
supersedes: ["typography-figtree-headings-calluna-dropped", "typography-figtree-bold-display-inter-body"]
timestamp: "2026-09-16T18:51:15.662Z"
---

# Typography: Poppins headings — Figtree dropped

## Context

On 2026-09-16 the display face had just been swapped Calluna → Figtree (see [[typography-figtree-headings-calluna-dropped]]). Immediately after, the user asked: **"for headings use Poppins."** Body and mono roles were not up for discussion — only the display face churns again.

## Choice

- `--font-display: 'Poppins', 'Inter', sans-serif` in `src/app.css`.
- Poppins **static weights 400/600/700** loaded from Google Fonts in `src/app.html`; **Figtree removed** from the link.
- Headings keep `font-weight: 700`.
- Body **Inter** and **Geist Mono** data role unchanged — the three-role split survives, only the display face changed.

## Alternatives considered

- **Figtree** (previous decision) — worked fine (true variable font), but user preferred Poppins's geometric look.
- **Calluna** — not on Google Fonts; `css2` silently drops it (see [[calluna-not-on-google-fonts-css2-drops-silently]]).
- **Squada One / Libre Baskerville** — superseded earlier the same day (single-weight Squada → synthetic bold).

## Rationale

User preference, same-day. Poppins is on Google Fonts with exactly the weights the app uses — no self-hosting, no synthetic-bold trap, font link stays one request.

## Consequences

- Only 400/600/700 are loaded — display weights outside that range (e.g. 800/900) fall back to Inter. Stick to 700 on display text (per [[inter-for-ui-text-geist-mono-only-for-data-detail]]).
- Tour HTML captures embed the Google-Fonts @import → **all 8 tours recaptured** after this swap (see [[tour-html-captures-embed-google-fonts-import]]).
- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated: Poppins display / Inter body / Geist Mono data.
