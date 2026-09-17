---
type: Decision
title: "Typography: Bricolage Grotesque display — Poppins dropped"
description: Context
tags: [design, typography, frontend]
status: accepted
supersedes: typography-poppins-headings-figtree-dropped
timestamp: "2026-09-16T18:52:23.404Z"
---

# Typography: Bricolage Grotesque display — Poppins dropped

## Context

The display face has churned rapidly (2026-09-16): Inter → Squada One/Libre Baskerville → Calluna → Figtree → Poppins, searching for the right voice for headings. Poppins was the incumbent ([[typography-poppins-headings-figtree-dropped]]). Inter body and Geist Mono data-detail roles are stable and untouched.

## Decision

Display font is **Bricolage Grotesque** — loaded in `app.html` as the variable font (optical size axis 12–96, weights 200–800) and set on `--font-display: 'Bricolage Grotesque', 'Inter', sans-serif` in `src/app.css`. **Poppins is dropped.** Headings keep `font-weight: 700`; Inter (`--font-body`) and Geist Mono (`--font-mono`) roles unchanged.

## Alternatives considered

- Poppins — previous incumbent, dropped by direct user instruction ("use Bricolage Grotesque").

## Consequences

- Variable-font load makes all weights 200–800 available; bold-700 headings stay by convention, not by loading limits.
- Tour HTML captures embed the Google-Fonts @import, so existing tours still render Poppins until recaptured ([[tour-html-captures-embed-google-fonts-import]]).
- The three-font rule (rules/inter-for-ui-text-geist-mono-only-for-data-detail) now names Bricolage Grotesque as the display face.
