---
type: Entity
title: library-component-builder skill (.pi/skills)
description: "A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview → "
tags: [library, skill, pi-skills, tdd, agentskills]
timestamp: "2026-09-17T07:39:13.178Z"
---

# library-component-builder skill (.pi/skills)

A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview → plan → strict TDD build → registration → REPORT.md gate. It is the agent-side counterpart to the human-facing `/library/dev` guide page.

## Details

- **Location**: `.pi/skills/library-component-builder/`
- **Structure**:
  - `SKILL.md` — workflow: idea → interview (one question at a time, lettered options, ask only what the contract needs) → plan with agreed seams → strict TDD (red before green, vertical slices, tests in `tests/` only) → Karpathy behavioral rules as hard constraints → registration → REPORT.md gate (all-PASS with evidence or not done)
  - `references/CONTRACT.md` — portable copy of the `LibraryEntry` / `ChartTypeDefinition` package contract (def/demo/docs/index/REPORT, renderer props, code/data/logic checklist)
  - `references/E2E-REPORT.md` — the REPORT.md format: definitions unit-tested + works in UI (card, Preview, add-block, config panel) + checklist, evidence required
  - `evals/` — 3 prompt cases (plain build, pre-answered variant, adversarial out-of-vocabulary ask that must be pushed back on) + `grade.mjs` grader-aggregator
- **Portability rule**: TDD rules, Karpathy constraints, contract, and report format are all embedded inline — zero references to external skill paths; works isolated in any pi session in this repo
- **Conformance**: name matches directory, valid frontmatter, progressive disclosure, SKILL.md under 500 lines

## Relationships

- Implements [library-component-builder-canonical-path](../../decisions/library-component-builder-canonical-path.md) — the decision naming it the canonical build path
- Feeds [library-registry-system](./library-registry-system.md) — components it builds register via `registerLibraryComponent`
- `references/CONTRACT.md` is a portable copy of `src/lib/library/types.ts` — code wins on disagreement; update the reference when the contract changes
- `/library/dev` route is the human-readable mirror (linked from the `/library` grid as a "For developers" card)

## Lifecycle

- First added: 2026-09-17 — built per the agentskills.io spec with TDD + Karpathy rules embedded and evals attached; skipped the with/without-skill eval benchmark (needs harness runs per case; `grade.mjs` is ready when wanted)
