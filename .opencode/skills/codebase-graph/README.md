# Codebase Graph Skill

A typed knowledge graph for your entire codebase that captures entities (files, functions, classes, interfaces, variables, modules) and their relationships (imports, calls, extends, implements, contains, exports).

## Overview

This skill analyzes your source code and builds a comprehensive knowledge graph stored in `./.codebase-graph/graph.json`. The graph enables:

- **Dependency Analysis**: Trace imports and function calls across your codebase
- **Code Navigation**: Quickly find where symbols are defined and used
- **Refactoring Support**: Understand impact before making changes
- **Architecture Understanding**: Visualize module boundaries and relationships

## Installation

### Prerequisites

- Python 3.8+
- Access to source code directories

### Setup

1. **Install Python dependencies**:
   ```bash
   pip install -r .opencode/skills/codebase-graph/scripts/requirements.txt
   ```

2. **Verify the skill is available**:
   ```bash
   python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py --help
   ```

## Usage

### Building the Graph

```bash
# Full rebuild (always fresh)
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
```

This will:
- Discover all source files in `src/`, `.opencode/`, `scripts/`, `tools/`, `tests/`, etc.
- Extract symbols (functions, classes, interfaces, variables)
- Build relations (imports, calls, extends, implements)
- Validate against schema constraints
- Generate `graph.json`, `index.md`, and `build.log`

### Querying the Graph

```bash
# Find functions by name
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "verifyToken"

# Find all functions in a file
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --file "src/auth/jwt.ts"

# Find classes
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Class --name "Repository"
```

### Dependency Analysis

```bash
# What does this file import?
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/lib/auth.ts"

# What calls this function?
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --function "createSession"
```

### Graph Statistics

```bash
# View graph statistics
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py stats

# Validate graph integrity
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate
```

## Output Files

| File | Description |
|------|-------------|
| `.codebase-graph/graph.json` | Complete typed knowledge graph |
| `.codebase-graph/index.md` | Human-readable overview |
| `.codebase-graph/build.log` | Processing log with timestamps |
| `.codebase-graph/schema.yaml` | Type definitions and constraints |
| `.codebase-graph/AGENTS.md.check` | AGENTS.md integration status |

## Graph Structure

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
    {"from": "func_001", "type": "calls", "to": "func_002"}
  ]
}
```

## Entity Types

| Type | Description |
|------|-------------|
| `CodeFile` | Source code file with imports/exports |
| `Function` | Function/method with signature and call relations |
| `Class` | Class definition with inheritance |
| `Interface` | Interface/type definition |
| `Variable` | Variable/constant with scope |
| `Module` | Module/package unit |

## Relations

| Relation | From | To | Description |
|----------|------|-----|-------------|
| `imports` | CodeFile | CodeFile | File import |
| `calls` | Function | Function | Function call |
| `extends` | Class | Class | Inheritance |
| `implements` | Class | Interface | Interface implementation |
| `contains` | CodeFile | Symbol | File contains symbol |
| `exports` | Module | Symbol | Module exports symbol |

## Source Directories

### Included
- `src/` - Main source code
- `.opencode/` - Skills and tools
- `scripts/` - Python scripts
- `tools/` - TypeScript tools
- `tests/` - Test files
- `forms/`, `labels/`, `statuses/`, `sources/`, `te9.dev/` - Project-specific

### Excluded
- `node_modules/` - Dependencies
- `docs/` - Documentation
- `build/`, `dist/` - Build output
- `.svelte-kit/` - Framework output
- `venv/`, `.venv/` - Python virtual environments

## Supported Languages

| Language | Extensions | Extraction Method |
|----------|------------|-------------------|
| TypeScript | `.ts`, `.tsx` | Regex + pattern matching |
| JavaScript | `.js`, `.jsx` | Regex + pattern matching |
| Python | `.py` | AST parsing |
| Svelte | `.svelte` | Regex + pattern matching |
| Rust | `.rs` | Regex (basic) |
| Go | `.go` | Regex (basic) |

## AGENTS.md Integration

When the graph is built, the skill checks if `AGENTS.md` contains a "Codebase Graph" section:

- **If present**: No action needed (idempotent)
- **If missing**: Creates `.codebase-graph/AGENTS.md.check` with section content for approval

To manually add to AGENTS.md:

```markdown
## Codebase Graph

The codebase graph is a typed knowledge graph of the entire source code.

**Location**: `./.codebase-graph/graph.json`

**Purpose**:
- Understand code structure and dependencies
- Trace function calls and class hierarchies
- Navigate complex multi-file projects
- Query symbol relationships efficiently

**Commands**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "<name>"
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "<path>"
```
```

## Troubleshooting

### Graph Not Found
```bash
# Build the graph first
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
```

### Python Dependencies Missing
```bash
# Install requirements
pip install -r .opencode/skills/codebase-graph/scripts/requirements.txt
```

### Parse Errors
Check the build log for details:
```bash
cat .codebase-graph/build.log
```

### Large Codebases
The skill skips files >1MB. Adjust in `codebase-graph.py` if needed:
```python
if file_path.stat().st_size > 1024 * 1024:  # Change this value
```

## Performance

| Codebase Size | Build Time | Graph Size |
|---------------|------------|------------|
| 50 files | <10s | <500KB |
| 200 files | <30s | <2MB |
| 500 files | <60s | <5MB |
| 1000+ files | <120s | <10MB |

## Best Practices

1. **Rebuild before major work**: Always build fresh before refactoring
2. **Use the index**: Check `index.md` for quick overview
3. **Validate regularly**: Run `validate` to ensure graph integrity
4. **Query efficiently**: Use specific filters instead of dumping all entities
5. **Check build log**: Review `build.log` for any processing issues

## Related Skills

- **codebase-context**: LLM context optimization using graph
- **ontology**: Typed knowledge graph patterns
- **skill-creator**: Creating new skills from workflows

## License

Part of the kees.pippeloi.nl project.