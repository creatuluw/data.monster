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

## Symptom 3 - backslashes eaten through heredoc chains (2026-09-22, files-009)

Editing a tab-indented Svelte file via bash replace chains: anchors written with `	` never matched (the file uses tab indentation), and `\t` sent through the heredoc chain arrived as a *real* tab - a backslash gets eaten somewhere in the chain.

## Rules

- When a file's content contains non-ASCII (en/em-dashes, arrows), don't write it through an inline heredoc — write the script to a file first, or use python.
- To patch existing mojibake, use python with explicit escapes (`\ufffd`) so the match can't be mangled by the shell.
- Patching tab-indented files? Tabs never survive typing into a heredoc - match with explicit `	` escapes and assert each replacement's count (expect 1) so a silent miss cannot pass.
- Never send literal backslash escapes through a heredoc chain (`\t` can arrive as a real tab). Build the replacement from placeholders in the script itself (e.g. `TAB = chr(9)`) so the shell never sees a backslash.
- Before "fixing" a suspected bad character, check the actual bytes (e.g. python `open(..., encoding='utf-8').read()` + `ord()`) — terminal rendering is not evidence of corruption.
