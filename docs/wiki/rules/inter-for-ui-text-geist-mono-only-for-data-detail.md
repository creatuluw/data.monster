---
type: Rule
title: "Two-font rule: Inter for all UI (display + body), Geist Mono for data detail"
description: Inter everywhere for UI text; Geist Mono reserved for data detail (tables, chart ticks, tags, IDs).
tags: [design, typography, frontend]
status: accepted
timestamp: "2026-09-16T18:55:43.079Z"
---

# Two-font rule: Inter for all UI (display + body), Geist Mono for data detail

*(Display face churned through eleven faces on 2026-09-16 — Inter → Squada One/Libre Baskerville → Calluna → Figtree → Poppins → Bricolage → Space Grotesk → Syne → Geist → Host Grotesk — before the user reverted and **settled on Inter everywhere**; see [[typography-settles-inter-everywhere-geist-mono]].)*

## Guideline
Every new UI surface follows the two-font rule set in `src/app.css`:

- **Inter** (`--font-display` AND `--font-body`, variable 100–900, loaded in `app.html`) for headings/titles, body copy, prose, buttons, nav, modals. Headings use `font-weight: 700` by convention.
- **Geist Mono** (`--font-mono`) for **data detail**: table cells/headers (`.data-table th/td`), chart axis ticks/legends, in-cell chart labels, `.tag` badges/labels, code/IDs.

## When it applies
Any new component, route, or chart. If the text renders a data value, helps read one off a chart/table, or is a badge/label → mono. Everything else → Inter.

## Rationale
User decision (2026-09-16, settled): mono visually marks "machine data" vs UI chrome; `tabular-nums` alignment in mono keeps dense tables legible. Inter everywhere keeps the app neutral and professional. The css2 link stays one request (Inter + Geist Mono). Never hardcode a font-family — use the CSS vars so token changes propagate. Do not reintroduce a separate display face without the user explicitly reopening typography.
