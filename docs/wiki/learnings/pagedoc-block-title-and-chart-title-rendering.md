---
type: Learning
title: PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there
description: "In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks) **`chart.title` / `"
tags: [central-charts, pagedoc, spec]
timestamp: "2026-09-15T11:54:58.537Z"
---

# PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there

In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks) **`chart.title` / `chart.subtitle`**. Chart renderers read only the chart-level ones.

**Symptom (2026-09-15):** the BlockInspector's Title field wrote block-level `title` — edits did nothing visible. Found via CDP: the field was writing, the chart just never read it.

**Fix:** inspector now writes `chart.title` / `chart.subtitle` (and a Subtitle field was added).

**Rule of thumb:** when adding inspector/spec fields, write to the level the renderer actually reads — and verify field→render wiring in the running app, not just the store.
