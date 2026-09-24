---
type: Learning
title: dm live-reload events don't replay — e2e must navigate first, then write the file
description: "Hit while e2e-testing the dm:error pipeline (workspace-file-first pass, 2026-09-22): dropping a broken-JSON file into `dm/pages/` **before** navigating to /page"
tags: [e2e, cdp, dm-events, testing-patterns]
timestamp: "2026-09-22T15:47:07.718Z"
---

# dm live-reload events don't replay — e2e must navigate first, then write the file

Hit while e2e-testing the dm:error pipeline (workspace-file-first pass, 2026-09-22): dropping a broken-JSON file into `dm/pages/` **before** navigating to /pages produced no problem banner — the `dm:error` event fired and was gone before any view subscribed. Navigating to the page **first**, then writing the broken file, made the banner appear.

`dm:changed` / `dm:error` are fire-and-forget Tauri events: they are delivered only to currently-subscribed listeners, never replayed. When e2e-testing any event-driven surface (error banners, live-reload flashes), the ordering is always **navigate/subscribe → then mutate the file**. A missed banner in this test shape means a test-ordering bug, not a broken feature — re-check the sequence before debugging the app.
