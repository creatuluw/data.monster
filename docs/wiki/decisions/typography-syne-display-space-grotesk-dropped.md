---
type: Decision
title: "Typography: Syne display — Space Grotesk dropped"
description: "Typography: Syne display — Space Grotesk dropped"
tags: [design, typography, frontend]
status: accepted
supersedes: typography-space-grotesk-display-bricolage-dropped
timestamp: "2026-09-16T18:55:36.053Z"
---

# Typography: Syne display — Space Grotesk dropped

# Typography: Syne display — Space Grotesk dropped

## Context

On 2026-09-16 the display face churned again, one step after the Space Grotesk swap ([[typography-space-grotesk-display-bricolage-dropped]]). The user said: **"use syne."** Only the display face changed — body and mono roles untouched.

## Choice

- `--font-display: 'Syne', 'Inter', sans-serif` in `src/app.css`.
- **Syne variable weights 400–800** loaded from Google Fonts (single css2 link) in `src/app.html`; Space Grotesk removed.
- Headings keep `font-weight: 700`; body **Inter** and **Geist Mono** data role unchanged.

## Alternatives considered

- **Space Grotesk** — worked, but superseded by user preference for Syne's more distinctive geometric look.
- **Bricolage Grotesque / Poppins / Figtree / Calluna / Squada One** — earlier steps in the same churn, all dropped.

## Rationale

User preference. Syne is on Google Fonts as a true variable font covering 400–800 — includes the 700 bold the app uses, no synthetic-bold trap, single font request.

## Consequences

- 400 is the lightest weight available — display text below 400 is not possible, but the app standard is 700 headings only.
- Tour HTML captures embed the Google-Fonts @import → tours need recapture after this swap (see [[tour-html-captures-embed-google-fonts-import]]).
- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated to Syne display / Inter body / Geist Mono data.
