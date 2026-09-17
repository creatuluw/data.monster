---
type: Entity
title: LLM prompt button (/library detail)
description: "A `Bot`-icon button in the top-right of the `/library/[id]` page head (tooltip on hover explains what it is). Click copies a ready-to-paste bootstrap prompt tha"
tags: [library, frontend, agents, prompts]
timestamp: "2026-09-17T08:41:47.662Z"
---

# LLM prompt button (/library detail)

A `Bot`-icon button in the top-right of the `/library/[id]` page head (tooltip on hover explains what it is). Click copies a ready-to-paste bootstrap prompt that gives any LLM coding agent the full context to build, change, or maintain that component extension.

## Why it matters

It turns the library detail page into the handshake surface between the human browsing components and the agent that will edit them — the app itself now generates agent-ready prompts (echoing the [llm-agent-connection-research-report](../artifacts/llm-agent-connection-research-report.md) direction of app-generated agent entry points).

## Details

- **Location**: `/library/[id]` detail page, page head (top-right)
- **Interface**: button → clipboard copy; tooltip explains purpose
- **Prompt structure**: skill reference → component id/label → package path + file list → registration file → `Task: <placeholder>` → the enforced rules (interview, red-green, thin renderer, vitest/svelte-check/build + preview verify, all-PASS REPORT.md)
- **Gotcha**: the prompt embeds the skill's enforced rules — keep it in sync when the [library-component-builder-skill-pi-skills](./library-component-builder-skill-pi-skills.md) contract changes

## Relationships

- References [library-component-builder-skill-pi-skills](./library-component-builder-skill-pi-skills.md) — the prompt bootstraps that skill's workflow
- Lives on the detail pages of [library-registry-system](./library-registry-system.md)

## Lifecycle

- First added: 2026-09 — code-tab polish pass (Prism highlighting, path headers, copy buttons)
