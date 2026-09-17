---
type: Rule
title: "Demo" means an app-tour-demo UI tour, not eval suites
description: Guideline
tags: [demos, tours, skills, app-tour-demo, ambiguity]
timestamp: "2026-09-16T13:38:21.324Z"
---

# "Demo" means an app-tour-demo UI tour, not eval suites

## Guideline

When a request in this project says **"demo"** of a feature, it means an
**interactive UI tour of the real app** — the `app-tour-demo` pattern from
`E:\skills.te9.dev\app-tour-demo\SKILL.md`: Playwright captures the real app
(`capture.mjs` → `tour-assets.json` + `steps.json`), producing a
self-contained `<feature>-tour.html` player. Reference exemplar:
`E:\kees.pippeloi.nl\docs\explainers\apps-declaraties\declaraties-tour`.

It does **not** mean eval suites / skill-contract demos via
`add-evals-to-skill`. Taking that reading once produced 8 eval JSON files
(`docs/features/`) when 8 interactive UI tours were wanted — see
[[feature-skill-catalog-docs-features]] (status of those suites: pending
user decision).

## Process gate

When a user names a skill/path but the expected deliverable shape
(e.g. "demo" → interactive HTML tour) mismatches what that skill can output,
**ask one clarifying question before building**. One question costs seconds;
defaulting to the literal reading cost a full detour. Applies at high
ambiguity only — ordinary defaults still stand.

## Tauri wrinkle for tour capture

`app-tour-demo`'s capture drives a dev-server URL with Playwright, but
data.monster is Tauri — `invoke()` needs the real shell. Use the known
workaround: run the Tauri app with `--remote-debugging-port` and connect
Playwright over CDP ([[drive-data-monster-s-real-ui-over-cdp]]).

## Rationale

Post-mortem 2026-09-16: the two candidate deliverables (eval JSON vs UI
tours) are nothing alike, and kees.pippeloi.nl is this repo's reference
project — the signal was available before shipping. Discovered when the user
rejected the eval-suite deliverables.
