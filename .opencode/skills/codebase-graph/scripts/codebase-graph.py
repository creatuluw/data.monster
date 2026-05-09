#!/usr/bin/env python3
"""
codebase-graph.py - Creates a typed knowledge graph of the entire codebase

Usage:
    python3 codebase-graph.py build          # Full rebuild
    python3 codebase-graph.py query --type Function --name <name>
    python3 codebase-graph.py deps --file <path>
    python3 codebase-graph.py validate
    python3 codebase-graph.py stats
    python3 codebase-graph.py update-agents
"""

import ast
import hashlib
import json
import os
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple

import yaml

# Configuration
PROJECT_ROOT = Path(__file__).parent.parent.parent.parent.parent  # kees.pippeloi.nl
GRAPH_DIR = PROJECT_ROOT / ".codebase-graph"
GRAPH_FILE = GRAPH_DIR / "graph.json"
SCHEMA_FILE = GRAPH_DIR / "schema.yaml"
INDEX_FILE = GRAPH_DIR / "index.md"
FILE_TREE_FILE = GRAPH_DIR / "file-tree.md"
FILES_SUMMARY_FILE = GRAPH_DIR / "files.json"
BUILD_LOG = GRAPH_DIR / "build.log"
AGENTS_CHECK = GRAPH_DIR / "AGENTS.md.check"
AGENTS_FILE = PROJECT_ROOT / "AGENTS.md"

# Summary configuration - 25 token max per file
SUMMARY_TOKEN_LIMIT = 25  # Max tokens per file summary

# Source directories to include - only src/ folders (application source code)
SOURCE_DIRS = [
    "src",
]

# Directories to exclude
EXCLUDE_DIRS = [
    "node_modules",
    "docs",
    "build",
    "dist",
    ".svelte-kit",
    "venv",
    ".venv",
    "target",
    "vendor",
    "_archive",
    "playwright-report",
    "test-results",
    ".codebase-graph",
]

# File extensions to process
EXTENSIONS = {
    ".ts": "typescript",
    ".tsx": "typescript",
    ".js": "javascript",
    ".jsx": "javascript",
    ".py": "python",
    ".rs": "rust",
    ".go": "go",
    ".svelte": "svelte",
    ".json": "json",
    ".yaml": "yaml",
    ".yml": "yaml",
    ".md": "markdown",
}


class Logger:
    """Handles logging to both console and build.log"""

    def __init__(self, log_file: Path):
        self.log_file = log_file
        self.logs = []

    def log(self, message: str):
        timestamp = datetime.now().isoformat()
        entry = f"[{timestamp}] {message}"
        self.logs.append(entry)
        print(entry)

    def save(self):
        self.log_file.write_text("\n".join(self.logs) + "\n")


class CodebaseGraph:
    """Main class for building and managing the codebase knowledge graph"""

    def __init__(self, logger: Logger):
        self.logger = logger
        self.entities: Dict[str, List[Dict]] = {
            "CodeFile": [],
            "Function": [],
            "Class": [],
            "Interface": [],
            "Variable": [],
            "Module": [],
        }
        self.relations: List[Dict] = []
        self.file_cache: Dict[str, str] = {}  # path -> content
        self.symbol_index: Dict[str, str] = {}  # symbol_name -> file_path
        self.summary_cache: Dict[str, str] = {}  # path -> summary

    def discover_files(self) -> List[Path]:
        """Discover all source files in scope - only src/ folders"""
        self.logger.log("Discovering source files in src/ directories...")
        files = []

        # Code extensions only (no documentation or config files)
        CODE_EXTENSIONS = [".ts", ".tsx", ".js", ".jsx", ".py", ".rs", ".go", ".svelte"]

        for ext in CODE_EXTENSIONS:
            for pattern in [f"*{ext}"]:
                for file_path in PROJECT_ROOT.rglob(pattern):
                    # Check if in source directory
                    rel_path = file_path.relative_to(PROJECT_ROOT)
                    parts = rel_path.parts

                    # Skip excluded directories
                    if any(exclude in parts for exclude in EXCLUDE_DIRS):
                        continue

                    # Only include files in src/ directories
                    # This captures: src/, dev-docs/src/, schema-mgr/src/, etc.
                    if "src" not in parts:
                        continue

                    # Skip if file doesn't exist or is too large (>1MB)
                    if not file_path.exists():
                        continue
                    if file_path.stat().st_size > 1024 * 1024:
                        self.logger.log(f"Skipping large file: {rel_path}")
                        continue

                    files.append(file_path)

        self.logger.log(f"Discovered {len(files)} source files in src/ directories")
        return sorted(files)

    def read_file(self, path: Path) -> str:
        """Read file content with caching"""
        path_str = str(path)
        if path_str not in self.file_cache:
            try:
                self.file_cache[path_str] = path.read_text(encoding="utf-8")
            except Exception as e:
                self.logger.log(f"Error reading {path}: {e}")
                self.file_cache[path_str] = ""
        return self.file_cache[path_str]

    def extract_ts_symbols(
        self, path: Path, content: str
    ) -> Tuple[List[Dict], List[Dict], List[Dict]]:
        """Extract TypeScript/JavaScript symbols using regex and simple parsing"""
        functions = []
        classes = []
        interfaces = []

        lines = content.split("\n")
        rel_path = str(path.relative_to(PROJECT_ROOT))

        # Export patterns
        export_func_pattern = re.compile(
            r"export\s+(?:default\s+)?(?:async\s+)?function\s+(\w+)\s*\(([^)]*)\)(?:\s*:\s*([^{;]+))?"
        )
        export_class_pattern = re.compile(
            r"export\s+(?:default\s+)?class\s+(\w+)(?:\s+extends\s+(\w+))?(?:\s+implements\s+([^ {]+))?"
        )
        export_interface_pattern = re.compile(
            r"export\s+(?:default\s+)?interface\s+(\w+)(?:\s+extends\s+([^ {]+))?"
        )
        export_const_pattern = re.compile(
            r"export\s+(?:const|let|var)\s+(\w+)(?:\s*:\s*([^=;]+))?"
        )
        export_type_pattern = re.compile(r"export\s+type\s+(\w+)\s*=")

        # Function patterns (not exported)
        func_pattern = re.compile(
            r"^(?:export\s+)?(?:async\s+)?function\s+(\w+)\s*\(([^)]*)\)(?:\s*:\s*([^{;]+))?"
        )
        class_pattern = re.compile(
            r"^(?:export\s+)?class\s+(\w+)(?:\s+extends\s+(\w+))?(?:\s+implements\s+([^ {]+))?"
        )

        for i, line in enumerate(lines, 1):
            stripped = line.strip()

            # Skip comments
            if stripped.startswith("//") or stripped.startswith("*"):
                continue

            # Exported functions
            match = export_func_pattern.search(stripped)
            if match:
                name, params, return_type = match.groups()
                functions.append(
                    {
                        "id": f"{rel_path}#{name}",
                        "name": name,
                        "signature": stripped.split("{")[0].strip(),
                        "file": rel_path,
                        "line_start": i,
                        "line_end": i + self._count_braces_forward(lines, i - 1),
                        "parameters": self._parse_params(params or ""),
                        "return_type": return_type or "void",
                        "exported": True,
                        "calls": [],
                        "called_by": [],
                    }
                )
                self.symbol_index[name] = rel_path

            # Exported classes
            match = export_class_pattern.search(stripped)
            if match:
                name, extends, implements = match.groups()
                classes.append(
                    {
                        "id": f"{rel_path}#{name}",
                        "name": name,
                        "file": rel_path,
                        "line_start": i,
                        "line_end": i + self._count_braces_forward(lines, i - 1),
                        "extends": f"{self.symbol_index.get(extends, '')}#{extends}"
                        if extends
                        else None,
                        "implements": [
                            f"{self.symbol_index.get(i.strip(), '')}#{i.strip()}"
                            for i in implements.split(",")
                        ]
                        if implements
                        else [],
                        "methods": [],
                        "properties": [],
                    }
                )
                self.symbol_index[name] = rel_path

            # Exported interfaces
            match = export_interface_pattern.search(stripped)
            if match:
                name, extends = match.groups()
                interfaces.append(
                    {
                        "id": f"{rel_path}#{name}",
                        "name": name,
                        "file": rel_path,
                        "line_start": i,
                        "line_end": i + self._count_braces_forward(lines, i - 1),
                        "properties": [],
                        "implemented_by": [],
                    }
                )
                self.symbol_index[name] = rel_path

            # Exported constants/variables
            match = export_const_pattern.search(stripped)
            if match:
                name, var_type = match.groups()
                # We'll track these separately if needed

            # Type exports
            match = export_type_pattern.search(stripped)
            if match:
                name = match.group(1)
                self.symbol_index[name] = rel_path

        return functions, classes, interfaces

    def extract_python_symbols(
        self, path: Path, content: str
    ) -> Tuple[List[Dict], List[Dict]]:
        """Extract Python symbols using AST"""
        functions = []
        classes = []

        rel_path = str(path.relative_to(PROJECT_ROOT))

        try:
            tree = ast.parse(content)
        except SyntaxError as e:
            self.logger.log(f"Python parse error in {rel_path}: {e}")
            return functions, classes

        for node in ast.walk(tree):
            if isinstance(node, ast.FunctionDef):
                # Get decorator info
                decorators = [
                    d.id if isinstance(d, ast.Name) else str(d)
                    for d in node.decorator_list
                ]

                # Get parameter info
                params = []
                for arg in node.args.args:
                    params.append(
                        {
                            "name": arg.arg,
                            "type": ast.unparse(arg.annotation)
                            if arg.annotation
                            else "Any",
                        }
                    )

                # Get return type
                return_type = ast.unparse(node.returns) if node.returns else "None"

                functions.append(
                    {
                        "id": f"{rel_path}#{node.name}",
                        "name": node.name,
                        "signature": f"def {node.name}({', '.join(p['name'] for p in params)}) -> {return_type}",
                        "file": rel_path,
                        "line_start": node.lineno,
                        "line_end": node.end_lineno or node.lineno,
                        "parameters": params,
                        "return_type": return_type,
                        "exported": not node.name.startswith("_"),
                        "decorators": decorators,
                        "calls": [],
                        "called_by": [],
                    }
                )
                self.symbol_index[node.name] = rel_path

            elif isinstance(node, ast.ClassDef):
                bases = [
                    b.id if isinstance(b, ast.Name) else str(b) for b in node.bases
                ]

                classes.append(
                    {
                        "id": f"{rel_path}#{node.name}",
                        "name": node.name,
                        "file": rel_path,
                        "line_start": node.lineno,
                        "line_end": node.end_lineno or node.lineno,
                        "extends": bases[0] if bases else None,
                        "implements": [],
                        "methods": [],
                        "properties": [],
                    }
                )
                self.symbol_index[node.name] = rel_path

        return functions, classes

    def extract_imports(self, path: Path, content: str) -> List[str]:
        """Extract import statements from a file"""
        imports = []
        rel_path = str(path.relative_to(PROJECT_ROOT))

        # TypeScript/JavaScript imports
        ts_import_patterns = [
            re.compile(r'import\s+.*?\s+from\s+[\'"]([^\'"]+)[\'"]'),
            re.compile(r'import\s+[\'"]([^\'"]+)[\'"]'),
            re.compile(r'require\s*\([\'"]([^\'"]+)[\'"]\)'),
        ]

        for pattern in ts_import_patterns:
            for match in pattern.finditer(content):
                import_path = match.group(1)
                # Resolve relative imports
                if import_path.startswith("."):
                    resolved = self._resolve_import(path, import_path)
                    if resolved:
                        imports.append(str(resolved.relative_to(PROJECT_ROOT)))
                else:
                    imports.append(import_path)  # External import

        # Python imports
        py_import_patterns = [
            re.compile(r"^import\s+(\w+)"),
            re.compile(r"^from\s+([\w.]+)\s+import"),
        ]

        for pattern in py_import_patterns:
            for match in pattern.finditer(content, re.MULTILINE):
                imports.append(match.group(1))

        return list(set(imports))

    def extract_exports(self, path: Path, content: str) -> List[str]:
        """Extract export statements from a file"""
        exports = []

        # TypeScript/JavaScript exports
        export_patterns = [
            re.compile(
                r"export\s+(?:default\s+)?(?:function|class|interface|type|const|let|var)\s+(\w+)"
            ),
            re.compile(r"export\s*\{\s*([^}]+)\s*\}"),
        ]

        for pattern in export_patterns:
            for match in pattern.finditer(content):
                if match.group(1):
                    # Handle both single names and destructured exports
                    names = [
                        n.strip().split(" as ")[-1].strip()
                        for n in match.group(1).split(",")
                    ]
                    exports.extend(names)

        return list(set(exports))

    def _count_braces_forward(self, lines: List[str], start_idx: int) -> int:
        """Count lines until closing brace (simple heuristic)"""
        brace_count = 0
        started = False
        for i in range(start_idx, min(start_idx + 200, len(lines))):
            line = lines[i]
            brace_count += line.count("{") - line.count("}")
            if line.count("{") > 0:
                started = True
            if started and brace_count <= 0:
                return i - start_idx + 1
        return 50  # Default if we can't find closing

    def _parse_params(self, param_str: str) -> List[Dict]:
        """Parse function parameters from string"""
        if not param_str.strip():
            return []

        params = []
        for part in param_str.split(","):
            part = part.strip()
            if ":" in part:
                name, ptype = part.split(":", 1)
                params.append({"name": name.strip(), "type": ptype.strip()})
            else:
                params.append({"name": part, "type": "any"})
        return params

    def _resolve_import(self, current_file: Path, import_path: str) -> Optional[Path]:
        """Resolve a relative import path to an actual file"""
        current_dir = current_file.parent
        resolved = (current_dir / import_path).resolve()

        # Try with extensions
        for ext in ["", ".ts", ".tsx", ".js", ".jsx", "/index.ts", "/index.js"]:
            test_path = Path(str(resolved) + ext)
            if test_path.exists():
                return test_path

        return None

    def build_graph(self):
        """Build the complete codebase graph"""
        self.logger.log("Starting full rebuild")

        # Discover files
        files = self.discover_files()

        # Process each file
        for file_path in files:
            rel_path = str(file_path.relative_to(PROJECT_ROOT))
            content = self.read_file(file_path)
            ext = file_path.suffix.lower()

            self.logger.log(f"Processing: {rel_path}")

            # Create CodeFile entity
            code_file = {
                "id": rel_path,
                "path": rel_path,
                "language": EXTENSIONS.get(ext, "unknown"),
                "size": file_path.stat().st_size,
                "last_modified": datetime.fromtimestamp(
                    file_path.stat().st_mtime
                ).isoformat(),
                "imports": [],
                "exports": [],
            }

            # Extract symbols based on language
            if ext in [".ts", ".tsx", ".js", ".jsx", ".svelte"]:
                functions, classes, interfaces = self.extract_ts_symbols(
                    file_path, content
                )
                code_file["imports"] = self.extract_imports(file_path, content)
                code_file["exports"] = self.extract_exports(file_path, content)

                self.entities["Function"].extend(functions)
                self.entities["Class"].extend(classes)
                self.entities["Interface"].extend(interfaces)

            elif ext == ".py":
                functions, classes = self.extract_python_symbols(file_path, content)
                code_file["imports"] = self.extract_imports(file_path, content)
                code_file["exports"] = [f["name"] for f in functions if f["exported"]]

                self.entities["Function"].extend(functions)
                self.entities["Class"].extend(classes)

            self.entities["CodeFile"].append(code_file)

        # Build relations
        self.logger.log("Building relations...")
        self._build_relations()

        # Validate
        self.logger.log("Validating graph...")
        violations = self._validate_graph()
        if violations:
            self.logger.log(f"Found {len(violations)} constraint violations")
            for v in violations[:10]:
                self.logger.log(f"  - {v}")
        else:
            self.logger.log("No constraint violations found")

        # Write graph
        self.logger.log("Writing graph.json...")
        self._write_graph()

        # Generate index
        self.logger.log("Generating index.md...")
        self._generate_index()

        # Generate file tree
        self.logger.log("Generating file-tree.md...")
        self._generate_file_tree()

        # Generate file summaries with AI
        self.logger.log("Generating files.md with AI summaries...")
        self._generate_files_summary()

        # Check AGENTS.md
        self.logger.log("Checking AGENTS.md integration...")
        self._check_agents()

        self.logger.log("Build complete")

    def _build_relations(self):
        """Build all relations between entities"""
        # Contains relations (file -> symbols)
        for func in self.entities["Function"]:
            self.relations.append(
                {"from": func["file"], "type": "contains", "to": func["id"]}
            )

        for cls in self.entities["Class"]:
            self.relations.append(
                {"from": cls["file"], "type": "contains", "to": cls["id"]}
            )

        for iface in self.entities["Interface"]:
            self.relations.append(
                {"from": iface["file"], "type": "contains", "to": iface["id"]}
            )

        # Import relations (file -> file)
        for code_file in self.entities["CodeFile"]:
            for imp in code_file.get("imports", []):
                # Check if it's a local file
                if imp in [f["id"] for f in self.entities["CodeFile"]]:
                    self.relations.append(
                        {"from": code_file["id"], "type": "imports", "to": imp}
                    )

        # Build call graph (simple text-based for now)
        self._build_call_graph()

    def _build_call_graph(self):
        """Build function call relations by scanning for function calls"""
        func_names = {f["name"]: f["id"] for f in self.entities["Function"]}

        for func in self.entities["Function"]:
            file_path = PROJECT_ROOT / func["file"]
            content = self.read_file(file_path)

            # Simple pattern matching for function calls
            for name, func_id in func_names.items():
                if name != func["name"]:  # Don't count self-calls
                    pattern = re.compile(rf"\b{re.escape(name)}\s*\(")
                    # Check if this function calls that function
                    if pattern.search(content):
                        self.relations.append(
                            {"from": func["id"], "type": "calls", "to": func_id}
                        )
                        # Update called_by
                        func["called_by"].append(func_id)

    def _validate_graph(self) -> List[str]:
        """Validate graph against constraints"""
        violations = []

        # Check all relations point to existing entities
        all_entity_ids = set()
        for entity_type, entities in self.entities.items():
            for entity in entities:
                all_entity_ids.add(entity["id"])

        for rel in self.relations:
            if rel["to"] not in all_entity_ids and rel["type"] != "imports":
                violations.append(
                    f"Relation {rel['from']} --{rel['type']}--> {rel['to']} (target missing)"
                )

        return violations

    def _write_graph(self):
        """Write the graph to graph.json"""
        graph = {
            "metadata": {
                "version": "1.0",
                "generated_at": datetime.now().isoformat(),
                "root_directory": str(PROJECT_ROOT.name),
                "total_files": len(self.entities["CodeFile"]),
                "total_entities": sum(len(e) for e in self.entities.values()),
                "total_relations": len(self.relations),
            },
            "entities": self.entities,
            "relations": self.relations,
            "constraints": {
                "required_relations": ["imports", "contains"],
                "acyclic_relations": ["calls", "extends"],
                "validation_rules": [
                    "All relation targets must exist",
                    "No circular function calls",
                    "Class extends must reference existing class",
                ],
            },
        }

        GRAPH_DIR.mkdir(parents=True, exist_ok=True)
        GRAPH_FILE.write_text(json.dumps(graph, indent=2))
        self.logger.log(f"Wrote graph.json ({GRAPH_FILE.stat().st_size / 1024:.1f}KB)")

    def _generate_index(self):
        """Generate human-readable index.md"""
        # Count statistics
        stats = {k: len(v) for k, v in self.entities.items()}

        # Find entry points
        entry_points = []
        for code_file in self.entities["CodeFile"]:
            if (
                "main" in code_file["path"]
                or "index" in code_file["path"]
                or "app" in code_file["path"]
            ):
                entry_points.append(code_file)

        # Build module dependency summary
        import_counts = {}
        for code_file in self.entities["CodeFile"]:
            for imp in code_file.get("imports", []):
                import_counts[imp] = import_counts.get(imp, 0) + 1

        top_imported = sorted(import_counts.items(), key=lambda x: -x[1])[:10]

        index_content = f"""# Codebase Graph Index

Generated: {datetime.now().isoformat()}

## Overview

| Entity Type | Count |
|-------------|-------|
| Code Files | {stats.get("CodeFile", 0)} |
| Functions | {stats.get("Function", 0)} |
| Classes | {stats.get("Class", 0)} |
| Interfaces | {stats.get("Interface", 0)} |
| Variables | {stats.get("Variable", 0)} |
| Modules | {stats.get("Module", 0)} |

**Total Relations:** {len(self.relations)}

## Entry Points

| File | Language | Size |
|------|----------|------|
"""

        for ep in entry_points[:10]:
            index_content += (
                f"| `{ep['path']}` | {ep['language']} | {ep['size'] / 1024:.1f}KB |\n"
            )

        index_content += """
## Most Imported Files

| File | Import Count |
|------|--------------|
"""

        for file_path, count in top_imported:
            index_content += f"| `{file_path}` | {count} |\n"

        index_content += """
## Query Examples

### Find all functions in a file
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --file "src/auth/jwt.ts"
```

### Find what calls a function
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --function "verifyToken"
```

### Show file dependencies
```bash
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "src/lib/data.ts"
```

## Graph Structure

The graph is stored in `graph.json` with the following structure:

```json
{
  "metadata": {...},
  "entities": {
    "CodeFile": [...],
    "Function": [...],
    "Class": [...],
    ...
  },
  "relations": [
    {"from": "...", "type": "imports", "to": "..."},
    ...
  ]
}
```

## Usage Guidelines

1. **For LLM Context**: Reference specific entities by ID instead of including full file contents
2. **For Dependency Analysis**: Query the `relations` array for `imports` or `calls` types
3. **For Refactoring**: Check `called_by` to find all usages before modifying a function
4. **For Navigation**: Use the entity index to quickly locate symbols

## Files

- `graph.json` - Complete typed knowledge graph
- `schema.yaml` - Type definitions and constraints
- `index.md` - This file (human-readable overview)
- `build.log` - Processing log with timestamps
- `AGENTS.md.check` - AGENTS.md integration status
"""

        INDEX_FILE.write_text(index_content)

    def _generate_file_tree(self):
        """Generate file-tree.md with the full processed file tree"""
        # Organize files by directory
        tree = {}
        for code_file in self.entities["CodeFile"]:
            path = code_file["path"]
            parts = path.replace("\\", "/").split("/")

            current = tree
            for part in parts[:-1]:
                if part not in current:
                    current[part] = {}
                current = current[part]
            current[parts[-1]] = code_file

        # Build tree content
        content = f"""# Codebase File Tree

Generated: {datetime.now().isoformat()}

**Total Files:** {len(self.entities["CodeFile"])}

This file contains the complete list of all source code files processed by the codebase graph.
Use this file to quickly search and locate files in the codebase.

## Directory Structure

```
"""

        def build_tree_str(node, prefix="", is_last=True):
            result = ""
            items = sorted(
                node.items(),
                key=lambda x: (isinstance(x[1], dict) and "path" not in x[1], x[0]),
            )
            for i, (name, child) in enumerate(items):
                is_last_item = i == len(items) - 1
                connector = "└── " if is_last_item else "├── "

                # Check if it's a file entity (has 'path' key) or directory
                if isinstance(child, dict) and "path" in child:
                    # It's a file entity
                    result += f"{prefix}{connector}{name} ({child['language']}, {child['size'] / 1024:.1f}KB)\n"
                elif isinstance(child, dict):
                    # It's a directory
                    result += f"{prefix}{connector}{name}/\n"
                    extension = "    " if is_last_item else "│   "
                    result += build_tree_str(child, prefix + extension, is_last_item)
            return result

        content += build_tree_str(tree)
        content += """```

## File List by Language

"""

        # Group by language
        by_language = {}
        for code_file in self.entities["CodeFile"]:
            lang = code_file["language"]
            if lang not in by_language:
                by_language[lang] = []
            by_language[lang].append(code_file)

        for lang in sorted(by_language.keys()):
            files = sorted(by_language[lang], key=lambda x: x["path"])
            content += f"### {lang.title()} ({len(files)} files)\n\n"
            for f in files:
                content += f"- `{f['path']}` ({f['size'] / 1024:.1f}KB)\n"
            content += "\n"

        content += """## Search Tips

- Use Ctrl+F (or Cmd+F) to search for file names
- Files are organized by directory structure above
- Files are grouped by language below for quick reference
- File sizes are shown in KB
"""

        FILE_TREE_FILE = GRAPH_DIR / "file-tree.md"
        FILE_TREE_FILE.write_text(content, encoding="utf-8")
        self.logger.log(
            f"Wrote file-tree.md ({FILE_TREE_FILE.stat().st_size / 1024:.1f}KB)"
        )

    def _generate_file_summary(self, file_path: str, content: str) -> str:
        """Generate purpose description for a file"""
        # Check cache first
        if file_path in self.summary_cache:
            return self.summary_cache[file_path]

        # Extract key info
        file_name = Path(file_path).name
        exports = self.extract_exports(PROJECT_ROOT / file_path, content)
        functions = [
            f["name"] for f in self.entities["Function"] if f["file"] == file_path
        ]
        classes = [c["name"] for c in self.entities["Class"] if c["file"] == file_path]

        # Generate purpose description
        purpose = self._detect_purpose(
            file_path, file_name, exports, functions, classes
        )

        self.summary_cache[file_path] = purpose
        return purpose

    def _detect_purpose(
        self,
        file_path: str,
        file_name: str,
        exports: list,
        functions: list,
        classes: list,
    ) -> str:
        """Detect file purpose from path and content analysis"""
        path_lower = file_path.lower()

        # Component files
        if file_name.endswith(".svelte"):
            component_name = file_name.replace(".svelte", "")
            if "form" in path_lower:
                return (
                    f"A form component for {component_name.lower().replace('-', ' ')}"
                )
            elif "table" in path_lower or "list" in path_lower:
                return f"A table/list component for displaying data"
            elif "modal" in path_lower or "drawer" in path_lower:
                return f"A modal/drawer component for overlays"
            else:
                return f"A UI component for {component_name.lower().replace('-', ' ')}"

        # API/Client files
        elif "api" in path_lower or "client" in path_lower:
            return f"HTTP client for API communication and data fetching"

        # Store files
        elif "store" in path_lower:
            return f"State management store for application data"

        # Route/Page files
        elif "route" in path_lower or "+" in file_name:
            if "server" in file_name:
                return f"Server-side page handler and data loader"
            else:
                return f"Page component and client-side route handler"

        # Utility files
        elif "util" in path_lower or "helper" in path_lower:
            return f"Utility functions for common operations"

        # Type definition files
        elif "type" in path_lower or file_name.endswith(".d.ts"):
            return f"TypeScript type definitions and interfaces"

        # Config files
        elif "config" in path_lower:
            return f"Configuration settings and constants"

        # Service files
        elif "service" in path_lower:
            service_name = file_name.replace(".ts", "").replace(".js", "")
            return f"Service logic for {service_name.replace('-', ' ')}"

        # Manager files
        elif "manager" in path_lower or "mgr" in path_lower:
            return f"Management logic for coordinating operations"

        # Schema/Database files
        elif "schema" in path_lower or "database" in path_lower:
            return f"Database schema definitions and migrations"

        # Logger files
        elif "logger" in path_lower or "log" in path_lower:
            return f"Logging utilities for application events"

        # Default based on exports
        if exports:
            return f"Module providing {', '.join(exports[:2])}"
        elif functions:
            return f"Functions: {', '.join(functions[:2])}"
        elif classes:
            return f"Class definition: {classes[0]}"

        return f"Source code module"

    def _generate_files_summary(self):
        """Generate files.json with AI-assisted summaries for each file"""
        # Load existing cache if available
        cache_file = GRAPH_DIR / "summaries_cache.json"
        if cache_file.exists():
            try:
                self.summary_cache = json.loads(cache_file.read_text(encoding="utf-8"))
                self.logger.log(f"Loaded {len(self.summary_cache)} cached summaries")
            except:
                self.summary_cache = {}
        self.logger.log("Calculating file usage counts...")

        # Sort files by path for consistent output
        sorted_files = sorted(self.entities["CodeFile"], key=lambda x: x["path"])

        files_data = []

        for i, code_file in enumerate(sorted_files):
            path = code_file["path"]
            content_str = self.read_file(PROJECT_ROOT / path)

            # Generate or retrieve summary (just the purpose string)
            purpose = self._generate_file_summary(path, content_str)

            # Build simple file entry - just path and purpose
            file_entry = {
                "path": path,
                "purpose": purpose
                if isinstance(purpose, str)
                else purpose.get("purpose", "Source code module"),
            }

            files_data.append(file_entry)

            # Progress logging
            if (i + 1) % 50 == 0:
                self.logger.log(f"  Processed {i + 1}/{len(sorted_files)} files...")

        # Save cache for incremental builds
        cache_file.write_text(json.dumps(self.summary_cache), encoding="utf-8")

        # Build JSON output
        output = {
            "generated": datetime.now().isoformat(),
            "total_files": len(sorted_files),
            "files": files_data,
        }

        # Write files.json
        FILES_SUMMARY_FILE.write_text(json.dumps(output, indent=2), encoding="utf-8")
        self.logger.log(
            f"Wrote files.json ({FILES_SUMMARY_FILE.stat().st_size / 1024:.1f}KB) with {len(sorted_files)} file summaries"
        )

    def _check_agents(self):
        """Check and update AGENTS.md integration - asks for approval before updating"""
        agents_section = """## Codebase Graph

The codebase graph is a typed knowledge graph of the entire source code.

**Location**: `./.codebase-graph/`

**Files**:
- `graph.json` - Complete typed knowledge graph
- `index.md` - Human-readable overview with statistics
- `file-tree.md` - Full processed file tree for easy searching
- `files.json` - Structured file summaries with purpose and dependencies
- `build.log` - Processing log with timestamps

**Purpose**:
- Understand code structure and dependencies
- Trace function calls and class hierarchies
- Navigate complex multi-file projects
- Query symbol relationships efficiently
- Search all processed files quickly using `file-tree.md`

**Usage**:
1. Reference the graph in prompts: "Query the codebase graph for all functions that call X"
2. Read the index: `./.codebase-graph/index.md` for human-readable overview
3. Search files: `./.codebase-graph/file-tree.md` for complete file listing
4. Query files: `./.codebase-graph/files.json` for structured summaries
5. Use the graph to understand dependencies before refactoring

**Commands**:
```bash
# Build/rebuild the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py build

# Query the graph
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py query --type Function --name "<name>"

# Show dependencies
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py deps --file "<path>"

# View statistics
python3 .opencode/skills/codebase-graph/scripts/codebase-graph.py stats
```

**Important**: The graph is rebuilt from scratch each time to ensure consistency and completeness.
"""

        if not AGENTS_FILE.exists():
            self.logger.log("[WARN] AGENTS.md not found")
            self.logger.log(
                "[INFO] Please create AGENTS.md or add the Codebase Graph section manually"
            )
            AGENTS_CHECK.write_text(
                json.dumps(
                    {
                        "checked_at": datetime.now().isoformat(),
                        "status": "not_found",
                        "action": "none",
                        "message": "AGENTS.md not found - please create it",
                    },
                    indent=2,
                ),
                encoding="utf-8",
            )
            return

        content = AGENTS_FILE.read_text(encoding="utf-8")

        if "## Codebase Graph" in content:
            self.logger.log("✅ AGENTS.md already contains Codebase Graph section")
            AGENTS_CHECK.write_text(
                json.dumps(
                    {
                        "checked_at": datetime.now().isoformat(),
                        "status": "present",
                        "action": "none",
                    },
                    indent=2,
                ),
                encoding="utf-8",
            )
            return

        # Section not found - ASK FOR APPROVAL
        self.logger.log("[WARN] Codebase Graph section NOT found in AGENTS.md")
        self.logger.log("")
        self.logger.log("[PROPOSAL] PROPOSED SECTION TO ADD:")
        self.logger.log("=" * 60)
        for line in agents_section.strip().split("\n"):
            self.logger.log(f"   {line}")
        self.logger.log("=" * 60)
        self.logger.log("")
        self.logger.log(
            "[ACTION] APPROVAL NEEDED: Would you like to add this section to AGENTS.md?"
        )
        self.logger.log("")
        self.logger.log("   To approve, run:")
        self.logger.log(
            "   python .opencode/skills/codebase-graph/scripts/codebase-graph.py approve-agents"
        )
        self.logger.log("")
        self.logger.log("   Or manually add the section above to AGENTS.md")

        # Save for approval
        AGENTS_CHECK.write_text(
            json.dumps(
                {
                    "checked_at": datetime.now().isoformat(),
                    "status": "missing",
                    "action": "needs_approval",
                    "section_to_add": agents_section.strip(),
                    "approval_command": "approve-agents",
                },
                indent=2,
            ),
            encoding="utf-8",
        )

        if not AGENTS_FILE.exists():
            self.logger.log("AGENTS.md not found, skipping integration")
            AGENTS_CHECK.write_text(
                json.dumps(
                    {
                        "checked_at": datetime.now().isoformat(),
                        "status": "not_found",
                        "action": "none",
                    }
                ),
                encoding="utf-8",
            )
            return

        content = AGENTS_FILE.read_text(encoding="utf-8")

        if "## Codebase Graph" in content:
            self.logger.log("AGENTS.md already contains Codebase Graph section")
            AGENTS_CHECK.write_text(
                json.dumps(
                    {
                        "checked_at": datetime.now().isoformat(),
                        "status": "present",
                        "action": "none",
                    }
                ),
                encoding="utf-8",
            )
            return

        # Section not found - need to add it
        self.logger.log("Codebase Graph section not found in AGENTS.md")

        # For now, just record that it needs to be added
        # The actual addition requires user approval which happens in the skill workflow
        AGENTS_CHECK.write_text(
            json.dumps(
                {
                    "checked_at": datetime.now().isoformat(),
                    "status": "missing",
                    "action": "needs_approval",
                    "section_to_add": agents_section.strip(),
                }
            ),
            encoding="utf-8",
        )

        self.logger.log("AGENTS.md update requires user approval (see AGENTS.md.check)")

    def query(self, entity_type: str, **filters):
        """Query entities by type and filters"""
        if entity_type not in self.entities:
            print(f"Unknown entity type: {entity_type}")
            return []

        results = []
        for entity in self.entities[entity_type]:
            match = True
            for key, value in filters.items():
                if key not in entity or entity[key] != value:
                    match = False
                    break
            if match:
                results.append(entity)

        print(f"Found {len(results)} {entity_type} entities")
        for r in results[:20]:  # Show first 20
            print(json.dumps(r, indent=2))

        return results

    def deps(self, file_path: str = None, function_name: str = None):
        """Show dependencies for a file or function"""
        if file_path:
            # Find the file
            code_file = next(
                (
                    f
                    for f in self.entities["CodeFile"]
                    if f["id"] == file_path or f["path"] == file_path
                ),
                None,
            )
            if not code_file:
                print(f"File not found: {file_path}")
                return

            print(f"\n### Dependencies for `{file_path}`\n")

            # Show imports
            print("**Imports:**")
            for imp in code_file.get("imports", []):
                print(f"  - `{imp}`")

            # Show what imports this file
            importers = [
                r["from"]
                for r in self.relations
                if r["type"] == "imports" and r["to"] == file_path
            ]
            if importers:
                print("\n**Imported by:**")
                for imp in importers:
                    print(f"  - `{imp}`")

        if function_name:
            # Find the function
            func = next(
                (f for f in self.entities["Function"] if f["name"] == function_name),
                None,
            )
            if not func:
                print(f"Function not found: {function_name}")
                return

            print(f"\n### Dependencies for function `{function_name}`\n")
            print(f"**Location:** `{func['file']}#{func['line_start']}`")

            # Show calls
            calls = [
                r["to"]
                for r in self.relations
                if r["type"] == "calls" and r["from"] == func["id"]
            ]
            if calls:
                print("\n**Calls:**")
                for call in calls:
                    print(f"  - `{call}`")

            # Show called by
            called_by = [
                r["from"]
                for r in self.relations
                if r["type"] == "calls" and r["to"] == func["id"]
            ]
            if called_by:
                print("\n**Called by:**")
                for caller in called_by:
                    print(f"  - `{caller}`")

    def validate(self):
        """Validate graph integrity"""
        violations = self._validate_graph()

        if not violations:
            print("✓ Graph validation passed - no constraint violations")
        else:
            print(f"✗ Found {len(violations)} constraint violations:")
            for v in violations:
                print(f"  - {v}")

        return len(violations) == 0

    def stats(self):
        """Display graph statistics"""
        if not GRAPH_FILE.exists():
            print("Graph not found. Run 'build' first.")
            return

        with open(GRAPH_FILE) as f:
            graph = json.load(f)

        print("\n### Codebase Graph Statistics\n")
        print(f"**Generated:** {graph['metadata']['generated_at']}")
        print(f"**Root:** {graph['metadata']['root_directory']}")
        print("\n**Entities:**")
        for entity_type, count in {
            k: len(v) for k, v in graph["entities"].items()
        }.items():
            print(f"  - {entity_type}: {count}")
        print(f"\n**Relations:** {len(graph['relations'])}")
        print(f"\n**File Size:** {GRAPH_FILE.stat().st_size / 1024:.1f}KB")


def main():
    """Main entry point"""
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    command = sys.argv[1]

    # Ensure graph directory exists
    GRAPH_DIR.mkdir(parents=True, exist_ok=True)

    # Initialize logger
    logger = Logger(BUILD_LOG)
    graph = CodebaseGraph(logger)

    if command == "build":
        graph.build_graph()
        logger.save()

    elif command == "query":
        # Load existing graph
        if not GRAPH_FILE.exists():
            print("Graph not found. Run 'build' first.")
            sys.exit(1)

        with open(GRAPH_FILE) as f:
            data = json.load(f)
            graph.entities = data["entities"]
            graph.relations = data["relations"]

        # Parse query args
        entity_type = "Function"
        filters = {}
        for i in range(2, len(sys.argv), 2):
            if sys.argv[i] == "--type":
                entity_type = sys.argv[i + 1]
            elif sys.argv[i] == "--name":
                filters["name"] = sys.argv[i + 1]
            elif sys.argv[i] == "--file":
                filters["file"] = sys.argv[i + 1]

        graph.query(entity_type, **filters)

    elif command == "deps":
        # Load existing graph
        if not GRAPH_FILE.exists():
            print("Graph not found. Run 'build' first.")
            sys.exit(1)

        with open(GRAPH_FILE) as f:
            data = json.load(f)
            graph.entities = data["entities"]
            graph.relations = data["relations"]

        file_path = None
        function_name = None
        for i in range(2, len(sys.argv), 2):
            if sys.argv[i] == "--file":
                file_path = sys.argv[i + 1]
            elif sys.argv[i] == "--function":
                function_name = sys.argv[i + 1]

        graph.deps(file_path=file_path, function_name=function_name)

    elif command == "validate":
        if not GRAPH_FILE.exists():
            print("Graph not found. Run 'build' first.")
            sys.exit(1)

        with open(GRAPH_FILE) as f:
            data = json.load(f)
            graph.entities = data["entities"]
            graph.relations = data["relations"]

        success = graph.validate()
        sys.exit(0 if success else 1)

    elif command == "stats":
        graph.stats()

    elif command == "update-agents":
        graph._check_agents()

    elif command == "approve-agents":
        # Approve and add the section to AGENTS.md
        if not AGENTS_CHECK.exists():
            print("No pending AGENTS.md update found. Run 'build' first.")
            sys.exit(1)

        check_data = json.loads(AGENTS_CHECK.read_text(encoding="utf-8"))
        if check_data.get("status") != "missing":
            print("No approval needed. AGENTS.md is already up to date.")
            sys.exit(0)

        section = check_data.get("section_to_add")
        if not section:
            print("No section to add found in AGENTS.md.check")
            sys.exit(1)

        # Read current AGENTS.md
        content = AGENTS_FILE.read_text(encoding="utf-8")

        # Add section before the last line (before the final tagline)
        lines = content.rstrip().split("\n")
        # Find a good insertion point - before the last section or at the end
        insert_idx = len(lines)
        for i, line in enumerate(lines):
            if line.startswith("---") and i > len(lines) // 2:
                insert_idx = i
                break

        # Insert the new section
        lines.insert(insert_idx, "")
        lines.insert(insert_idx + 1, section)
        lines.insert(insert_idx + 2, "")

        # Write updated AGENTS.md
        AGENTS_FILE.write_text("\n".join(lines) + "\n", encoding="utf-8")

        # Update check file
        AGENTS_CHECK.write_text(
            json.dumps(
                {
                    "checked_at": datetime.now().isoformat(),
                    "status": "approved_and_added",
                    "action": "completed",
                },
                indent=2,
            ),
            encoding="utf-8",
        )

        print("[OK] Codebase Graph section added to AGENTS.md")

    else:
        print(f"Unknown command: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
