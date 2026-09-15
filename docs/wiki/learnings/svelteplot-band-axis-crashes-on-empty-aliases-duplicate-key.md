---
type: Learning
title: svelteplot band axis crashes on empty aliases (duplicate key)
description: "Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted."
tags: [svelteplot, central-charts]
timestamp: "2026-09-15T10:55:52.612Z"
---

# svelteplot band axis crashes on empty aliases (duplicate key)

Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted.

Root cause: charts mounted before their dimension/measure **aliases** were resolved — empty-string aliases made multiple bands share the same key. Fix: don't mount the chart until aliases are resolved.

General rule: svelteplot axis marks derive keys from scale domains; an empty/placeholder alias that repeats across bands is enough to crash. Guard mounting on resolved spec fields, not just on data presence.

Related smoke-bug from the same session: ChartCard renderers initially **masked query errors as "No data"** — always distinguish "query failed" from "query returned zero rows" in chart state machines. Extends [[svelteplot-datum-identity-empty-guard]].
