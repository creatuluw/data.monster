---
type: Learning
title: "Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}"
description: Symptom
tags: [local-llm, llama-cpp, performance, analyst]
timestamp: "2026-09-11T22:14:27.107Z"
---

# Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}

## Symptom

Analyst chat with a local (llama.cpp) model sat on a blank screen for ~20s before any text appeared. Queries/ingest were fine — the delay was all in the LLM stream.

## Root cause

The local server streams `reasoning_content` (thinking) deltas **before** `content` deltas. The app's stream parser in `src-tauri/src/commands/local_llm.rs` only emits `delta.content` and silently discards `reasoning_content` — so the model's hidden reasoning (~737 tokens, ~18s) played out invisibly while the user waited.

## Fix

Add `"thinking": {"type": "disabled"}` to the request body in `local_llm.rs`. Measured on a realistic analyst prompt:

| mode | total time | reasoning chunks |
|---|---|---|
| thinking on (before) | 23.9s | 737 |
| thinking disabled | 5.5s | 0 |

## Gotchas

- One early test still returned reasoning despite the flag; repeat/stability checks showed 0 reasoning chunks — verify with more than one request before concluding the flag is ignored.
- If thinking is ever wanted back (e.g. better answers on hard prompts), the parser needs to handle `reasoning_content` and the UI needs a collapsible "thinking" section — that was deliberately skipped.
