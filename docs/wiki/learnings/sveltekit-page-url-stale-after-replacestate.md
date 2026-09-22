---
type: Learning
title: SvelteKit page.url is stale after replaceState — never guard write-effects by reading it back
description: Discovered 2026-09-22 while making /data tab selection URL-addressable (`TableOverview.svelte`, verified over CDP against the live dev app).
tags: [sveltekit, shallow-routing, replacestate, cdp, bug]
timestamp: "2026-09-22T08:01:17.161Z"
---

# SvelteKit page.url is stale after replaceState — never guard write-effects by reading it back

Discovered 2026-09-22 while making /data tab selection URL-addressable (`TableOverview.svelte`, verified over CDP against the live dev app).

Symptom: a `$effect` guarded by `page.url.searchParams.get('tab')` worked on the first tab click but silently stopped writing on the second — the tab content switched but the URL froze.

Root cause: SvelteKit's `replaceState()` from `$app/navigation` updates `location.href` and history state immediately, but **`page.url` keeps returning the URL at mount time** — replaceState-written params are invisible to `page.url` reads (observed on @sveltejs/kit 2.57; matches open kit issues about `page.url` not reflecting shallow-routing writes). So a guard like `if (page.url.get('tab') === activeTab) return;` compares against a stale URL: the first write succeeds, but on the next change the guard sees the old param (or null) and early-returns wrongly.

Rules that follow:

- Never guard shallow-routing write effects by reading back `page.url` — reads are stale after pushState/replaceState.
- Make the effect track only the source state (`untrack` any `page.url` reads) and write unconditionally; make the first-run write idempotent (write the URL it would already be at). Self-rerun loops are impossible when `page.url` isn't a dependency.
- CDP e2e on the Tauri webview (port 9223 via dev-cdp.cmd) is the ground truth for URL-state bugs — console instrumentation through HMR (`console.debug` in the effect) revealed the stale reads in one run after three wrong theories (stale webview, hydration race, popstate reverts).
