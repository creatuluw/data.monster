---
type: Rule
title: Render markdown via marked + .prose-chat, never a new pipeline
description: When rendering any markdown anywhere in the app (docs tabs, chat, notes), parse with `marked` (already a dependency) and wrap the output in the `.prose-chat` cl
tags: [frontend, markdown, design-system]
timestamp: "2026-09-17T08:04:25.249Z"
---

# Render markdown via marked + .prose-chat, never a new pipeline

When rendering any markdown anywhere in the app (docs tabs, chat, notes), parse with `marked` (already a dependency) and wrap the output in the `.prose-chat` class. Do not add a new markdown parser or hand-roll prose styles.

## When it applies

Any UI surface that must display markdown as styled, design-system-consistent content. Established on the library detail page docs tab (`src/routes/library/[id]/+page.svelte`), which reuses the same pipeline the analyst chat uses.

## Rationale

`marked` + `.prose-chat` is the app's single markdown pipeline — reuse keeps typography and spacing consistent across surfaces and avoids duplicate prose stylesheets. Check for an existing rendering path before writing a new one.
