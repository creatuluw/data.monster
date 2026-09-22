---
type: Decision
title: Drawer chrome restyle reverted — control kit stands, lms/kees motifs rejected
description: Context
tags: [drawers, design-system, pr-18, revert]
status: accepted
timestamp: "2026-09-17T18:07:13.000Z"
---

# Drawer chrome restyle reverted — control kit stands, lms/kees motifs rejected

## Context

PR #18 shipped the shared controls kit (commit `2af2dfd`, kept) and — in a second commit — a drawer chrome restyle (`dc2c3a6`) that studied `E:\lms.pippeloi.nl` and `E:\kees.pippeloi.nl` and folded their motifs into the drawers:

- 2px backdrop blur on overlay drawers
- bordered 28px square close button (hover fills sunken)
- body/header padding aligned to the modal rhythm (space-4/5)
- `Section` flattened from hairline cards into flat content groups separated by dashed hairlines (matching the skeleton cards / role-picker modal motif)

## Decision

The chrome restyle is **reverted** (revert commit `974d37e`, PR #18). The user rejected the visual change; only the controls kit from the first commit stands. Drawers keep their pre-restyle chrome: plain 0.3-opacity backdrop, borderless padded close button, and `Section` as a hairline card (border + radius + surface).

## Rationale

- User preference wins over synthesized cross-project patterns — the lms/kees reference study produced a direction the user didn't want on second look.
- Keeping the revert as its own commit (rather than squash-editing) preserves the history; branch tip `974d37e` can be squash-merged cleanly.

## Consequences

- **Do not re-apply** the backdrop blur, bordered square close, or Section flattening. The prior rule "Drawer sections are flat content groups with dashed hairline separators" has been withdrawn (deleted 2026-09-17).
- [[drawer-form-controls-come-from-the-shared-controls]] still stands — form controls must come from the kit.
- If a future drawer-chrome pass is wanted, get user sign-off on the visual direction before wiring reference-project motifs into the shared drawer shell.

## Related

- [[shared-controls-kit-charts-controls]] — the kit that survived the revert
- [[chartconfigdrawer-component]] — the drawer shell whose chrome reverted
