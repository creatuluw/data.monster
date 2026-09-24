---
title: Audit & clean up my workspace
goal: actions
description: Find broken refs, duplicate content, stale files - propose before deleting.
tags: maintenance, interview
---
My data.monster workspace folder is: `<PASTE WORKSPACE FOLDER PATH>`

Read `README.md`, then `dm/docs/INDEX.md` + `dm/docs/formats/` as needed.

Task: audit my content. Check every `dm/pages/*.json` validates (spans 1-12, refs
resolve to existing `dm/master-items/` files, tables exist), find duplicate measures,
and list saved queries whose SQL references missing tables. Report findings as a table
with file paths. PROPOSE fixes and wait for my approval before deleting or rewriting
anything - one confirmation question at a time.


## Before you start - the only way to fill gaps
Before doing ANY work: list every argument, variable, or choice this task needs
(names, paths, columns, formats, options, audience, ...). For each one I have not
already given you, ask me ONE QUESTION AT A TIME, each with lettered options
(a/b/c/d + "other") when a choice makes sense. Never assume, never pick silent
defaults, never start with an unknown value. Interviewing me is the ONLY way to
fill missing variables - begin working only after every one is confirmed by me.
