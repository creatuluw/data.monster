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
	const send = (method, params = {}) => new Promise((res) => { const m = ++id; pending.set(m, res); ws.send(JSON.stringify({ id: m, method, params })); });
	return new Promise((res) => (ws.onopen = () => res({ send })));
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

const { send } = await connect((await target()).webSocketDebuggerUrl);
await send('Page.navigate', { url: 'http://localhost:6123/page/smoke-test' });
await sleep(3000);
const expr = async (expression) => (await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true })).result?.result?.value;

// what did the saved doc look like? and what's in the textarea
await expr(`(() => { const b = [...document.querySelectorAll('button')].find((x) => x.textContent.includes('Code')); b?.click(); })()`);
await sleep(500);
const doc = await expr("document.querySelector('textarea')?.value?.slice(0, 600)");
console.log('--- saved doc ---\n', doc);

// back to design, wait, inspect block state
await expr(`(() => { const b = [...document.querySelectorAll('button')].find((x) => x.textContent.includes('Design')); b?.click(); })()`);
await sleep(3500);
const errText = await expr("[...document.querySelectorAll('.chart-card')].map((c) => c.textContent.slice(0, 100)).join(' || ')");
console.log('--- chart cards ---\n', errText);
process.exit(0);
