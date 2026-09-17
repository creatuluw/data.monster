/**
 * app-tour-demo capture — Tauri adaptation for data.monster.
 * Machinery diffs vs the skill's scripts/capture.mjs:
 *   · connects over CDP to the RUNNING app (tauri dev + --remote-debugging-port)
 *     instead of chromium.launch + HTTP login (data.monster has no login)
 *   · seed/cleanup go through window.__TAURI_INTERNALS__.invoke — the same
 *     real API the app's own invoke() calls; never writes the DB directly
 *   · frame.url === null → stay on the current page (the previous frame's
 *     voor() navigated)
 *   · never closes the app (browser.close() would kill the webview)
 *   · Google Fonts via @import appended to the CSS blob (app.html loads them
 *     from the network; file:// players may not fetch, but a stylesheet
 *     @import is allowed)
 * EDIT ONLY THE DRAAIBOEK BLOCK for a new tour.
 */
import { chromium } from '@playwright/test';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';

const HIER = dirname(fileURLToPath(import.meta.url));
const TUSSENSTAND = resolve(tmpdir(), 'app-tour-demo-tour-assets.json');

// ══════════════════════════ DRAAIBOEK — pas dit aan ══════════════════════════
const DRAAIBOEK = {
	cdp: 'http://localhost:9222',
	base: 'http://localhost:6123',
	prod: 'http://localhost:6123', // herschrijft relatieve url's in snapshots
	fontsImport:
		"https://fonts.googleapis.com/css2?family=Geist+Mono:wght@100..900&family=Inter:ital,wght@0,100..900;1,100..900&display=swap",
	viewport: { width: 1440, height: 900 },

	// zaaien/opruimen via de ÉCHTE app-api (invoke) — geen directe DB-schrijfacties
	// /pages bouwt automatisch op app.tables[0] (= alfabetisch eerste tabel), dus de
	// demo-tabel moet vóór "regions" sorteren. CTAS via execute_query (echte /query weg),
	// opruimen via drop_table. app.tables ververst pas op /data-mount: vandaar frame 'data'.
	seed: async ({ page, invoke }) => {
		await invoke(page, 'execute_query', { sql: 'DROP TABLE IF EXISTS "pages_demo_sales"' }).catch(() => {});
		await invoke(page, 'execute_query', {
			sql: 'CREATE TABLE pages_demo_sales AS SELECT region, category, ROUND(SUM(sales), 2)::DOUBLE AS total_sales, COUNT(*)::BIGINT AS orders FROM superstore GROUP BY region, category'
		});
	},
	cleanup: async ({ page, invoke }) => {
		await invoke(page, 'drop_table', { tableName: 'pages_demo_sales' }).catch(() => {});
		// /data-mount ververs app.tables zodat de weggegooide demo-tabel verdwijnt uit de UI
		await page.goto('http://localhost:6123/data').catch(() => {});
		await page.waitForTimeout(1500).catch(() => {});
	},

	frames: [
		{
			naam: 'data',
			url: '/data',
			wacht: 500,
			voor: async (page) => {
				await page.waitForSelector('.card-title', { timeout: 60000 });
				await page.waitForFunction(
					() => [...document.querySelectorAll('.card-title')].some((t) => t.textContent.trim() === 'pages_demo_sales'),
					null,
					{ timeout: 60000 }
				); // echte inhoud: de gezaaide demo-tabel staat in de lijst
			},
			anchors: [
				['demo-table', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'pages_demo_sales')`]
			]
		},
		{
			naam: 'dashboard',
			url: '/pages',
			wacht: 500,
			voor: async (page) => {
				await page.waitForSelector('.chart-card .chart-row', { timeout: 60000 });
				await page.waitForSelector('.pages-table-title', { timeout: 60000 }); // wacht op echte data, niet op spinner
			},
			anchors: [
				['chart-region', `() => [...document.querySelectorAll('.chart-title')].find(t => t.textContent.includes('region'))`],
				['chart-category', `() => [...document.querySelectorAll('.chart-title')].find(t => t.textContent.includes('category'))`],
				['filter-bar', `() => document.querySelector('.filter-bar')`],
				['row-region', `() => [...document.querySelectorAll('.chart-card')].find(c => c.querySelector('.chart-title')?.textContent.includes('region'))?.querySelector('.chart-row')`],
				['table-title', `() => document.querySelector('.pages-table-title')`]
			]
		},
		{
			// ECHTE gedrag: klik selecteert + requery, maar loadChartData() reset de selectie —
			// de chip verschijnt heel even en de filter-bar is daarna weer leeg. Eerlijk vatten.
			naam: 'after-click',
			url: null,
			wacht: 400,
			voor: async (page) => {
				await page.locator('.chart-card', { hasText: 'by region' }).locator('.chart-row').first().click();
				await page.waitForSelector('.filter-bar-empty', { timeout: 60000 }); // requery klaog EN selectie weggevallen
				await page.waitForTimeout(800); // tabel ook herladen
			},
			anchors: [
				['filter-bar', `() => document.querySelector('.filter-bar')`],
				['chart-action', `() => [...document.querySelectorAll('.chart-card')].find(c => c.querySelector('.chart-title')?.textContent.includes('region'))?.querySelector('.chart-action')`],
				['table-title', `() => document.querySelector('.pages-table-title')`]
			]
		},
		{
			naam: 'detail',
			url: null,
			wacht: 400,
			voor: async (page) => {
				await page.locator('.chart-card', { hasText: 'by region' }).locator('.chart-action').click();
				await page.waitForURL('**/pages/chart/**', { timeout: 30000 });
				await page.waitForSelector('.drawer-empty', { timeout: 60000 });
				await page.waitForSelector('.chart-card .chart-row', { timeout: 60000 }); // echte chart-balken, geen spinner
			},
			anchors: [
				['back-btn', `() => document.querySelector('.back-btn')`],
				['breadcrumb-pages', `() => [...document.querySelectorAll('.breadcrumb-link')].find(a => a.textContent.trim() === 'Pages')`],
				['drawer-title', `() => document.querySelector('.drawer-title')`],
				['drawer-empty', `() => document.querySelector('.drawer-empty')`]
			]
		},
		{
			naam: 'back',
			url: null,
			wacht: 400,
			voor: async (page) => {
				// de back-btn deelt pixels met de breadcrumb-bar (negatieve marge) —
				// de echte klikbare terugweg is de breadcrumb 'Pages' link
				await page.locator('.breadcrumb-link', { hasText: 'Pages' }).click();
				await page.waitForURL('**/pages', { timeout: 30000 });
				await page.waitForSelector('.chart-card .chart-row', { timeout: 60000 });
				await page.waitForSelector('.pages-table-title', { timeout: 60000 }); // tabel laadt na de charts
			},
			anchors: [
				['chart-region', `() => [...document.querySelectorAll('.chart-title')].find(t => t.textContent.includes('region'))`],
				['table-title', `() => document.querySelector('.pages-table-title')`]
			]
		}
	]
};
// ═════════════════════════ tot hier ═════════════════════════════════════════

const cssRegels = new Set();
const frames = [];

// verbind met de DRAAIENDE app — geen launch, geen login, geen close
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
const h = { page, invoke };

// DOM+CSS-serialisatie — identiek contract aan het skill-origineel
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
