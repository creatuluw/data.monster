# te9.dev Skills

Simplified workflow for software development with AI.

## Skills

| Skill | Purpose |
|-------|---------|
| **te9-spec** | Spec-driven development with TDD workflow |
| **skill-creator** | Create and package effective skills |
| **spec-writer** | Write feature specifications |
| **tdd-workflow** | Test-Driven Development principles |
| **agent-browser** | Browser automation for AI agents |
| **codebase-context** | Document codebases for LLM understanding |
| **endpoint-inspect** | Analyze API endpoints and dependencies |
| **formbricks-mgr** | Formbricks survey management |
| **llm-docs** | Generate LLM-optimized documentation |
| **railway-docs** | Railway platform documentation |
| **mastering-typescript** | Enterprise TypeScript development |
| **modern-frontend-design** | Premium frontend design principles |
| **svelte5-best-practices** | Svelte 5 runes and patterns |
| **sveltekit-svelte5-tailwind-skill** | SvelteKit + Svelte 5 + Tailwind integration |

## Skill Descriptions

### Development Workflows

- **te9-spec** — Transform feature requests into executable TDD task plans. Use when building software, implementing features, or managing development workflows with spec-driven methodology.

- **spec-writer** — Write feature specifications that capture requirements and acceptance criteria. Use when writing specs, documenting functional requirements, defining acceptance criteria, or structuring product ideas.

- **tdd-workflow** — Test-Driven Development workflow principles. RED-GREEN-REFACTOR cycle. Use when implementing features with TDD, writing tests before code, or need TDD best practices.

### Tools & Automation

- **agent-browser** — Browser automation CLI for AI agents. Use when navigating websites, filling forms, clicking buttons, taking screenshots, extracting data, or automating browser tasks.

- **formbricks-mgr** — Formbricks survey creation and management. Use when creating surveys, managing survey structures, generating single-use links, exporting survey definitions, or implementing conditional question logic.

### Documentation & Analysis

- **codebase-context** — Document and structure codebases for LLM understanding using graph-based context. Use when working with large codebases, refactoring legacy code, onboarding to new projects, or optimizing LLM context.

- **llm-docs** — Generate LLM-optimized documentation for any codebase. Use when documenting routes/endpoints, mapping dependencies, analyzing architecture, or preparing codebase context for AI-assisted development.

- **endpoint-inspect** — Analyze a single API endpoint and its dependency chain. Use when understanding, documenting, or improving endpoint design, performance, security, or maintainability.

- **railway-docs** — Fetch up-to-date Railway platform documentation. Use when asked about Railway features, deployment, pricing, or when user shares a docs.railway.com URL.

### Language & Framework Skills

- **mastering-typescript** — Enterprise-grade TypeScript development with type-safe patterns. Use when building React/NestJS apps, migrating JS to TS, implementing advanced type patterns, or configuring modern toolchains.

- **svelte5-best-practices** — Svelte 5 runes, snippets, and modern patterns. Use when writing Svelte 5 components, migrating from Svelte 4, or need SvelteKit data loading patterns.

- **sveltekit-svelte5-tailwind-skill** — Comprehensive SvelteKit 2 + Svelte 5 + Tailwind CSS v4 integration. Use when building full-stack apps with this stack, configuring the integration, or troubleshooting deployment.

### Design

- **modern-frontend-design** — Premium frontend design principles and patterns. Use when designing user interfaces, planning visual systems, or validating design quality.

### Meta

- **skill-creator** — Guide for creating effective skills. Use when creating or updating a skill that extends agent capabilities with specialized knowledge, workflows, or tool integrations.

## Workflow

```
1. User enters prompt
2. spec-clarify → Ask questions (max 5)
3. spec-store → Save spec file
4. spec-execute → Implement and log
5. spec-branch-commit → Create branch and commit
6. spec-pr-create → Create pull request
7. spec-pr-review → Provide PR link for review
8. spec-track → Check status anytime
```

## Quick Start

### First Time Setup
```
te9-init
```

### Create Work
```
spec-clarify → spec-store → spec-execute → spec-branch-commit → spec-pr-create → spec-pr-review
```

### Check Status
```
spec-track                    # Overview
spec-track <spec-id>          # Details
spec-track <spec-id> --log    # Execution log
spec-track --commits          # Commit history
```

## Spec Status

- **PENDING** - Waiting to start
- **IN_PROGRESS** - Being executed
- **READY_FOR_COMMIT** - Ready for commit
- **COMPLETED** - Finished and pushed
- **FAILED** - Execution failed
- **BLOCKED** - Has blockers

## File Structure

```
te9.dev/
├── specs/
│   ├── spec-id-1/
│   │   └── spec.md
│   └── specs.json
└── logs/
    └── spec-id-1.log
```

## Technical Skills

Tools in `.opencode/tool/`:
- `daisyui.ts` - UI component templates
- `knowledge_graph.ts` - Memory graph export
- `melt.ts` - Code processing
- `uikit.ts` - UI helpers

## Memory System

OpenMemory is configured in `opencode.json` and documented in `openmemory.md`.

All spec-related work is automatically stored in OpenMemory for context.
