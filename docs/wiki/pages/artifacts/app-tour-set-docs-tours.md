---
type: Artifact
title: App tour set (docs/tours/)
description: "The interactive demo-tour deliverable for all 8 app features: one standalone HTML player per feature (connect, preview, query, data-tables, pages, labs, settings, analyst), built from real-UI CDP captures — not staged mocks."
tags: [tours, demo, app-tour-demo, playwright, cdp, tauri]
timestamp: "2026-09-16T14:45:48.748Z"
---

# App tour set (docs/tours/)

The interactive demo-tour deliverable for all 8 app features: one standalone HTML player per feature (connect, preview, query, data-tables, pages, labs, settings, analyst), each built from **real-UI captures** of the running Tauri app — not staged mocks ("honest beats staged": the analyst tour shows the chat's true empty state). Created 2026-09-16 with the app-tour-demo skill, adapted to Tauri. Born from the post-mortem that a "demo" request had wrongly produced eval suites (see the demo rule).

## What it documents

- The 8 app features, each as a clickable guided tour (frames + narration + click steps)
- `RUNBOOK.md` — the proven rebuild pipeline, written after connect-tour succeeded and followed by all 7 others

## Details

- **Location**: `docs/tours/<feature>-tour/` — four files each: `capture.mjs` (draaiboek + machinery), `steps.json` (narration schema: `cap` badge, `tekst` 20–240 chars, `klaar` on every click, no em-dashes), `tour-assets.json` (frames + anchors), `<feature>-tour.html` (the player, runs from `file://`)
- **Format**: standalone HTML players built by `E:/skills.te9.dev/app-tour-demo/scripts/build.mjs`; verified by `verify.mjs` — gate is **GESLAAGD**: every frame `anchors n/n`, 0.0px click deviation, zero console errors
- **Generated from**: Playwright driving the real webview over CDP (`connectOverCDP`, never `chromium.launch`); seeded via the real invoke API (`download_url_to_workspace` + `load_*_file(path)`) because native file dialogs are undrivable

## Tauri capture adaptation (key gotchas, full detail in RUNBOOK)

- Captures run against a dedicated demo workspace `E:/demo-tour-workspace` — never the user's real `E:\workspace`
- Player is `file://`, so Google Fonts must be injected as `@import` into the player CSS
- `url: '/route'` navigates; `url: null` stays on the previous frame's page
- Wait for real content anchors (`waitForSelector`), never fixed delays — the preview-tour bug was a captured spinner
- Tours are captured **sequentially** — they share one app instance; parallel captures fight over it

## Relationships

- ["Demo" means an app-tour-demo UI tour, not eval suites](../../rules/demo-means-app-tour-demo-not-eval-suites.md) — the rule this set fulfills
- [Settings-swap for tours must cover .env too](../../learnings/settings-swap-for-tours-must-cover-env-too.md) — CRITICAL safety rule while capturing settings/analyst
- [Drive data.monster's real UI over CDP](../../learnings/drive-data-monster-s-real-ui-over-cdp.md) and [WebView2 CDP gotchas](../../learnings/webview2-cdp-gotchas-env-var-flag-stale.md) — the CDP foundation under the capture machinery
- [settings-tour and analyst-tour built](../../learnings/settings-tour-and-analyst-tour-built.md) — completion of the set
- [Feature skill catalog (docs/features/)](./feature-skill-catalog-docs-features.md) — sibling deliverable from the same post-mortem

## Source

- `docs/tours/RUNBOOK.md` — the canonical pipeline doc (environment, per-tour steps, CRITICAL safety rules)
