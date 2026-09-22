---
type: Entity
title: LLM prompt button (/library detail)
description: LLM prompt button (/library detail)
tags: [library, frontend, agents, prompts]
timestamp: "2026-09-17T11:51:34.914Z"
---

# LLM prompt button (/library detail)

# LLM prompt button (/library detail)

A `Bot`-icon button in the top-right of the `/library/[id]` page head (tooltip on hover explains what it is). Click copies a ready-to-paste bootstrap prompt that gives any LLM coding agent the full context to build, change, or maintain that component extension — and puts the agent into **interview-first mode** before any code is written.

## Why it matters

It turns the library detail page into the handshake surface between the human browsing components and the agent that will edit them. Since 2026-09-17 the copied prompt no longer asks the human to pre-write the task: the `Task:` slot opens as `Task: OPEN — not decided yet`, instructing the agent to interview the user first — one question per turn, lettered multiple-choice options ending with "Other — tell me" — restate the agreed task in one sentence, get confirmation, and only then start the skill's build workflow. This bakes the repo's [interview-one-question-at-a-time rule](../../rules/interview-one-question-at-a-time.md) directly into the prompt.

## Details

- **Location**: `src/routes/library/[id]/+page.svelte` — a single `llmPrompt` template string is the single source (no other file references it)
- **Interface**: button → clipboard copy; tooltip explains purpose
- **Prompt structure**: skill reference → component id/label → package path + file list → registration file → `Task: OPEN` interview-first block (agent asks the questions, user confirms the task statement) → the enforced rules (red-green tests, thin renderer, vitest/svelte-check/build + /library preview verify, all-PASS REPORT.md)
- **Gotcha**: the prompt embeds the skill's enforced rules — keep it in sync when [library-component-builder-skill-pi-skills](./library-component-builder-skill-pi-skills.md) changes its contract

## Relationships

- References [library-component-builder-skill-pi-skills](./library-component-builder-skill-pi-skills.md) — the prompt bootstraps that skill's workflow
- Lives on the detail pages of [library-registry-system](./library-registry-system.md)
- Implements the repo rule: interview the user one question at a time with lettered options ([rules/interview-one-question-at-a-time](../../rules/interview-one-question-at-a-time.md))

## Lifecycle

- First added: 2026-09 — code-tab polish pass (Prism highlighting, path headers, copy buttons)
- 2026-09-17 — `Task: <placeholder>` replaced with the interview-first `Task: OPEN` block; the agent now asks the questions instead of the human writing the brief (1-line change to `llmPrompt`)
