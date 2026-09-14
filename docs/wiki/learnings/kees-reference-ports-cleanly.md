---
type: Learning
title: kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4
description: "`E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs."
tags: [charts, porting, reference, svelteplot]
timestamp: "2026-09-14T07:26:30.635Z"
---

# kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4

`E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs.

It shares **svelteplot 0.14.2** and **Tailwind 4** with data.monster, so components port nearly verbatim. Verified 2026-09-14 with the heatmap: the only change needed was swapping the header font to the app's `--font-display`. Check `package.json` version parity in both projects before porting anything new.
