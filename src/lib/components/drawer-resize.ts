/**
 * Svelte action: left-edge drag handle for right-anchored drawers.
 * Use on the drawer element itself: `<section use:drawerResize>`.
 * Dragging the edge LEFT widens the drawer (it is anchored right):
 * width = startW + (startX - clientX), clamped to [240px, 90vw].
 */
export function drawerResize(node: HTMLElement) {
	const handle = document.createElement('div');
	handle.title = 'Drag to resize';
	Object.assign(handle.style, {
		position: 'absolute',
		top: '0',
		bottom: '0',
		left: '-4px',
		width: '8px',
		cursor: 'col-resize',
		zIndex: '10',
		touchAction: 'none',
		backgroundColor: 'transparent',
		transition: 'background-color 150ms ease'
	} satisfies Partial<CSSStyleDeclaration>);
	handle.addEventListener('pointerenter', () => (handle.style.backgroundColor = 'rgba(0, 0, 0, 0.06)'));
	handle.addEventListener('pointerleave', () => (handle.style.backgroundColor = 'transparent'));
	node.prepend(handle);

	let startX = 0;
	let startW = 0;

	function down(e: PointerEvent) {
		startX = e.clientX;
		startW = node.getBoundingClientRect().width;
		handle.setPointerCapture(e.pointerId);
		document.body.style.cursor = 'col-resize';
		document.body.style.userSelect = 'none';
	}
	function move(e: PointerEvent) {
		if (!handle.hasPointerCapture(e.pointerId)) return;
		// right-anchored: moving the left edge leftwards (smaller clientX) widens
		const w = Math.min(window.innerWidth * 0.9, Math.max(240, startW + (startX - e.clientX)));
		node.style.width = `${w}px`;
	}
	function up(e: PointerEvent) {
		if (!handle.hasPointerCapture(e.pointerId)) return;
		handle.releasePointerCapture(e.pointerId);
		document.body.style.cursor = '';
		document.body.style.userSelect = '';
	}

	handle.addEventListener('pointerdown', down);
	handle.addEventListener('pointermove', move);
	handle.addEventListener('pointerup', up);
	handle.addEventListener('pointercancel', up);

	return {
		destroy() {
			handle.remove();
		}
	};
}
