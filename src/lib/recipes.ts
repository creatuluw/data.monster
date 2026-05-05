// Table recipe persistence — stored in localStorage

export interface ColumnOverride {
	originalName: string;
	newName: string;
	detectedType: string;
	overrideType: string | null; // null means keep detected type
}

export interface TableRecipe {
	tableName: string;
	createdAt: string;
	source: {
		type: 'file' | 'url';
		name: string; // filename or URL string
	};
	columnOverrides: ColumnOverride[];
	query: string | null; // null if direct file ingest (no custom SQL)
}

const STORAGE_KEY = 'data-monster-recipes';

function loadRecipes(): Record<string, TableRecipe> {
	if (!browser()) return {};
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		return raw ? JSON.parse(raw) : {};
	} catch {
		return {};
	}
}

function saveRecipes(recipes: Record<string, TableRecipe>): void {
	if (!browser()) return;
	localStorage.setItem(STORAGE_KEY, JSON.stringify(recipes));
}

function browser(): boolean {
	return typeof window !== 'undefined';
}

export function getRecipe(tableName: string): TableRecipe | null {
	return loadRecipes()[tableName] ?? null;
}

export function saveRecipe(recipe: TableRecipe): void {
	const recipes = loadRecipes();
	recipes[recipe.tableName] = recipe;
	saveRecipes(recipes);
}

export function deleteRecipe(tableName: string): void {
	const recipes = loadRecipes();
	delete recipes[tableName];
	saveRecipes(recipes);
}

export function getAllRecipes(): TableRecipe[] {
	return Object.values(loadRecipes());
}

export function clearAllRecipes(): void {
	if (browser()) {
		localStorage.removeItem(STORAGE_KEY);
	}
}
