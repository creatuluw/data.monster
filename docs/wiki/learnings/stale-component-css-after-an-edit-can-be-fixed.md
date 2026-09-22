---
type: Learning
title: Stale component CSS after an edit can be fixed with touch — no dev-server restart needed
description: "Extends [[stale-vite-module-graph-can-survive-reloads-only-arestart]]."
tags: [vite, hmr, stale-cache, debugging, dev-server]
timestamp: "2026-09-18T10:55:35.019Z"
---

# Stale component CSS after an edit can be fixed with touch — no dev-server restart needed

Extends [[stale-vite-module-graph-can-survive-reloads-only-arestart]].

Symptom: after editing a Svelte component (e.g. `controls/Section.svelte`), the dev server served the NEW JS module but the OLD compiled CSS from `?svelte&type=style` — HMR even fired with fresh `?t=` URLs, hard-reloads didn't help, and only SOME components were stale (Field picked up its new CSS, Section didn't). Curling the style module showed the old rules verbatim.

Root cause: the file-watcher missed the change, so vite's transform cache for the style module never invalidated. Editing through tools that do atomic replace (write-new-file + rename) seems to trigger this more often than in-editor saves.

Fix WITHOUT restarting the user's dev app: `touch` the changed files. The watcher fires, the style module re-transforms, and the next page load gets the new CSS. Verify with `curl "http://localhost:6123/src/lib/<file>.svelte?svelte&type=style&lang.css"` and grep for a marker from the new CSS.

Also learned while verifying: a plain `curl` of `File.svelte?type=style&lang.css` (no `svelte` param) 500s through the tailwind plugin (`Invalid declaration: Snippet` — it parses the whole SFC as CSS). That error form is an artifact of the direct request, not the real graph import (`?svelte&type=style&lang.css`) — don't chase it.
