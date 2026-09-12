---
type: Learning
title: Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees
description: "The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target/` (Rust build artif"
tags: [gotcha, performance, tooling]
timestamp: "2026-09-12T10:56:35.295Z"
---

# Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees

The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target/` (Rust build artifacts). Recursive scans over them stall — a `du -sh .archive/` hung long enough for the user to ask "what is taking so long", and one build alone made file-change detection report ~3485 changed files.

When gauging sizes or scanning the repo, work from the root listing and skip these trees. They are also why `.wiki_ignore` excludes them ([[wiki-ignore-staleness-policy]]).
