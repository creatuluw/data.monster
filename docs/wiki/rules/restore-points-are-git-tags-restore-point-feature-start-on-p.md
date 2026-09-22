---
type: Rule
title: Restore points are git tags restore-point/<feature>-start on pushed master HEAD
description: The rule
tags: [git, restore-point, workflow, feature-loop]
timestamp: "2026-09-22T13:41:24.698Z"
---

# Restore points are git tags restore-point/<feature>-start on pushed master HEAD

## The rule

Before starting a feature initiative, create a restore point as a **git tag**: `restore-point/<feature>-start`, cut on the pushed `master` HEAD and pushed to origin.

- Naming precedent: `restore-point/central-charts-start` (pre central-charts build) → `restore-point/workspace-file-first-start` (2026-09-22, commit `9445c50`, pre files-001 workspace-file work).
- The tag must point at **pushed** master HEAD — it is a rollback anchor, not a local bookmark.
- Caveat: a tag captures only committed work. Uncommitted drift (e.g. the DuckDB `["bundled","json"]` fix in `Cargo.toml`, `settings.rs`, `internal_db.rs` at files-001 start) is **not** in the restore point — commit it separately first, or accept it sits outside the anchor.

## When it applies

Any "before we do anything, create a restore point" request and any new multi-session feature start in this repo.

## Rationale

Master is the current line of development (see [[central-charts-work-lives-on-feature-branch]] — the old restore-point-as-master-HEAD pattern evolved into tags once PRs merged back to master). A named tag per feature start gives an unambiguous rollback point without holding master hostage.
