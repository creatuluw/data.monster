---
type: Learning
title: Tour HTML captures embed the Google-Fonts @import — font changes require recapturing tours
description: "Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]])."
tags: [tours, typography, fonts, gotcha, app-tour-demo]
timestamp: "2026-09-16T17:30:26.216Z"
---

# Tour HTML captures embed the Google-Fonts @import — font changes require recapturing tours

Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).

The `app-tour-demo` captures in `docs/tours/*/` are **self-contained HTML snapshots** — they bake in whatever `@import`/font links the live `app.html` had at capture time. The 8 existing tours embed the old Google Fonts stack (Inter), so after the typography change they render with stale fonts even though `src/app.css`/`src/app.html` are correct.

## Rule of thumb
**Any token-level restyle (fonts, colors) invalidates tour captures.** Re-run each tour's `capture.mjs` once the typography/design settles — don't debug the live app for a font problem that only shows in a tour (and vice versa).

Related: [[tour-dom-snapshots-scale-with-live-dom]] (captures are snapshots of the live DOM, so they freeze whatever was loaded at capture time).
