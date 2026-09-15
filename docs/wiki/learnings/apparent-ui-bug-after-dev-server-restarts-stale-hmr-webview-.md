---
type: Learning
title: Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging
description: Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel).
tags: [development, hmr, vite, webview, verification, gotcha]
timestamp: "2026-09-15T11:57:51.641Z"
---

# Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging

Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel).

**Symptom**: user reports a just-built UI flow "doesn't work" (e.g. clicking the chart ⚙ cog showed no drawer). Live CDP verification minutes later shows the flow working perfectly end-to-end.

**Root cause**: the agent had restarted the dev server several times during verification (kills + relaunches). The user's webview was mid-restart or serving a stale HMR component tree — clicks hit a dead/stale tree, so the new UI never mounts. Dev-mode HMR can leave a stale component tree after structural changes; a production build won't have this.

**Fix / rule**: before debugging an "it doesn't work" report in dev mode, have the user (or drive via CDP) **Ctrl+R the app window** and retry. If the flow works after reload, it was staleness, not a bug — report verified-working instead of hunting phantom code defects.

Related environment traps: [[labs-hang-vite-reload-loop]], [[webview2-cdp-gotchas-env-var-flag-stale]], [[drive-data-monster-s-real-ui-over-cdp]].
