---
type: Artifact
title: LLM Sensitive-Data White Paper — Finance Edition
description: "Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Fully rewritten 2026-09-12 around one spine: *"wie traint er mee, en w"
tags: [research, llm, privacy, whitepaper, finance, html-doc]
timestamp: "2026-09-11T22:05:56.103Z"
---

# LLM Sensitive-Data White Paper — Finance Edition

Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Fully rewritten 2026-09-12 around one spine: *"wie traint er mee, en wie niet?"* — the provider evidence is now the document's organizing structure, not an appendix table.

## What it documents

- §01 **Wie traint er níet op uw API-data?** — three color-coded verdict cards: ✓ 10 providers that don't train · ⚠ free tiers do train + Kimi's policy-vs-marketing contradiction ("bij twijfel telt het beleid, niet de marketing") · ✗ DeepSeek (default-on, no opt-out, China)
- §02 **"Niet trainen" ≠ "niet bewaren"** — the 30-day retention norm, stateful features that always store (files, cache, 24h session, Google's un-disableable 30-day search data), aggregator hops, and the Together-hosted-DeepSeek-in-US workaround
- §03 **Full 12-provider table** — OpenAI, Anthropic, Gemini, Azure, Mistral, xAI, Cohere, DeepSeek, Kimi, Z.ai, Together, Qwen
- §04 **ZDR in three tiers** — self-serve (xAI/Together/Z.ai by default) · via contract (OpenAI/Anthropic/Google/Azure/Mistral/Cohere) · not available (DeepSeek/Kimi)
- §05 Wet (condensed to 2 cards) · §06 Veiligste routes (eigen systemen → eigen cloud → slimme routes: Together-US DeepSeek, open weights Qwen/GLM, Cloudflare-ZDR)
- §07 Kernpatroon + SVG diagram · §08 checklist (incl. "controleer gratis tiers dubbel") · §09 supplier questions — question 1 is now the Kimi lesson: *"staat dat in de voorwaarden, of alleen op uw website?"* · §10 conclusie

## Editorial decisions

- 2026-09-12 rewrite: risks section dropped (subsumed by the verdict/storage sections); everything tightened to stay mobile-readable
- Tech jargon stays removed: "schema-only text-to-SQL" → *"de AI stelt de vragen, uw eigen systemen geven de antwoorden"*

## Details

- **Format**: single self-contained HTML file, mobile-first, dark mode, print button; color-coded verdict cards (green/amber/blue)
- **Location**: `docs/research/llm-gevoelige-data-whitepaper-financieel.html`

## Source

- [llm-api-data-retention-no-training-no-storage-local-models-a](../../learnings/llm-api-data-retention-no-training-no-storage-local-models-a.md) and [llm-provider-retention-part-2-kimi-z-ai-together-qwen-kimi-p](../../learnings/llm-provider-retention-part-2-kimi-z-ai-together-qwen-kimi-p.md) — provider facts woven through the doc
- [llm-sensitive-data-white-paper](./llm-sensitive-data-white-paper.md) — the technical original this edition rewrites for a business audience
