---
type: Learning
title: Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec
description: Requirement (user-stated, 2026-09-15 interview)
tags: [charts, labs, design, architecture, reusable-charts]
timestamp: "2026-09-15T08:08:11.840Z"
---

# Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec

## Requirement (user-stated, 2026-09-15 interview)

Charts must be addable via **two surfaces** later on:

1. **Code only** — writing markup / a spec object (the code surface).
2. **Interacting with UI elements/behaviors** — an interactive builder surface.

## Why it matters

Whatever the code surface can express, the UI surface must be able to **produce and persist** (and vice versa: code authors need a stable target to hand-write). That pushes the chart contract toward a **serializable chart spec object** that both surfaces consume — not just Svelte props bound at authoring time.

This is a hard constraint on the central reusable-chart design being interviewed right now (see [[evidence-chart-architecture]] and [[chart-fundament]]): any design that can't round-trip a chart instance to/from a persisted spec will fail the second surface.

## Still open (interview in progress)

- Q2 pending: do the surfaces author chart **instances** (configure existing types), **new chart types** (add to the 32-card catalog), or instances-now/types-code-only?
- If instances: where do UI-authored charts persist — `/pages`, a new "charts" store, or exported code?
