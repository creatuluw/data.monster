---
type: Learning
title: /data tab keys ≠ labels — "Metadata" writes ?tab=definitions
description: "Discovered 2026-09-22 while CDP-verifying the /data tab URL sync (port 9223): a probe matching tabs by `textContent.includes('Definitions')` never matched — the"
tags: [cdp, e2e, data-page, tabs, probes]
timestamp: "2026-09-22T08:11:07.571Z"
---

# /data tab keys ≠ labels — "Metadata" writes ?tab=definitions

Discovered 2026-09-22 while CDP-verifying the /data tab URL sync (port 9223): a probe matching tabs by `textContent.includes('Definitions')` never matched — the tab labeled **Metadata** writes `?tab=definitions`. The URL key is a legacy/storage key, not the display label.

Consequences:

- When hand-testing deep links, a URL like `/data?tab=definitions` opening "Metadata" is **correct**, not a bug.
- When writing CDP/e2e probes against tabs, select by `href` (`?tab=…`) or the tab's key, never by label text — labels are user-facing and can drift from keys.

Same session also hit the sibling trap: a nav probe matched the "Data Monster" brand link (`href='/'`) instead of the real nav item — scope link probes by unique href, not just text. (Same family as [[cdp-form-probes-must-be-container-scoped-shared]].)
