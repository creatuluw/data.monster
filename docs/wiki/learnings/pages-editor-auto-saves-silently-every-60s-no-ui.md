---
type: Learning
title: Pages editor auto-saves silently every 60s — no UI signal is deliberate
description: "User-requested behavior on `src/routes/pages/[slug]/+page.svelte` (2026-09-18, /pages/revenue): auto-save runs every 60s via `handleSave(true)`, which **skips t"
tags: [pages-editor, auto-save, ux]
timestamp: "2026-09-21T07:28:17.171Z"
---

# Pages editor auto-saves silently every 60s — no UI signal is deliberate

User-requested behavior on `src/routes/pages/[slug]/+page.svelte` (2026-09-18, /pages/revenue): auto-save runs every 60s via `handleSave(true)`, which **skips the `saving`/`saved` state flips** — the save button never changes while auto-saving. Failures still surface via the existing red error line; silently swallowing a failed save would lose work unnoticed.

Two things that look like bugs but are deliberate — do not "fix":
- **No saving/saved feedback on auto-save**: the user explicitly asked for the save to be silent ("not showing any signal/change in the ui when saving it"). Manual saves keep the state feedback.
- **No dirty-checking**: it saves even when nothing changed — a no-op write was judged cheaper than change tracking (`ponytail:` skip in the turn).

The save button is also icon-only now; Saved/Saving state lives on `title`/`aria-label` (hover only).
