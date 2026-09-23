---
title: Add a reusable measure or dimension
goal: insights
description: Create a named, table-bound expression every chart can reference.
tags: master-items
---
My data.monster workspace folder is: `<PASTE WORKSPACE FOLDER PATH>`

Read `README.md` there, then `dm/docs/formats/master-items.md`.

Task: create a reusable measure or dimension for me. Ask what it should compute (one
question at a time if unclear), then create `dm/master-items/measures/<id>.json` (or
`dimensions/`) following the format exactly - filename = id, table bound, raw DuckDB
expression. Show me the file and an example `{"ref": "<id>"}` snippet I can use in any
chart.


## Before you start - the only way to fill gaps
Before doing ANY work: list every argument, variable, or choice this task needs
(names, paths, columns, formats, options, audience, ...). For each one I have not
already given you, ask me ONE QUESTION AT A TIME, each with lettered options
(a/b/c/d + "other") when a choice makes sense. Never assume, never pick silent
defaults, never start with an unknown value. Interviewing me is the ONLY way to
fill missing variables - begin working only after every one is confirmed by me.
