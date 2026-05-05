# LRS Design System

**Specification:** DS2-GLACIER-7742  
**Version:** 2.0  
**Color Space:** OKLCH  
**Type Scale:** 1.25x Major Third  
**Grid Base:** 4px

A design system for teaching, learning, and progress tracking. Built for clarity and reduced cognitive load. No decoration without purpose.

## Files

| File | Description |
|---|---|
| `design-system.html` | Self-contained visual reference. Open in a browser to see every token, component, and animation rendered live. |
| `design-system-variables.md` | Complete node tree of all CSS tokens and component specs in a structured, readable format. |
| `design-tokens.jsonl` | Machine-readable token extract (JSONL). One JSON object per line. Used by tooling and MCP. |

## Token Categories

- **Color** — Accent, surface, text, border, and semantic tokens (OKLCH)
- **Typography** — Source Serif 4 (display), Manrope (body), Courier New (mono), 9-step type scale
- **Spacing** — 4pt base grid, 9 steps from 4px to 96px
- **Radius** — 6 steps from 2px to full-round
- **Shadow** — 3 elevation levels
- **Motion** — 3 easing curves, 4 duration steps
- **Layout** — Max-width container, responsive gutter

## Components

- Buttons (primary, secondary, ghost, danger, sm, lg)
- Tags (default, accent, success, warning, danger)
- Inputs with labels and hints
- Toggles
- Cards
- Accordion
- Modal (confirm, edit, danger variants)
- Search-ahead select with chips
- Toast notifications (default, success, warning, danger)

## Usage

Open `design-system.html` in a browser to browse the full system visually. Use `design-tokens.jsonl` for programmatic access to token values.
