---
type: Entity
title: Workspaces page (/workspaces)
description: Dedicated workspace-switcher page at /workspaces — header folder button lands here; every workspace ever opened shows as a chip, and a chip click runs the full portable-workspace reload.
tags: [workspace, frontend, route, svelte]
timestamp: "2026-09-22T13:01:24.620Z"
---

# Workspaces page (/workspaces)

## What is it?

The dedicated workspace-switcher page at the `/workspaces` route — the header folder button lands here instead of opening a dialog. Labs-inspired: section title + subtitle, auto-fill card grid, icon tile, same design tokens. Every workspace ever opened shows as a chip (folder name + path in mono, ellipsized), ordered by last opened (newest first); the current workspace is outlined with a check, switching shows a spinner, and a **+ Add** button (top right) opens the native folder dialog to add + switch in one step. Breadcrumb label comes from `routeLabels` like every route.

## Why it matters

Workspace switching becomes visible and reusable: the user sees their workspace history at a glance and jumps between recent workspaces instead of re-picking folders through a native dialog. It is the browsing surface over the portable-workspace switch flow — every chip click runs the same full switch (shutdown → reload DB → reload tables → home).

## Details

- **Location**: `src/routes/workspaces/+page.svelte` (page); history + `list_workspaces` in `src-tauri/src/commands/workspace.rs`; `selectWorkspaceByPath(path)` in `src/lib/stores/app.svelte.ts`; header button wired in `src/routes/+layout.svelte`
- **Interface**: chips call `selectWorkspaceByPath(path)`; **+ Add** reuses the existing dialog flow — both upsert the history file on every pick
- **Configuration**: none — history is auto-created in the app-data dir, not per-workspace
- **Carve-out**: the first-run welcome gate keeps its direct folder dialog — it renders *instead of* the router, so navigating to `/workspaces` from it would render nothing

## Relationships

- [Workspace command module (Rust)](./workspace-command-module-rust.md) — owns the `workspaces.json` history, the `list_workspaces` command, and the switch a chip click triggers
- [Workspaces are fully portable — switching reloads data, content, and settings](../../decisions/workspaces-are-fully-portable-switch-reloads.md) — the switch semantics each chip click executes

## Lifecycle

- First added: 2026-09-22 — replaced the header dialog per user request (labs-inspired page with + Add, chips ordered by last opened). `list_workspaces` is a new Rust command: the dev app needs one restart to pick it up.
