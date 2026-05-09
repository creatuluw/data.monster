# PRD Interview Transcript — Reporting & Dashboard Pages

**Date:** 2026-05-09
**Interviewer:** AI PRD Conductor
**Initiative:** Reporting & Dashboard Pages for Data Monster

---

## Phase 1: Opening

**Q: What is the initiative or capability you want to define?**
A: Ability for data analysts and business users to create reporting/dashboard pages using data, visualization, control, search, filter, analysis, and LLM components. Creating, sharing, and distributing insights from the data models in Data Monster.

**Q: What project or product does this belong to? What is the tech stack?**
A: Data Monster — analyzed from codebase:
- Desktop app: Tauri v2
- Frontend: SvelteKit + Svelte 5 runes + TypeScript + Tailwind CSS 4
- Database: DuckDB 1.1 (embedded)
- Backend: Rust
- Charting: LayerChart v2.0.0-next.62, custom canvas bar charts
- Icons: Lucide

**Existing Pages feature:** `/pages` auto-generates bar charts from the first table with click-to-filter and paginated table view. Zero configuration, zero user control.

---

## Phase 2: Discovery

### Problem & Motivation

**Q: Who are the users who need this today, and what are they doing now?**
A: Don't know what they use today, but this tool targets analytics use cases for data analysts.

**Q: Why is this important to solve now?**
A: Content and insight creation from data is currently missing. The existing querying features are for engineers and developers. There's no way to turn results into shareable insights.

### Users & Personas

**Q: Are there different types of users who would use this differently?**
A: This needs to address users who don't want to use complex patterns to get what they want. Non-technical users who need a simple, guided experience — no SQL required.

**Q: What does that user's ideal workflow look like after this feature ships?**
A: They create a new page, select a component/block from the library (to which anyone can add reusable components). In that component they define patterns, attributes, and data needed for that specific insight. They can add multiple components to the page. When ready, they share with others by printing a PDF as the first sharing method. They can use these pages for data analysis and showing content in sessions with decision makers. The most important thing is a broad range of options to present, control, and share data in ways that make insights actionable.

### Scope & Boundaries

**Q: What is the absolute MVP?**
A:
1. Reusable bar chart, treemap, table, line chart, and scatter chart
2. A filter component
3. A search component that shows anything from components, pages, data records based on terms and scope
4. A business professional designed PDF report
5. A way to create a bookmark/snapshot of a specific view or slice of data

**Q: What is explicitly out of scope?**
A: Authentication, emailing reports, embedding dashboards externally, real-time data streaming, dashboard templates/duplication. Focus only on the five MVP items listed above.

### Success Criteria

**Q: How will we know this succeeded?**
A: A user can create a page and start adding components from the library within 1 minute and be up and running quickly. It should be very easy and guided to add components/blocks to the page and populate them with data/content/controls to get rich analysis content.

**Q: What would make a user say "this is broken"?**
A: When it takes a lot of time to get components working with data and creating insights quickly.

### High-Level Capabilities

**Q: How do users wire components to data?**
A: They pick fields from the total list of fields. A split-screen editor opens and the user defines the list of fields to be shown. Behind every field they can see which table it comes from and sample example values to guide their instincts.

**Q: How does the filter component work?**
A: It filters all components, except components that don't subscribe to the filter — that's an opt-out setting the user needs to make on purpose.

**Q: How does the search component work?**
A: It searches within the page's scope — the content, components, data used in a page. Shows a ranked hitlist of things closest to the keyword match.

**Q: What level of control does the user have over PDF output?**
A: It shows the page as-is, but has exceptionally good design for reading, alignment, breaking, sections, and grouping. WYSIWYG with professional print typography.

**Q: How do bookmarks work?**
A: Bookmarks are on a global level. A user can open a bookmark and be redirected to the page with that specific combination of props to show exactly what it was when it was bookmarked.

### Technical Approach

**Q: Charting library preference?**
A: Use LayerChart. We're currently prototyping reusable charts. We want pages to have a detailed configurable grid spacing with component snapping. Need to find a good lib for that.

**Grid layout research findings:**
Conducted web research across 4 subagents. Evaluated:
- Svelte-native options: svelte-grid (abandoned), svelte-grid-extended (Svelte 4, low maintenance)
- Framework-agnostic: GridStack.js (active, v12.6.0, 8.9k stars, drag+resize+snap+serialize)
- react-grid-layout/core algorithms (framework-agnostic layout engine)
- DnD libraries: @neodrag/svelte, svelte-dnd-action, @thisux/sveltednd

**Decision:** GridStack.js — user chose after reviewing URL.

**Q: How should pages be stored and persisted?**
A: Both — a new folder for definitions in the app workspace (portable, importable, future version control) and internal DB metadata for fast lookups.

**Q: Should each component have a standard interface/contract?**
A: Yes, contracts for all components and logic in the app in the future as well.

**Q: Performance considerations for multiple components?**
A: We should be able to calculate what hardware specs are needed for the app/workspace at runtime so users can decide what to do with that info.

### Risks & Open Questions

**Q: What are the biggest risks?**
A: Without content creation and sharing, this app doesn't have too much value. This is the value-defining feature.

### Phasing & Rollout

**Q: Should MVP items ship in phases, and what ordering?**
A: The ordering listed is the phasing:
1. Charts & table components
2. Filter component
3. Search component
4. PDF export
5. Bookmarks

---

## Phase 3: Spec Candidates

Identified 8 spec candidates:

| # | Spec | Priority |
|---|------|----------|
| 1 | Page Builder Core | Must-have |
| 2 | Component Contract & Registry | Must-have |
| 3 | Chart & Table Components | Must-have |
| 4 | Page Filter Component | Should-have |
| 5 | Page Search Component | Should-have |
| 6 | PDF Export | Should-have |
| 7 | Bookmarks | Nice-to-have |
| 8 | Hardware Profiling | Nice-to-have |

User confirmed: no additions, removals, or reprioritizations needed.

---

## Phase 4: PRD Generated

PRD written to `.prds/reporting-dashboard-pages/prd.md`.
Transcript written to `.prds/reporting-dashboard-pages/transcript.md`.
