# Codebase Graph Test Cases

This document contains test cases for validating the codebase-graph skill functionality.

---

## TC-001: Full Rebuild

**Prompt**: "Create the codebase graph"

**Expected Output**:
- `.codebase-graph/graph.json` created with all entities
- `.codebase-graph/index.md` generated with human-readable overview
- `.codebase-graph/build.log` with complete processing history
- `.codebase-graph/AGENTS.md.check` with integration status
- `.codebase-graph/schema.yaml` present (copied from references)

**Validation Steps**:
1. Verify graph.json exists and is valid JSON
2. Check metadata contains: version, generated_at, root_directory, total_files, total_entities, total_relations
3. Verify all entity types present: CodeFile, Function, Class, Interface, Variable, Module
4. Check build.log shows: discovery, processing, relation building, validation, writing
5. Verify AGENTS.md.check status is "present" or "needs_approval"

**Success Criteria**:
- [ ] graph.json is valid JSON
- [ ] At least 10 CodeFile entities
- [ ] At least 20 Function entities
- [ ] At least 5 relations exist
- [ ] No constraint violations reported
- [ ] Build log shows completion

---

## TC-002: File Dependency Query

**Prompt**: "Show me what depends on src/auth/jwt.ts"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/auth/jwt.ts"
```

**Expected Results**:
- List of files that import jwt.ts
- List of functions in jwt.ts
- Functions that call functions from jwt.ts
- Visual dependency tree showing upstream and downstream dependencies

**Success Criteria**:
- [ ] File found in graph
- [ ] Imports listed correctly
- [ ] Importers (files that import this) listed
- [ ] Function call relationships shown

---

## TC-003: Function Callers Query

**Prompt**: "What calls the verifyToken function?"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --function "verifyToken"
```

**Expected Results**:
- Function location: `file_path#line_number`
- List of function names that call verifyToken
- File locations where calls occur
- Call chain visualization if available

**Success Criteria**:
- [ ] Function found in graph
- [ ] Location displayed correctly
- [ ] All callers listed
- [ ] Call chain traceable

---

## TC-004: Class Hierarchy Query

**Prompt**: "Show the inheritance tree for BaseEntity"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Class --name "BaseEntity"
```

**Expected Results**:
- Parent class (if BaseEntity extends something)
- Child classes that extend BaseEntity
- Interfaces implemented by BaseEntity
- Methods defined in BaseEntity

**Success Criteria**:
- [ ] Class found in graph
- [ ] Extends relation shown (if applicable)
- [ ] Implements relations shown (if applicable)
- [ ] Child classes discoverable via relations

---

## TC-005: Graph Validation

**Prompt**: "Validate the codebase graph integrity"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate
```

**Expected Results**:
- Validation passes with no errors, OR
- Specific constraint violations listed with details

**Success Criteria**:
- [ ] Validation completes without crashing
- [ ] All relation targets verified
- [ ] Cycle detection runs (for calls and extends)
- [ ] Clear pass/fail result reported

---

## TC-006: Graph Statistics

**Prompt**: "Show me statistics about the codebase graph"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py stats
```

**Expected Results**:
```
### Codebase Graph Statistics

**Generated:** 2025-01-15T14:30:22Z
**Root:** kees.pippeloi.nl

**Entities:**
  - CodeFile: 150
  - Function: 450
  - Class: 85
  - Interface: 30
  - Variable: 200
  - Module: 25

**Relations:** 3400

**File Size:** 2500.5KB
```

**Success Criteria**:
- [ ] All entity counts displayed
- [ ] Relation count displayed
- [ ] File size shown
- [ ] Generation timestamp present

---

## TC-007: AGENTS.md Integration Check

**Prompt**: "Check if AGENTS.md references the codebase graph"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py update-agents
```

**Expected Results**:
- If AGENTS.md has "## Codebase Graph" section: status = "present"
- If AGENTS.md missing section: status = "needs_approval" with section content
- If AGENTS.md not found: status = "not_found"

**Success Criteria**:
- [ ] AGENTS.md.check file updated
- [ ] Status accurately reflects integration state
- [ ] Section content provided if needs approval

---

## TC-008: Large File Handling

**Prompt**: "Build the codebase graph (with large files present)"

**Setup**: Place a file >1MB in a source directory

**Expected Results**:
- Large file skipped with log message
- Build continues successfully
- Other files processed normally

**Success Criteria**:
- [ ] Build log shows "Skipping large file: <path>"
- [ ] Build completes without error
- [ ] Graph contains all other valid files

---

## TC-009: Multi-Language Support

**Prompt**: "Build the codebase graph with TypeScript and Python files"

**Setup**: Codebase contains both .ts and .py files

**Expected Results**:
- TypeScript symbols extracted (functions, classes, interfaces)
- Python symbols extracted (functions, classes via AST)
- Both language entities present in graph
- Import relations captured for both

**Success Criteria**:
- [ ] TypeScript files processed
- [ ] Python files processed
- [ ] Language field correctly set per file
- [ ] Symbols extracted for both languages

---

## TC-010: Excluded Directory Handling

**Prompt**: "Build the codebase graph"

**Setup**: Files exist in node_modules/, docs/, .svelte-kit/

**Expected Results**:
- Files in excluded directories NOT in graph
- Files in source directories (src/, .opencode/, scripts/) ARE in graph
- Build log shows files discovered count matches expectation

**Success Criteria**:
- [ ] No node_modules files in graph
- [ ] No docs/ files in graph
- [ ] No .svelte-kit/ files in graph
- [ ] Source directory files present

---

## TC-011: Circular Dependency Detection

**Prompt**: "Validate the codebase graph for circular dependencies"

**Setup**: Create test files with circular imports/calls

**Expected Results**:
- Circular dependencies detected
- Warning logged (not error, as cycles can exist in code)
- Graph still built successfully
- Cycles marked in graph or log

**Success Criteria**:
- [ ] Cycles detected and reported
- [ ] Build completes despite cycles
- [ ] Cycle information available for review

---

## TC-012: Incremental Query Performance

**Prompt**: "Query for all functions named 'init'"

**Expected Output**:
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "init"
```

**Performance Criteria**:
- Query completes in <2 seconds
- Results returned even if graph has 1000+ entities
- Output formatted clearly

**Success Criteria**:
- [ ] Query completes quickly
- [ ] All matching functions returned
- [ ] Output readable and useful

---

## TC-013: Graph Rebuild Consistency

**Prompt**: "Rebuild the codebase graph twice and compare"

**Setup**: 
1. Build graph
2. Save graph hash
3. Rebuild graph
4. Compare hashes (should be similar, timestamps excepted)

**Expected Results**:
- Same files discovered
- Same entity count (assuming no code changes)
- Same relation count
- Only metadata timestamps differ

**Success Criteria**:
- [ ] Entity counts match between builds
- [ ] Relation counts match between builds
- [ ] Build is deterministic (excluding timestamps)

---

## TC-014: Error Handling - Missing Graph

**Prompt**: "Query the codebase graph" (when graph doesn't exist)

**Expected Output**:
```
Graph not found. Run 'build' first.
```

**Success Criteria**:
- [ ] Clear error message shown
- [ ] Instruction to build first provided
- [ ] No crash or stack trace

---

## TC-015: Error Handling - Parse Failure

**Prompt**: "Build the codebase graph" (with syntax error files present)

**Setup**: Include a file with invalid syntax

**Expected Results**:
- Parse error logged with file path
- Build continues with other files
- Graph built successfully (minus problematic file symbols)

**Success Criteria**:
- [ ] Error logged: "Parse error in <file>: <message>"
- [ ] Build completes
- [ ] Other files processed normally

---

## Test Execution Checklist

Run these tests in order for a complete validation:

```bash
# 1. Build the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build

# 2. Validate the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate

# 3. Check statistics
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py stats

# 4. Test queries
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "main"
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/main.ts"

# 5. Check AGENTS.md integration
cat .codebase-graph/AGENTS.md.check

# 6. Review build log
cat .codebase-graph/build.log
```

---

## Regression Testing

When modifying the codebase-graph skill, run all test cases and verify:

- [ ] TC-001: Full rebuild still works
- [ ] TC-005: Validation still passes
- [ ] TC-013: Rebuild consistency maintained
- [ ] All queries return expected results
- [ ] No new constraint violations introduced

---

## Performance Benchmarks

| Codebase Size | Build Time | Graph Size | Query Time |
|---------------|------------|------------|------------|
| 50 files | <10s | <500KB | <1s |
| 200 files | <30s | <2MB | <1s |
| 500 files | <60s | <5MB | <2s |
| 1000+ files | <120s | <10MB | <2s |

**Note**: Build times may vary based on file complexity and language mix.
