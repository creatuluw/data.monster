---
type: Learning
title: CDP gate assertions need settle time after doc mutations, and svg counts must be chart-scoped
description: "Two CDP-e2e traps hit while testing the needsSetup gate (2026-09-17, PR #10):"
tags: [cdp, e2e, page-editor]
timestamp: "2026-09-17T15:32:10.396Z"
---

# CDP gate assertions need settle time after doc mutations, and svg counts must be chart-scoped

Two CDP-e2e traps hit while testing the needsSetup gate (2026-09-17, PR #10):

1. **Settle time after doc mutations.** The page-editor runtime re-loads (re-queries) on every doc change. A probe that deletes a measure row and immediately asserts the chart state sees the *pre-reload* UI — a phantom FAIL. Wait for the reload to settle before asserting; the gate does re-engage on role removal.
2. **Scope svg counts to the chart container.** Page-wide `document.querySelectorAll('svg').length` counts lucide icons too — assert on the chart/card subtree only when "chart rendered vs skeleton" is the question.
