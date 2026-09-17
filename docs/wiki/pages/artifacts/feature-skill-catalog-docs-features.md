---
type: Artifact
title: Feature skill catalog (docs/features/)
description: What it is
tags: [documentation, evals, skills, features]
timestamp: "2026-09-16T13:22:40.050Z"
---

# Feature skill catalog (docs/features/)

## What it is

`docs/features/` is the app broken into 8 feature skills, each demoed through the [`add-evals-to-skill`](E:/skills.te9.dev/add-evals-to-skill/SKILL.md) workflow (analyze → scaffold → real cases → run & prove). It doubles as a living feature map: every skill documents what its feature does, where it lives (route(s) + Rust commands), and its contract, plus a guarded eval suite.

## What it documents

The 8 features (one skill folder each): **connect** (`/connect`), **preview** (`/preview`), **query** (`/query`), **data-tables** (`/data`, `/table/[name]`), **analyst** (`/analyst`), **pages** (`/pages`, `/page/<slug>`), **labs** (`/labs`), **settings** (`/settings`).

Per feature:

- `SKILL.md` — feature description, location, contract, and an appended `## Evals` loop
- `evals/evals.json` — 3 cases (2 realistic phrasings + 1 edge/negative) in the agentskills.io format, each guarding a named failure mode (e.g. query refuses `DELETE`, connect refuses `.xlsx`, pages doesn't claim a scatter block)
- `evals/grade.mjs` — generated grader for with-skill vs without-skill runs

## Details

- **Format**: skill-shaped folders (`docs/features/<slug>/`) + JSON evals + `.mjs` graders; `README.md` is the index table
- **Validation**: `node docs/features/validate.mjs` — 152 checks, exit 0 = all demo suites valid; includes a negative self-check proving the validator can fail
- **Generated from**: session 2026-09-16, driving the external add-evals-to-skill skill's own `analyze` + `scaffold` machinery
- **Not covered**: with-skill vs without-skill LLM-harness runs (needs an endpoint); `grade.mjs` is wired for when wanted

## Relationships

- Related entities documented elsewhere: [remote-chat-command](../entities/remote-chat-command.md) (analyst feature), [labsplaceholder-component](../entities/labsplaceholder-component.md) and [barchart-component](../entities/barchart-component.md) (labs feature)
- [overview](../../overview.md) — the app structure this catalog decomposes

## Source

- `docs/features/README.md` — index and run instructions
- `docs/features/validate.mjs` — deterministic validator
