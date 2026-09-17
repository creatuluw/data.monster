---
type: Learning
title: wiki_note_page wikilinks resolve ./-relative to the page's own folder — cross-folder links need explicit paths
description: "Discovered 2026-09-16 while writing the [[feature-skill-catalog-docs-features]] artifact page."
tags: [wiki, okf, wikilinks]
timestamp: "2026-09-16T13:25:25.720Z"
---

# wiki_note_page wikilinks resolve ./-relative to the page's own folder — cross-folder links need explicit paths

Discovered 2026-09-16 while writing the [[feature-skill-catalog-docs-features]] artifact page.

## Symptom

`wiki_validate` flagged W4 "links to missing ./remote-chat-command.md" for a page in `pages/artifacts/` that wikilinked an entity in `pages/entities/`.

## Root cause

`wiki_note_page` converts `[[slug]]` wikilinks into **`./`-relative markdown links resolved from the new page's own folder**. Cross-folder links (artifact → entity, entity → concept) therefore break: `./remote-chat-command.md` doesn't exist under `pages/artifacts/`.

## Rule

For cross-folder page links, skip `[[slug]]` and write the explicit relative path as a plain markdown link from the start:

- from `pages/artifacts/x.md` → a plain markdown link whose target is `../entities/barchart-component.md` — relative to the **page's own folder**, so it resolves to `pages/entities/barchart-component.md`
- to a core concept (e.g. from `pages/artifacts/`) → target `../../overview.md`

Also note: patching the file with the original `[[...]]` source text won't match — the stored file already contains the converted markdown-link form with a `./slug.md` target; patch that.
