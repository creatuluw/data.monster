---
type: Learning
title: CDP e2e cannot synthesize trusted clicks on svelteplot marks
description: "Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, even on known-good labs"
tags: [cdp, e2e, svelteplot, testing]
timestamp: "2026-09-15T10:55:52.612Z"
---

# CDP e2e cannot synthesize trusted clicks on svelteplot marks

> ⚠️ **SUPERSEDED — this conclusion was wrong.** CDP `Input.dispatchMouseEvent` DOES click svelteplot marks; the failures were stale coordinates, `element.click()` (no clientX for hit-testing), and an accidentally closed page target. See [[cdp-can-click-svelteplot-marks-input-dispatchmouseevent-with]].

Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, even on known-good labs pages where manual clicks work. Hit 2026-09-15 while CDP-smoke-testing the central-charts build (`tests/smoke-cdp.mjs`): 11/11 checks passed except click interactions had to be excluded.

What this means for testing strategy:

- CDP smoke can verify render, persistence, routes, and data presence — but **not** interactions that depend on trusted/pointer-event semantics inside svelteplot marks.
- Cover selection logic with unit tests + port click handlers verbatim from code verified manually; ask a human to click a bar to confirm cross-filtering.
- `tests/smoke-cdp.mjs` stays the repeatable e2e (needs the app running with `--remote-debugging-port=9222`).

Extends [[drive-data-monster-s-real-ui-over-cdp]] and [[cdp-repro-traps-duckdb-lock]] — same repro environment, one more limitation.
