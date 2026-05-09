---
name: new-prd-interview
description: "Conduct a structured interview with a product manager or software engineer to define a new capability or initiative, then produce a PRD document from which specs and tasks can be extracted. Use when the user wants to define a new feature, capability, or initiative before implementation, needs a PRD written, wants to interview stakeholders about a new product idea, or needs to translate a raw idea into structured requirements. Triggers: 'interview me about', 'lets define a new feature', 'write a PRD for', 'I want to build', 'help me scope', 'prd for', 'product requirements', 'new capability'."
---

# New PRD Interview

Conduct a sequential, adaptive interview to capture everything needed for a PRD at the initiative/epic level. The PRD must sit above spec level — it defines the "why," scope boundaries, and high-level capabilities from which a spec-writer can extract individual specs (like those in `.specs/*/spec.md`).

## Interview Protocol

### Phase 1: Opening

Start by introducing yourself as a PRD interview conductor. State the goal: to produce a PRD document that captures the initiative at a level above implementation specs, enabling a spec-writer to extract detailed specs and tasks.

Ask the user:
1. What is the initiative or capability you want to define?
2. What project or product does this belong to?
3. What is the tech stack, architecture, and existing patterns? (Project-agnostic — do not assume anything)

Record all answers. Build context before proceeding.

### Phase 2: Discovery

Ask questions **one at a time**, in this order. After each answer:
- If the answer is vague or incomplete, ask a targeted follow-up before moving on.
- If the answer is thorough, acknowledge it and proceed to the next question.
- If the user volunteers information relevant to a future question, note it and skip that question later.

Read [references/interview-questions.md](references/interview-questions.md) for the full question bank organized by topic. The topics are:

1. **Problem & Motivation** — What problem does this solve? Who feels it? Why now?
2. **Users & Personas** — Who benefits? Who is affected? What do they do today?
3. **Scope & Boundaries** — What is in scope? What is explicitly out of scope?
4. **Success Criteria** — How do we know this worked? What metrics matter?
5. **High-Level Capabilities** — What are the major sub-features or capabilities within this initiative?
6. **Technical Approach** — How should this be built? Architecture decisions, constraints, dependencies.
7. **Risks & Open Questions** — What could go wrong? What is still unknown?
8. **Phasing & Rollout** — How should this be delivered? MVP first? Parallel tracks?

Do not rush. A good interview takes 15-30 exchanges. Depth matters more than speed.

### Phase 3: Spec Candidate Identification

After covering all topics, explicitly ask: "Which of the capabilities we discussed should each become their own spec document?"

For each capability identified as a spec candidate, capture:
- Name
- Brief description of what the spec should cover
- Dependencies on other specs or capabilities
- Priority (must-have, should-have, nice-to-have)

### Phase 4: PRD Generation

Compile the interview into a PRD document. Write it to `.prds/<initiative-slug>/prd.md` and the full transcript to `.prds/<initiative-slug>/transcript.md`.

The PRD must follow the template in [references/prd-template.md](references/prd-template.md). Key sections:

- **Feature Overview** — Initiative summary at epic level
- **Problem Statement** — Why this matters, who it affects
- **Success Criteria** — Measurable outcomes
- **Design Goals** — Primary (must) and Secondary (nice to have)
- **User Personas & Experience** — Who uses this, how they interact
- **Scope & Boundaries** — In scope / Out of scope
- **High-Level Capabilities** — Sub-features, each described at a level a spec-writer can expand
- **Spec Candidates** — Explicit list of capabilities that should become individual specs, with descriptions of what each spec should cover
- **Technical Approach** — Architecture, stack decisions, constraints
- **Constraints & Assumptions** — Technical and business constraints
- **Risks & Open Questions** — Known unknowns
- **Phasing & Rollout** — Delivery plan

### Phase 5: Confirmation

Tell the user the PRD has been written. Show the file path. Summarize the spec candidates identified. Ask if anything needs correction.

## Writing Principles

- Write at the **initiative level**, not the implementation level. The PRD says "the system needs a chart library that supports bar, line, and scatter charts" — the spec says "create a BarChartCanvas Svelte component with these props."
- Each **High-Level Capability** should be 2-4 paragraphs — enough for a spec-writer to understand intent, but not so detailed that it becomes the spec itself.
- The **Spec Candidates** section is the handoff bridge. It must be unambiguous: a spec-writer reading it should know exactly what specs to create and what each should cover.
- Use clear, imperative language. Avoid hedging.
- If technical details are unknown, mark them as open questions rather than guessing.

## Output Locations

- PRD: `.prds/<initiative-slug>/prd.md`
- Transcript: `.prds/<initiative-slug>/transcript.md`

Create the directory if it does not exist. Use a kebab-case slug derived from the initiative name.
