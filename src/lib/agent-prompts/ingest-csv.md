---
title: Ingest a new CSV file
description: Place a CSV, create the table, and get a quick look at its contents.
tags: data, ingest
---
My data.monster workspace folder is: `<PASTE WORKSPACE FOLDER PATH>`

Read `README.md` there, then `dm/docs/concepts.md`.

Task: I have a CSV file at `<PASTE CSV PATH>`. Copy it into `data/main/`, then write a
saved query `dm/saved-queries/<name>-preview.sql` that shows its shape
(`SELECT * FROM <table> LIMIT 20`). Tell me to run Connect (or refresh the table) and
report the columns you expect based on the file header.
