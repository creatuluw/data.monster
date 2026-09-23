---
type: Entity
title: E2E CDP driver
description: "`e2e/cdp.mjs` — the committed CLI driver for e2e-verifying data.monster's real UI over the Chrome DevTools Protocol (port 9223), node >= 21 native WebSocket, no dependencies."
tags: [e2e, cdp, testing, devtools, tooling]
timestamp: "2026-09-23T05:31:38.552Z"
---

# E2E CDP driver

`e2e/cdp.mjs` — the committed CLI driver for e2e-verifying data.monster's real UI over CDP. It packages the ad-hoc probe pattern used across the CDP learnings (see [driving the real UI over CDP](../../learnings/drive-data-monster-s-real-ui-over-cdp.md)) into one reusable script: node >= 21 native WebSocket, zero dependencies, ~65 lines.

It connects to the app's `/json` target list on port **9223** (pick the page target matching `6123|tauri|localhost`), opens the WebSocket, and returns four primitives: `send(method, params)` (raw CDP), `evalJs(expression)` (Runtime.evaluate with `awaitPromise`, throws on page exceptions), `nav(path)` (navigate to `http://localhost:6123<path>` + 1.5s settle), and `text()` (body innerText). It enables `Page` + `Runtime` domains and calls `Page.bringToFront` on connect — the [minimized-window throttling guard](../../learnings/minimized-occluded-webview2-throttles-page.md).

## Details

- **Location**: `e2e/cdp.mjs` · step scripts in `e2e/steps/*.mjs` (loaded by `run` via dynamic import)
- **CLI**: `node e2e/cdp.mjs nav <path>` · `text` (dump body text) · `eval <expr>` · `run <step>` (named step from `steps/`)
- **Requires**: the dev app running CDP-drivable — `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223` (exact restart procedure in [the CDP-verify preference](../../preferences/cdp-verify-the-dev-app-via-webview2-additional.md))

## Relationships

- [dm live-reload events don't replay](../../learnings/dm-live-reload-events-don-t-replay-e2e.md) — event-driven surfaces need navigate-first test ordering when driven through this script
- [CDP CAN click svelteplot marks](../../learnings/cdp-can-click-svelteplot-marks-dispatchmouseevent.md) — `Input.dispatchMouseEvent` via `send()` is the click primitive
- [dm-changed with no subscriber](../../learnings/dm-changed-with-no-subscriber-silently-no-ops.md) — the header-table-count fix was found and verified through this driver

## Lifecycle

- First added: 2026-09-22 (`feature/workspace-file-first`, used to e2e-verify the incoming-drop ingest and the /data tab URL sync)
