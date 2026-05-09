# PRD Interview Transcript — Table Relationships

## Participants
- Interviewer: PRD Interview Agent
- Interviewee: User (Data Monster developer/owner)

## Transcript

### Phase 1: Opening

**Interviewer**: Hi! I'm here to conduct a PRD interview for a new feature. My goal is to produce a PRD document that captures the initiative at a level above implementation specs — so that a spec-writer can later extract detailed specs and tasks from it.

From what I can see, your app is a Svelte 5 + SvelteKit + Tauri application called "Data Monster" with a `/data` page that shows a table overview (using `TableOverview` and `TableDrawer` components), backed by what appears to be a local database.

Let's start with the basics: What is the initiative or capability you want to define?

**User**: i want a feature in /data that will help me to define relationships between tables

### Phase 2: Discovery

#### Problem & Motivation

**Interviewer**: Right now, how do you cope without this feature? When you need to think about how tables relate to each other, what do you do today?

**User**: without it the app cant work

**Interviewer**: When you say the app can't work without relationships — what specifically breaks? Is this for:
1. The AI analyst (so it knows how to JOIN tables when answering questions)?
2. Query building (so users can compose queries across tables)?
3. Something else entirely?

**User**: query building but also for cross table operations where field combinations need to benefit from the links between tables to do cross table calculations, selections and create dynamic/linking logic within re-usable and chart components that can than function knowing how the dynamic queries can surface results and data outcomes

#### Users & Personas

**Interviewer**: Who will be defining these relationships? Is it you (the builder/developer), or will end-users also create and manage relationships?

**User**: all users should be able to do this

#### Scope & Boundaries

**Interviewer**: Let me understand the core of this. What is the minimum that must work for this feature to be useful? For example:
- Is it enough to define simple one-to-many relationships between two tables based on matching column names?
- Or do you need many-to-many, composite keys, custom join conditions?
- Should the system auto-detect potential relationships based on matching column names/values?

**User**: relationships in whatever form need to be stored into the metadata of the tables for the rest of the app to use, two ways of connecting, creating same fieldnames in the query editor while using sql, and a grid in the relationships tab in /data that lets users drag tables from the list into the grid and there they can connect tables based on fieldnames that connect

**Interviewer**: For the relationships grid:
- When you drag two tables into the grid and connect them by a field, is that always a simple equality join (e.g., `tableA.id = tableB.user_id`)?
- Can a field connect to multiple other fields (one-to-many, many-to-many)?
- Should the system suggest potential connections automatically when it finds matching column names?

**User**: 1. yes 2. yes, but we need to think about circular references and loops we dont want 3. it can do that with ingested tables, yes

**Interviewer**: when crosstable field additions to components work and put data side by side in a table or when these links are stored and the system can calculate a dense relationship comparison between both fields

#### Success Criteria

**Interviewer**: What would make you say "this is great"? Is it when a chart component automatically knows how to JOIN two tables without the user writing SQL?

**User**: yes

**Interviewer**: What would make you say "this is broken"? Is it when relationships create incorrect data, or when the UI is too complex?

**User**: both, but incorrect data is unacceptable, reconciliation functions need to be present to spot these errors when they do occur, but we need preventive logic to combat this

**Interviewer**: Are there performance requirements? If you have large tables (millions of rows), should relationship calculations still be fast?

**User**: maybe we can use indexing, or use pagination in low level or granular data components, by default, but be able to opt out

#### High-Level Capabilities

**Interviewer**: Let me map out the major capabilities. Based on what you've told me, I think we have:
1. Relationship Metadata Storage
2. Visual Relationship Grid (the Relationships tab)
3. Query Editor Implicit Relationships
4. Auto-Detection of Relationships
5. Cross-Table Component Fields
6. Relationship Comparison/Density
7. Circular Reference Prevention
8. Reconciliation Functions

Does this list capture everything?

**User**: this is correct

#### Technical Approach

**Interviewer**: For relationship storage:
- Should relationships live in the same metadata tables that already store labels, tags, groups, and table types?
- Or do you want a separate `relationships` table in the DuckDB database?
- For the visual grid — do you have a preferred library (like a canvas-based approach), or should it be built with SVG/DOM elements?

**User**: a separate one - visual grid: use https://svelteflow.dev/

**Interviewer**: What does the relationship record look like? Should relationships be named/labeled? For the reconciliation/density calculation — should that be computed on-demand, or cached and refreshed?

**User**: for all do what is common and best practice

#### Risks & Open Questions

**Interviewer**: What do you see as the biggest risks? Are there any dependencies on other teams, systems, or decisions that could block this?

**User**: skip for now

#### Phasing & Rollout

**Interviewer**: Should this be delivered all at once, or in phases? If phased, what's the MVP?

**User**: all at once

### Phase 3: Spec Candidate Identification

**Interviewer**: Which of these capabilities should each become their own spec document?

**User**: you decide what is the best grouping based on dependencies, but to me there is a split between backend and frontend

**Interviewer**: Based on dependencies, here's my recommended grouping:

**Backend specs:**
1. Spec: Relationship Metadata Storage — Schema, CRUD operations, relationship types, persistence layer
2. Spec: Relationship Engine — Circular reference prevention, relationship density/comparison calculation, reconciliation functions, auto-detection

**Frontend specs:**
3. Spec: Visual Relationship Grid — Svelte Flow canvas, drag-and-drop table connections
4. Spec: Cross-Table Component Integration — Components pulling fields from related tables, query editor implicit relationships

Does this grouping work for you?

**User**: yes

### Phase 4: PRD Generation

PRD written to `.prds/table-relationships/prd.md`
Transcript written to `.prds/table-relationships/transcript.md`
