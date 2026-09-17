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
	prod: 'http://localhost:6123',
	fontsImport:
		"https://fonts.googleapis.com/css2?family=Geist+Mono:wght@100..900&family=Inter:ital,wght@0,100..900;1,100..900&display=swap",
	viewport: { width: 1440, height: 900 },

	// /labs en de bar-chart playground zijn volledig static + synthetisch:
	// geen seed, geen cleanup, geen tafels geraakt.
	seed: null,
	cleanup: null,

	frames: [
		{
			naam: 'catalog',
			url: '/labs',
			wacht: 800,
			voor: async (page) => {
				await page.waitForSelector('a[href="/labs/bar-chart"]', { timeout: 60000 }); // grid echt geladen
			},
			anchors: [
				['card-bar', `() => document.querySelector('a[href="/labs/bar-chart"]')`],
				['card-heatmap', `() => document.querySelector('a[href="/labs/heatmap"]')`],
				['card-placeholder', `() => document.querySelector('a[href="/labs/sankey"]')`]
			]
		},
		{
			naam: 'bar-chart',
			url: null, // de kaart-klik hieronder navigeert, net als in de tour
			wacht: 400,
			voor: async (page) => {
				await page.click('a[href="/labs/bar-chart"]');
				await page.waitForURL('**/labs/bar-chart', { timeout: 30000 });
				await page.waitForSelector('.bar-chart-title', { timeout: 60000 });
				await page.waitForSelector('g.bar-x rect', { state: 'attached', timeout: 60000 }); // ECHTE balken, geen spinner
			},
			anchors: [
				['chart-title', `() => document.querySelector('.bar-chart-title')`],
				['first-bar', `() => document.querySelector('g.bar-x rect')`],
				['config-btn', `() => document.querySelector('button.config-btn')`]
			]
		},
		{
			naam: 'bar-selected',
			url: null,
			wacht: 400,
			voor: async (page) => {
				await page.locator('g.bar-x rect').first().click(); // click-to-select
				await page.waitForSelector('.bar-chart-sel', { timeout: 60000 }); // de mono readout verschijnt
			},
			anchors: [
				['sel-pill', `() => document.querySelector('.bar-chart-sel')`],
				['config-btn', `() => document.querySelector('button.config-btn')`]
			]
		},
		{
			naam: 'config',
			url: null,
			wacht: 400,
			voor: async (page) => {
				await page.click('button.config-btn');
				await page.waitForSelector('.drawer-open', { timeout: 60000 }); // drawer echt open
			},
			anchors: [
				['drawer-title', `() => document.querySelector('.drawer .drawer-title')`],
				['cfg-title', `() => document.querySelector('#cfg-title')`],
				['cfg-color', `() => document.querySelector('#cfg-color')`],
				['cfg-height', `() => document.querySelector('#cfg-height')`]
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
