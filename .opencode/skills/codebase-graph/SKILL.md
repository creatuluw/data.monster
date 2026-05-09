---
name: codebase-graph
description: Creates a typed knowledge graph of the entire codebase with entities (CodeFile, Function, Class, Interface, Variable, Module) and relations (imports, calls, extends, implements, contains, exports). Output stored in ./.codebase-graph/graph.json. Use when user explicitly references "codebase graph", "code graph", "symbol graph", or asks about code structure, dependencies, or relationships.
---

# Codebase Graph Skill

## Purpose

This skill creates a comprehensive, typed knowledge graph of your codebase that captures:
- **Entities**: Files, functions, classes, interfaces, variables, modules
- **Relations**: Imports, calls, extends, implements, contains, exports
- **Constraints**: Type validation, referential integrity, acyclic dependencies

The graph is stored in `./.codebase-graph/graph.json` and can be queried to understand code structure, trace dependencies, and navigate complex codebases efficiently.

## When to Use

| Trigger | Action |
|---------|--------|
| "Create the codebase graph" | Build full graph from scratch |
| "Update the codebase graph" | Full rebuild (always fresh) |
| "Show me dependencies for X" | Query graph after ensuring it exists |
| "What calls this function?" | Graph traversal query |
| "Map the code structure" | Generate graph + overview |
| "Index the codebase" | Build symbol graph |

**Important**: This skill is ONLY used when explicitly referenced in a prompt or instruction. Do not auto-invoke.

## Core Architecture

### Entity Types

```yaml
CodeFile:
  id: string (file path relative to root)
  language: string (ts, js, py, rs, go, etc.)
  size: number (bytes)
  last_modified: timestamp
  imports: string[] (paths to imported files)
  exports: string[] (exported symbol names)

Function:
  id: string (file_path#function_name)
  name: string
  signature: string
  file: string (path to containing file)
  line_start: number
  line_end: number
  parameters: {name, type}[]
  return_type: string
  calls: string[] (function IDs this function calls)
  called_by: string[] (function IDs that call this function)

Class:
  id: string (file_path#class_name)
  name: string
  file: string
  line_start: number
  line_end: number
  extends: string (parent class ID)
  implements: string[] (interface IDs)
  methods: string[] (function IDs)
  properties: {name, type, visibility}[]

Interface:
  id: string (file_path#interface_name)
  name: string
  file: string
  line_start: number
  line_end: number
  properties: {name, type}[]
  implemented_by: string[] (class IDs)

Variable:
  id: string (file_path#variable_name)
  name: string
  type: string
  file: string
  scope: string (module, function, class)
  usages: string[] (function IDs where used)

Module:
  id: string (path or package name)
  name: string
  path: string
  exports: string[] (symbol IDs)
  dependencies: string[] (module IDs)
```

### Relations

```yaml
imports: CodeFile -> CodeFile (many-to-many)
calls: Function -> Function (many-to-many)
extends: Class -> Class (many-to-one)
implements: Class -> Interface (many-to-many)
contains: CodeFile -> Function|Class|Variable (one-to-many)
exports: Module -> Function|Class|Variable|Interface (one-to-many)
```

## Output Structure

```
.codebase-graph/
  ├── graph.json           # Complete typed knowledge graph
  ├── schema.yaml          # Type definitions and constraints
  ├── index.md             # Human-readable overview for LLM context
  ├── build.log            # Processing log with timestamps
  └── AGENTS.md.check      # Tracks AGENTS.md integration status
```

### graph.json Structure

```json
{
  "metadata": {
    "version": "1.0",
    "generated_at": "2025-01-15T14:30:22Z",
    "root_directory": "kees.pippeloi.nl",
    "total_files": 150,
    "total_entities": 1250,
    "total_relations": 3400
  },
  "entities": {
    "CodeFile": [...],
    "Function": [...],
    "Class": [...],
    "Interface": [...],
    "Variable": [...],
    "Module": [...]
  },
  "relations": [
    {"from": "file_001", "type": "imports", "to": "file_002"},
    {"from": "func_001", "type": "calls", "to": "func_002"},
    ...
  ],
  "constraints": {
    "required_relations": ["imports", "contains"],
    "acyclic_relations": ["calls", "extends"],
    "validation_rules": [...]
  }
}
```

## Implementation Workflow

### Step 1: Discover Source Directories

Identify all source code directories (not docs, not node_modules):

```bash
# TypeScript/JavaScript
find . -path ./node_modules -prune -o -path ./docs -prune -o -name "*.ts" -print
find . -path ./node_modules -prune -o -path ./docs -prune -o -name "*.js" -print

# Python
find . -name "*.py" -not -path "./venv/*" -not -path "./.venv/*"

# Rust
find . -name "*.rs" -not -path "./target/*"

# Go
find . -name "*.go" -not -path "./vendor/*"
```

**Scope Rules**:
- ✅ Include: `src/`, `.opencode/`, `scripts/`, `tools/`, `tests/`
- ❌ Exclude: `node_modules/`, `docs/`, `build/`, `dist/`, `.svelte-kit/`

### Step 2: Extract Symbols (Hybrid Approach)

For each file, extract:

```bash
# Grep-based structure (fast)
grep -n "export.*function\|export.*class\|export.*interface" file.ts

# TypeScript compiler API (accurate, for details)
tsc --showConfig
# Parse AST for precise signatures

# Python AST
python3 -c "import ast; ..."
```

### Step 3: Build Graph

1. Create entity records for each symbol
2. Resolve relations (imports, calls, extends)
3. Validate against schema constraints
4. Write to `graph.json`

### Step 4: Generate Index

Create `index.md` with:
- Entry points overview
- Module dependency graph (text visualization)
- Symbol catalog by domain
- Query examples for LLMs

### Step 5: Check AGENTS.md

1. Read `AGENTS.md`
2. Search for "Codebase Graph" section
3. If **not found**:
   - Generate generic section text
   - Show preview to user
   - Request approval to add
   - If approved, insert under new "Codebase Graph" heading
4. If **found**: Skip (idempotent)

### Step 6: Log Build Process

Write to `build.log`:
```
[2025-01-15T14:30:22Z] Starting full rebuild
[2025-01-15T14:30:23Z] Discovered 150 source files
[2025-01-15T14:30:45Z] Extracted 450 functions
[2025-01-15T14:30:50Z] Extracted 85 classes
[2025-01-15T14:31:00Z] Resolved 3400 relations
[2025-01-15T14:31:05Z] Validated constraints (0 violations)
[2025-01-15T14:31:10Z] Wrote graph.json (2.5MB)
[2025-01-15T14:31:12Z] Generated index.md
[2025-01-15T14:31:15Z] AGENTS.md check: section added
[2025-01-15T14:31:15Z] Build complete
```

### Step 7: Add instructions for agents

**Important:** Add an AGENTS.md to the .\codebase-graph folder to instruct agents how they can update the docs with the skills found in .\.opencode\skills\codebase-graph\SKILL.md. In shorthand explain the commands, how to use them and what they do.

## Scripts

### Main Script: `scripts/codebase-graph.py`

```bash
# Full rebuild
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build

# Query graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "verifyToken"

# Show dependencies
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/auth/jwt.ts"

# Validate graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate
```

### Script Options

```
build          - Full rebuild of codebase graph
query          - Query entities by type/name
deps           - Show dependencies for a file/symbol
validate       - Check graph integrity
stats          - Display graph statistics
update-agents  - Check/update AGENTS.md integration
```

## Constraints

Defined in `references/schema.yaml`:

```yaml
entities:
  Function:
    required: [id, name, file, line_start, line_end]
    line_constraint: "line_end > line_start"
    
  Class:
    required: [id, name, file]
    extends_constraint: "extends must exist in Class entities"
    
  CodeFile:
    required: [id, language]
    language_enum: [ts, js, py, rs, go, svelte, json, yaml]

relations:
  calls:
    acyclic: true  # No circular function calls
    validate_targets: true  # Both functions must exist
    
  imports:
    allow_missing: true  # External imports ok
    resolve_relative: true  # Resolve ./../ paths
    
  extends:
    single_parent: true  # No multiple inheritance
```

## AGENTS.md Integration

When the skill adds a reference to AGENTS.md, it uses this generic template:

```markdown
## Codebase Graph

The codebase graph is a typed knowledge graph of the entire source code.

**Location**: `./.codebase-graph/graph.json`

**Purpose**: 
- Understand code structure and dependencies
- Trace function calls and class hierarchies
- Navigate complex multi-file projects
- Query symbol relationships efficiently

**Usage**:
1. Reference the graph in prompts: "Query the codebase graph for all functions that call X"
2. Read the index: `./.codebase-graph/index.md` for human-readable overview
3. Use the graph to understand dependencies before refactoring

**Commands**:
```bash
# Build/rebuild the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build

# Query the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "<name>"

# Show dependencies
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "<path>"
```

**Important**: The graph is rebuilt from scratch each time to ensure consistency and completeness.
```

## Best Practices

### Graph Completeness
- ✅ Process ALL source files in scope
- ✅ Extract ALL exported symbols
- ✅ Resolve ALL import relations
- ✅ Log ALL processing steps
- ❌ Never skip files silently
- ❌ Never miss dependency relations

### Consistency
- Always rebuild from scratch (no incremental)
- Use deterministic ordering (sorted paths, names)
- Include timestamps in metadata
- Validate after every build

### Query Efficiency
- Index entities by ID for O(1) lookup
- Pre-compute transitive relations
- Cache frequently-traversed paths
- Use line-delimited format for streaming

## Test Cases

Save test cases to `references/test_cases.md`:

```markdown
# Test Cases

## TC-001: Full Rebuild
**Prompt**: "Create the codebase graph"
**Expected**: 
- graph.json created with all entities
- index.md generated
- build.log with complete processing history
- AGENTS.md checked/updated

## TC-002: Dependency Query
**Prompt**: "Show me what depends on src/auth/jwt.ts"
**Expected**:
- List of files importing jwt.ts
- List of functions calling functions from jwt.ts
- Visual dependency tree

## TC-003: Function Callers
**Prompt**: "What calls the verifyToken function?"
**Expected**:
- List of function names and file locations
- Call chain visualization
- Line numbers where calls occur

## TC-004: Class Hierarchy
**Prompt**: "Show the inheritance tree for BaseEntity"
**Expected**:
- Parent classes (if any)
- Child classes extending it
- Interfaces implemented
```

## Error Handling

| Error | Action |
|-------|--------|
| File not found | Log warning, continue processing |
| Parse failure | Log error with file path, skip file |
| Circular dependency | Detect, log, mark in graph |
| Missing target relation | Log, mark as external |
| Schema violation | Fail build, report violations |

## Related Skills

- **codebase-context**: For LLM context optimization using graph
- **ontology**: For typed knowledge graph patterns
- **skill-creator**: For creating new skills from this workflow

## References

- `references/schema.yaml` — Full type definitions
- `references/queries.md` — Query language examples
- `references/test_cases.md` — Test case documentation

# Important - During execution

- During execution dont store files into the root of a project. Always use a .\.codebase-graph\tmp folder to store files needed for processing.
