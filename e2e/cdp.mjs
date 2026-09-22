// CDP driver for data.monster e2e (node >= 21 native WebSocket)
const PORT = 9223;

async function connect() {
	const targets = await (await fetch(`http://127.0.0.1:${PORT}/json`)).json();
	const page = targets.find((t) => t.type === 'page' && /6123|tauri|localhost/.test(t.url));
	if (!page) throw new Error('no page target: ' + JSON.stringify(targets.map((t) => t.url)));
	const ws = new WebSocket(page.webSocketDebuggerUrl);
	await new Promise((res, rej) => {
		ws.onopen = res;
		ws.onerror = rej;
	});
	let id = 0;
	const pending = new Map();
	ws.onmessage = (ev) => {
		const msg = JSON.parse(ev.data);
		if (msg.id && pending.has(msg.id)) {
			const { resolve, reject } = pending.get(msg.id);
			pending.delete(msg.id);
			msg.error ? reject(new Error(JSON.stringify(msg.error))) : resolve(msg.result);
		}
	};
	const send = (method, params = {}) =>
		new Promise((resolve, reject) => {
			const mid = ++id;
			pending.set(mid, { resolve, reject });
			ws.send(JSON.stringify({ id: mid, method, params }));
		});
	await send('Page.enable');
	await send('Runtime.enable');
	await send('Page.bringToFront'); // minimized window throttles the page
	const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

	const evalJs = async (expression) => {
		const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
		if (r.exceptionDetails) throw new Error('page threw: ' + JSON.stringify(r.exceptionDetails.exception?.description || r.exceptionDetails.text));
		return r.result.value;
	};
	const nav = async (path) => {
		await send('Page.navigate', { url: `http://localhost:6123${path}` });
		await sleep(1500);
	};
	const text = () => evalJs('document.body.innerText');
	return { send, evalJs, nav, text, sleep };
}

const [, , cmd, ...args] = process.argv;
const cdp = await connect();
if (cmd === 'nav') {
	await cdp.nav(args[0]);
	console.log('navigated', args[0]);
} else if (cmd === 'text') {
	const t = await cdp.text();
	console.log(t.slice(0, 3000));
} else if (cmd === 'eval') {
	console.log(JSON.stringify(await cdp.evalJs(args[0])));
} else if (cmd === 'run') {
	// run a named step from steps/ dir
	const step = (await import(`./steps/${args[0]}.mjs`)).default;
	await step(cdp);
} else {
	console.log('usage: cdp.mjs nav|text|eval|run ...');
}
process.exit(0);
