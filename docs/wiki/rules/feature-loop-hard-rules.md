---
type: Rule
title: "Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal"
description: Guideline
tags: [git, workflow, feature-loop, pr]
timestamp: "2026-09-14T07:52:32.055Z"
---

# Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal

## Guideline

When running the feature-loop workflow (or any change work) in this repo:

- **Never push or merge directly to master.** Every change ships via a PR, even small fixes. Railway deploys from master, so a branch push deploys nothing — the merge is the release trigger.
- **Worktrees are opt-in.** Ask, wait for an explicit yes, and only then create one. No answer = work on a feature branch in the main tree. Recommend, never decide.
- **Never `--force` a worktree removal or `git branch -D` without asking first.** Stale worktrees are swept only when provably lossless: clean `git status --porcelain` AND empty `git log --oneline origin/master..<branch>`. Skip anything locked, dirty, or with unmerged commits — likely another session's active task.
- **Branch names carry the task slug** (`feature/<slug>`), never generic or improvised names. Always branch from fresh master (`git pull --ff-only` first; stop and report if it fails).
- **Type-check is the staging-miss gate.** Run `svelte-check` (and tests) before opening a PR and again after merge alignment — selective staging can silently omit files (2026-09-14: the `BarDatum` interface→type fix missed staging, shipped 3 type errors to master, fix-forwarded as PR #2).

## When it applies

Every session that makes changes to this repo, especially feature-loop runs. The user merges PRs themselves — the agent never merges.

## Rationale

Stated by the user as "hard rules — no exceptions" (2026-09-14 feature-loop run). Protects master as the deploy branch, avoids clobbering concurrent sessions' worktrees, and keeps the human in control of merge decisions.
