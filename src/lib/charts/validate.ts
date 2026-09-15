/**
 * Structural validation for page specs (FR-1).
 * Pure, never throws — used by Code mode, save, and render-time guards.
 * Role counts per chart type live in the registry (FR-3); this checks structure.
 */
import type { ValidationError } from './spec-types';

// ponytail: hardcoded until FR-3 swaps these for registry lookups
const BLOCK_TYPES = ['chart', 'table', 'text'];
const CHART_TYPES = ['bar', 'heatmap'];
const GRAINS = ['hour', 'day', 'week', 'month', 'quarter', 'year', 'day_of_week', 'month_of_year'];
const FILTER_OPS = ['=', '!=', '>', '<', '>=', '<=', 'in', 'like'];

const SLUG_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/;

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null;
const isStr = (v: unknown): v is string => typeof v === 'string';

export function validatePageDoc(doc: unknown): ValidationError[] {
	const errors: ValidationError[] = [];
	if (!isObj(doc)) return [{ path: '', message: 'page document must be an object' }];

	if (!isStr(doc.slug) || !SLUG_RE.test(doc.slug))
		errors.push({ path: 'slug', message: 'slug must be lowercase letters, numbers and hyphens' });
	if (!isStr(doc.title) || doc.title.trim() === '')
		errors.push({ path: 'title', message: 'title is required' });

	if (doc.rows !== undefined && !Array.isArray(doc.rows))
		errors.push({ path: 'rows', message: 'rows must be an array' });

	((doc.rows as unknown[] | undefined) ?? []).forEach((row, ri) => {
		if (!isObj(row) || !Array.isArray(row.blocks)) {
			errors.push({ path: `rows[${ri}].blocks`, message: 'row must have a blocks array' });
			return;
		}
		row.blocks.forEach((block, bi) => validateBlock(block, `rows[${ri}].blocks[${bi}]`, errors));
	});

	return errors;
}

function validateBlock(block: unknown, path: string, errors: ValidationError[]): void {
	if (!isObj(block)) {
		errors.push({ path, message: 'block must be an object' });
		return;
	}
	if (!BLOCK_TYPES.includes(block.type as string)) {
		errors.push({ path: `${path}.type`, message: `unknown block type "${String(block.type)}"` });
		return;
	}
	if (block.span !== undefined && (typeof block.span !== 'number' || !Number.isInteger(block.span) || block.span < 1 || block.span > 12))
		errors.push({ path: `${path}.span`, message: 'span must be an integer between 1 and 12' });

	if (block.type === 'chart') return validateChart(block.chart, `${path}.chart`, errors);
	if (block.type === 'table') {
		if (!isStr(block.table) || block.table === '')
			errors.push({ path: `${path}.table`, message: 'table is required' });
		if (block.filters !== undefined) validateFilters(block.filters, `${path}.filters`, errors);
		return;
	}
	// text
	if (!isStr(block.text) || block.text === '')
		errors.push({ path: `${path}.text`, message: 'text is required' });
}

function validateChart(chart: unknown, path: string, errors: ValidationError[]): void {
	if (!isObj(chart)) {
		errors.push({ path, message: 'chart must be an object' });
		return;
	}
	if (!CHART_TYPES.includes(chart.type as string))
		errors.push({ path: `${path}.type`, message: `unknown chart type "${String(chart.type)}"` });

	const source = chart.source;
	if (!isObj(source) || !isStr(source.table) || source.table === '')
		errors.push({ path: `${path}.source.table`, message: 'source table is required' });

	if (!Array.isArray(chart.dimensions))
		errors.push({ path: `${path}.dimensions`, message: 'dimensions must be an array' });
	else
		chart.dimensions.forEach((d, i) => {
			const p = `${path}.dimensions[${i}]`;
			if (!isObj(d)) return errors.push({ path: p, message: 'dimension must be an object' });
			if ('ref' in d) {
				if (!isStr(d.ref) || d.ref === '') errors.push({ path: `${p}.ref`, message: 'ref must be a non-empty id' });
				return;
			}
			if (!isStr(d.col) || d.col === '') errors.push({ path: `${p}.col`, message: 'col is required' });
			if (d.grain !== undefined && !GRAINS.includes(d.grain as string))
				errors.push({ path: `${p}.grain`, message: `unknown grain "${String(d.grain)}"` });
		});

	if (!Array.isArray(chart.measures))
		errors.push({ path: `${path}.measures`, message: 'measures must be an array' });
	else
		chart.measures.forEach((m, i) => {
			const p = `${path}.measures[${i}]`;
			if (!isObj(m)) return errors.push({ path: p, message: 'measure must be an object' });
			const hasRef = 'ref' in m && isStr(m.ref) && m.ref !== '';
			const hasExpr = 'expr' in m && isStr(m.expr) && m.expr !== '';
			if (hasRef && hasExpr)
				errors.push({ path: p, message: 'measure must have either ref or expr (exactly one)' });
			else if (!hasRef && !hasExpr) {
				if ('expr' in m) errors.push({ path: `${p}.expr`, message: 'expr must be a non-empty expression' });
				else errors.push({ path: p, message: 'measure must have either ref or expr (exactly one)' });
			}
		});

	if (chart.filters !== undefined) validateFilters(chart.filters, `${path}.filters`, errors);
}

function validateFilters(filters: unknown, path: string, errors: ValidationError[]): void {
	if (!Array.isArray(filters)) {
		errors.push({ path, message: 'filters must be an array' });
		return;
	}
	filters.forEach((f, i) => {
		const p = `${path}[${i}]`;
		if (!isObj(f)) return errors.push({ path: p, message: 'filter must be an object' });
		if (!isStr(f.col) || f.col === '') errors.push({ path: `${p}.col`, message: 'col is required' });
		if (!FILTER_OPS.includes(f.op as string))
			errors.push({ path: `${p}.op`, message: `unknown filter op "${String(f.op)}"` });
		if (f.value === undefined) errors.push({ path: `${p}.value`, message: 'value is required' });
	});
}
