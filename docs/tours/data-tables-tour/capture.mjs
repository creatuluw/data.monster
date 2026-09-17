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
	// seed: regions aanmaken (download → CTAS) + taggen als 'demo data' (blijft staan, per opdracht)
	// cleanup: superstore terug naar pre-tour staat (zonder de live getagde 'retail' tag)
	seed: async ({ page, invoke }) => {
		try {
			const res = await invoke(page, 'list_tables', {});
			const names = (res?.tables ?? res ?? []).map((t) => t?.name ?? t).filter(Boolean);
			if (!names.includes('regions')) {
				const { path } = await invoke(page, 'download_url_to_workspace', { url: 'http://localhost:8123/regions.csv' });
				const p = String(path).replace(/\\/g, '/');
				await invoke(page, 'create_table_from_query', {
					tableName: 'regions',
					sql: `SELECT * FROM read_csv_auto('${p}')`
				});
			}
		} catch (e) {
			console.error('SEED regions mislukt:', e?.message ?? e);
		}
		try {
			await invoke(page, 'save_table_labels', { tableName: 'regions', tags: 'demo data', group: null });
		} catch (e) {
			console.error('SEED tag mislukt:', e?.message ?? e);
		}
		return {};
	},
	cleanup: async ({ page, invoke }) => {
		try {
			await invoke(page, 'save_table_labels', { tableName: 'superstore', tags: '', group: null });
		} catch {}
	},

	frames: [
		{
			naam: 'start',
			url: '/data',
			wacht: 2200,
			voor: async (page) => {
				await page.waitForSelector('.card-title', { timeout: 60000 }); // echte kaarten, geen 'Loading metadata…'
			},
			anchors: [
				['card-superstore', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'superstore')`],
				['card-regions', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'regions')`],
				['open-superstore', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'superstore')?.querySelector('.card-action')`]
			]
		},
		{
			naam: 'drawer',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.locator('.card', { hasText: 'superstore' }).first().locator('.card-title').click();
				await page.waitForSelector('.drawer .tag-text-input', { timeout: 60000 }); // drawer echt geladen
				await page.waitForTimeout(450); // slide-in animatie
			},
			anchors: [
				['drawer-title', `() => document.querySelector('.drawer-title')`],
				['group-input', `() => [...document.querySelectorAll('.drawer .tag-text-input')].find(i => (i.placeholder || '').includes('Add group'))`],
				['tags-input', `() => [...document.querySelectorAll('.drawer .tag-text-input')].find(i => (i.placeholder || '').includes('Add tag'))`],
				['save-btn', `() => [...document.querySelectorAll('.drawer button')].find(b => b.textContent.trim() === 'Save changes')`]
			]
		},
		{
			naam: 'tag-typed',
			url: null,
			wacht: 200,
			voor: async (page) => {
				const inp = page.locator('.drawer .tag-text-input').nth(1); // 0 = group, 1 = tags
				await inp.click();
				await inp.fill('retail');
				await inp.press('Enter');
				await page.waitForSelector('.drawer .tag-chip', { timeout: 5000 }); // chip staat er pas na Enter
			},
			anchors: [
				['new-tag-chip', `() => [...document.querySelectorAll('.drawer .tag-chip')].find(c => c.textContent.trim().startsWith('retail'))`],
				['tags-input', `() => [...document.querySelectorAll('.drawer .tag-text-input')].find(i => (i.placeholder || '').includes('Add tag'))`],
				['save-btn', `() => [...document.querySelectorAll('.drawer button')].find(b => b.textContent.trim() === 'Save changes')`]
			]
		},
		{
			naam: 'saved-list',
			url: null,
			wacht: 200,
			voor: async (page) => {
				await page.locator('.drawer button', { hasText: 'Save changes' }).click();
				await page.waitForTimeout(700); // saveTableLabels afronden
				await page.locator('.drawer-close').click();
				await page.waitForTimeout(350); // drawer dicht
				await page.goto('http://localhost:6123/data'); // lijst laadt labels opnieuw
				await page.waitForFunction(
					() => [...document.querySelectorAll('.tag-label')].some((t) => t.textContent.trim() === 'retail'),
					null,
					{ timeout: 60000 }
				); // de nieuwe tag staat écht op de kaart
			},
			anchors: [
				['tag-retail-card', `() => [...document.querySelectorAll('.card .tag-label')].find(t => t.textContent.trim() === 'retail')`],
				['tag-demo-card', `() => [...document.querySelectorAll('.card .tag-label')].find(t => t.textContent.trim() === 'demo data')`],
				['filter-retail', `() => [...document.querySelectorAll('.filter-chip')].find(c => c.textContent.trim() === 'retail')`],
				['filter-demo-data', `() => [...document.querySelectorAll('.filter-chip')].find(c => c.textContent.trim() === 'demo data')`]
			]
		},
		{
			naam: 'filter-active',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.locator('.filter-chip', { hasText: 'retail' }).click();
				await page.waitForFunction(
					() => ![...document.querySelectorAll('.card-title')].some((t) => t.textContent.trim() === 'regions'),
					null,
					{ timeout: 10000 }
				); // regions valt weg, superstore blijft
			},
			anchors: [
				['filter-retail-active', `() => [...document.querySelectorAll('.filter-chip')].find(c => c.textContent.trim() === 'retail')`],
				['card-superstore', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'superstore')`],
				['open-superstore', `() => [...document.querySelectorAll('.card')].find(c => c.querySelector('.card-title')?.textContent.trim() === 'superstore')?.querySelector('.card-action')`]
			]
		},
		{
			naam: 'table-detail',
			url: null,
			wacht: 300,
			voor: async (page) => {
				await page.locator('.card-action').click(); // Open → (superstore is de enige kaart na filter)
				await page.waitForURL('**/table/superstore', { timeout: 30000 });
				await page.waitForSelector('.data-table tbody tr', { timeout: 60000 }); // ECHTE rijen, geen 'Loading table…'
				await page.waitForTimeout(400);
			},
			anchors: [
				['viewer-title', `() => document.querySelector('.viewer-title')`],
				['rows-badge', `() => document.querySelector('.viewer-header .tag-accent')`],
				['first-row', `() => document.querySelector('.data-table tbody tr')`],
				['next-btn', `() => [...document.querySelectorAll('.pagination button')].find(b => b.textContent.trim().startsWith('next'))`],
				['query-btn', `() => [...document.querySelectorAll('.viewer-actions button')].find(b => b.textContent.trim() === 'Query this table')`]
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
