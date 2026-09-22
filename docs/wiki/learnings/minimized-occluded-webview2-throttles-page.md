---
type: Learning
title: Minimized/occluded WebView2 window throttles the page — bringToFront before CDP UI automation
description: Discovered 2026-09-22 while CDP-testing bug fixes on the dev app (reports/pages-e2e-feedback.md).
tags: [cdp, webview2, e2e, throttling, gotcha]
timestamp: "2026-09-22T11:32:08.621Z"
---

# Minimized/occluded WebView2 window throttles the page — bringToFront before CDP UI automation

Discovered 2026-09-22 while CDP-testing bug fixes on the dev app (reports/pages-e2e-feedback.md).

When the data.monster window is **minimized or fully occluded**, WebView2 throttles the renderer: timers clamp, IntersectionObservers never fire, async UI flows stall mid-flight. Symptoms that mimic distinct bugs:

- Svelte components "fail to mount" (e.g. the ItemEditor form never appearing after clicking Add measure).
- Charts rendering empty / svelteplot SVGs with zero size (plot needs a layout/observer pass).
- Validation stuck at "checking…" (invoke resolved but the state update flush was starved).
- Intermittent CDP weirdness (evaluate slow, Page.navigate timing out).

Fix at the automation layer: `Page.bringToFront` (CDP) before interacting, re-issuing it whenever a UI flow stalls, plus `window.dispatchEvent(new Event('resize'))` to force relayout of already-mounted charts.

Rule: before diagnosing an "app bug" over CDP, confirm the window is fronted — at least half of one session's "failures" were throttling artifacts.
