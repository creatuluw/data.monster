# Table Relationships

## Feature Overview

Table relationships are the connective tissue of the Data Monster application. They enable cross-table queries, calculations, and dynamic linking logic that powers reusable chart components and data views. Without relationships, the app cannot function — components cannot automatically JOIN tables, cross-table calculations are impossible, and the AI analyst lacks the structural knowledge to compose multi-table queries.

This feature introduces two ways to define relationships: a visual drag-and-drop grid in the `/data` page (built with Svelte Flow) where users connect tables by matching field names, and implicit relationship detection in the query editor where creating same field names across SQL queries establishes connections. All relationships are persisted into a dedicated metadata table that the rest of the app consumes.

The system also provides auto-detection for ingested tables, relationship density comparison to verify connection quality, circular reference prevention to block loops, and reconciliation functions to spot errors when they occur. Data correctness is non-negotiable — preventive logic must combat incorrect relationships before they corrupt downstream results.

## Problem Statement

Without table relationships, Data Monster operates on isolated tables. Users cannot compose queries that span multiple tables, chart components cannot automatically JOIN related data, and cross-table calculations are impossible. The AI analyst cannot reason about how tables connect. Every query must be written manually with explicit JOINs, and there is no system-level understanding of the data model.

This affects all users — from technical builders writing SQL to non-technical users who need to drag-and-drop fields from related tables into components. The problem is foundational: without relationships, the app is a collection of disconnected data silos rather than a unified data platform.

## Success Criteria

- Users can define relationships between tables via visual grid or query editor, and those relationships are persisted and available to all app features
- Chart and data components can pull fields from related tables and display data side-by-side correctly
- The system calculates and displays relationship density (match quality) between connected fields
- Zero silent data corruption — incorrect relationships are prevented or flagged with reconciliation reports
- Circular references are detected and blocked before they can cause infinite loops
- Relationship calculations are performant — indexing and pagination handle large tables by default, with opt-out

## Design Goals

### Primary (Must)

- Relationships stored in a dedicated DuckDB table, accessible by all app features
- Visual relationship grid using Svelte Flow — drag tables from list, connect by field names
- Query editor implicit relationships — same field names across SQL queries define connections
- Circular reference prevention — detect and block loops in the relationship graph
- Relationship density calculation — show how well connected fields actually match
- Reconciliation functions — spot and report relationship errors
- Data correctness is non-negotiable — preventive logic + error detection

### Secondary (Nice to Have)

- Auto-detection of potential relationships for ingested tables based on matching column names
- Named/labeled relationships for user reference
- Performance optimization via indexing and pagination with opt-out

## User Personas & Experience

### Personas

- **All Users**: Anyone using Data Monster needs to define and consume table relationships. This includes technical users writing SQL queries and non-technical users building visual components.

### User Experience

Before this feature, users work with isolated tables. They must manually write JOINs for every cross-table query, and components only show data from a single table.

After this feature ships, users can:
1. Open the `/data` page and navigate to the Relationships tab
2. Drag tables from the table list into the Svelte Flow grid
3. Connect tables by matching field names (e.g., `orders.user_id` → `users.id`)
4. View relationship density to verify connection quality
5. Use the query editor where creating same field names implicitly defines relationships
6. Drop fields from related tables into chart components, and the system automatically resolves the JOIN
7. Run reconciliation checks to verify relationship correctness

## Scope & Boundaries

### In Scope

- Dedicated `relationships` table in DuckDB for storing relationship metadata
- Visual relationship grid (Svelte Flow) on the `/data` Relationships tab
- Query editor implicit relationship detection via matching field names
- Auto-detection of relationships for ingested tables
- Cross-table field support in components
- Relationship density/comparison calculation
- Circular reference prevention
- Reconciliation functions for error detection
- Indexing and pagination for performance

### Out of Scope

- Custom join conditions beyond equality joins
- Complex relationship types beyond one-to-one, one-to-many, many-to-many
- Relationship versioning or history
- Relationship sharing across workspaces

## High-Level Capabilities

### Relationship Metadata Storage

A dedicated `relationships` table in DuckDB stores all relationship definitions. Each record captures the source table, source column, target table, target column, relationship type (one-to-one, one-to-many, many-to-many), and metadata such as creation timestamp and optional label. This table is the single source of truth that all other features consume. CRUD operations must be atomic and validated — no relationship can be created that would introduce a circular reference. The storage layer must support efficient querying by table name and column name for fast relationship lookups.

### Relationship Engine

The engine operates on stored relationships to provide: circular reference detection (traversing the relationship graph to prevent loops before they can be created), relationship density calculation (computing what percentage of rows in connected columns have matching values), reconciliation functions (scanning relationships and reporting mismatches, orphaned references, and data quality issues), and auto-detection (analyzing newly ingested tables for column name matches against existing tables and suggesting potential relationships). All calculations must be correct — incorrect results are worse than no results. The engine uses DuckDB's native query capabilities for efficient computation.

### Visual Relationship Grid

Built with Svelte Flow, the Relationships tab on `/data` provides a canvas where users drag tables from the table list and connect them by matching field names. The grid shows tables as nodes with their columns as connection points. Users draw edges between matching columns to define relationships. The grid visualizes relationship type, direction, and density. Circular reference attempts are blocked with clear error messages. The grid reads from and writes to the relationship metadata store.

### Cross-Table Component Integration

Components (charts, tables, data views) can pull fields from related tables. When a user drops a field from Table B into a component showing Table A, the system resolves the JOIN path using stored relationships. The query editor also supports implicit relationships — when the same field name appears across multiple SQL queries, the system recognizes this as a potential relationship. Data surfaces side-by-side correctly, with performance managed through indexing and pagination by default (opt-out available).

## Spec Candidates

The following capabilities should each become their own spec document. A spec-writer should create one spec per item below.

### Spec: Relationship Metadata Storage

- **Description**: Database schema for the `relationships` table, CRUD operations, validation rules, relationship types (one-to-one, one-to-many, many-to-many), metadata fields (labels, timestamps), and integration with existing metadata operations. Must support efficient lookups by table and column name.
- **Dependencies**: None — this is the foundation
- **Priority**: Must-have

### Spec: Relationship Engine

- **Description**: Circular reference detection algorithm (graph traversal), relationship density calculation (match percentage between connected columns), reconciliation functions (error scanning and reporting), auto-detection logic for ingested tables (column name matching), and performance optimization (indexing, caching). All logic operates on the stored relationship metadata.
- **Dependencies**: Relationship Metadata Storage
- **Priority**: Must-have

### Spec: Visual Relationship Grid

- **Description**: Svelte Flow canvas implementation on the `/data` Relationships tab. Table drag-and-drop from sidebar, column-level connection points, edge creation and visualization, relationship type selection, density display, circular reference blocking with error UI, and read/write integration with the relationship metadata store. Follows existing app design patterns.
- **Dependencies**: Relationship Metadata Storage, Relationship Engine
- **Priority**: Must-have

### Spec: Cross-Table Component Integration

- **Description**: Component-level support for pulling fields from related tables, automatic JOIN path resolution, query editor implicit relationship detection (matching field names across SQL queries), data surfacing side-by-side, and performance management (indexing, pagination with opt-out). Must guarantee data correctness.
- **Dependencies**: Relationship Metadata Storage, Relationship Engine
- **Priority**: Must-have

## Technical Approach

The app is a Svelte 5 + SvelteKit + Tauri desktop application using DuckDB as the embedded database. Relationships are stored in a dedicated `relationships` table in DuckDB, separate from existing metadata tables (labels, tags, groups, table types).

The visual relationship grid uses Svelte Flow (https://svelteflow.dev/) for the canvas-based node-and-edge interface. Tables are nodes, columns are connection points, and relationships are edges.

The relationship engine uses DuckDB's native query capabilities for density calculations and reconciliation. Circular reference detection uses graph traversal (DFS/BFS) on the relationship graph stored in memory or computed on-demand.

Query editor implicit relationships detect matching field names across SQL queries and surface them as potential relationships for confirmation.

Cross-table components resolve JOIN paths by traversing the relationship graph from the primary table to the target table, building the appropriate SQL JOIN clauses.

Performance is managed through DuckDB indexing on relationship columns and pagination in data-heavy components by default, with opt-out available.

## Constraints & Assumptions

- **DuckDB**: All relationship storage and computation happens within DuckDB — no external database
- **Svelte Flow**: The visual grid must use Svelte Flow as the canvas library
- **Equality joins only**: Relationships are simple equality joins (`tableA.col = tableB.col`)
- **Data correctness**: Incorrect relationships must be prevented or flagged — silent corruption is unacceptable
- **All users**: The feature must be usable by both technical and non-technical users

## Risks & Open Questions

### Risks

- **Circular references**: Complex relationship graphs could create infinite loops in queries. Mitigation: graph traversal detection before relationship creation, blocking at the storage layer.
- **Data correctness**: If a user defines a wrong relationship, downstream components show incorrect data. Mitigation: reconciliation functions, density calculation, preventive validation.
- **Performance**: Relationship density calculation on large tables could be slow. Mitigation: DuckDB indexing, caching, pagination.

### Open Questions

- Should relationship density be computed on-demand or cached with periodic refresh?
- Should relationships have a name/label field for user reference?
- How should the system handle schema changes (column renamed or deleted) that break existing relationships?

## Phasing & Rollout

### Phase 1: Full Release

All capabilities delivered together:
- Relationship metadata storage
- Relationship engine (circular reference prevention, density calculation, reconciliation, auto-detection)
- Visual relationship grid (Svelte Flow)
- Cross-table component integration (query editor implicit relationships, component field support)

### Rollout Plan

- No feature flags — delivered as a complete feature
- Existing `/data` Relationships tab placeholder is replaced with the Svelte Flow grid
- Documentation for relationship creation and management
- Reconciliation reports available for users to verify relationship correctness
