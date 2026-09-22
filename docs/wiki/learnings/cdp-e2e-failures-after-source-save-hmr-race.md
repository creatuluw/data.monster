---
type: Learning
title: CDP e2e failures right after a source save are often HMR races — re-run before debugging
description: Discovered 2026-09-22 while debugging the /data tab URL-sync bug (verified over CDP, port 9223).
tags: [cdp, e2e, hmr, sveltekit, testing-gotcha]
timestamp: "2026-09-22T08:03:33.049Z"
---

# CDP e2e failures right after a source save are often HMR races — re-run before debugging

Discovered 2026-09-22 while debugging the /data tab URL-sync bug (verified over CDP, port 9223).

Symptom: a CDP e2e step fails right after an assistant source-file save — the transition that fails in one run passes reliably in a fresh one, and in-page instrumentation (`window.__log`) vanishes mid-test with double `vite connecting` entries plus benign `[TAURI] Couldn't find callback id` warnings (see [[couldn-t-find-callback-id-tauri-warning]]): the page reloaded itself mid-script as HMR applied the edit.

Why it bites: HMR reload racing the verify script produces false negatives that look exactly like real bugs. This turn it cost three wrong theories (stale webview → hydration race → popstate reverts) before console instrumentation showed the actual root cause ([[sveltekit-page-url-stale-after-replacestate]]).

Working method:

- A step failing *immediately after* a file save is suspect by default — reload/settle the webview and re-run that step in isolation before theorizing about the code.
- Instrumentation written onto `window` does not survive the HMR reload; inject it fresh in each run (or instrument via source through HMR, as ground truth).
- Keep generous gaps between steps on a freshly reloaded webview; the settled-app re-run is the real signal.

Family: extends [[apparent-ui-bug-stale-hmr-webview]] and [[stale-vite-module-graph-can-survive-reloads-only-a]] from "UI looks stale" to "test results are stale/raced".
