---
type: Decision
title: "Q15 locked: annotation vocabulary = svelteplot basic marks (Arrow, Dot, Line, Text, Rect), per-type whitelisted"
description: Context
tags: [charts, spec-design, annotations, q15, svelteplot]
status: accepted
supersedes: decisions/q15-reframed-annotations-should-speak-svelteplot-s-own-mark-
timestamp: "2026-09-15T09:03:48.900Z"
---

# Q15 locked: annotation vocabulary = svelteplot basic marks (Arrow, Dot, Line, Text, Rect), per-type whitelisted

## Context

Follow-up to [[q15-reframed-annotations-should-speak-svelteplot-s-own-mark-]] (proposed option A: whitelisted svelteplot marks). The user added the concrete basic-marks set to the annotation vocabulary, which locks the pick and pins the whitelist.

## Choice

Annotation vocabulary = **svelteplot basic marks: Arrow, Dot, Line, Text, Rect** — per chart type whitelisted; values are **literals or measure expressions** (engine-evaluated, per [[measures-dimensions-are-duckdb-expressions-not-column-agg-su]]).

- Each chart type's registry entry declares which of the five marks it hosts (non-cartesian types opt out, as in the Q15 lock).
- **RuleX/RuleY** ride along as Line's axis-anchored siblings — the docs' canonical one-line reference line mark. Flagged to the user as an inclusion; awaiting a shout if Line alone should cover it.
- No-code surface lists the type's supported marks + fields ("Add annotation" menu).

## Alternatives considered

- Evidence.dev's `reference_line/area/point` trio (original Q15 lock) — re-invents types svelteplot already names; superseded by the mark whitelist.
- Arbitrary layered marks — power users get that via the `plot_options` escape hatch instead.

## Consequences

- Spec's `annotations` list entries speak svelteplot mark names directly (old `type: "line"` becomes `mark: "ruleY"` etc.).
- Registry needs a draw hook per hosted mark per chart type.
- Keeps spec serializable → code-mode/UI-mode parity ([[two-surface-report-page-format]]).

## Related

- Supersedes the *proposal* record for Q15's reframe; mechanism (declarative list, engine-evaluated `at`, per-type whitelist) unchanged from [[q15-locked-reference-annotations-are-a-declarative-list-refe]].
- Builds on [[svelteplot-sole-chart-engine]].

(Next open question in the series: **Q16** — no-code editing surface. Assistant leans A: dual-mode Design⇄Code page editor in `/pages`, inspector = labs bolt drawer generalized. Not locked; do not treat as decided.)
