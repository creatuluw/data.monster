# PRD Template

Use this template when generating the PRD at the end of the interview. Fill in each section based on interview answers. Replace bracketed placeholders with actual content.

```markdown
# [Initiative Name]

## Feature Overview

[2-3 paragraph summary of the initiative at the epic level. What is it, why does it exist, what does it enable?]

## Problem Statement

[Describe the problem this initiative solves. Who experiences it, how do they cope today, why does it matter now?]

## Success Criteria

- [Measurable outcome 1]
- [Measurable outcome 2]
- [Measurable outcome 3]
- [User-facing success indicator]

## Design Goals

### Primary (Must)

- [Non-negotiable requirement 1]
- [Non-negotiable requirement 2]

### Secondary (Nice to Have)

- [Desirable but not blocking 1]
- [Desirable but not blocking 2]

## User Personas & Experience

### Personas

- **[Persona Name]**: [Description, role, goals, pain points]

### User Experience

[Describe the end-to-end user journey. What does the user do before this feature? What changes after? Walk through the key interactions.]

## Scope & Boundaries

### In Scope

- [Capability or feature that is part of this initiative]

### Out of Scope

- [Capability or feature explicitly excluded — prevents scope creep]

## High-Level Capabilities

### [Capability 1 Name]

[2-4 paragraphs describing what this capability does, who uses it, expected behavior, and how it fits into the larger initiative. Written at a level a spec-writer can expand into a detailed spec.]

### [Capability 2 Name]

[Same structure as above.]

[Continue for each capability identified during the interview.]

## Spec Candidates

The following capabilities should each become their own spec document. A spec-writer should create one spec per item below.

### Spec: [Capability 1 Name]

- **Description**: [What this spec should cover — the detailed implementation of this capability]
- **Dependencies**: [Other specs or capabilities this depends on, if any]
- **Priority**: [Must-have / Should-have / Nice-to-have]

### Spec: [Capability 2 Name]

- **Description**: [What this spec should cover]
- **Dependencies**: [Dependencies]
- **Priority**: [Priority]

[Continue for each spec candidate.]

## Technical Approach

[Describe the architecture, tech stack decisions, integration points, data flow, and any technical constraints that shape implementation. Include specific technology choices if known.]

## Constraints & Assumptions

- **[Constraint 1]**: [Description]
- **[Assumption 1]**: [Description]
- Continue as needed.

## Risks & Open Questions

### Risks

- **[Risk 1]**: [Description and potential mitigation]

### Open Questions

- [Question that needs to be resolved before or during implementation]
- Continue as needed.

## Phasing & Rollout

### Phase 1: MVP

[What is the minimum viable version? What capabilities are included?]

### Phase 2: [Name]

[What comes next? What additional capabilities or refinements?]

### Rollout Plan

[Feature flags, beta testing, gradual release, documentation, training, support — whatever applies.]
```
