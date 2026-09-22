---
title: Add a reusable measure or dimension
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
