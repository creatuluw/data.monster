# Learnings

- [dm live-reload events don't replay — e2e must navigate first, then write the file](./dm-live-reload-events-don-t-replay-e2e.md) - Hit while e2e-testing the dm:error pipeline (workspace-file-first pass, 2026-09-22): dropping a broken-JSON file into `dm/pages/` **before** navigating to /page
