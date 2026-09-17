---
type: Preference
title: Never start npm run dev / tauri dev — the user owns the dev app
description: The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns the dev app.
tags: [workflow, dev-server, process-management]
timestamp: "2026-09-15T14:40:44.675Z"
---

# Never start npm run dev / tauri dev — the user owns the dev app

The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns the dev app.

If work needs the running app (CDP verification, e2e smoke, visual checks), the LLM says so explicitly: **"Please start the app (npm run dev)"** and waits.

Why (from this session, 2026-09-15): repeated LLM-driven kill/restart cycles caused stale-HMR webviews that looked like real bugs, recycled the CDP debug port mid-probe, and disrupted the user's open window. Restart churn cost more time than the bugs it chased. Related: [[apparent-ui-bug-stale-hmr-webview]], [[webview2-cdp-gotchas-env-var-flag-stale]].

Exception: one-shot build/test commands (`cargo check`, `vitest run`, `vite build`) are fine — they exit and don't own the app window.
