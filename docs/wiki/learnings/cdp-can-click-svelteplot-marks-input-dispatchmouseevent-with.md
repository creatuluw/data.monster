---
type: Learning
title: CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot
description: "Correction to [[cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `onclick` via "
tags: [cdp, e2e, svelteplot, central-charts]
timestamp: "2026-09-15T11:29:53.059Z"
---

# CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot

Correction to [[cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `onclick` via `addEventHandlers` addEventListener fires, selection + cross-filter re-query verified end-to-end, 2026-09-15).

What actually broke the first attempts:

1. **Coordinate staleness** — measuring `getBoundingClientRect()` in one `Runtime.evaluate`, then dispatching later, risks a moved/resized viewport (the app window resized between sessions). Re-measure and dispatch immediately in the same probe run.
2. **`element.click()` / synthetic `new MouseEvent` don't work** — svelteplot's wrapped handler reads `event.clientX/clientY` for scale inversion; `element.click()` has clientX=0 → hit-test lands nowhere. Always use `Input.dispatchMouseEvent` (real trusted events with real coordinates).
3. **A closed page target kills the webview content** — never call `/json/close/<id>` on the app's only page (blank window; restart required). Also a webview recycle can silently drop the `--remote-debugging-port` (stale `msedgewebview2.exe` processes) — kill all `msedgewebview2.exe` before relaunching.

Working recipe (see `tests/smoke-cdp.mjs` + `probe20/21` pattern): navigate → sleep → measure widest bar rect → `Input.dispatchMouseEvent` mousePressed+mouseReleased at its center → assert selection label + cross-filtered table row count.
