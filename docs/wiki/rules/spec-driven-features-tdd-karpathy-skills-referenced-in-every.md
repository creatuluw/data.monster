---
type: Rule
title: "Spec-driven features: TDD + Karpathy skills referenced in every todo"
description: Guideline
tags: [spec, tdd, todos, conventions]
timestamp: "2026-09-15T09:18:44.117Z"
---

# Spec-driven features: TDD + Karpathy skills referenced in every todo

## Guideline

New spec-driven features follow the **chart-lib pattern**:

- Spec lives at `.specs/<name>/spec.md` — FRs grouped into phases.
- Tasks mirror the FRs in `.specs/<name>/tasks.json`.
- Todos are created in `.pi/todos`, each tagged with its phase.
- **Every todo instructs reading the `tdd-workflow` and `karpathy-guidelines` skills before starting** — TDD and Karpathy constraints are baked into each task's acceptance criteria, not left to the implementer's judgment.
- On completion, a task-log entry is appended to the spec.

## When it applies

Any new feature large enough to warrant a spec (central-charts, chart-lib, field-function-library, local-llm, tauri-migration all follow it).

## Rationale

Keeps acceptance criteria uniform across sessions and implementers; the skills-loaded-first instruction prevents TDD from silently degrading to test-after on later todos.
