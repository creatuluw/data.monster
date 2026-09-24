---
type: Decision
title: Agent prompts are interview-first — the LLM gathers every missing variable via one-at-a-time questions before starting
description: Context
tags: [agent-prompts, interview, ux, prompt-contract]
status: accepted
timestamp: "2026-09-23T06:23:10.300Z"
---

# Agent prompts are interview-first — the LLM gathers every missing variable via one-at-a-time questions before starting

## Context

The six copy-paste prompts on `/agent` hand a coding agent a job (ingest data, build a page, add a measure…). Left alone, the receiving LLM fills gaps in the prompt by **assuming**: silent defaults, guessed paths, invented column names — the user only finds out after the work is done wrong.

## The choice

Every prompt now ends with a **"Before you start — the only way to fill gaps"** section that mandates an interview-first contract:

- enumerate every argument/variable the job needs
- ask the user **ONE QUESTION AT A TIME** with lettered options (per the house interview rule)
- never assume, never pick silent defaults
- start work only after everything is confirmed

This is the **single** sanctioned gap-filling method — no "reasonable defaults" escape hatch. Enforced by the test contract (`tests/agent-prompts.test.ts` requires `ONE QUESTION AT A TIME` + `Never assume` in every prompt body).

## Alternatives considered

- **Prompt-side defaults** (provide sensible fallbacks) — rejected: wrong guesses are discovered late, after the agent has already written files into `dm/`.
- **Schema-driven input forms in the app** — rejected: over-builds the app for what a prompt sentence does; the receiving agent is the one that knows which variables it lacks.

## Rationale

User-set direction (2026-09-23): "each missing argument/variable needs to be gathered by the llm via interview/poll as the single method of working with the user to get all the variables known before being able to start working." It reuses the proven one-question-at-a-time interview format as a product behavior of the shipped prompts, not just a dev working-method.

## Consequences

- New prompts must include the interview-first section (the test fails otherwise).
- The agent-prompts experience for end users is conversational-by-design: paste prompt → agent asks → user answers → work starts.
- Future prompt changes must preserve the section verbatim enough to match the test regexes.

Shipped in commit `b4ec93c`, 2026-09-23. See [[../../rules/interview-one-question-at-a-time]] (the dev-side interview rule this extends) and [[../../pages/entities/agent-prompts-page]] (the entity).
