---
type: Learning
title: Extending docs/features/ requires add-evals-to-skill's name-dir match and case pattern
description: "Constraints of add-evals-to-skill (hit while building [[feature-skill-catalog-docs-features]])"
tags: [evals, skills, add-evals-to-skill]
timestamp: "2026-09-16T13:22:40.050Z"
---

# Extending docs/features/ requires add-evals-to-skill's name-dir match and case pattern

## Constraints of add-evals-to-skill (hit while building [[feature-skill-catalog-docs-features]])

Extending the catalog with a 9th feature skill (or regenerating an existing one) hits non-obvious requirements of the skill's `analyze`/`scaffold` machinery:

- **Skill name ↔ directory name must match** — `analyze` fails if `docs/features/<slug>/SKILL.md` doesn't match the skill's declared name.
- **SKILL.md must have a description and stay ≤ 500 lines** — hard gates in `analyze`.
- **Instruction-skill flavor** scaffolds `evals/evals.json` + `evals/grade.mjs` and **appends** a `## Evals` section to SKILL.md (rerunning scaffold duplicates it — replace placeholder cases in place).
- **Case pattern**: 3 cases per feature — 2 realistic phrasings + 1 edge/negative that guards a named failure mode. An eval that can't fail is decoration; the validator includes a negative self-check.

## Proof / how to validate

```bash
node docs/features/validate.mjs   # exit 0 = all 8 demo suites valid (152 checks)
```

## Deferred

With-skill vs without-skill LLM-harness runs were skipped (needs an endpoint); `grade.mjs` is wired for when that's wanted.
