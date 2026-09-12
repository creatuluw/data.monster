---
name: html-docs
description: "Create richly interactive, self-contained HTML documents from user prompts. Use when the user wants: (1) exploration & planning docs (comparisons, design exploration, implementation plans), (2) code review & understanding (annotated PRs, architecture maps), (3) design references (design systems, component variants), (4) prototypes (animations, clickable flows), (5) diagrams & illustrations (SVG figures, flowcharts), (6) slide decks, (7) research & learning (explainers, tutorials), (8) reports (status, incident post-mortems), (9) custom editing interfaces (triage boards, flag editors, prompt tuners). Trigger on keywords: make an HTML, create a page, render as HTML, build a document, or any prompt asking for a visual/interactive document. Also trigger when the user describes a document use case that would benefit from interactive HTML rather than static markdown."
---

# html-docs

Create self-contained `.html` documents from the 20 templates at [thariqs.github.io/html-effectiveness](https://thariqs.github.io/html-effectiveness/). Each template is a single file — no build step, no external deps.

## Workflow

### 1. Classify the prompt

Read `references/CATEGORIES.md` to map the user prompt to one of 9 categories and 20 document types. If the prompt clearly matches, proceed. If ambiguous, ask the user.

### 2. Load the template

Read the corresponding template from `assets/templates/{filename}`. The template's HTML structure, CSS, and JS are the starting point.

### 3. Adapt the template

Replace placeholder content (titles, descriptions, data, examples) with the user's actual content. Preserve the original layout, interaction patterns, and visual design. Only deviate when the user's data demands it.

### 4. Handle unrecognized use cases

If the prompt doesn't match any existing template type:
1. Read `references/NEW_TEMPLATE_GUIDE.md` for the process
2. Propose a template structure to the user (describe what sections/components you'd include)
3. Wait for approval before generating
4. After approval, save the new template to `assets/templates/` for future reuse

## Design Conventions

Follow these for all output documents:
- **Single self-contained `.html`** — all CSS in `<style>`, all JS at end of `<body>`
- **No external dependencies** — no CDN links, no npm packages, no web fonts
- **Dark mode** — support via `prefers-color-scheme: dark` or a toggle button
- **Export/ Copy** — editors must include a way to export or copy the result
- **Responsive** — works on mobile and desktop
- **System font stack** — `system-ui, -apple-system, sans-serif`
- **Semantic HTML** — `<header>`, `<main>`, `<section>`, `<nav>`, `<article>`, `<aside>` as appropriate

## Template Index

The file `assets/templates/index.json` maps all 20 templates by category, slug, and filename. Use it for quick lookup.
