---
type: Learning
title: "Couldn't find callback id" Tauri warning is a benign reload artifact
description: "`[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is benign. It appears when "
tags: [tauri, debugging]
timestamp: "2026-09-11T21:48:43.081Z"
---

# "Couldn't find callback id" Tauri warning is a benign reload artifact

`[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is benign. It appears when the webview reloads (dev HMR or manual) while an async Rust operation still holds a callback — e.g. after a failed/blocked call that gets interrupted. Seen during the Analyst CORS incident: the warning was a side effect of the failed request + reload, not a bug itself, and disappeared once the root cause was fixed. Don't chase the callback id; look for the underlying failed async operation.
