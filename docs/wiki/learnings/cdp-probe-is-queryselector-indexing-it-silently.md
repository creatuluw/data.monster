---
type: Learning
title: CDP probe `$$` is querySelector — indexing it silently kills clicks
description: "Discovered 2026-09-17 shipping PR #12 (skeleton pick/create modal, CDP e2e steps 1–7)."
tags: [cdp, e2e, testing, probe, gotcha]
timestamp: "2026-09-17T16:12:55.452Z"
---

# CDP probe `$$` is querySelector — indexing it silently kills clicks

Discovered 2026-09-17 shipping PR #12 (skeleton pick/create modal, CDP e2e steps 1–7).

## The bug

In the CDP probe tooling, `$$` is **`querySelector` — it returns ONE element**, not an array. Writing `$$('...button')[0]` yields `undefined`, and `clickEl(undefined)` then **throws silently**. Symptom: a step "fails" while the app itself is fine — the click never happened.

## Why it's nasty

- The failure looks like an app bug (button inert / flow stops mid-way), sending you debugging the product instead of the probe.
- Subsequent step failures are a **cascade**: e.g. measure applied, dimension never did, so every downstream assertion fails too. One dead probe poisons the rest of the run.
- It surfaced only across cold-start runs — on a warm run with proven selectors, step 6 passed end-to-end.

## Rules

- `$$(...)` → one element or null. Use it directly: `clickEl($$('...'))`. Never index it.
- If you need a list, index via a scoped query inside a known container (see [[cdp-form-probes-must-be-container-scoped-shared]]).
- First run after a cold start: vite is still re-optimizing deps, so fixed sleeps under-wait. **Poll-until (condition with timeout) instead of fixed sleeps.**

## Source

- Session 2026-09-17, PR #12 verification run — 5 of 7 steps "failed", root cause was this one probe line; fixed probe → 7/7 green with zero app changes.
