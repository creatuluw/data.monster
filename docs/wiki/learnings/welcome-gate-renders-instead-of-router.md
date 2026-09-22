---
type: Learning
title: First-run welcome gate renders instead of the router — its actions must act directly, never navigate
description: In `src/routes/+layout.svelte`, the first-run welcome gate (shown when no workspace is open) renders **instead of** the routed content — there is no router-rend
tags: [frontend, routing, workspace, sveltekit]
timestamp: "2026-09-22T13:01:24.622Z"
---

# First-run welcome gate renders instead of the router — its actions must act directly, never navigate

In `src/routes/+layout.svelte`, the first-run welcome gate (shown when no workspace is open) renders **instead of** the routed content — there is no router-rendered page behind it.

Consequence: navigation from the gate (e.g. `goto('/workspaces')`) renders nothing, because the thing you navigated *to* is rendered *by* the router the gate is replacing. When the header workspace button moved to the new /workspaces page (2026-09-22), the gate's button therefore kept its direct native folder dialog.

Rule: any action surfaced in the welcome gate must invoke/act directly (dialog, command) — never navigate. Revisit if the gate ever becomes a real route.
