---
title: Build me a dashboard (interview first)
goal: content
description: The agent interviews you about goals and audience, then designs and builds the page.
tags: dashboard, interview
---
My data.monster workspace folder is: `<PASTE WORKSPACE FOLDER PATH>`

Read `README.md` there first, then `dm/docs/INDEX.md` and only the docs your task needs.

Task: build me a report page. INTERVIEW ME FIRST: ask questions ONE AT A TIME with
lettered multiple-choice options (a/b/c/d) until you know - the table and columns to
use, the 1-3 questions the page should answer, and my preferred chart types. Then:

1. Create reusable measures/dimensions per `dm/docs/formats/master-items.md`.
2. Create `dm/pages/<slug>.json` per `dm/docs/formats/page-doc.md` - title block,
   one chart per question, a table for detail.
3. Summarize what you built and how to tweak it (file path + which fields to change).


## Before you start - the only way to fill gaps
Before doing ANY work: list every argument, variable, or choice this task needs
(names, paths, columns, formats, options, audience, ...). For each one I have not
already given you, ask me ONE QUESTION AT A TIME, each with lettered options
(a/b/c/d + "other") when a choice makes sense. Never assume, never pick silent
defaults, never start with an unknown value. Interviewing me is the ONLY way to
fill missing variables - begin working only after every one is confirmed by me.
