---
type: Learning
title: z.ai GLM Coding Plan keys use the Anthropic endpoint — a valid key still 401s against /paas/v4
description: "Refinement of [[z-ai-401-code-1000-authentication]] — a 401 from z.ai does not always mean the key is bad. Discovered 2026-09-17 while checking whether little-c"
tags: [llm, z-ai, api-keys, endpoints]
timestamp: "2026-09-17T20:27:37.764Z"
---

# z.ai GLM Coding Plan keys use the Anthropic endpoint — a valid key still 401s against /paas/v4

Refinement of [[z-ai-401-code-1000-authentication]] — a 401 from z.ai does not always mean the key is bad. Discovered 2026-09-17 while checking whether little-coder (pi fork) can use a z.ai subscription.

## The fact

z.ai issues **two kinds of credentials** for **two different endpoints**:

- **Plain API credit** keys → OpenAI-compatible endpoint `https://api.z.ai/api/paas/v4` (what `remote_chat` and most OpenAI-style clients target).
- **GLM Coding Plan (subscription)** keys → the **Anthropic-compatible** endpoint `https://api.z.ai/api/anthropic`. These keys are not valid for the OpenAI-style `/paas/v4` route.

## Why it matters for this project

The app's [[remote-chat-command]] proxies z.ai `/chat/completions` (OpenAI-style, `/paas/v4`). If the user's key comes from a GLM Coding Plan subscription, the Analyst chat will 401 "code 1000 Authentication Failed" with a perfectly valid key — looking exactly like the bad-key case in [[z-ai-401-code-1000-authentication]]. The fix is a settings surface for the anthropic-compatible endpoint (or endpoint auto-detection), not a new key.

## Rule of thumb

Before declaring a z.ai key "bad": ask **which plan issued it**. Coding Plan → `api: anthropic`-style client against `https://api.z.ai/api/anthropic`; plain API key → OpenAI-style against `/api/paas/v4`.
