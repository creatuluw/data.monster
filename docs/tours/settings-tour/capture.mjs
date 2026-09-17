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
	// settings-tour is read-only: geen seed, geen cleanup (de dummy-API-key in
	// settings.json/.env is buiten dit script om gewisseld — zie RUNBOOK CRITICAL)
	seed: null,
	cleanup: null,

	frames: [
		{
			// LLM-configuratieformulier — wacht op het ÉCHTE formulier, niet 'Loading...'
			naam: 'llm-form',
			url: '/settings',
			wacht: 500,
			voor: async (page) => {
				await page.waitForSelector('#api-url', { timeout: 60000 });
			},
			anchors: [
				['mode-remote', `() => [...document.querySelectorAll('.mode-toggle-row .mode-btn')].find(b => b.textContent.includes('Remote API'))`],
				['mode-local', `() => [...document.querySelectorAll('.mode-toggle-row .mode-btn')].find(b => b.textContent.includes('Local AI'))`],
				['api-url', `() => document.querySelector('#api-url')`],
				['api-key', `() => document.querySelector('#api-key')`],
				['api-model', `() => document.querySelector('#api-model')`],
				['save-btn', `() => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === 'Save settings')`],
				['internal-db-link', `() => document.querySelector('a[href="/settings/internal-db"]')`]
			]
		},
		{
			// Internal DB-browser — wacht op de ÉCHTE tabellenlijst (d8a_monster_*)
			naam: 'internal-db',
			url: '/settings/internal-db',
			wacht: 500,
			voor: async (page) => {
				await page.waitForSelector('.idb-table-btn', { timeout: 60000 });
			},
			anchors: [
				['sidebar-internal-title', `() => [...document.querySelectorAll('.idb-sidebar-title')].find(t => t.textContent.trim() === 'Internal Tables')`],
			['sidebar-your-title', `() => [...document.querySelectorAll('.idb-sidebar-title')].find(t => t.textContent.trim() === 'Your Tables')`],
			['tbl-saved-queries', `() => [...document.querySelectorAll('.idb-table-btn')].find(b => b.querySelector('.idb-table-name')?.textContent.trim() === 'saved_queries')`],
			['tbl-field-functions', `() => [...document.querySelectorAll('.idb-table-btn')].find(b => b.querySelector('.idb-table-name')?.textContent.trim() === 'field_functions')`],
			['tbl-labels', `() => [...document.querySelectorAll('.idb-table-btn')].find(b => b.querySelector('.idb-table-name')?.textContent.trim() === 'table_labels')`]
			]
		},
		{
			// klik op d8a_monster_field_functions → echte datagrid is het resultaat
			naam: 'internal-table-data',
			url: null,
			wacht: 400,
			voor: async (page) => {
				await page.getByRole('button', { name: /field_functions/ }).click();
				await page.waitForSelector('.idb-toolbar-table', { timeout: 60000 }); // echte data, geen spinner
			},
			anchors: [
				['toolbar-table', `() => document.querySelector('.idb-toolbar-table')`],
				['toolbar-rows', `() => document.querySelector('.idb-toolbar-rows')`],
				['data-grid', `() => document.querySelector('.idb-grid-wrap')`]
			]
		}
	]
};
// ═════════════════════════ tot hier ═════════════════════════════════════════

// WebView2 spawns a shared_worker target that playwright 1.63 cannot attach
// (no browserContextId -> assert). Swallow ONLY that assert; everything else kills us.
process.on('uncaughtException', (e) => {
  if (!/shared_worker/.test(String(e?.message ?? e))) { console.error(e); process.exit(1); }
});

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
