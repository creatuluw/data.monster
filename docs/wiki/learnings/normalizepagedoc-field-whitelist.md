---
type: Learning
title: normalizePageDoc is a field whitelist — new PageDoc fields must be passed through or they're stripped on load
description: "Discovered 2026-09-17 fixing the `/pages` row-height persistence bug: user resized a row, revisited the page, height was gone — yet the save path stored it corr"
tags: [central-charts, spec-types, normalizePageDoc, persistence, bug]
timestamp: "2026-09-17T13:22:53.727Z"
---

# normalizePageDoc is a field whitelist — new PageDoc fields must be passed through or they're stripped on load

Discovered 2026-09-17 fixing the `/pages` row-height persistence bug: user resized a row, revisited the page, height was gone — yet the save path stored it correctly.

## Symptom
A PageDoc field saves fine but never survives a page reload / Code-mode apply.

## Root cause
`normalizePageDoc` (in `src/lib/charts/spec-types.ts`) **rebuilds each row/column as a fresh object** — it is a field whitelist. It ran `{ columns: … }` and dropped `row.height` even though `load_page` had returned it. Any PageDoc field not explicitly passed through in the rebuild is silently stripped on every load.

## Rule
When adding any new field to the PageDoc contract ([[chart-page-spec-spec-types-validator]]), update `normalizePageDoc` to carry it through — the save path alone proves nothing. Symptom signature: "saved in DB, gone on reload" = look at normalizePageDoc first.

Fix was one line: `({ height: row.height, columns: rowColumns(row) })`; column heights already survived. Round-trip tests (row + column height) added as the regression check.
