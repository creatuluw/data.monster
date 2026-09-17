---
type: Decision
title: "Typography: Figtree bold display, Inter body, Geist Mono data"
description: Context
tags: [design, typography, frontend]
status: superseded
supersedes: typography-calluna-headings-inter-body
timestamp: "2026-09-16T18:50:46.427Z"
---

*Superseded by [[typography-poppins-headings-figtree-dropped]] (2026-09-16). Duplicate of [[typography-figtree-headings-calluna-dropped]] — parallel recaps recorded the same Figtree state twice.*

# Typography: Figtree bold display, Inter body, Geist Mono data

## Context

2026-09-16 was a display-type churn day: Inter-only → Squada One/Libre Baskerville ([[typography-squada-one-headings-libre-baskerville]]) → Calluna ([[typography-calluna-headings-inter-body]]). This turn then moved headings to **Source Sans Pro, bold** at the user's request, and the working tree subsequently settled on **Figtree** (variable 300–900, loaded in `app.html`) — with Source Sans Pro no longer referenced anywhere in `src/`.

## The choice

- `--font-display: 'Figtree', 'Inter', sans-serif` — a boldable sans display face, replacing the serif Calluna look.
- **Every display-font rule block carries `font-weight: 700`** — this turn ran a mechanical pass setting all ~106 display blocks (base `h1–h6`, page titles, card/section headings) to 700; existing 600s became 700, blocks without a weight got one. The 700s persist under Figtree.
- Body (Inter) and Geist Mono data roles are untouched — the three-font split survives, see [[inter-for-ui-text-geist-mono-only-for-data-detail]].

## Why this over the alternatives

- **Calluna (serif)** — user moved away from the serif display look; also not on Google Fonts (see [[calluna-not-on-google-fonts-css2-drops-silently]]), needing self-hosting.
- **Source Sans Pro** — this turn's pick; replaced by Figtree within the same uncommitted retype (rationale not captured in-session; Figtree serves as a variable font on Google Fonts, so any weight incl. 700 resolves from one family).
- **Bold 700** — the user explicitly asked for bold headings; a single fixed display weight keeps hierarchy simple (no per-heading weight decisions).

## Consequences

- Tour HTML captures embed the Google-Fonts `@import` — font changes require recapturing tours (see [[tour-html-captures-embed-google-fonts-import]]).
- New surfaces: headings take `var(--font-display)` + weight 700; don't reintroduce serif display or weight-600 headings.
