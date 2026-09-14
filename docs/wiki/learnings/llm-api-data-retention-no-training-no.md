---
type: Learning
title: "LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction"
description: Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directly relevant to Data M
tags: [llm, privacy, security, analyst, compliance]
timestamp: "2026-09-11T21:49:47.031Z"
---

# LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction

Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directly relevant to Data Monster's Analyst feature, which calls remote LLM APIs and also supports local llama.cpp.

## Key non-obvious facts

- **"No training" ≠ "no storage."** Every major provider except current-policy Anthropic keeps ~30 days of inputs/outputs for abuse monitoring even when they never train on them. Zero Data Retention (ZDR) is what removes that, not "we don't train on it."
- **ZDR is a contract, mostly sales-gated.** OpenAI / Anthropic / Google / Azure require talking to sales; exceptions: xAI (self-serve Console toggle, team-wide, verifiable via `x-zero-data-retention: true` response header) and Mistral (paid-plan request).
- **Stateful features break ZDR everywhere.** Files APIs, Batch, server-side conversation state, RAG collections, explicit caching are all exempt from ZDR because they must persist to function.
- **Trust-and-safety flags override everything.** Anthropic can retain flagged content up to 2 years regardless of ZDR. No provider's "zero" survives a legal hold.
- **Provider specifics worth remembering:**
    - Anthropic: prompts/outputs not retained at rest by default; self-serve HIPAA/BAA option.
    - Google Gemini: **free tier trains on data**, paid does not; ZDR is per-project on paid.
    - Azure OpenAI: data stays in your Azure region/tenant; ZDR via Microsoft form; the regulated-industry favorite.
    - Cohere: can deploy fully in your own VPC — strongest "data never leaves" hosted option.
    - **DeepSeek: trains by default, one privacy policy covers app *and* API, no clean opt-out, stored in Hangzhou under mainland-Chinese law.** Assume public-grade handling.

## Data Monster relevance

- The Analyst feature's **local llama.cpp support is ZDR by construction** — the right escape hatch for sensitive workspaces. A DPA only promises behavior; an air gap guarantees it.
- If provider pickers are ever added in Settings, a one-line badge per endpoint ("trains by default" / "30d retention" / "local — nothing leaves this machine") would cover most of the compliance question users will ask.

Related: [[proxy-remote-llm-calls-through-rust-not-webview-fetch]] (how remote LLM calls are routed in this codebase).
