const BASE = 'http://localhost:9222';
async function target() { const l = await (await fetch(`${BASE}/json`)).json(); return l.find((t) => t.type === 'page'); }
function connect(wsUrl) { const ws = new WebSocket(wsUrl); let id = 0; const p = new Map();
	ws.onmessage = (ev) => { const m = JSON.parse(ev.data); if (m.id && p.has(m.id)) { p.get(m.id)(m); p.delete(m.id); } };
	const send = (method, params = {}) => new Promise((res) => { const i = ++id; p.set(i, res); ws.send(JSON.stringify({ id: i, method, params })); });
	return new Promise((res) => (ws.onopen = () => res({ send }))); }
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const { send } = await connect((await target()).webSocketDebuggerUrl);
const expr = async (e) => (await send('Runtime.evaluate', { expression: e, returnByValue: true, awaitPromise: true })).result?.result?.value;
const results = [];
const check = (name, ok, extra = '') => { results.push(`${ok ? 'PASS' : 'FAIL'}  ${name}${extra ? ' — ' + extra : ''}`); if (!ok) process.exitCode = 1; };

await send('Page.navigate', { url: 'http://localhost:6123/page/smoke-test' });
await sleep(5000);

// 1. normal mode: all blocks visible, cog present
const cogs = await expr("document.querySelectorAll('.config-open-btn').length");
const cards = await expr("document.querySelectorAll('.chart-card').length");
const blocksBefore = cogs;
check('canvas shows all blocks with cogs', Number(cogs) >= 2, `cogs=${cogs} cards=${cards}`);

// 2. clicking a block does NOT add a ring/border (box-shadow)
await expr("document.querySelector('.config-open-btn').parentElement.click()");
await sleep(600);
const ring = await expr("document.querySelector('.config-open-btn').parentElement.style.boxShadow");
check('no border/ring on block click', !ring, `boxShadow=${ring}`);

// 3. click the cog on the FIRST chart block
const opened = await expr(`(() => {
	const btn = document.querySelector('.config-open-btn');
	if (!btn) return 'no cog';
	btn.click();
	return 'clicked';
})()`);
await sleep(1200);
check('cog opens config UI', opened === 'clicked');

// 4. config view: only ONE block visible, drawer at 50vw
const visibleBlocks = await expr("[...document.querySelectorAll('.chart-card')].filter((b) => b.offsetParent !== null).length");
const chartLeft = await expr("[...document.querySelectorAll('.chart-card')].filter((c) => c.offsetParent !== null).length");
const drawer = await expr(`(() => { const d = document.querySelector('[data-drawer]'); if (!d) return 'none'; const r = d.getBoundingClientRect(); return JSON.stringify({ w: Math.round(r.width), vw: Math.round(innerWidth), right: Math.round(r.right) }); })()`);
check('only the configured chart visible (others hidden)', Number(visibleBlocks) === 1 && Number(chartLeft) === 1, `visible=${visibleBlocks}`);
let dw = '';
try { const d = JSON.parse(drawer); dw = `${d.w}/${d.vw}vw`; check('drawer is 50vw on the right', Math.abs(d.w - d.vw / 2) < 30 && Math.abs(d.right - d.vw) < 30, dw); } catch { check('drawer is 50vw on the right', false, String(drawer)); }

// 5. drawer contains the inspector; edit title -> left chart updates live
const hasInspector = await expr("!!document.querySelector('[data-drawer] input, [data-drawer] select')");
check('drawer hosts the config fields', hasInspector);
const titleInput = await expr(`(() => { const inputs = [...document.querySelectorAll('[data-drawer] input[type=text]')]; const t = inputs[0]; if (!t) return 'no-input'; t.value = 'Renamed chart'; t.dispatchEvent(new Event('input', { bubbles: true })); t.dispatchEvent(new Event('change', { bubbles: true })); return 'set'; })()`);
await sleep(1500);
const cardTitle = await expr("document.querySelector('.chart-card h2')?.textContent");
check('title edit updates the chart live', titleInput === 'set' && /Renamed/.test(String(cardTitle)), String(cardTitle));

// 6. close via X -> all blocks back
await expr("document.querySelector('[data-drawer] .drawer-close')?.click()");
await sleep(1000);
const blocksAfter = await expr("document.querySelectorAll('.config-open-btn').length");
const drawerGone = await expr("!document.querySelector('[data-drawer].drawer-open')");
check('close restores full canvas', Number(blocksAfter) === Number(blocksBefore) && drawerGone, `blocks=${blocksAfter}`);

console.log(results.join('\n'));
process.exit(process.exitCode ?? 0);
