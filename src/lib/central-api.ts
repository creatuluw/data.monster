/**
 * Invoke wrappers for the central-charts internal-DB commands (FR-10/14/15).
 * Rust side: src-tauri/src/commands/{pages,items,relationships}.rs
 */
import { invoke } from '@tauri-apps/api/core';
import type { PageDoc } from '$lib/charts/spec-types';
import type { MasterItem } from '$lib/charts/items';
import type { Relationship } from '$lib/charts/relationships';

export type PageMeta = { slug: string; title: string; createdAt?: string; updatedAt?: string };

export async function listPages(): Promise<PageMeta[]> {
	const result = await invoke<{ pages: PageMeta[] }>('list_pages');
	return result.pages;
}

export async function getPage(slug: string): Promise<PageDoc> {
	const result = await invoke<{ spec: string }>('get_page', { slug });
	return JSON.parse(result.spec) as PageDoc;
}

export async function savePage(doc: PageDoc): Promise<void> {
	return invoke<void>('save_page', { slug: doc.slug, title: doc.title, spec: JSON.stringify(doc, null, '\t') });
}

export async function deletePage(slug: string): Promise<void> {
	return invoke<void>('delete_page', { slug });
}

export async function listMasterItems(kind?: 'measure' | 'dimension'): Promise<MasterItem[]> {
	const result = await invoke<{ items: MasterItem[] }>('list_master_items', { kind: kind ?? null });
	return result.items;
}

export async function saveMasterItem(item: MasterItem): Promise<void> {
	return invoke<void>('save_master_item', {
		id: item.id,
		kind: item.kind,
		tableName: item.table,
		expr: item.expr,
		label: item.label,
		fmt: item.fmt ?? null,
		description: item.description ?? null
	});
}

export async function deleteMasterItem(id: string): Promise<void> {
	return invoke<void>('delete_master_item', { id });
}

export type RelationshipInput = {
	id?: string;
	fromTable: string;
	fromColumn: string;
	toTable: string;
	toColumn: string;
};

export async function listRelationships(): Promise<Relationship[]> {
	const result = await invoke<{ relationships: Relationship[] }>('list_relationships');
	return result.relationships;
}

export async function saveRelationship(rel: RelationshipInput): Promise<void> {
	return invoke<void>('save_relationship', {
		fromTable: rel.fromTable,
		fromColumn: rel.fromColumn,
		toTable: rel.toTable,
		toColumn: rel.toColumn,
		id: rel.id ?? null
	});
}

export async function deleteRelationship(id: string): Promise<void> {
	return invoke<void>('delete_relationship', { id });
}
