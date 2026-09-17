---
type: Decision
title: "Typography: Space Grotesk display, Bricolage dropped"
description: Context
tags: [design, typography, frontend]
status: accepted
supersedes: typography-bricolage-grotesque-display
timestamp: "2026-09-16T18:52:51.437Z"
---

# Typography: Space Grotesk display, Bricolage dropped

## Context

On 2026-09-16 the display face churned again: after Poppins there was a one-step swap to Bricolage Grotesque ([[typography-bricolage-grotesque-display]]). The user then said: **"use Space Grotesk."** Only the display face changed — body and mono roles untouched.

## Choice

- `--font-display: 'Space Grotesk', 'Inter', sans-serif` in `src/app.css`.
- Space Grotesk **variable weights 300–700** loaded from Google Fonts in `src/app.html` (same one-request css2 link); Bricolage Grotesque removed.
- Headings keep `font-weight: 700`; body **Inter** and **Geist Mono** data role unchanged.

## Alternatives considered

- **Bricolage Grotesque** (interim, unrecorded) — dropped by user preference.
- **Poppins** — worked, but superseded; user preferred Space Grotesk's more technical/monospaced-flavored grotesque.

## Rationale

User preference. Space Grotesk is on Google Fonts as a true variable font covering 300–700 — includes the 700 bold the app uses, no synthetic-bold trap, single font request.

## Consequences

- 700 is the heaviest weight Space Grotesk offers anywhere — display text must stay at `font-weight: 700` (per [[inter-for-ui-text-geist-mono-only-for-data-detail]]).
- Tour HTML captures embed the Google-Fonts @import → tours need recapture after this swap (see [[tour-html-captures-embed-google-fonts-import]]).
- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated to Space Grotesk display / Inter body / Geist Mono data.
