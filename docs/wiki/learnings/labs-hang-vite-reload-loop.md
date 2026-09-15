---
type: Learning
title: Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug
description: "Reported 2026-09-14: clicking the bar chart card in /labs hung the page (heatmap fine). Root cause found same day: vitest import reachable from src via $lib/charts tripped vite's dep-optimizer, amplified by tailwind re-emitting app.css on any file churn. Fixed in PR #3 (commit 9f18749)."
tags: [vite, debugging, labs, tailwind, frontend]
timestamp: "2026-09-14T08:59:18.656Z"
---

# Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug

Reported 2026-09-14: clicking the **bar chart** card in `/labs` "hangs the page" while the heatmap card works; user noted everything worked before the bar chart was built.

## What the symptom actually is

Not a component exception — an **infinite vite client reconnect loop**: the page performs endless full reloads, so it renders nothing and appears frozen. User-visible fingerprint: a **`[vite] connecting` message flickering in the console the whole time** (websocket dropping/reconnecting). Diagnostic fingerprints:

- Zero exceptions on page, zero render, body never populates.
- Console floods with vite client reconnect messages (server connection lost / polling).
- `src/app.css` **never changes on disk** — the Tailwind dev plugin regenerates its virtual output in a loop, which is what trips vite's reload, not a real file edit.

## Root cause (found 2026-09-14, same day)

Two stacked mechanisms:

1. **Likely original trigger — dep-optimizer + vitest import reachable from `src/`**: the bar-chart page was the *only* `/labs` page importing from `$lib/charts/`, where `fundament.test.ts` (importing `vitest`) sat. Vite's dev dep-optimizer choked on it → hang on that page only.
2. **Amplifier — file churn → tailwind re-emit → full-reload feedback loop**: the Tailwind dev watcher scans the repo tree by default, so *any* file write in the project root (vite's own log file, wiki writes, session files) makes tailwind re-emit `app.css` → SvelteKit full reload → more churn → forever. Proven by writing a vite nohup log into the repo root and watching the loop start.

Dead-end theories ruled out en route: nondeterministic tailwind output (served CSS diffed byte-identical), Tauri CSP blocking `ws://` (CSP is null), broken vite.config (plain).

## Fix (PR #3, commit `9f18749`, merged `69d151a`)

User-directed approach: **make the bar-chart page a structural mirror of the working heatmap page** — everything inline in the page, only the chart component imported, no `$lib/charts` import from the page. Plus:

- `fundament.test.ts` moved to `tests/` — the `vitest` import no longer sits inside `src/` where the dev server's dep-optimizer can trip over it. See [[tests-outside-src]].
- `app.css` pins Tailwind `@source` to `src/**` + `app.html` — wiki/log/session churn can no longer re-emit CSS into a reload loop. See [[pin-tailwind-source-scanning]].

Verified: 9/9 tests from `tests/`, `svelte-check` 0 errors, build clean.

## Debug gotchas (cost real time)

- **Environment contamination**: installing vitest for probes *silently bumped vite 8.0.7 → 8.0.9*, and running vite with `--host 0.0.0.0` diverges from the user's exact setup. When debugging user-env issues, verify `node_modules` versions match the user's before trusting any probe result.
- Mock/headless harness is a dead end for this class of bug — the user's real environment is ground truth (see [[cdp-repro-traps-duckdb-lock]] for the related traps).
- A vite instance left over from the chart-lib dep removals ([[svelteplot-sole-chart-engine]]) has stale `optimizeDeps` — clean restart required before page comparisons are meaningful.

**Triage rule: "Labs card hangs, no console errors, `[vite] connecting` flicker" → check for a vite reconnect loop before blaming the chart component.**
