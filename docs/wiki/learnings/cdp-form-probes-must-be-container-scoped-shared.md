---
type: Learning
title: CDP form probes must be container-scoped — shared placeholders between list rows and create forms cause silent wrong-input traps
description: Discovered 2026-09-17 while CDP-testing master-item creation (page editor measure form).
tags: [cdp, e2e, testing, svelte, master-items]
timestamp: "2026-09-17T14:10:03.156Z"
---

# CDP form probes must be container-scoped — shared placeholders between list rows and create forms cause silent wrong-input traps

Discovered 2026-09-17 while CDP-testing master-item creation (page editor measure form).

## Symptom

`createMeasure` silently early-returned: values visibly in the DOM, form still open, no error, nothing saved.

## Two-stage misdiagnosis

1. First blamed event synthesis — "synthetic `input` events aren't reaching Svelte's bind" — and switched to trusted CDP input (`Input.dispatchMouseEvent` + real text insertion, per [[cdp-can-click-svelteplot-marks-dispatchmouseevent]]). Reasonable, but not the cause.
2. Actual cause: the probe's placeholder-based selector (`$$all(...)[0]`) hit the **seeded measure row's inputs**, which share the exact same placeholders as the create form. The trusted keystrokes landed in the row, the create form stayed empty, and `createMeasure` early-returned on empty form fields.

## Heuristic

- **Scope form-fill probes to the form container** (`form $$ input[placeholder=...]`), never placeholder- or index-only `$$all(...)[0]` selectors — duplicate placeholders are common wherever list rows and a create/edit form coexist (master-items, labels, tags).
- Before blaming event synthesis / Svelte bind for "values in DOM but save no-ops", **verify the probe hit the right elements** — dump the matched elements' ancestors first.
