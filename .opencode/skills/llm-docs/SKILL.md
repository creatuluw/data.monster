---
name: llm-docs
description: Generate LLM-optimized documentation for any codebase using git-ingest. Extracts routes/endpoints, maps dependencies, creates modular docs in .llm-docs/ with progressive disclosure. Language/framework agnostic. Use when (1) documenting codebase for LLM understanding, (2) onboarding to a project, (3) analyzing architecture for improvements, (4) preparing codebase context for AI-assisted development. Triggers: "document my routes", "extract endpoints", "analyze codebase", "generate llm docs", "map dependencies", "git ingest".
---

# LLM Docs Generator

Generate structured, LLM-optimized documentation for any codebase using `git-ingest` as the extraction engine.

## Quick Start

```bash
# Generate codebase dump with git-ingest
npx git-ingest --output .llm-docs/codebase.md --format markdown

# Then run the documentation processor
# (LLM reads codebase.md and generates structured docs)
```

## Two Modes

### Mode 1: Documentation Generation

1. **Run git-ingest** - Extract codebase to markdown
   ```bash
   npx git-ingest --output .llm-docs/codebase.md --format markdown
   ```

2. **LLM processes output** - Read codebase.md and generate:
   - Route/endpoint extraction (framework-agnostic detection)
   - Dependency mapping (import analysis)
   - Per-endpoint documentation
   - Navigation index

3. **Output structure**:
   ```
   .llm-docs/
   ├── codebase.md           # Raw git-ingest output
   ├── INDEX.md              # Navigation entry point
   ├── endpoints/            # Per-endpoint docs
   │   ├── api-users.md
   │   └── api-posts.md
   ├── modules/              # Shared modules/libraries
   │   ├── auth.md
   │   └── database.md
   └── analysis/
       └── improvements.md
   ```

### Mode 2: Codebase Analysis

Analyze for improvements across categories. Optionally scope to specific paths:

```bash
# Analyze specific directory
npx git-ingest --include "src/api/**" --output .llm-docs/api-scope.md
```

Analysis categories:
| Category | What it checks |
|----------|----------------|
| Architecture | Layer separation, coupling, dependency direction |
| Patterns | Consistency, idiomatic usage, anti-patterns |
| Performance | N+1 queries, memory leaks, inefficient loops |
| Type Safety | Any usage, missing types, runtime vs compile gaps |
| Security | Input validation, auth, data exposure |
| UX/DX | Error handling, ergonomics, maintainability |

## Endpoint Detection (Framework-Agnostic)

Detect routes/endpoints by pattern matching:

| Framework | Pattern |
|-----------|---------|
| SvelteKit | `src/routes/**/+page.server.ts`, `+server.ts` |
| Next.js | `app/**/route.ts`, `pages/api/**` |
| Express | `app.METHOD()`, `router.METHOD()` |
| FastAPI | `@app.get()`, `@router.post()` |
| Rails | `routes.rb`, controllers |
| Django | `urls.py`, `views.py` |
| Go (net/http) | `http.HandleFunc`, `HandleFunc` |
| Spring | `@GetMapping`, `@PostMapping` |

## Workflow for LLM

When using this skill:

1. **Run git-ingest first**
   ```bash
   npx git-ingest --output .llm-docs/codebase.md
   ```

2. **Read INDEX.md** (if exists) to understand structure

3. **Detect framework** by scanning for patterns

4. **Extract endpoints** by pattern matching

5. **Map dependencies** by analyzing imports/requires

6. **Generate docs** following the template below

7. **Update INDEX.md** with navigation

## Per-Endpoint Doc Template

```markdown
# Endpoint: [HTTP_METHOD] /path/to/endpoint

## Purpose
[What this endpoint does - inferred from code]

## Request
- Method: [GET|POST|PUT|DELETE|...]
- Path: /api/resource/:id
- Params: { id: [type] }
- Body: [schema if applicable]
- Headers: [required headers]

## Response
- Success: [status code] [body structure]
- Errors: [status codes and conditions]

## Dependencies
[List imported modules and their purpose]

## Type Definitions
[Extracted types/interfaces/schemas]

## Considerations
- Security: [auth, validation]
- Performance: [caching, pagination]
- Edge Cases: [what to handle]
```

## INDEX.md Template

```markdown
# Codebase Documentation

## Overview
[Brief description of project purpose]

## Tech Stack
- Language: [detected]
- Framework: [detected]
- Database: [if found]

## Endpoints
| Path | Method | Purpose | Doc |
|------|--------|---------|-----|
| /api/users | GET | List users | [endpoints/api-users.md](./endpoints/api-users.md) |

## Modules
| Module | Purpose | Doc |
|--------|---------|-----|
| auth | Authentication | [modules/auth.md](./modules/auth.md) |

## Navigation Guide
1. Start here (INDEX.md)
2. Read relevant endpoint docs
3. Reference modules for shared logic
4. Check analysis for known issues
```

## References

- [Endpoint Patterns](./references/endpoint-patterns.md) - Framework-specific detection patterns
- [Analysis Patterns](./references/analysis-patterns.md) - Code review patterns by category
