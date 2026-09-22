---
type: Rule
title: Resize requests use the app's existing size classes — never ad-hoc multipliers
description: Resize requests map onto the app's existing size classes — never invent ad-hoc pixel multipliers.
tags: [design-system, styling, ui]
timestamp: "2026-09-22T13:01:40.028Z"
---

# Resize requests use the app's existing size classes — never ad-hoc multipliers

## The guideline

When asked to make a UI element bigger or smaller, **scale it via the app's existing size classes / design tokens** (e.g. `.btn-lg` in `app.css`), not by multiplying the current values (`font-size: 3em`, icon ×3, padding ×3).

Literal multipliers produce caricature-scale UI — a 33px-text pill button — and get rejected. The design system's coarse step up (`.btn-lg`: normal `text-sm`, roomier padding, slightly larger icon) is usually what "bigger" actually means.

## When it applies

Any "make X bigger/smaller" style request in the app UI.

## Rationale / evidence

2026-09-22, `/workspaces` Add button: user asked for 3× bigger, the literal implementation shipped, and the immediate response was "thats way too big" — reverted to the existing `.btn-lg` class with zero custom sizing. Also aligns with the ponytail ladder: what's already in the codebase wins over new code.
