/**
 * Library component packages — extension-style entries. Each package owns its
 * definition, demo data, docs and source references; registering it makes the
 * component available BOTH in /library and in the /pages editor (block picker
 * + schema-driven config panel), because registration feeds the same central
 * chart registry the page runtime consumes.
 */
import type { ChartTypeDefinition } from '$lib/charts/registry';
import type { Component } from 'svelte';

export type LibraryDemo = {
	/** pre-aggregated demo rows (keys = dimension/measure aliases) */
	rows: Record<string, unknown>[];
	dimensionAliases: string[];
	measureAliases: string[];
	/** overrides on top of the definition's option defaults */
	options?: Record<string, unknown>;
	title: string;
	subtitle?: string;
	/** table blocks: visible columns (defaults to row keys) */
	columns?: string[];
	/** text blocks: sample body */
	text?: string;
};

export type LibraryEntry = {
	/** central-charts chart type definition — drives validation, query hooks and config panels */
	def: ChartTypeDefinition;
	/** which /pages block kind this component instantiates. 'chart' entries feed
	 *  the chart registry (add-block picker + config panel); 'table'/'text' are the
	 *  built-in block kinds (rendered by PageGrid directly). */
	blockKind?: 'chart' | 'table' | 'text';
	/** the REAL renderer used on /pages — demos never use a clone */
	// Component<any>: renderer prop types are ChartRendererProps-shaped but inferred
	// as Component<$$ComponentProps> per component — variance kills a tighter type
	renderer: Component<any>;
	description: string;
	/** usage notes (markdown text, shown on the Docs tab) */
	docs: string;
	demo: LibraryDemo;
	/** filename → source, shown on the Code tab */
	code: Record<string, string>;
};
