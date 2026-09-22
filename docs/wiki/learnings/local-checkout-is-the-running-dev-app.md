---
type: Learning
title: Local checkout is the running dev app — branch switches live-revert it until all PRs merge
description: "Discovered 2026-09-22 while handling the post-merge state of PR #19 (bug fixes) and its stranded follow-up commit `dd44774` (opened as PR #20, session artifacts"
tags: [git, devops, dev-app, branches]
timestamp: "2026-09-22T11:58:46.631Z"
---

# Local checkout is the running dev app — branch switches live-revert it until all PRs merge

Discovered 2026-09-22 while handling the post-merge state of PR #19 (bug fixes) and its stranded follow-up commit `dd44774` (opened as PR #20, session artifacts only).

The local checkout **is** the running dev app's source tree (vite serves from the working tree), so `git checkout master` while the dev app runs live-reverts the app to master content — any fixes that exist only on the unmerged branch appear to vanish. This is the same trap as the earlier "/pages work is completely lost" report ([[central-charts-work-lives-on-feature-branch]]).

Rule of thumb: keep the checkout on the feature branch until **all** its PRs (including artifacts/docs follow-ups) are merged; only then sync local master and switch. A commit pushed after the merge stays stranded on the branch — handle it with a follow-up PR (consistent with the PR-only shipping rule), never a direct push to master.

Also useful: branch content can be identical except the artifacts PR — compare before assuming a switch is safe.
