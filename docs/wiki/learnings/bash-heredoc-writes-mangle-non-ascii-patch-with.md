---
type: Learning
title: Bash heredoc writes mangle non-ASCII — patch with python explicit escapes, and verify bytes before assuming corruption
description: "Hit twice while rewiring the drawers (PR #18, 2026-09-17)."
tags: [bash, encoding, workflow, gotcha]
timestamp: "2026-09-17T17:25:31.321Z"
---

# Bash heredoc writes mangle non-ASCII — patch with python explicit escapes, and verify bytes before assuming corruption

Hit twice while rewiring the drawers (PR #18, 2026-09-17).

## Symptom 1 — heredoc writes corrupt

Writing large Svelte files via a bash heredoc left literal `U+FFFD` replacement characters where en-dashes should be (`–` → `�`).

## Symptom 2 — terminal render lies

Later, a file that looked like it contained `U+FFFD` actually contained a **proper em-dash** — the terminal just can't render it. "Fixing" it would have corrupted a good file.

## Rules

- When a file's content contains non-ASCII (en/em-dashes, arrows), don't write it through an inline heredoc — write the script to a file first, or use python.
- To patch existing mojibake, use python with explicit escapes (`\ufffd`) so the match can't be mangled by the shell.
- Before "fixing" a suspected bad character, check the actual bytes (e.g. python `open(..., encoding='utf-8').read()` + `ord()`) — terminal rendering is not evidence of corruption.
