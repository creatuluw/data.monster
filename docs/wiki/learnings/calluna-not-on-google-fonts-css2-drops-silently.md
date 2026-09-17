---
type: Learning
title: Calluna is not on Google Fonts — css2 returns 200 but silently drops it
description: "Discovered 2026-09-16 while recording the Calluna/Inter retype ([[typography-calluna-headings-inter-body]])."
tags: [google-fonts, typography, gotcha, calluna, frontend]
timestamp: "2026-09-16T18:48:20.570Z"
---

# Calluna is not on Google Fonts — css2 returns 200 but silently drops it

Discovered 2026-09-16 while recording the Calluna/Inter retype ([[typography-calluna-headings-inter-body]]).

## Symptom

`src/app.html` requests `family=Calluna:...` from Google Fonts css2, but headings render in **Inter** (the next entry in `--font-display: 'Calluna', 'Inter', serif`). No console error.

## Verification

- Bare request `css2?family=Calluna&display=swap` → **HTTP 400** (family unknown).
- The full combined URL (`Calluna` + `Geist Mono` + `Inter`) → **HTTP 200**, but the CSS body contains **zero `@font-face` for Calluna** — Google silently drops the unknown family and serves the rest. A 200 does NOT mean every requested family was served.
- Calluna is a commercial Exljbris font, not in the Google Fonts catalog. Also not installed locally on this machine.

## Fix paths

1. Self-host: buy/download Calluna woff2, put in `static/fonts/`, add `@font-face` in `app.css`, drop it from the Google Fonts URL.
2. Rely on a locally-installed Calluna (works only per-machine — fragile).
3. Swap to a Google-Fonts serif (e.g. Lora, Fraunces) in the token.

## General rule

When a Google Fonts family silently doesn't apply, `curl` the css2 URL and **grep the response body for the family name** — don't trust the status code.
