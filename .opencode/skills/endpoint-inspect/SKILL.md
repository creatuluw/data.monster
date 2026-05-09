---
name: endpoint-inspect
description: Analyze a single API endpoint and its entire dependency chain to produce architectural insights and improvement recommendations. Use when you need to understand, document, or improve a specific endpoint's design, performance, security, or maintainability. Triggers: (1) "analyze endpoint X", (2) "study the /api/users endpoint", (3) "review endpoint architecture", (4) "endpoint advice for", (5) "deep dive into endpoint".
---

# Endpoint Advice

Analyze a single endpoint comprehensively, tracing all dependencies, understanding the architecture, and providing actionable insights following Karpathy-inspired principles.

## Workflow

### 1. Identify the Target Endpoint

Before analysis, clarify:
- The specific endpoint to analyze (route/path + HTTP method)
- The context name for the output file (used in `.endpoint-findings/<context>.md`)

If the user doesn't specify the context name, derive it from the endpoint path (e.g., `/api/users` → `users-endpoint`).

### 2. Locate the Endpoint Entry Point

Search the codebase to find:
- Route definitions (framework-specific: Express routes, FastAPI decorators, etc.)
- Handler/controller functions
- Middleware applied to the route

**Search patterns:**
- `grep` for route decorators/definitions
- Look for framework-specific patterns (e.g., `@app.route`, `router.get`, `@GetMapping`)

### 3. Trace the Dependency Chain

Follow the complete execution flow:

```
Entry Point → Handler → Service Layer → Data Access → External Services
     ↓              ↓           ↓              ↓              ↓
  Middleware    Validation   Business      Database       API Calls
                Transform     Logic         Queries        Cache/Queue
```

For each layer, identify:
- **Files involved**: Full paths to all relevant files
- **Functions called**: Named functions and their signatures
- **Data structures**: Request/response types, DTOs, models
- **Dependencies**: External services, databases, caches
- **Configuration**: Environment variables, feature flags

### 4. Apply Karpathy Principles During Analysis

Reference `references/karpathy-guidelines.md` and `references/karpathy-examples.md` for detailed guidance.

#### Think Before Coding (Analysis Phase)
- State assumptions about the endpoint's purpose explicitly
- Note any ambiguities in the implementation
- Identify multiple interpretations if behavior is unclear
- List what you don't understand and need to investigate

#### Simplicity First (Code Review Phase)
- Identify over-engineered sections
- Flag unnecessary abstractions
- Note speculative features not currently needed
- Highlight code that could be simpler

#### Surgical Changes (Improvement Phase)
- Only suggest changes that directly address identified issues
- Preserve existing code style
- Don't refactor unrelated code
- Note dead code but don't suggest removal unless critical

#### Goal-Driven Execution (Documentation Phase)
- Define verifiable success criteria for any recommendations
- Provide test scenarios that would validate fixes
- Include before/after verification steps

### 5. Document Findings

Create a comprehensive analysis document following this structure:

```markdown
# Endpoint Analysis: [ENDPOINT_PATH]

## Overview

- **Route**: `[METHOD] /path/to/endpoint`
- **Purpose**: [What this endpoint does]
- **Entry Point**: `path/to/file.py::function_name`
- **Last Analyzed**: [Date]

## Dependency Graph

[Visual or textual representation of the call chain]

## Code Walkthrough

### Entry Point
[Handler function analysis with line references]

### Service Layer
[Business logic analysis]

### Data Access
[Database/API calls analysis]

### External Dependencies
[Third-party services, caches, queues]

## Request/Response Flow

### Input Validation
[How inputs are validated]

### Transformations
[Data transformations applied]

### Output Formatting
[Response serialization]

## Issues Identified

### Critical
[Security vulnerabilities, data loss risks, etc.]

### Major
[Performance issues, maintainability problems]

### Minor
[Style inconsistencies, minor improvements]

## Recommendations

### Priority 1: [Issue Name]
- **Problem**: [Description]
- **Solution**: [Specific fix]
- **Verification**: [How to test the fix works]

### Priority 2: [Issue Name]
[Continue for each recommendation]

## Assumptions Made

- [Assumption 1]
- [Assumption 2]

## Uncertainties

- [What needs clarification]

## Code References

### Key Files
- `path/to/file1.py` - [Brief description]
- `path/to/file2.py` - [Brief description]

### Key Functions
- `function_name()` at `file.py:L123-145` - [Purpose]

## Success Criteria

For any recommended changes, verify:
1. [Specific test case] passes
2. [Performance benchmark] improves
3. [Existing tests] still pass
```

### 6. Write Output

Save findings to `.endpoint-findings/<context>.md` where `<context>` is:
- User-specified name, OR
- Derived from endpoint path (sanitized: lowercase, hyphens, no special chars)

**Example paths:**
- `/api/users` → `.endpoint-findings/users-endpoint.md`
- `/api/v1/orders/{id}/items` → `.endpoint-findings/orders-id-items.md`
- Custom context → `.endpoint-findings/<context>.md`

## Analysis Checklist

Before completing analysis, verify:

- [ ] All imported modules traced to their source files
- [ ] Database queries identified and explained
- [ ] External API calls documented with their purpose
- [ ] Error handling paths mapped
- [ ] Authentication/authorization checks identified
- [ ] Rate limiting or throttling logic documented
- [ ] Caching strategies identified
- [ ] Middleware chain documented
- [ ] Request/response schemas documented
- [ ] Side effects identified (emails, webhooks, etc.)

## References

For detailed guidelines and examples, see:
- `references/karpathy-guidelines.md` - Core principles for code analysis
- `references/karpathy-examples.md` - Real-world examples of good vs. bad patterns
- `references/karpathy-readme.md` - Context on why these principles matter

## Example Usage

**User**: "Analyze the /api/users POST endpoint"

**Agent**:
1. Locates route definition in `src/routes/users.py`
2. Traces handler → service → repository layers
3. Identifies validation, business logic, database operations
4. Applies Karpathy principles to identify issues
5. Creates `.endpoint-findings/users-post-endpoint.md`

**User**: "Give me endpoint advice for the checkout flow"

**Agent**:
1. Asks clarifying question: "Which specific endpoint? The checkout process may have multiple endpoints (create-order, process-payment, complete-checkout)."
2. Once clarified, follows full analysis workflow
