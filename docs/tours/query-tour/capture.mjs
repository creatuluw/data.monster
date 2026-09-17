/**
 * app-tour-demo capture — query tour for data.monster /query.
 * Machinery identical to docs/tours/connect-tour/capture.mjs (see its header).
 * Only the DRAAIBOEK block differs.
 *
 * Flow captured (real UI, real DuckDB):
 *   1. landing on /query (sidebar lists superstore 51,290 rows)
 *   2. editor with the aggregation typed
 *   3. Run -> real result grid (4 region rows) — waits for actual rows
 *   4. Save modal (name filled)
 *   5. Saved queries panel showing the saved card
 *   6. new tab, SELECT * FROM superstore -> pagination UI (6 pages)
 *   7. Ingest modal
 * Statements used: SELECT only.
 */
import { chromium } from '@playwright/test';
import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';

const HIER = dirname(fileURLToPath(import.meta.url));
const TUSSENSTAND = resolve(tmpdir(), 'app-tour-demo-tour-assets.json');

// ══════════════════════════ DRAAIBOEK — pas dit aan ══════════════════════════
const DRAAIBOEK = {
	cdp: 'http://localhost:9222',
	base: 'http://localhost:6123',
	prod: 'http://localhost:6123',
	fontsImport:
		"https://fonts.googleapis.com/css2?family=Geist+Mono:wght@100..900&family=Inter:ital,wght@0,100..900;1,100..900&display=swap",
	viewport: { width: 1440, height: 900 },

	seed: null,

	// opruimen: de in de tour opgeslagen query weer weghalen (echte app-api)
	// N.B. na goto('/') kan het oude page-target vervangen zijn — zoek het opnieuw.
	cleanup: async ({ page, invoke, browser }) => {
		const qs = async () => {
			const p = browser.contexts()[0].pages().find((x) => x.url().startsWith('http://localhost:6123')) || page;
			const res = await p.evaluate(() => window.__TAURI_INTERNALS__.invoke('list_saved_queries', {}));
			return res?.queries ?? [];
		};
		for (const q of await qs()) {
			if (q.name === 'Sales by region') {
				const p = browser.contexts()[0].pages().find((x) => x.url().startsWith('http://localhost:6123')) || page;
				await p.evaluate((slug) => window.__TAURI_INTERNALS__.invoke('delete_saved_query', { slug }), q.slug);
				console.log('cleanup: saved query', q.slug, 'verwijderd');
			}
		}
	},

	frames: [
		{
			naam: 'start',
			url: '/query',
			wacht: 1200,
			voor: async (page) => {
				await page.waitForSelector('.sidebar-table-item', { timeout: 60000 }); // echte tabellenlijst (superstore)
			},
			anchors: [
				['editor-textarea', `() => document.querySelector('.editor-textarea')`],
				['superstore-sidebar', `() => [...document.querySelectorAll('.sidebar-table-item')].find(b => b.textContent.includes('superstore'))`],
				['run-btn', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Run')`]
			]
		},
		{
			naam: 'editor-typed',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.fill('.editor-textarea', 'SELECT region, SUM(sales) AS sales FROM superstore GROUP BY region ORDER BY sales DESC;');
			},
			anchors: [
				['run-btn', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Run')`]
			]
		},
		{
			naam: 'result-grid',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.getByRole('button', { name: 'Run' }).click();
				await page.waitForSelector('.results-table-wrap table tbody tr', { timeout: 60000 }); // echte rijen, geen vaste ms
				await page.waitForSelector('.tag-accent', { timeout: 60000 });
			},
			anchors: [
				['rows-badge', `() => document.querySelector('.tag-accent')`],
				['save-btn', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Save')`],
				['tab-new', `() => document.querySelector('.tab-new')`]
			]
		},
		{
			naam: 'save-modal',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.getByRole('button', { name: 'Save', exact: true }).click();
				await page.waitForSelector('#save-query-name', { timeout: 10000 });
				await page.fill('#save-query-name', 'Sales by region');
			},
			anchors: [
				['save-name', `() => document.querySelector('#save-query-name')`],
				['save-confirm', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Save query')`]
			]
		},
		{
			naam: 'saved-panel',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.getByRole('button', { name: 'Save query' }).click();
				await page.waitForSelector('#save-query-name', { state: 'detached', timeout: 30000 });
				await page.getByRole('button', { name: 'Saved' }).click();
				await page.waitForSelector('.saved-card', { timeout: 30000 }); // echte opgeslagen kaart
			},
			anchors: [
				['saved-card', `() => document.querySelector('.saved-card')`],
				['saved-close', `() => document.querySelector('.modal-close')`]
			]
		},
		{
			naam: 'result-back',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.locator('.modal-close').click();
				await page.waitForSelector('.saved-card', { state: 'detached', timeout: 10000 });
				await page.waitForSelector('.results-table-wrap table tbody tr', { timeout: 10000 });
			},
			anchors: [
				['ingest-btn', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Ingest')`],
				['tab-new', `() => document.querySelector('.tab-new')`]
			]
		},
		{
			naam: 'ingest-modal',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.getByRole('button', { name: 'Ingest' }).click();
				await page.waitForSelector('#ingest-table-name', { timeout: 10000 });
			},
			anchors: [
				['ingest-name', `() => document.querySelector('#ingest-table-name')`],
				['ingest-cancel', `() => [...document.querySelectorAll('.modal-footer button')].find(b => b.textContent.trim() === 'Cancel')`]
			]
		},
		{
			naam: 'pagination',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.locator('.modal-footer button', { hasText: 'Cancel' }).click();
				await page.waitForSelector('#ingest-table-name', { state: 'detached', timeout: 10000 });
				await page.locator('.tab-new').click();
				await page.fill('.editor-textarea', 'SELECT * FROM superstore;');
				await page.getByRole('button', { name: 'Run' }).click();
				await page.waitForSelector('.pagination', { timeout: 60000 }); // echte paginering
			},
			anchors: [
				['pagination-info', `() => document.querySelector('.pagination-info')`]
			]
		}
	]
};
// ═════════════════════════ tot hier ═════════════════════════════════════════

// WebView2 spawns a shared_worker target that playwright 1.63 cannot attach
// (no browserContextId -> assert). Swallow ONLY that assert; everything else kills us.
process.on("uncaughtException", (e) => {
  if (!/shared_worker/.test(String(e?.message ?? e))) { console.error(e); process.exit(1); }
});

const cssRegels = new Set();
const frames = [];

const browser = await chromium.connectOverCDP(DRAAIBOEK.cdp);
const context = browser.contexts()[0];
let page = context.pages().find((p) => p.url().startsWith(DRAAIBOEK.base));
if (!page) page = await context.newPage();
const cdp = await context.newCDPSession(page);
if (DRAAIBOEK.viewport)
	await cdp.send('Emulation.setDeviceMetricsOverride', {
		width: DRAAIBOEK.viewport.width,
		height: DRAAIBOEK.viewport.height,
		deviceScaleFactor: 1,
		mobile: false
	});

const invoke = (page, cmd, args) =>
	page.evaluate(({ cmd, args }) => window.__TAURI_INTERNALS__.invoke(cmd, args), { cmd, args });
const h = { page, invoke, browser };

const SERIAL = ([anchors, PROD]) => {
	for (const [naam, vinden] of anchors) {
		try {
			const fn = eval('(' + vinden + ')');
			const el = typeof fn === 'function' ? fn() : fn;
			if (el && el.setAttribute) el.setAttribute('data-demo-anchor', naam);
		} catch {}
	}
	const scrolls = [];
	(function wandel(el, pad) {
		for (let i = 0; i < el.children.length; i++) {
			const c = el.children[i];
			const p = [...pad, i];
			if (c.scrollTop > 4 || c.scrollLeft > 4) scrolls.push({ path: p, top: c.scrollTop, left: c.scrollLeft });
			wandel(c, p);
		}
	})(document.body, []);
	const doc = document.documentElement.cloneNode(true);
	doc.querySelectorAll('script, style, link, noscript').forEach((n) => n.remove());
	doc.querySelector('head')?.appendChild(document.createComment('CSS0'));
	const abs = (n, attr) => {
		const v = n.getAttribute(attr);
		if (v && v.startsWith('/')) n.setAttribute(attr, PROD + v);
	};
	doc.querySelectorAll('[src]').forEach((n) => abs(n, 'src'));
	doc.querySelectorAll('[href]').forEach((n) => abs(n, 'href'));
	doc.querySelectorAll('[srcset]').forEach((n) => abs(n, 'srcset'));
	const regels = [];
	for (const sheet of document.styleSheets) {
		try {
			for (const r of sheet.cssRules) regels.push(r.cssText);
		} catch {}
	}
	return { html: doc.outerHTML, scrolls, regels };
};

async function vang(page, naam, anchors) {
	const res = await page.evaluate(SERIAL, [anchors, DRAAIBOEK.prod]);
	res.regels.forEach((r) => cssRegels.add(r));
	frames.push({ naam, html: res.html, scrolls: res.scrolls });
	console.log(
		'frame',
		naam,
		'· anchors',
		anchors.filter(([n]) => res.html.includes('data-demo-anchor="' + n + '"')).length + '/' + anchors.length,
		'· css',
		res.regels.length
	);
}

let zaaisel = null;
try {
	if (DRAAIBOEK.seed) zaaisel = await DRAAIBOEK.seed(h);
	for (const f of DRAAIBOEK.frames) {
		if (f.url) await page.goto(DRAAIBOEK.base + f.url);
		await page.waitForTimeout(f.wacht ?? 1500);
		if (f.voor) await f.voor(page);
		await vang(page, f.naam, f.anchors ?? []);
		writeFileSync(TUSSENSTAND, JSON.stringify({ frames, css: [...cssRegels].join('\n') }));
	}
} finally {
	if (DRAAIBOEK.cleanup) {
		await page.goto(DRAAIBOEK.base + '/').catch(() => {});
		await page.waitForTimeout(500).catch(() => {});
		try {
			await DRAAIBOEK.cleanup(h, zaaisel);
		} catch (e) {
			console.error('CLEANUP MISLUKT — handmatig opruimen:', e?.message ?? e);
		}
	}
}

let css = [...cssRegels].join('\n');
css = css.replace(/url\((['"]?)(\/(?!\/)[^'")]+)\1\)/g, (m, q, pad) => `url(${q}${DRAAIBOEK.prod}${pad}${q})`);
if (DRAAIBOEK.fontsImport) css = `@import url('${DRAAIBOEK.fontsImport}');\n` + css;

writeFileSync(resolve(HIER, 'tour-assets.json'), JSON.stringify({ frames, css }));
console.log(frames.length, 'frames + css', Math.round(css.length / 1024) + 'kB', '→ tour-assets.json');
process.exit(0); // NIET browser.close() — dat zou de app killen
