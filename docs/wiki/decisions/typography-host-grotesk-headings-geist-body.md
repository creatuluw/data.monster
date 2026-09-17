---
type: Decision
title: "Typography: Host Grotesk headings, Geist body — Inter dropped"
description: "Typography: Host Grotesk headings, Geist body — Inter dropped"
status: accepted
supersedes: typography-geist-display-syne-dropped
timestamp: "2026-09-16T18:56:37.531Z"
---

# Typography: Host Grotesk headings, Geist body — Inter dropped

# Typography: Host Grotesk headings, Geist body — Inter dropped

## Context

The display face churned through Inter → Squada One/Libre Baskerville → Calluna → Figtree → Poppins → Bricolage Grotesque → Space Grotesk → Syne → Geist ([[typography-geist-display-syne-dropped]]). The user then said: **"use geist for body and host grotesk for headings."** This is the first swap in the chain that changed the *body* face — Inter had been the body font since the original Inter-everywhere decision ([[typography-inter-mono-for-data]]).

## Choice

- `--font-display: 'Host Grotesk', sans-serif` and `--font-body: 'Geist', sans-serif` in `src/app.css` — Inter dropped from both token fallbacks.
- `src/app.html` css2 link now loads **Geist 100..900 + Geist Mono 100..900 + Host Grotesk 300..700** — Inter removed entirely.
- Headings keep `font-weight: 700` (Host Grotesk variable covers 300–700, so bold is real, not synthesized).
- **Geist Mono** data role unchanged.

## Alternatives considered

- Keep Inter body — superseded by direct user preference.
- Geist for everything (display + body) — user explicitly split roles: Host Grotesk headings, Geist body.

## Rationale

User preference. Host Grotesk is a grotesque designed for headlines (variable 300–700, bold intact); Geist body + Geist Mono data keep the UI/data superfamily pairing from the previous decision — only the display slot moved out of that superfamily.

## Consequences

- Rule [[inter-for-ui-text-geist-mono-only-for-data-detail]] updated to Host Grotesk display / Geist body / Geist Mono data.
- Host Grotesk tops out at 700 — heavier display weights (800/900) don't exist; 700 remains the convention.
- Tour HTML captures embed the Google-Fonts @import → tours need recapture after this swap (see [[tour-html-captures-embed-google-fonts-import]]).
