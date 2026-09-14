---
type: Learning
title: z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code
description: Symptom
tags: [llm, z-ai, debugging, api-keys]
timestamp: "2026-09-11T22:02:04.805Z"
---

# z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code

## Symptom

Analyst/Settings LLM calls to **z.ai** fail with:

```
API error 401 Unauthorized: {"error":{"code":"1000","message":"Authentication Failed"}}
```

## Diagnosis (2026-09-11 session)

- The stored-key round-trip in the app was verified correct (save → load → send).
- Testing the same key directly with `curl` against z.ai — on **both** the coding endpoint and the normal API endpoint — returned the identical 401.
- Conclusion: **the key itself is invalid/expired**, not an app bug. It had never been detectable before because CORS used to block every webview request, so no request ever reached z.ai's auth layer. Once remote LLM calls were proxied through Rust (see [[proxy-remote-llm-calls-through-rust-not-webview-fetch]]), the real provider response finally surfaced.

## Rule of thumb

When a remote LLM provider returns 401 from the app, **test the stored key with `curl` outside the app before touching app code**. If curl gets the same 401, the fix is on the provider side (create a new key at z.ai → API Keys, paste into Settings → API Key), not in Data Monster.

## z.ai specifics

- Same `code 1000 Authentication Failed` on coding and non-coding endpoints — the endpoint choice does not change auth behavior.
- Key management: https://z.ai → console → API Keys.
