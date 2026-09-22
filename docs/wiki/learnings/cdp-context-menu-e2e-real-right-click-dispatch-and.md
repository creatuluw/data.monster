---
type: Learning
title: "CDP context-menu e2e: real right-click dispatch, and check the binding before blaming synthetic events"
description: "Discovered 2026-09-17 shipping PR #16 (virtual tab system, CDP e2e steps 1–7). Extends the synthetic-event family: [[cdp-can-click-svelteplot-marks-dispatchmous"
tags: [cdp, e2e, contextmenu, debugging]
timestamp: "2026-09-17T16:58:47.818Z"
---

# CDP context-menu e2e: real right-click dispatch, and check the binding before blaming synthetic events

Discovered 2026-09-17 shipping PR #16 (virtual tab system, CDP e2e steps 1–7). Extends the synthetic-event family: [[cdp-can-click-svelteplot-marks-dispatchmouseevent]], [[cdp-cannot-synthesize-clicks-on-svelteplot-marks]].

## Symptom

CDP e2e of the right-click → "Open in new tab" flow: a synthetic `contextmenu` dispatched from `Runtime.evaluate` bubbled fine, but the handler stayed silent. Switching to a **real** `Input.dispatchMouseEvent` with `button: 'right', clickCount: 1` also stayed silent — and that's what exposed the truth: the handler existed in `<script>` but `<svelte:window oncontextmenu={...}>` was never written into the markup.

## Lessons

- For context-menu e2e, the real user path is `Input.dispatchMouseEvent` with `button: 'right'` (synthesized DOM events may not reach Svelte window-level handlers).
- **Before blaming event synthesis, verify the binding actually exists** — grep the file for the `<svelte:window …>` / `on:…` attribute. A "synthetic events don't work" conclusion on top of a missing binding sends you debugging the wrong layer.
- Read probe results *after* the action completes, not before — a reordered `before` read made a working tab-switch look failed.
