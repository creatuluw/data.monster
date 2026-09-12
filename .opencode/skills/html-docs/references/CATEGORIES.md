# Document Type Classification Guide

## Category → Type Mapping

### 1. Exploration & Planning
Used when user is exploring options, comparing approaches, or planning implementation.

| Type | Template | When to Use |
|------|----------|-------------|
| **code-approaches** | `01-exploration-code-approaches.html` | "Show me different ways to solve X", "Compare approaches for Y", trade-off analysis |
| **visual-designs** | `02-exploration-visual-designs.html` | "Explore visual directions", "Show me design options", layout/palette exploration |
| **implementation-plan** | `16-implementation-plan.html` | "Plan the implementation", "Create a spec", milestones, data flow, risk table |

### 2. Code Review & Understanding
Used when reviewing code, understanding architecture, or writing PR descriptions.

| Type | Template | When to Use |
|------|----------|-------------|
| **pr-annotated** | `03-code-review-pr.html` | "Review this PR", "Annotate this diff", margin notes with severity tags |
| **pr-writeup** | `17-pr-writeup.html` | "Write up my PR for reviewers", motivation, file-by-file tour, test plan |
| **module-map** | `04-code-understanding.html` | "How does X work?", "Explain this package", architecture with call graph |

### 3. Design
Used for design systems, component libraries, and visual design review.

| Type | Template | When to Use |
|------|----------|-------------|
| **design-system** | `05-design-system.html` | "Show me the design tokens", "Create a design system reference", colors/type/spacing |
| **component-variants** | `06-component-variants.html` | "Show all states of this component", variant matrix with controls |

### 4. Prototyping
Used for motion/interaction prototypes that need to be felt, not described.

| Type | Template | When to Use |
|------|----------|-------------|
| **animation-sandbox** | `07-prototype-animation.html` | "Tune this animation", "Show me the transition", duration/easing sliders |
| **clickable-flow** | `08-prototype-interaction.html` | "Prototype this interaction", clickable multi-screen flow, drag-to-reorder |

### 5. Illustrations & Diagrams
Used for inline SVG figures, flowcharts, and diagrams.

| Type | Template | When to Use |
|------|----------|-------------|
| **svg-figures** | `10-svg-illustrations.html` | "Draw diagrams for this blog post", inline SVG with download buttons |
| **flowchart** | `13-flowchart-diagram.html` | "Draw this pipeline/process", annotated clickable flowchart |

### 6. Decks
Used for slide decks that can be navigated with arrow keys.

| Type | Template | When to Use |
|------|----------|-------------|
| **slide-deck** | `09-slide-deck.html` | "Create a presentation", "Make slides for this", <section>-based carousel |

### 7. Research & Learning
Used for explainers, feature deep-dives, and educational content.

| Type | Template | When to Use |
|------|----------|-------------|
| **feature-explainer** | `14-research-feature-explainer.html` | "Explain how X works in this codebase", collapsible steps, tabs, FAQ |
| **concept-explainer** | `15-research-concept-explainer.html` | "Teach me about X concept", interactive demo, comparison table, glossary |

### 8. Reports
Used for status updates, incident post-mortems, and recurring reports.

| Type | Template | When to Use |
|------|----------|-------------|
| **status-report** | `11-status-report.html` | "Weekly status", "What shipped this week", metrics chart |
| **incident-report** | `12-incident-report.html` | "Post-mortem for this incident", timeline, root cause, action items |

### 9. Custom Editing Interfaces
Used for throwaway UIs that make it easier to manipulate structured data.

| Type | Template | When to Use |
|------|----------|-------------|
| **triage-board** | `18-editor-triage-board.html` | "Let me drag-sort these items", kanban/triage with export |
| **flag-editor** | `19-editor-feature-flags.html` | "Let me toggle feature flags", dependency warnings, copy diff |
| **prompt-tuner** | `20-editor-prompt-tuner.html` | "Let me edit this template with live preview", slot-based editor |

## Classification Heuristics

Scan the user prompt for these signals:

- **Goal keywords**: explore, compare, options, alternatives, decide → Exploration
- **Code keywords**: review, PR, diff, architecture, call graph, understand → Code Review
- **Design keywords**: design system, tokens, component, variant, swatch → Design
- **Motion keywords**: animate, transition, ease, prototype, interaction → Prototyping
- **Drawing keywords**: diagram, flowchart, SVG, figure, illustrate → Diagrams
- **Presentation keywords**: slide, deck, present, talk, meeting → Decks
- **Learning keywords**: explain, how does, understand, teach, learn, research → Research
- **Report keywords**: status, report, post-mortem, incident, weekly, shipped → Reports
- **Editor keywords**: editor, triage, toggle, tuner, dashboard, drag → Editors

When a prompt could match multiple types, ask the user to clarify.
