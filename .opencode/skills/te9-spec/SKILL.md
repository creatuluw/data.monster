---
name: te9-spec
description: Spec-driven development workflow for AI agents. Orchestrates requirements gathering, implementation, testing, and deployment in a structured 6-step process. Use when building software, implementing features, or managing development workflows.
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, Task, TodoWrite
---

# TE9-Spec: Spec-Driven Development

Transform high-level feature requests into executable TDD task plans.

## Workflow

### Step 1: Create Spec Directory

```bash
mkdir -p .specs/feature-name/
```

### Step 2: Write Complete Spec

Create `.specs/feature-name/spec.md` with all required sections:

| Section | Content |
|---------|---------|
| Feature Overview | 2-3 paragraphs: what, who, problem solved |
| Success Criteria | Measurable outcomes defining "done" |
| Design Goals | Primary (must) and secondary (nice to have) |
| User Experience | 1-2 paragraphs: interaction, journey |
| Design Rationale | 1-2 paragraphs: why this approach, trade-offs |
| Constraints/Assumptions | Technical constraints, business assumptions |
| Functional Requirements | FR-N format, max 6-8, with acceptance criteria |
| Edge Cases | Unusual inputs, failure scenarios |

### Step 3: Generate tasks.json

Break spec into TDD tasks in `.specs/feature-name/tasks.json`:

**CRITICAL: Before generating tasks, extract every FR-N from the spec and verify each will have at least one corresponding task. After generating tasks, perform a bidirectional coverage check: every FR-N must have a task with `mapsTo: "FR-N"`, and every task must map to a valid FR-N, Edge Case, or setup/teardown concern.**

```json
{
  "project": "feature-name",
  "spec": ".specs/feature-name/spec.md",
  "developmentMethodology": "TDD (RED-GREEN-REFACTOR cycle)",
  "tasks": [
    {
      "id": "core-001",
      "phase": "Core Functionality",
      "title": "Implement feature requirement",
      "description": "FR-1: Description of requirement",
      "acceptanceCriteria": [
        "TDD: Write test for behavior before implementation",
        "TDD: Test fails when feature is missing",
        "TDD: Refactor after test passes"
      ],
      "priority": 1,
      "status": "pending",
      "mapsTo": "FR-1"
    }
  ],
  "tddWorkflow": {
    "enforced": true,
    "cycle": "RED-GREEN-REFACTOR",
    "rules": [
      "Write test first (RED) - test must fail",
      "Write minimal code to pass (GREEN) - no optimization",
      "Refactor only after test passes (REFACTOR) - maintain quality",
      "All tasks must follow TDD cycle as acceptance criteria"
    ]
  }
}
```

### Step 4: Task Structure

Each task includes:

- **id**: Unique identifier (phase-NNN)
- **phase**: Project phase (Setup, Core, Testing, etc.)
- **title**: Short task name
- **description**: What the task does, mapped to spec requirement
- **acceptanceCriteria**: TDD workflow (RED → GREEN → REFACTOR)
- **priority**: 1 (must have), 2 (should have), 3 (nice to have)
- **status**: pending, in_progress, completed, cancelled
- **mapsTo**: Spec requirement (FR-N, Edge Cases, etc.)

### Step 5: TDD Acceptance Criteria Pattern

Every task acceptance criteria follows:

```markdown
- TDD: Write test for [behavior] before implementation
- TDD: Test fails when [condition]
- TDD: Refactor after test passes
```

### Step 6: Execute Development

**MANDATORY: Load ALL tasks into todo list before writing any code.**

1. Read ALL tasks from `tasks.json` — do not skip any, do not start coding yet
2. Count total tasks and record the number
3. Use `TodoWrite` to create a todo item for EVERY task in `tasks.json` before writing a single line of code:
   - Each todo item: `[task-id] task-title` with status `pending`
   - The full todo list must be loaded in one `TodoWrite` call
4. For each task (by priority, lowest number first):
   - **Before starting**: Confirm it is the highest-priority task with status `pending`. Do not skip tasks. Do not jump ahead.
   - RED: Write failing test
   - GREEN: Write minimal code to pass
   - REFACTOR: Improve quality
   - Run Step 7: Task Completion Checklist
   - Update task status to `completed` in `tasks.json`
   - Mark the corresponding todo item as `completed` via `TodoWrite`
5. **If a task cannot be completed**: Mark its status as `blocked` in `tasks.json` with a note explaining why. Do NOT silently skip tasks. Do NOT mark as completed.
6. **Mid-execution verification**: After every 3 tasks completed, re-read `tasks.json` and confirm no tasks were accidentally skipped or left in `pending` state behind completed ones.
7. **Session boundary**: If the session ends before all tasks are complete, record which tasks remain incomplete in the todo list so the next session can resume from the correct position.

### Step 7: Task Completion Checklist

Before marking any task as "completed" in tasks.json, verify the following:

#### 1. Task Logging

Write task summary to `tasks-log.json` with:
- Task ID and title
- Summary of work performed
- Working functionality confirmed
- Date/time of completion
- Any notes or decisions made

[ task_id | title | summary | is_working | timestamp | notes | decisions_made ]

#### 2. Code Review (Karpathy Guidelines)

Review all code changes against these behavioral guidelines:

**1. Think Before Coding**
- State assumptions explicitly
- Present multiple interpretations if they exist
- Identify simpler approaches when available
- Name what's confusing and ask before proceeding

**2. Simplicity First**
- No features beyond what was asked
- No abstractions for single-use code
- No unrequested "flexibility" or "configurability"
- No error handling for impossible scenarios
- If 200 lines could be 50, rewrite it

**3. Surgical Changes**
- Touch only what's necessary
- Don't "improve" adjacent code, comments, or formatting
- Don't refactor things that aren't broken
- Match existing style
- Remove only imports/variables/functions made unused by YOUR changes
- Every changed line should trace to the user's request

**4. Goal-Driven Execution**
- Define verifiable success criteria
- Loop until verified
- For multi-step tasks, state a brief plan with verification checks

#### 3. Documentation Updates

Update `/docs` or `README.md` if:
- New functionality was added
- API surface changed
- Configuration options modified
- Usage patterns affected

## Important: Acceptance Criteria to add to a task in tasks.json

All points from the "Step 7: Task Completion Checklist" need to be added as acceptanceCriteria in the tasks.json. These being:

1. Task Logging
2. Code Review (Karpathy Guidelines)
3. Documentation Updates

**Mandatory:** These points need to be part of every task in tasks.json and need to be defined in it while writing the tasks.json.

### Again; don't forget:

**All these points and their actions need to be an integral part of any tasks' acceptanceCriteria in the tasks.json file. These criteria need to be met before any task can be switched to complete.**

### Step 8: Feature Completion Gate

**MANDATORY: Before declaring the feature complete, execute this gate. Do NOT skip.**

1. Read ALL tasks from `tasks.json`
2. Verify every single task has `status: "completed"` — no exceptions
3. Count completed tasks and confirm the count equals the total task count
4. Cross-reference the todo list: every todo item must be `completed`
5. If ANY task is `pending`, `in_progress`, `blocked`, or `cancelled` — the feature is NOT done. Return to Step 6.
6. Perform bidirectional coverage check one final time:
   - Every FR-N in `spec.md` has at least one task with `mapsTo: "FR-N"`
   - Every task in `tasks.json` maps to a valid FR-N, Edge Case, or setup/teardown concern
7. Only after ALL checks pass may the feature be declared complete.

### Example task

```json
{
      "id": "core-006",
      "phase": "Core",
      "title": "Implement bounce suppression service",
      "description": "FR-5: Create src/lib/services/bounceService.ts that manages the _bpm_email_suppressions table. Provides addSuppression(), removeSuppression(), isSuppressed(), getSuppressedAddresses(). Hard bounces auto-suppress, soft bounces do not.",
      "acceptanceCriteria": [
        "TDD: Write test that addSuppression inserts a record into _bpm_email_suppressions",
        "TDD: Write test that isSuppressed returns true for suppressed addresses",
        "TDD: Write test that removeSuppression removes a suppression record",
        "TDD: Write test that soft bounces do NOT trigger suppression",
        "TDD: Refactor to add caching layer for frequently checked addresses",
        "Task Logging: .specs/feature-name/tasks.json -> task_id | title | summary | is_working | timestamp | notes | decisions_made",
        "Code Review: Use section -> 2. Code Review (Karpathy Guidelines) in .opencode\skills\te9-spec\SKILL.md",
        "Update Docs: Use section -> 3. Documentation Updates in .opencode\skills\te9-spec\SKILL.md"
      ],
      "priority": 1,
      "status": "completed",
      "mapsTo": "FR-5"
    }
```


## Output Locations

- **Spec**: `.specs/feature-name/spec.md`
- **Tasks**: `.specs/feature-name/tasks.json`

## Differences from spec-writer

| Aspect | spec-writer | te9-spec |
|--------|-------------|----------|
| Spec filename | README.md | spec.md |
| Spec directory | specs/ | .specs/ |
| Tasks | Not generated | tasks.json with TDD |
| Development method | Not specified | TDD enforced |
| Output | Spec only | Spec + executable task plan |

## Example Usage

When user requests a feature:

1. Gather requirements
2. Create `.specs/feature-name/spec.md` (complete spec)
3. Generate `.specs/feature-name/tasks.json` (TDD tasks with bidirectional FR coverage)
4. Load ALL tasks into `TodoWrite` before writing any code
5. Execute tasks in priority order, updating todo list as you go
6. Run Feature Completion Gate (Step 8) before declaring done

## Validation Checklist

- [ ] Spec in `.specs/*/spec.md` (not README.md)
- [ ] Tasks in `.specs/*/tasks.json`
- [ ] All tasks have TDD acceptance criteria
- [ ] Tasks mapped to spec requirements (FR-N)
- [ ] Every FR-N in spec.md has at least one task with `mapsTo: "FR-N"` (bidirectional coverage)
- [ ] Priority ordering for MVP sequencing
- [ ] Edge cases covered in tasks
- [ ] ALL tasks loaded into `TodoWrite` before implementation started
- [ ] Task logged to `tasks-log.json` before completion
- [ ] Code reviewed against Karpathy Guidelines
- [ ] Documentation updated if functionality changed
- [ ] ALL tasks in `tasks.json` have status `completed` (feature completion gate)
- [ ] Todo list shows all items `completed`
