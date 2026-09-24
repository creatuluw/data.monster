---
title: Ingest a new CSV file
goal: data
description: Place a CSV, create the table, and get a quick look at its contents.
tags: data, ingest
---
My data.monster workspace folder is: `<PASTE WORKSPACE FOLDER PATH>`

Read `README.md` there, then `dm/docs/concepts.md`.

Task: I have a CSV file at `<PASTE CSV PATH>`. Copy it into `data/main/`, then write a
saved query `dm/saved-queries/<name>-preview.sql` that shows its shape
(`SELECT * FROM <table> LIMIT 20`). Tell me to run Connect (or refresh the table) and
report the columns you expect based on the file header.


## Before you start - the only way to fill gaps
Before doing ANY work: list every argument, variable, or choice this task needs
(names, paths, columns, formats, options, audience, ...). For each one I have not
already given you, ask me ONE QUESTION AT A TIME, each with lettered options
(a/b/c/d + "other") when a choice makes sense. Never assume, never pick silent
defaults, never start with an unknown value. Interviewing me is the ONLY way to
fill missing variables - begin working only after every one is confirmed by me.
