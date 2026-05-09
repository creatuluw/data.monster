# Codebase Graph Query Examples

This document provides examples of how to query the codebase graph for common use cases.

---

## Basic Queries

### Query by Entity Type

```bash
# Get all functions
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function

# Get all classes
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Class

# Get all code files
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type CodeFile
```

### Query by Name

```bash
# Find function by name
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "verifyToken"

# Find class by name
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Class --name "Repository"
```

### Query by File

```bash
# Find all functions in a file
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --file "src/lib/auth.ts"

# Find all classes in a file
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Class --file "src/data/repository.ts"
```

---

## Dependency Queries

### File Dependencies

```bash
# What does this file import?
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/auth/jwt.ts"

# What files import this one?
# (Shown in the "Imported by" section of deps output)
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/lib/utils.ts"
```

### Function Dependencies

```bash
# What does this function call?
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --function "createSession"

# What calls this function?
# (Shown in the "Called by" section of deps output)
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --function "validateInput"
```

### Full Dependency Tree

To get the full dependency tree for a file, chain multiple `deps` calls:

```bash
# Step 1: Get direct imports
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/main.ts"

# Step 2: For each imported file, get its imports
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/lib/init.ts"
# ... repeat for each import
```

---

## Graph Statistics

```bash
# Get overall graph statistics
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py stats

# Validate graph integrity
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate
```

---

## Advanced Queries (Direct JSON Access)

For more complex queries, you can directly parse the `graph.json` file:

### Find All Callers of a Function (Python)

```python
import json

with open('.codebase-graph/graph.json') as f:
    data = json.load(f)

target_function = "verifyToken"

# Find the function entity
func = next(
    (f for f in data['entities']['Function'] if f['name'] == target_function),
    None
)

if func:
    func_id = func['id']
    
    # Find all relations where this function is the target
    callers = [
        r['from'] for r in data['relations']
        if r['type'] == 'calls' and r['to'] == func_id
    ]
    
    print(f"Functions that call {target_function}:")
    for caller in callers:
        print(f"  - {caller}")
```

### Find Circular Dependencies

```python
import json
from collections import defaultdict

with open('.codebase-graph/graph.json') as f:
    data = json.load(f)

# Build adjacency list for calls relation
call_graph = defaultdict(list)
for rel in data['relations']:
    if rel['type'] == 'calls':
        call_graph[rel['from']].append(rel['to'])

# DFS to detect cycles
def find_cycles(graph):
    visited = set()
    rec_stack = set()
    cycles = []
    
    def dfs(node, path):
        visited.add(node)
        rec_stack.add(node)
        path.append(node)
        
        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                cycle = dfs(neighbor, path)
                if cycle:
                    return cycle
            elif neighbor in rec_stack:
                # Found cycle
                cycle_start = path.index(neighbor)
                return path[cycle_start:] + [neighbor]
        
        path.pop()
        rec_stack.remove(node)
        return None
    
    for node in graph:
        if node not in visited:
            cycle = dfs(node, [])
            if cycle:
                cycles.append(cycle)
    
    return cycles

cycles = find_cycles(call_graph)
if cycles:
    print("Circular dependencies detected:")
    for cycle in cycles:
        print(" -> ".join(cycle))
else:
    print("No circular dependencies found")
```

### Find Most Central Files (by Import Count)

```python
import json
from collections import Counter

with open('.codebase-graph/graph.json') as f:
    data = json.load(f)

# Count how many times each file is imported
import_counts = Counter()
for rel in data['relations']:
    if rel['type'] == 'imports':
        import_counts[rel['to']] += 1

print("Most imported files:")
for file_path, count in import_counts.most_common(10):
    print(f"  {count:3d} imports: {file_path}")
```

### Find Orphaned Functions (Never Called)

```python
import json

with open('.codebase-graph/graph.json') as f:
    data = json.load(f)

# Get all function IDs
all_functions = {f['id'] for f in data['entities']['Function']}

# Get all called functions
called_functions = {
    r['to'] for r in data['relations']
    if r['type'] == 'calls'
}

# Find orphans (excluding entry points)
entry_points = {'main', 'init', 'setup', 'bootstrap'}
orphaned = [
    f for f in data['entities']['Function']
    if f['id'] not in called_functions
    and f['name'] not in entry_points
    and f.get('exported', False)
]

print("Potentially orphaned exported functions:")
for func in orphaned[:20]:
    print(f"  - {func['name']} in {func['file']}")
```

---

## Query Patterns for LLMs

When working with an LLM, reference the graph in your prompts:

### Pattern 1: Understand Before Modifying

```
Before I modify the `verifyToken` function, query the codebase graph 
to find all files that depend on it. I need to understand the impact 
of my changes.
```

### Pattern 2: Trace Execution Flow

```
Using the codebase graph, trace the call chain starting from 
`main.ts#main()` down to the database connection logic. Show me 
each function in the chain with its file location.
```

### Pattern 3: Find Implementation

```
Query the codebase graph for all classes that implement the 
`Repository` interface. For each implementation, list the file 
location and any classes that extend it.
```

### Pattern 4: Identify Refactoring Candidates

```
Analyze the codebase graph to find functions with the most callers. 
These might be good candidates for optimization or caching. Show 
the top 10 with their caller counts.
```

### Pattern 5: Understand Module Boundaries

```
Using the import relations in the codebase graph, identify the 
module boundaries in this project. Which files form the core 
modules, and how do they depend on each other?
```

---

## Integration with Other Tools

### VS Code Integration

You can create VS Code tasks to run common queries:

```json
// .vscode/tasks.json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Build Codebase Graph",
      "type": "shell",
      "command": "python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build",
      "group": "build"
    },
    {
      "label": "Query Codebase Graph",
      "type": "shell",
      "command": "python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name ${input:functionName}",
      "inputs": [
        {
          "id": "functionName",
          "type": "promptString",
          "description": "Enter function name"
        }
      ]
    }
  ]
}
```

### CI/CD Integration

Add graph validation to your CI pipeline:

```yaml
# .github/workflows/codebase-graph.yml
name: Codebase Graph

on:
  push:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Graph
        run: python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
      
      - name: Validate Graph
        run: python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py validate
```

---

## Performance Tips

1. **Use the index**: For quick lookups, check `index.md` first before querying the full graph

2. **Filter early**: When writing custom queries, filter entities before building relation lists

3. **Cache results**: For repeated queries, cache the parsed graph in memory

4. **Batch operations**: When modifying the graph, batch relation updates together

5. **Use stats command**: Before heavy queries, run `stats` to understand graph size

---

## Troubleshooting

### Graph Not Found
```bash
# Build the graph first
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
```

### Query Returns No Results
- Check that the entity name is exact (case-sensitive)
- Verify the entity type is correct
- Try querying without filters to see available entities

### Validation Failures
```bash
# Check the build log for details
cat .codebase-graph/build.log

# Rebuild the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build
```

### Missing Dependencies in Graph
- Ensure the file is in a source directory (src/, .opencode/, scripts/, etc.)
- Check that the file extension is supported
- Verify the file is not in an excluded directory (node_modules/, docs/, etc.)