---
type: Artifact
title: aisure.uk pricing research report
description: "Fractal-research (te9-research skill, `recursive_research`, depth 1, 3 leaves) answering a standalone question — not app-internal research: *why is https://aisu"
tags: [research, report, te9-research]
timestamp: "2026-09-17T17:31:28.139Z"
---

# aisure.uk pricing research report

Fractal-research (te9-research skill, `recursive_research`, depth 1, 3 leaves) answering a standalone question — not app-internal research: *why is https://aisure.uk/ ("every AI tool, one platform, free to start, £9.99/mo unlimited") so cheap?*

## What it documents

- **Verdict**: the advertised promise cannot cost what it charges — a heavy user of the named frontier models costs $70–300+/mo at list API prices while Pro is £9.99 with no fair-use clause. The gap is closed by some mix of: invisible server-side throttling, cheaper models served under premium labels, grey-market model supply, an ad-monetized free tier, and a growth subsidy — layered over trust red flags (domain registered Mar 2026, hidden WHOIS, Scamadviser 0/100, zero organic reviews) against a real UK company (Noahsure Group LTD, Companies House 12364489, Stripe merchant-of-record).
- **Leaves**: `d1-001` offer (verified from the site's own JS bundles — daily caps, ads, terms), `d1-002` economics (Poe/Merlin comparator metering, grey-market token brokers), `d1-003` trust (scam scores, WHOIS, the one cautionary hands-on review).

## Details

- **Location**: `reports/2026-09-17-aisure-why-cheap/`
- **Format**: standard te9-research run dir — `research.log` (JSONL event stream), `agents/*.md` (leaf files with search trails + `d0-001-orchestrator.md` root synthesis), `metrics.json` (4 nodes, 3 leaves, ~4k words), `report.html` (rendered HTML report).
- **Generated**: 2026-09-17 by the te9-research fractal-research skill (sub-agent fan-out + bottom-up synthesis).
- Sibling report artifact: [llm-agent-connection-research-report](./llm-agent-connection-research-report.md).
