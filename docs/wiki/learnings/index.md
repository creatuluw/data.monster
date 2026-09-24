# Learnings

- [dm live-reload events don't replay — e2e must navigate first, then write the file](./dm-live-reload-events-don-t-replay-e2e.md) - Hit while e2e-testing the dm:error pipeline (workspace-file-first pass, 2026-09-22): dropping a broken-JSON file into `dm/pages/` **before** navigating to /page
- [18 root components duplicate the ds/ showcase set — slug resolution prefers the ds/ twin](./ds-twins-win-slug-collisions.md) - Discovered 2026-09-23 fixing `/components/Accordion` showing no visual (commit `f19dd80`).
- [Svelte 5 snippets can't ride a name-keyed registry — snippet demos are inline markup; BarChart takes accessors](./svelte-5-snippets-can-t-ride-a-name-keyed-registry-snippet-d.md) - Discovered 2026-09-23 building `ComponentDemo.svelte` (the /components visual-demo registry):
- [loadAppComponents dedupes by folder preference — 18 root twins are dead code](./loadappcomponents-dedupes-by-folder-preference-18-root-twins.md) - Discovered 2026-09-23 while removing the "Also at:" dup note from /components detail pages (commit `e8e329a`).
