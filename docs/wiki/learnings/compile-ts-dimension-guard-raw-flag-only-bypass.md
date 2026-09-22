---
type: Learning
title: "compile.ts dimension guard: raw flag is the only validation bypass — never blanket-catch checkColumn failures"
description: Symptom
tags: [security, validation, query-compiler, sql, trust-boundary, central-charts]
timestamp: "2026-09-22T12:47:26.521Z"
---

# compile.ts dimension guard: raw flag is the only validation bypass — never blanket-catch checkColumn failures

## Symptom

Two vitest tests failed after the workspace session: the dimension schema guard no longer rejected unknown dimension columns — including a SQL-injection-shaped string (`month; DROP TABLE x`) — silently passing them through the query compiler.

## Root cause

`compile.ts` wrapped dimension `checkColumn` validation in a **blanket catch** that swallowed *all* schema-check failures. Reason: `resolveItems` inlines master-item **expression** dims into `col` as raw DuckDB SQL — indistinguishable from a plain column pick at validation time. To let expression dims through, the catch had to swallow everything, and unknown columns went with them. That made the compiler's dimension path a trust-boundary hole: anything not a real column was interpolated raw.

## Fix

The only expression-dim constructor is `resolveItems` — it now marks resolved dims `raw: true` (field added to `ResolvedDimension` in `spec-types.ts`, set in `items.ts`). `compile.ts` validates every dimension *not* marked raw; only explicit `raw: true` bypasses schema validation. All 119 vitest tests green.

## Rule of thumb

`raw: true` on a resolved dimension is the **only** sanctioned validation bypass in the query compiler. Never widen a validation catch to make a known-exceptional case pass — mark the exceptional case explicitly and keep the boundary strict. Any new dimension constructor must either produce schema-valid columns or carry `raw: true` deliberately.
