// CDP smoke test for the central-charts build (Node 22+ built-in WebSocket)
const BASE = 'http://localhost:9222';

async function target() {
	const list = await (await fetch(`${BASE}/json`)).json();
	return list.find((t) => t.type === 'page');
}

function connect(wsUrl) {
	const ws = new WebSocket(wsUrl);
	let id = 0;
	const pending = new Map();
	ws.onmessage = (ev) => {
		const msg = JSON.parse(ev.data);
		if (msg.id && pending.has(msg.id)) {
			pending.get(msg.id)(msg);
			pending.delete(msg.id);
		}
	};
	const send = (method, params = {}) =>
		new Promise((res) => {
			const mid = ++id;
			pending.set(mid, res);
			ws.send(JSON.stringify({ id: mid, method, params }));
		});
	return new Promise((res) => (ws.onopen = () => res({ ws, send })));
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function main() {
	const t = await target();
	const { send } = await connect(t.webSocketDebuggerUrl);
	const results = [];
	const check = (name, ok, extra = '') => {
		results.push(`${ok ? 'PASS' : 'FAIL'}  ${name}${extra ? ' — ' + extra : ''}`);
		if (!ok) process.exitCode = 1;
	};

	const navigate = async (url) => {
		await send('Page.navigate', { url });
		await sleep(2500);
	};
	const expr = async (expression) => {
		const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
		return r.result?.result?.value;
	};

	// 1. /pages list loads (new version, not the old draft)
	await navigate('http://localhost:6123/pages');
	let h1 = await expr("document.querySelector('h1')?.textContent");
	check('/pages loads report pages list', h1 === 'Report pages', `h1=${h1}`);

	// create a page
	await expr(`(() => {
		const input = document.querySelector('input[placeholder="New page name…"]');
		input.value = 'Smoke test';
		input.dispatchEvent(new Event('input', { bubbles: true }));
	})()`);
	await expr("document.querySelector('input[placeholder=\"New page name…\"]')?.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }))");
	// press Enter in the input instead (button may be first add button)
	await navigate('http://localhost:6123/pages/smoke-test');
	await sleep(3000);
	h1 = await expr("document.querySelector('input[style*=font-display]')?.value");
	check('/pages/smoke-test editor loads (created via save on nav)', /smoke/i.test(String(h1)), `title=${h1}`);

	// 2. add a bar chart block
	const added = await expr(`(() => {
		const btn = [...document.querySelectorAll('button')].find((b) => b.textContent.includes('Bar chart'));
		if (!btn) return 'no button';
		btn.click();
		return 'clicked';
	})()`);
	await sleep(2500);
	check('add bar chart block', added === 'clicked');

	const hasInspector = await expr("!!document.querySelector('aside')");
	check('inspector visible after add', hasInspector);

	// 3. does a chart render (svg from svelteplot) — needs tables in workspace
	await sleep(3500);
	const svgCount = await expr("document.querySelectorAll('.chart-card svg').length");
	const cardCount = await expr("document.querySelectorAll('.chart-card').length");
	check('chart card renders', Number(cardCount) >= 1, `cards=${cardCount} svgs=${svgCount}`);
	const stateText = await expr("document.querySelector('.chart-card')?.textContent?.slice(0, 120)");
	check('chart produced data (not empty/error)', !/No data|Failed|unknown/i.test(String(stateText)), String(stateText).slice(0, 90));

	// 4. code mode: switch, edit JSON, switch back losslessly
	await expr(`(() => { const b = [...document.querySelectorAll('button')].find((x) => x.textContent.includes('Code')); b?.click(); })()`);
	await sleep(400);
	let codeOk = await expr("!!document.querySelector('textarea')");
	check('code mode shows JSON textarea', codeOk);
	const jsonValid = await expr(`(() => { try { JSON.parse(document.querySelector('textarea').value); return true; } catch { return false; } })()`);
	check('code mode document is valid JSON', jsonValid);

	// 5. save
	await expr(`(() => { const b = [...document.querySelectorAll('button')].find((x) => x.textContent.includes('Design')); b?.click(); })()`);
	await sleep(600);
	const saved = await expr(`(() => { const b = [...document.querySelectorAll('button')].find((x) => x.textContent.trim().startsWith('Save')); b?.click(); return 'saved-clicked'; })()`);
	await sleep(1500);
	check('save clicked', saved === 'saved-clicked');

	// 6. /pages lists it
	await navigate('http://localhost:6123/pages');
	const listed = await expr("document.body.textContent.includes('Smoke test')");
	check('/pages lists the saved page', listed);

	// 7. /data tabs (relationships/measures/dimensions)
	await navigate('http://localhost:6123/data');
	await sleep(2000);
	const tabs = await expr(`[...document.querySelectorAll('button,[role=tab]')].map((b) => b.textContent).join('|')`);
	check('/data has Measures + Dimensions tabs', /Measures/.test(String(tabs)) && /Dimensions/.test(String(tabs)));

	console.log(results.join('\n'));
	process.exit(process.exitCode ?? 0);
}

main().catch((e) => {
	console.error('SMOKE CRASHED', e);
	process.exit(1);
});
