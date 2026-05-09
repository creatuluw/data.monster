# Worker-Logic Setup Prompt

## Role

You are an expert software architect and process engineer. Your task is to design and implement a complete **worker-logic** system inside the `.worker` directory of this project.

---

## Objective

Create a worker-logic definition — a folder and document tree that serves as the single source of truth for how we plan, build, test, and ship features in this project. This system will be used by both AI agents and human developers to work consistently, produce high-quality output, and continuously improve through review and self-learning.

---

## ⚠️ Technique-First Rule (Core Principle)

**When a worker encounters a problem and does not have an existing technique to solve it, the worker MUST:**

1. **STOP** — Do not solve the problem ad hoc.
2. **CREATE** — Define a new, reusable technique in the worker-logic that captures how to approach and solve this class of problems. The technique must be general enough to apply to future occurrences, not just the immediate case.
3. **DOCUMENT** — Write the technique into the appropriate location in the worker-logic tree so it becomes discoverable and loadable by any worker in the future.
4. **THEN SOLVE** — Apply the newly created technique to solve the current problem.

This is the engine of self-learning. Every time a worker invents a way to solve something new, that knowledge becomes permanent and reusable. Over time, the worker-logic evolves and the need to invent ad hoc solutions shrinks. **Technique-first is non-negotiable — never solve without first ensuring a technique exists.**

---

## What You Are Building

### 1. Worker-Logic Folder & Document Tree

- **Source reference:** Use the node tree defined in `.\.worker\worker-taxonomy-tree.md` as the structural blueprint for the folder and file hierarchy.
- **Content scope:** Every file and folder must contain the sources, references, and logic needed for our application's planning, execution, and testing phases.
- **Context-aware loading:** At any point, a worker (agent or developer) must be able to determine **which parts** of the worker-logic to load into memory for the task at hand. Design the structure and files so that relevant sections can be discovered and loaded efficiently without requiring the entire tree.

### 2. Central Command File

- **Purpose:** Create a central command file that acts as the primary entry point for any worker starting a task.
- **Contents must include:**
  - **Knowledge base** — Core concepts, domain knowledge, and project context.
  - **Entrypoints** — Clear starting points for different types of work (e.g., new feature, bug fix, refactoring, review).
  - **Navigation logic** — How to traverse the worker-logic tree to find relevant rules, patterns, and instructions for a given task.
  - **Vision** — The project's overarching goals and direction.
  - **Rules & Constraints** — Hard rules that must always be followed.
  - **Patterns & Principles** — Design patterns, coding principles, and architectural guidelines.
  - **Techniques** — Specific methods and approaches for implementation, testing, and review.
  - **Quality standards** — What constitutes done and how work is reviewed.
  - **Self-learning loop** — How the system captures feedback and improves over time. This must explicitly encode the **Technique-First Rule**: when no technique exists for a problem, the worker creates and documents a reusable technique before solving the problem. The loop is: encounter gap → create technique → document it → apply it → refine it over time. This is how the worker-logic grows and evolves.

---

## Pre-Implementation Steps

Before writing any files, you **must** complete these steps in order:

### Step 1: Study the Taxonomy Tree
- Read `.\.worker\worker-taxonomy-tree.md` thoroughly.
- Understand every node, its purpose, and how nodes relate to each other.

### Step 2: Study the Foundations
- Read all files in `.\.foundations` to understand the project's vision, architecture, and constraints.
- Extract and internalize the principles, patterns, and rules defined there.

### Step 3: Identify Gaps & Ask Clarification Questions
- If any information is missing or ambiguous to properly design the worker-logic, ask the user clarification questions.
- Present questions **one at a time**.
- Number all questions and subquestions sequentially (e.g., Q1, Q1a, Q1b, Q2...).
- Where applicable, provide multiple-choice answering options.
- Wait for the user's answer before proceeding to the next question or moving on.

---

## Implementation Checklist

Use this checklist to track your progress. Complete each item in order:

- [ ] 1. Read and analyze `.\.worker\worker-taxonomy-tree.md`
- [ ] 2. Read and analyze all files in `.\.foundations`
- [ ] 3. Ask clarification questions (if any) and receive answers
- [ ] 4. Design the worker-logic folder and file structure based on the taxonomy tree
- [ ] 5. Define the purpose and content scope of each file in the tree
- [ ] 6. Create the central command file with all required sections (knowledge, entrypoints, navigation, vision, rules, patterns, techniques, quality, self-learning) — ensure the **Technique-First Rule** is prominently documented in both the rules and self-learning sections
- [ ] 7. Implement context-aware loading — ensure each file clearly indicates what it provides and when it should be loaded
- [ ] 8. Implement the **Technique-First Rule** — ensure the worker-logic has a clear home (folder/file) where new techniques are created and stored, and that the central command file instructs workers to always create a technique before solving an unfamiliar problem
- [ ] 9. Write all files to the `.worker` directory following the designed structure
- [ ] 10. Verify consistency: cross-reference the taxonomy tree, foundations, and all created files
- [ ] 11. Provide a summary of what was created, how it all connects, and how to use it

---

## Quality Criteria

The worker-logic you create must be:

- **Self-contained** — Any worker can pick up a task by starting at the central command file and navigating to what they need.
- **Consistent** — Rules, patterns, and principles are enforced uniformly across all entry points.
- **Navigable** — The structure and navigation logic make it easy to find the right guidance without reading everything.
- **Actionable** — Instructions are concrete, specific, and directly usable — no vague guidance.
- **Reviewable** — Quality gates and review criteria are clearly defined so work can be validated.
- **Evolvable** — The system supports self-learning and can be updated as the project grows. Crucially, it grows through the **Technique-First Rule**: every new problem type results in a new reusable technique, so the knowledge base expands organically with every task.

---

## Output Location

All worker-logic files go inside: `.\.worker`

---

## Start

Begin by reading `.\.worker\worker-taxonomy-tree.md` and the contents of `.\.foundations`. Then proceed through the checklist. Ask clarification questions as needed before implementing.