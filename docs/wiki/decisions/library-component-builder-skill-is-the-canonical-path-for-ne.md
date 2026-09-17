---
type: Decision
title: library-component-builder skill is the canonical path for new library components
description: Context
tags: [library, skill, tdd, e2e]
status: accepted
timestamp: "2026-09-17T07:37:52.221Z"
---

# library-component-builder skill is the canonical path for new library components

## Context

/library became the central component library (registry drives editor + library). Component devs need a repeatable path from idea → registered, tested component. The user asked for a pi skill that owns this path, built per the agentskills.io spec, with TDD + Karpathy conformance embedded, evals attached, and a mandatory e2e report per component. Plus a dev-facing guide page in /library.

## Choice

Create `.pi/skills/library-component-builder/` (agentskills.io spec: name matches dir, frontmatter valid, references/ + evals/, SKILL.md < 500 lines, progressive disclosure). The skill is **portable**: TDD rules, Karpathy behavioral rules, the LibraryEntry contract and the e2e report format are all embedded inline (references/CONTRACT.md + references/E2E-REPORT.md) — no external skill or path dependencies.

Workflow inside the skill: idea → interview (one question at a time, lettered options, ask only what the contract needs) → plan with agreed seams → TDD build (red before green, vertical slices, tests in tests/ only) → registration → verification (vitest/svelte-check/build + UI checks against the user-owned dev app) → REPORT.md gate (definitions tested, works in UI, code/data/logic checklist — all PASS with evidence or not done).

Evals: evals/evals.json (3 prompt cases: plain build request, pre-answered variant, out-of-vocabulary adversarial ask) + evals/grade.mjs grader-aggregator per the add-evals-to-skill format.

Dev guide: /library/dev route (static beats [id]) explains the package contract, registration, test-first build and report gate, and points to the skill; linked from the /library grid as a "For developers" card.

## Alternatives

- Point the skill at the external TDD/Karpathy skills on E:\skills.te9.dev — rejected: not portable, breaks isolation.
- Web docs page only (no skill) — rejected by user: the interview + gates must live in the skill so any agent session follows them.

## Consequences

- New components SHOULD be built through the skill; the /library/dev page is the human-readable mirror of the same contract.
- Every new component package gains a 5th file: REPORT.md (the done gate).
- CONTRACT.md is a portable copy of src/lib/library/types.ts — code wins on disagreement; update the reference when the contract changes.
